use crate::base::expression::Expression;

#[derive(Debug, Clone)]
pub struct Operation {
    argument: Expression,
}

/**
 *  Operations applied on an Expression
 */
impl Operation {
    pub fn new(argument: Expression) -> Self {
        Self { argument: argument }
    }
    pub fn argument(&self) -> Expression {
        self.argument.clone()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for Operation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.argument().hash(state);
    }
}
