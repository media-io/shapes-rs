use srdf::{RDFNode, SHACLPath};
use std::fmt::Display;

use crate::{component::Component, target::Target};

#[derive(Debug, Clone)]
pub struct PropertyShape {
    // id: RDFNode,
    path: SHACLPath,
    components: Vec<Component>,
    targets: Vec<Target>,
    property_shapes: Vec<RDFNode>,
    closed: bool,
    // ignored_properties: Vec<IriRef>,
    // deactivated: bool,
    // message: MessageMap,
    // severity: Option<Severity>,
    // name: MessageMap,
    // description: MessageMap,

    // SHACL spec says that the values of sh:order should be decimals but in the examples they use integers. `NumericLiteral` also includes doubles.
    // order: Option<NumericLiteral>,

    // group: Option<RDFNode>,
    // source_iri: Option<IriRef>,
    // annotations: Vec<(IriRef, RDFNode)>,
}

impl PropertyShape {
    pub fn new(_id: RDFNode, path: SHACLPath) -> Self {
        PropertyShape {
            // id,
            path,
            components: Vec::new(),
            targets: Vec::new(),
            property_shapes: Vec::new(),
            closed: false,
            // ignored_properties: Vec::new(),
            // deactivated: false,
            // message: MessageMap::new(),
            // severity: None,
            // name: MessageMap::new(),
            // description: MessageMap::new(),
            // order: None,
            // group: None,
            // source_iri: None,
            // annotations: Vec::new()
        }
    }

    pub fn with_targets(mut self, targets: Vec<Target>) -> Self {
        self.targets = targets;
        self
    }

    pub fn with_property_shapes(mut self, property_shapes: Vec<RDFNode>) -> Self {
        self.property_shapes = property_shapes;
        self
    }

    pub fn with_components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    pub fn with_closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }
}

impl Display for PropertyShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "       PropertyShape")?;
        writeln!(f, "       path: {}", self.path)?;
        for target in self.targets.iter() {
            writeln!(f, "       {target}")?
        }
        if self.closed {
            writeln!(f, "       closed: {}", self.closed)?
        }
        for property in self.property_shapes.iter() {
            writeln!(f, "       Property {property}")?
        }
        for component in self.components.iter() {
            writeln!(f, "       {component}")?
        }
        write!(f, "}}")?;
        Ok(())
    }
}
