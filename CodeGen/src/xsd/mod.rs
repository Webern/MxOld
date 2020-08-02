#[macro_use]
// has macros, must go first
mod utils;

pub mod annotation;
pub mod attribute;
pub mod attribute_group;
pub mod attributes;
pub mod choice;
pub mod common;
pub mod complex_content;
pub mod complex_type;
pub mod constants;
pub mod element;
pub mod extension;
pub mod group;
pub mod id;
pub mod import;
pub mod list;
pub mod primitives;
pub mod restriction;
pub mod sequence;
pub mod simple_content;
pub mod simple_type;
pub mod union;

use crate::error::Result;
use crate::xsd::annotation::Annotation;
use crate::xsd::attribute_group::AttributeGroup;
use crate::xsd::complex_type::ComplexType;
use crate::xsd::constants::{
    ANNOTATION, ATTRIBUTE_GROUP, BASE, COMPLEX_TYPE, DEFAULT, ELEMENT, FIXED, GROUP, IMPORT, MAX_OCCURS, MIN_OCCURS, NAME, NAMESPACE, REF, REQUIRED,
    SIMPLE_TYPE, TYPE, UNBOUNDED, USE, VALUE,
};
use crate::xsd::element::Element;
use crate::xsd::group::GroupDefinition;
use crate::xsd::id::{Id, Lineage, RootNodeType};
use crate::xsd::import::Import;
use crate::xsd::simple_type::SimpleType;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Xsd {
    entries: Vec<Entry>,
    prefix: String,
}

impl Default for Xsd {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            prefix: "xs".to_owned(),
        }
    }
}

impl Xsd {
    pub fn load<P: AsRef<Path>>(filepath: P) -> Result<Self> {
        let xml_str = wrap!(std::fs::read_to_string(filepath.as_ref()), "unable to load '{}'", filepath.as_ref().display())?;
        let doc = exile::parse(&xml_str).unwrap();
        Self::parse(doc.root())
    }

    pub fn parse(root: &exile::Element) -> Result<Self> {
        if root.name != "schema" {
            return raise!("expected the root node to be named 'schema'");
        }
        let mut prefix = "";
        for (k, v) in root.attributes.map() {
            if v.as_str() == "http://www.w3.org/2001/XMLSchema" {
                if k.starts_with("xmlns:") {
                    let mut split = k.split(':');
                    let _ = split.next().ok_or(make_err!("expected to find xmlns:"))?;
                    let ns: &str = split.next().ok_or(make_err!("expected to find xmlns prefix"))?;
                    prefix = ns;
                    break;
                }
            }
        }
        if prefix.is_empty() {
            return raise!("xmlns prefix is empty");
        }
        let mut xsd = Xsd {
            entries: Vec::new(),
            prefix: prefix.to_owned(),
        };
        for (i, entry_node) in root.children().enumerate() {
            let entry = Entry::from_xml(entry_node, Lineage::Index(i as u64), &xsd)?;
            xsd.add_entry(entry)?;
        }
        Ok(xsd)
    }

    pub fn new<S: AsRef<str>>(prefix: S) -> Self {
        Self {
            entries: Vec::new(),
            prefix: prefix.as_ref().into(),
        }
    }
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    pub fn add_entry(&mut self, entry: Entry) -> Result<()> {
        // TODO - make an efficient storage
        self.entries.push(entry);
        Ok(())
    }

    pub fn find(&self, id: &Id) -> Result<&Entry> {
        // TODO - make an efficient lookup
        for entry in &self.entries {
            if entry.id() == id {
                return Ok(entry);
            }
        }
        raise!("id '{}' not found", id)
    }

    pub fn remove(&mut self, id: &Id) -> Result<Entry> {
        // TODO - efficient removal
        let mut pos = None;
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.id() == id {
                pos = Some(i);
                break;
            }
        }
        if let Some(i) = pos {
            // Note - this can panic, but shouldn't unless a data race occurs.
            Ok(self.entries.remove(i))
        } else {
            raise!("entry '{}' not found", id)
        }
    }

    // TODO - this should be an iterator so the underlying data structure can change.
    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

impl Display for Xsd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for entry in &self.entries {
            writeln!(f, "{}", entry.id())?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Entry {
    Annotation(Annotation),
    AttributeGroup(AttributeGroup),
    ComplexType(ComplexType),
    Element(Element),
    Group(GroupDefinition),
    Import(Import),
    SimpleType(SimpleType),
}

impl Entry {
    pub fn from_xml(node: &exile::Element, lineage: Lineage, xsd: &Xsd) -> Result<Self> {
        let n = node.name.as_str();
        let t = RootNodeType::parse(n)?;
        match t {
            RootNodeType::Annotation => Ok(Entry::Annotation(Annotation::from_xml(node, lineage, xsd)?)),
            RootNodeType::AttributeGroup => Ok(Entry::AttributeGroup(AttributeGroup::from_xml(node, lineage, xsd)?)),
            RootNodeType::ComplexType => Ok(Entry::ComplexType(ComplexType::from_xml(node, lineage, xsd)?)),
            RootNodeType::Element => Ok(Entry::Element(Element::from_xml(node, lineage, xsd)?)),
            RootNodeType::Group => Ok(Entry::Group(GroupDefinition::from_xml(node, lineage, xsd)?)),
            RootNodeType::Import => Ok(Entry::Import(Import::from_xml(node, lineage, xsd)?)),
            RootNodeType::SimpleType => Ok(Entry::SimpleType(SimpleType::from_xml(node, lineage, xsd)?)),
        }
    }

