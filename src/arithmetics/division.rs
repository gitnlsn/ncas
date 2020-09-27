use crate::{
    arithmetics::multiplication::Multiplication, base::expression::Expression,
    exponential::power::Power, symbols::number::Number,
};

#[derive(std::fmt::Debug)]
pub struct Division {}

impl Division {
    pub fn new(left_hand_side: Expression, right_hand_side: Expression) -> Expression {
        Multiplication::new(vec![
            left_hand_side,
            Power::new(right_hand_side, Number::new(-1.0)),
        ])
    }
}

/**
 * Overloads plus (/) Operation
 */
impl std::ops::Div for Expression {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Division::new(self, other)
    }
}

impl std::ops::Div<&Expression> for Expression {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        Division::new(self, other.clone())
    }
}

impl std::ops::Div<&Expression> for &Expression {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        Division::new(self.clone(), other.clone())
    }
}

impl std::ops::Div<Expression> for &Expression {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Division::new(self.clone(), other)
    }
}
