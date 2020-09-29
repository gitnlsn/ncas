use crate::base::expression::Expression;
use crate::manipulation::{
    differentiate::Differentiable, expand::Expandable, identifiable::Identifiable,
    numeric_evaluation::NumericEvaluable, simplifiable::Simplifiable,
};

use std::fmt::{Debug, Display};

/**
 *  Associations between several Expressions.
 *  Elements must satisfy:
 *      - associativity
 *      - commuativity
 */
pub trait CommutativeAssociation:
    Debug + Display + NumericEvaluable + Expandable + Identifiable + Simplifiable
{
    fn items(&self) -> Vec<Expression>;
    fn boxed_clone(&self) -> Box<dyn CommutativeAssociation>;
}

impl Clone for Box<dyn CommutativeAssociation> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for dyn CommutativeAssociation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
        for item in self.items().iter() {
            item.hash(state);
        }
    }
}
