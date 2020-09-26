use crate::base::expression::Expression;
use crate::manipulation::{
    differentiate::Differentiable, expand::Expandable, expression_measure::ExpressionMeasure,
    identifiable::Identifiable, numeric_evaluation::NumericEvaluable,
};

use std::fmt::{Debug, Display};

/**
 *  Associations between several Expressions.
 *  Elements must satisfy:
 *      - associativity
 *      - commuativity
 */
pub trait CommutativeAssociation:
    Debug + Display + NumericEvaluable + Expandable + ExpressionMeasure + Identifiable
/* + Simplifiable + Sortable */
{
    fn items(&self) -> Vec<Expression>;
    fn boxed_clone(&self) -> Box<dyn CommutativeAssociation>;
}

impl Clone for Box<dyn CommutativeAssociation> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
    }
}
