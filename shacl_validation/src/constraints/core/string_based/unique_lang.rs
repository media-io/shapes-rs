use std::cell::RefCell;
use std::rc::Rc;

use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
pub(crate) struct UniqueLang {
    unique_lang: bool,
}

impl UniqueLang {
    pub fn new(unique_lang: bool) -> Self {
        UniqueLang { unique_lang }
    }
}

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for UniqueLang {
    fn evaluate(
        &self,
        _validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        if !self.unique_lang {
            return Ok(ValidationResults::default());
        }

        let langs = Rc::new(RefCell::new(Vec::new()));

        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                let langs = Rc::clone(&langs);
                let mut langs = langs.borrow_mut();

                if let Some(literal) = S::term_as_literal(value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if langs.contains(&lang) {
                            Some(ValidationResult::new(
                                focus_node,
                                &evaluation_context,
                                Some(value_node),
                            ))
                        } else {
                            langs.push(lang);
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for UniqueLang {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for UniqueLang {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
