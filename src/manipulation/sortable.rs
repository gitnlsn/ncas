use crate::base::expression::Expression;

pub trait Sortable {
    fn sort(&self) -> Result<Expression, Expression>;
}
