use crate::error::Result;
use crate::xsd::annotation::Annotation;
use crate::xsd::annotation::Item::Documentation;
use crate::xsd::{
    name_attribute, ref_attribute, type_attribute, use_required, EntryType, ANNOTATION,
    ATTRIBUTE_GROUP, ID,
};
use std::convert::TryInto;

pub const ATTRIBUTE: &str = "attribute";

pub struct AttributeGroup {
    pub id: ID,
    pub index: u64,
    pub annotation: Option<Annotation>,
    pub members: Vec<Member>,
}

pub struct AttributeType {
    pub id: ID,
    pub index: u64,
    pub name: String,
    pub type_: String,
    pub required: bool,
}

pub struct AttributeRef {
    pub id: ID,
    pub index: u64,
    pub ref_: String,
    pub required: bool,
    pub default: Option<String>,
    pub fixed: Option<String>,
}

pub struct AttributeGroupRef {
    pub id: ID,
    pub index: u64,
    pub ref_: String,
}

pub enum Member {
    AttributeType(AttributeType),
    AttributeRef(AttributeRef),
    AttributeGroupRef(AttributeGroupRef),
}

impl AttributeGroup {
    pub fn documentation(&self) -> String {
        if let Some(annotation) = &self.annotation {
            return annotation.documentation();
        }
        return "".to_owned();
    }

    pub fn from_xml(node: &exile::Element, index: u64) -> Result<Self> {
        if node.name.as_str() != ATTRIBUTE_GROUP {
            return raise!(
                "expected '{}', got '{}'",
                ATTRIBUTE_GROUP,
                node.name.as_str()
            );
        }
        let mut annotation = None;
        let mut members = Vec::new();
        for inner in node.children() {
            let t = inner.name.as_str();
            match t {
                ANNOTATION => {
                    annotation = Some(Annotation::from_xml(inner, index)?);
                }
                ATTRIBUTE => members.push(parse_attribute(inner, index)?),
                ATTRIBUTE_GROUP => {
                    let name = ref_attribute(inner)?;
                    members.push(Member::AttributeGroupRef(AttributeGroupRef {
                        id: ID {
                            entry_type: EntryType::Other(ATTRIBUTE.into()),
                            name: name.clone(),
                        },
                        index,
                        ref_: name,
                    }))
                }
                _ => return raise!("unexpected node '{}'", t),
            }
        }
        let id = ID {
            entry_type: EntryType::AttributeGroup,
            name: name_attribute(node)?,
        };
        Ok(AttributeGroup {
            id,
            index,
            annotation,
            members,
        })
    }
}

fn parse_attribute(inner: &exile::Element, index: u64) -> Result<Member> {
    // TODO parse either AttributeType or AttributeRef
    let name = name_attribute(inner)?;
    Ok(Member::AttributeType(AttributeType {
        id: ID {
            entry_type: EntryType::Other(ATTRIBUTE.into()),
            name: name.clone(),
        },
        index,
        name,
        type_: type_attribute(inner)?,
        required: use_required(inner),
    }))
}

fn parse_attribute_type(inner: &exile::Element, index: u64) -> Result<AttributeType> {
    let name = name_attribute(inner)?;
}

#[test]
fn parse_attribute_type_test() {
    let xml_str = r#"
    	<xs:attributeGroup name="bend-sound">
		<xs:annotation>
			<xs:documentation>blargh</xs:documentation>
		</xs:annotation>
		<xs:attribute name="accelerate" type="yes-no"/>
		<xs:attribute name="beats" type="trill-beats" use="required"/>
		<xs:attribute name="first-beat" type="percent"/>
		<xs:attribute name="last-beat" type="percent"/>
	</xs:attributeGroup>"#;
    let doc = exile::parse(xml_str).unwrap();
    let xml = doc.root();
    let want_index: u64 = 3;
    let ag = AttributeGroup::from_xml(&xml, want_index).unwrap();
    assert_eq!(ag.index, want_index);
    let got_id = ag.id.to_string();
    let want_id = "bend-sound (attributeGroup)".to_owned();
    assert_eq!(got_id, want_id);
    let got_doc = ag.documentation();
    let want_doc = "blargh";
    assert_eq!(got_doc, want_doc);
    assert_eq!(ag.members.len(), 4);
    let member = ag.members.get(0).unwrap();
    match member {
        Member::AttributeType(ar) => {
            assert_eq!(ar.name.as_str(), "accelerate");
            assert_eq!(ar.type_.as_str(), "yes-no");
            assert!(!ar.required);
        }
        _ => panic!("expected 'AttributeRef'"),
    }
    let member = ag.members.get(1).unwrap();
    match member {
        Member::AttributeType(ar) => {
            assert_eq!(ar.name.as_str(), "beats");
            assert_eq!(ar.type_.as_str(), "trill-beats");
            assert!(ar.required);
        }
        _ => panic!("expected 'AttributeRef'"),
    }
}

