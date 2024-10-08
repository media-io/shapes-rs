use prefixmap::IriRef;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Disjoint {
    iri_ref: IriRef,
}

impl Disjoint {
    pub fn new(iri_ref: IriRef) -> Self {
        Disjoint { iri_ref }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Disjoint {
    fn evaluate_default(
        &self,
        _validation_context: &ValidationContext<S>,
        _evaluation_context: EvaluationContext,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Disjoint {
    fn evaluate_sparql(
        &self,
        _validation_context: &ValidationContext<S>,
        _evaluation_context: EvaluationContext,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
