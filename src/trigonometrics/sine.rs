use crate::base::{expression::Expression, operation::Operation, symbol::Symbol};

impl Expression {
    pub fn sin(angle: Expression) -> Expression {
        return Expression::Sine(Operation::new(angle));
    }
}
