use std::fmt::Display;

use prefixmap::IriRef;
use srdf::{lang::Lang, literal::Literal, RDFNode};

use crate::{node_kind::NodeKind, value::Value};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Component {
    Class(RDFNode),
    Datatype(IriRef),
    NodeKind(NodeKind),
    MinCount(isize),
    MaxCount(isize),
    MinExclusive(Literal),
    MaxExclusive(Literal),
    MinInclusive(Literal),
    MaxInclusive(Literal),
    MinLength(isize),
    MaxLength(isize),
    Pattern {
        pattern: String,
        flags: Option<String>,
    },
    UniqueLang(bool),
    LanguageIn {
        langs: Vec<Lang>,
    },
    Equals(IriRef),
    Disjoint(IriRef),
    LessThan(IriRef),
    LessThanOrEquals(IriRef),
    Or {
        shapes: Vec<RDFNode>,
    },
    And {
        shapes: Vec<RDFNode>,
    },
    Not {
        shape: RDFNode,
    },
    Xone {
        shapes: Vec<RDFNode>,
    },
    Closed {
        is_closed: bool,
        ignored_properties: Vec<IriRef>,
    },
    Node {
        shape: RDFNode,
    },
    HasValue {
        value: Value,
    },
    In {
        values: Vec<Value>,
    },
    QualifiedValueShape {
        shape: RDFNode,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
    },
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Component::Class(cls) => write!(f, "class({cls})"),
            Component::Datatype(dt) => write!(f, "datatype({dt})"),
            Component::NodeKind(nk) => write!(f, "nodeKind({nk})"),
            Component::MinCount(mc) => write!(f, "minCount({mc})"),
            Component::MaxCount(mc) => write!(f, "maxCount({mc})"),
            Component::MinExclusive(me) => write!(f, "minExclusive({me})"),
            Component::MaxExclusive(me) => write!(f, "maxExclusive({me})"),
            Component::MinInclusive(mi) => write!(f, "minInclusive({mi})"),
            Component::MaxInclusive(mi) => write!(f, "maxInclusive({mi})"),
            Component::MinLength(ml) => write!(f, "minLength({ml})"),
            Component::MaxLength(ml) => write!(f, "maxLength({ml})"),
            Component::Pattern { pattern, flags } => match flags {
                Some(flags) => write!(f, "pattern({pattern}, {flags})"),
                None => write!(f, "pattern({pattern})"),
            },
            Component::UniqueLang(ul) => write!(f, "uniqueLang({ul})"),
            Component::LanguageIn { .. } => todo!(), // write!(f, "languageIn({langs})"),
            Component::Equals(e) => write!(f, "equals({e})"),
            Component::Disjoint(d) => write!(f, "disjoint({d})"),
            Component::LessThan(lt) => write!(f, "uniqueLang({lt})"),
            Component::LessThanOrEquals(lte) => write!(f, "uniqueLang({lte})"),
            Component::Or { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "or [{str}]")
            }
            Component::And { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "and [{str}]")
            }
            Component::Not { shape } => {
                write!(f, "not [{shape}]")
            }
            Component::Xone { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "xone [{str}]")
            }
            Component::Closed { .. } => todo!(),
            Component::Node { shape } => write!(f, "node({shape})"),
            Component::HasValue { value } => write!(f, "hasValue({value})"),
            Component::In { values } => {
                let str = values.iter().map(|v| v.to_string()).join(" ");
                write!(f, "In [{str}]")
            }
            Component::QualifiedValueShape { .. } => todo!(),
        }
    }
}
