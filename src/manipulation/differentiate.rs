use crate::base::expression::Expression;

pub trait Differentiable {
    fn differentiate(&self) -> Result<Expression, Expression>;
}