#[test]
fn parse_group_ref() {
    let xml_str = r#"
	<xs:attributeGroup name="image-attributes">
		<xs:annotation>
			<xs:documentation>flerbin</xs:documentation>
		</xs:annotation>
		<xs:attribute name="source" type="xs:anyURI" use="required"/>
		<xs:attribute name="type" type="xs:token" use="required"/>
		<xs:attributeGroup ref="position"/>
		<xs:attributeGroup ref="halign"/>
		<xs:attributeGroup ref="valign-image"/>
	</xs:attributeGroup>"#;
    let doc = exile::parse(xml_str).unwrap();
    let xml = doc.root();
    let want_index: u64 = 3;
    let ag = AttributeGroup::from_xml(&xml, want_index).unwrap();
    assert_eq!(ag.index, want_index);
    let got_id = ag.id.to_string();
    let want_id = "image-attributes (attributeGroup)".to_owned();
    assert_eq!(got_id, want_id);
    let got_doc = ag.documentation();
    let want_doc = "flerbin";
    assert_eq!(got_doc, want_doc);
    assert_eq!(ag.members.len(), 5);
    let member = ag.members.get(0).unwrap();
    match member {
        Member::AttributeType(ar) => {
            assert_eq!(ar.name.as_str(), "source");
            assert_eq!(ar.type_.as_str(), "xs:anyURI");
            assert!(ar.required);
        }
        _ => panic!("expected 'AttributeRef'"),
    }
    let member = ag.members.get(2).unwrap();
    match member {
        Member::AttributeGroupRef(agr) => {
            assert_eq!(agr.ref_.as_str(), "position");
        }
        _ => panic!("expected 'AttributeGroupRef'"),
    }
}

#[test]
fn parse_attribute_ref() {
    let xml_str = r#"
	<xs:attributeGroup name="link-attributes">
		<xs:annotation>
			<xs:documentation>The link-attributes group includes all the simple XLink attributes supported in the MusicXML format.</xs:documentation>
		</xs:annotation>
		<!--<xs:attribute ref="xmnls:xlink" fixed="http://www.w3.org/1999/xlink"/>-->
		<xs:attribute ref="xlink:href" use="required"/>
		<xs:attribute ref="xlink:type" fixed="simple"/>
		<xs:attribute ref="xlink:role"/>
		<xs:attribute ref="xlink:title"/>
		<xs:attribute ref="xlink:show" default="replace"/>
		<xs:attribute ref="xlink:actuate" default="onRequest"/>
	</xs:attributeGroup>"#;
    let doc = exile::parse(xml_str).unwrap();
    let xml = doc.root();
    let want_index: u64 = 3;
    let ag = AttributeGroup::from_xml(&xml, want_index).unwrap();
    assert_eq!(ag.index, want_index);
    let got_id = ag.id.to_string();
    let want_id = "image-attributes (attributeGroup)".to_owned();
    assert_eq!(got_id, want_id);
    let got_doc = ag.documentation();
    let want_doc = "flerbin";
    assert_eq!(got_doc, want_doc);
    assert_eq!(ag.members.len(), 5);
    let member = ag.members.get(0).unwrap();
    match member {
        Member::AttributeType(ar) => {
            assert_eq!(ar.name.as_str(), "source");
            assert_eq!(ar.type_.as_str(), "xs:anyURI");
            assert!(ar.required);
        }
        _ => panic!("expected 'AttributeRef'"),
    }
    let member = ag.members.get(2).unwrap();
    match member {
        Member::AttributeGroupRef(agr) => {
            assert_eq!(agr.ref_.as_str(), "position");
        }
        _ => panic!("expected 'AttributeGroupRef'"),
    }
}