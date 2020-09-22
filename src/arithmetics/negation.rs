use crate::base::expression::Expression;
use crate::base::symbols::number::Number;

impl std::ops::Neg for Expression {
    type Output = Expression;
    fn neg(self) -> Self {
        Number::new(-1.0) * self
    }
}
