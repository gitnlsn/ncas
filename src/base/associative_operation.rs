use crate::base::expression::Expression;
use crate::manipulation::{
    differentiate::Differentiable, expand::Expandable, identifiable::Identifiable,
    numeric_evaluation::NumericEvaluable, simplifiable::Simplifiable,
};

use std::fmt::{Debug, Display};

/**
 * Operations applied on an Expression, given an Expresssion as parameter
 */
pub trait AssociativeOperation:
    Debug + Display + NumericEvaluable + Expandable + Identifiable + Simplifiable
{
    fn argument(&self) -> &Box<Expression>;
    fn modifier(&self) -> &Box<Expression>;
    fn boxed_clone(&self) -> Box<dyn AssociativeOperation>;
}

impl Clone for Box<dyn AssociativeOperation> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for dyn AssociativeOperation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
        self.argument().hash(state);
        self.modifier().hash(state);
    }
}
