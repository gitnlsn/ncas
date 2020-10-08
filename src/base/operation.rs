use crate::base::expression::Expression;
use crate::manipulation::{
    differentiate::Differentiable, expand::Expandable, identifiable::Identifiable,
    numeric_evaluation::NumericEvaluable, pattern_matchable::PatternMatchable,
    simplifiable::Simplifiable,
};

use std::fmt::{Debug, Display};

/**
 *  Operations applied on an Expression
 */
pub trait Operation:
    Debug + Display + NumericEvaluable + Expandable + Identifiable + Simplifiable + PatternMatchable
{
    fn argument(&self) -> &Box<Expression>;
    fn boxed_clone(&self) -> Box<dyn Operation>;
}

impl Clone for Box<dyn Operation> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for dyn Operation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
        self.argument().hash(state);
    }
}
