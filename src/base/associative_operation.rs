use crate::base::expression::Expression;

#[derive(Debug, Clone)]
pub struct AssociativeOperation {
    argument: Box<Expression>,
    modifier: Box<Expression>,
}

/**
 * Operations applied on an Expression, given an Expresssion as parameter
 */
impl AssociativeOperation {
    pub fn new(argument: Expression, modifier: Expression) -> Self {
        Self {
            argument: Box::new(argument),
            modifier: Box::new(modifier),
        }
    }
    pub fn argument(&self) -> Expression {
        self.argument.as_ref().clone()
    }
    pub fn modifier(&self) -> Expression {
        self.modifier.as_ref().clone()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for AssociativeOperation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.argument().hash(state);
        self.modifier().hash(state);
    }
}
