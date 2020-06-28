use crate::error::Result;
use crate::xsd::annotation::Annotation;
use crate::xsd::annotation::Item::Documentation;
use crate::xsd::{name_attribute, EntryType, ANNOTATION, GROUP, ID};
use std::convert::TryInto;

pub struct Group {
    pub id: ID,
    pub index: u64,
    pub annotation: Option<Annotation>,
}

impl Group {
    pub fn documentation(&self) -> String {
        if let Some(annotation) = &self.annotation {
            return annotation.documentation();
        }
        return "".to_owned();
    }

    pub fn from_xml(node: &exile::Element, index: u64) -> Result<Self> {
        if node.name.as_str() != GROUP {
            return raise!("expected '{}', got '{}'", GROUP, node.name.as_str());
        }
        let mut annotation = None;
        for inner in node.children() {
            let t = inner.name.as_str();
            if t == ANNOTATION {
                annotation = Some(Annotation::from_xml(inner, index)?);
                break;
            }
        }
        let id = ID {
            entry_type: EntryType::Group,
            name: name_attribute(node)?,
        };
        Ok(Group {
            id,
            index,
            annotation,
        })
    }
}

#[test]
fn parse() {
    unimplemented!();
}