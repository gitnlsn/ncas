use crate::base::{associative_operation::AssociativeOperation, expression::Expression};

#[derive(std::fmt::Debug)]
pub struct Power {
    base: Box<Expression>,
    exp: Box<Expression>,
}

impl Power {
    pub fn new(base: Expression, exp: Expression) -> Expression {
        Expression::AssociativeOperation(Box::new(Self {
            base: Box::new(base),
            exp: Box::new(exp),
        }))
    }
}

impl AssociativeOperation for Power {
    fn argument(&self) -> &Box<Expression> {
        &self.base
    }
    fn modifier(&self) -> &Box<Expression> {
        &self.exp
    }
    fn boxed_clone(&self) -> Box<dyn AssociativeOperation> {
        Box::new(Self {
            base: self.base.clone(),
            exp: self.exp.clone(),
        })
    }
}

/**
 * Overloads plus (+) Operation
 */
impl std::ops::BitXor for Expression {
    type Output = Expression;
    fn bitxor(self, other: Expression) -> Expression {
        Power::new(self, other)
    }
}

impl std::ops::BitXor<&Expression> for Expression {
    type Output = Expression;
    fn bitxor(self, other: &Expression) -> Expression {
        Power::new(self, other.clone())
    }
}

impl std::ops::BitXor<&Expression> for &Expression {
    type Output = Expression;
    fn bitxor(self, other: &Expression) -> Expression {
        Power::new(self.clone(), other.clone())
    }
}

impl std::ops::BitXor<Expression> for &Expression {
    type Output = Expression;
    fn bitxor(self, other: Expression) -> Expression {
        Power::new(self.clone(), other.clone())
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ^ {})", self.base, self.exp)
    }
}
