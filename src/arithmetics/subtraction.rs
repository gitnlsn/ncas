use crate::{
    arithmetics::{addition::Addition, multiplication::Multiplication},
    base::expression::Expression,
    symbols::number::Number,
};

#[derive(std::fmt::Debug)]
pub struct Subtraction {}

impl Subtraction {
    pub fn new(left_hand_side: Expression, right_hand_side: Expression) -> Expression {
        Addition::new(vec![
            left_hand_side,
            Multiplication::new(vec![Number::new(-1.0), right_hand_side]),
        ])
    }
}

/**
 * Overloads minus (-) Operation
 */
impl std::ops::Sub for Expression {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Subtraction::new(self, other)
    }
}

impl std::ops::Sub<&Expression> for Expression {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Subtraction::new(self, other.clone())
    }
}

impl std::ops::Sub<&Expression> for &Expression {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Subtraction::new(self.clone(), other.clone())
    }
}

impl std::ops::Sub<Expression> for &Expression {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Subtraction::new(self.clone(), other)
    }
}
