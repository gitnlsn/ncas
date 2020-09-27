use crate::base::expression::Expression;
use crate::manipulation::{
    differentiate::Differentiable,
    expand::Expandable,
    identifiable::{Identifiable, Identity},
    numeric_evaluation::NumericEvaluable,
};
use std::fmt::{Debug, Display};

/**
 *  Operations applied on an Expression
 */
pub trait Operation /* Debug + Display + Evaluable + Expandable */ /* + Simplifiable + Sortable */ {
    fn argument(&self) -> &Box<Expression>;
}
