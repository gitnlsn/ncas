use crate::base::{associative_operation::AssociativeOperation, expression::Expression};
use crate::manipulation::identifiable::{Identifiable, Identity};
use crate::symbols::number::Number;

#[derive(std::fmt::Debug)]
pub struct Power {
    base: Box<Expression>,
    exp: Box<Expression>,
}

impl Power {
    /**
     *  Builds power associative operation
     *      - ignores neutral exponent
     */
    pub fn new(base: Expression, exp: Expression) -> Expression {
        match &exp {
            Expression::Symbol(s) => {
                if (&exp).id() == Identity::Number {
                    if s.value() == Some(1.0) || s.label().eq(&String::from("1")) {
                        return base.clone();
                    }
                    if s.value() == Some(0.0) || s.label().eq(&String::from("0")) {
                        return Number::new(1.0);
                    }
                }
            }
            _ => {}
        }
        return Expression::AssociativeOperation(Box::new(Self {
            base: Box::new(base),
            exp: Box::new(exp),
        }));
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
