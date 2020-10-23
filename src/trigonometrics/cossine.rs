use crate::base::{expression::Expression, operation::Operation, symbol::Symbol};

impl Expression {
    pub fn cos(angle: Expression) -> Expression {
        return Expression::Cossine(Operation::new(angle));
    }
}