    pub fn id(&self) -> &Id {
        match self {
            Entry::Annotation(x) => &x.id,
            Entry::AttributeGroup(x) => x.id(),
            Entry::ComplexType(x) => &x.id,
            Entry::Element(x) => x.id(),
            Entry::Group(x) => &x.id,
            Entry::Import(x) => &x.id,
            Entry::SimpleType(x) => &x.id,
        }
    }

    pub fn documentation(&self) -> String {
        match self {
            Entry::Annotation(x) => x.documentation(),
            Entry::AttributeGroup(x) => x.documentation(),
            Entry::ComplexType(x) => x.documentation(),
            Entry::Element(x) => x.documentation(),
            Entry::Group(x) => x.documentation(),
            Entry::Import(x) => x.documentation(),
            Entry::SimpleType(x) => x.documentation(),
        }
    }
}

pub(crate) fn get_attribute<S: AsRef<str>>(node: &exile::Element, attribute_name: S) -> Result<String> {
    Ok(node
        .attributes
        .map()
        .get(attribute_name.as_ref())
        .ok_or(make_err!("'{}' attribute not found in '{}' node", attribute_name.as_ref(), node.name.as_str()))?
        .clone())
}

pub(crate) fn name_attribute(node: &exile::Element) -> Result<String> {
    get_attribute(node, NAME)
}

pub(crate) fn namespace_attribute(node: &exile::Element) -> Result<String> {
    get_attribute(node, NAMESPACE)
}

pub(crate) fn value_attribute(node: &exile::Element) -> Result<String> {
    get_attribute(node, VALUE)
}

pub(crate) fn ref_attribute(node: &exile::Element) -> Result<String> {
    get_attribute(node, REF)
}

pub(crate) fn type_attribute(node: &exile::Element) -> Result<String> {
    get_attribute(node, TYPE)
}

pub(crate) fn use_required(node: &exile::Element) -> bool {
    match get_attribute(node, USE) {
        Ok(val) => val.as_str() == REQUIRED,
        Err(_) => false,
    }
}

pub(crate) fn default_attribute(node: &exile::Element) -> Option<String> {
    node.attributes.map().get(DEFAULT).cloned()
}

pub(crate) fn fixed_attribute(node: &exile::Element) -> Option<String> {
    node.attributes.map().get(FIXED).cloned()
}

pub(crate) fn is_ref(node: &exile::Element) -> bool {
    node.attributes.map().get(REF).is_some()
}

pub(crate) fn base_attribute(node: &exile::Element) -> Result<String> {
    get_attribute(node, BASE)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Occurs {
    pub min_occurs: u64,

    /// None means `unbounded`
    pub max_occurs: Option<u64>,
}

impl Default for Occurs {
    fn default() -> Self {
        Self {
            min_occurs: 1,
            max_occurs: Some(1),
        }
    }
}

impl Occurs {
    pub fn from_xml(node: &exile::Element) -> Result<Occurs> {
        Ok(Self::from_map(node.attributes.map())?)
    }

    pub fn from_map(map: &BTreeMap<String, String>) -> Result<Occurs> {
        let min_occurs: u64 = if let Some(sval) = map.get(MIN_OCCURS) {
            wrap!(sval.parse::<u64>())?
        } else {
            1
        };

        let max_occurs: Option<u64> = if let Some(sval) = map.get(MAX_OCCURS) {
            if sval.as_str() == UNBOUNDED {
                None
            } else {
                Some(wrap!(sval.parse::<u64>())?)
            }
        } else {
            Some(1)
        };
        if let Some(the_max) = max_occurs {
            if min_occurs > the_max {
                return raise!(
                    "{} cannot be greater than {}, in this case {} is {} and {} is {}",
                    MIN_OCCURS,
                    MAX_OCCURS,
                    MIN_OCCURS,
                    min_occurs,
                    MAX_OCCURS,
                    the_max
                );
            }
        }
        Ok(Self { min_occurs, max_occurs })
    }
}

#[test]
fn parse_occurs() {
    let test_cases = vec![
        (
            r#"<xyz minOccurs="1"/>"#,
            Occurs {
                min_occurs: 1,
                max_occurs: Some(1),
            },
        ),
        (
            r#"<xyz maxOccurs="unbounded"/>"#,
            Occurs {
                min_occurs: 1,
                max_occurs: None,
            },
        ),
        (
            r#"<xyz/>"#,
            Occurs {
                min_occurs: 1,
                max_occurs: Some(1),
            },
        ),
        (
            r#"<xyz minOccurs="2" maxOccurs="3"/>"#,
            Occurs {
                min_occurs: 2,
                max_occurs: Some(3),
            },
        ),
    ];

    for (xml, want) in test_cases {
        let doc = exile::parse(xml).unwrap();
        let got = Occurs::from_xml(doc.root()).unwrap();
        assert_eq!(got, want)
    }
}

#[test]
fn parse_occurs_err() {
    let test_cases = vec![r#"<xyz minOccurs="10" maxOccurs="1"/>"#, r#"<xyz maxOccurs="unexpectedString"/>"#];

    for xml in test_cases {
        let doc = exile::parse(xml).unwrap();
        assert!(Occurs::from_xml(doc.root()).is_err());
    }
}
