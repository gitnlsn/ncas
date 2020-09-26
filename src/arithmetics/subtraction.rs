use crate::base::{association::Association, expression::Expression};

#[derive(std::fmt::Debug)]
pub struct Subtraction {
    right_hand_side: Box<Expression>,
    left_hand_side: Box<Expression>,
}

impl Subtraction {
    pub fn new(left_hand_side: Expression, right_hand_side: Expression) -> Expression {
        Expression::Association(Box::new(Self {
            left_hand_side: Box::new(left_hand_side),
            right_hand_side: Box::new(right_hand_side),
        }))
    }
}

impl Association for Subtraction {
    fn right_hand_side(&self) -> &Box<Expression> {
        &self.right_hand_side
    }
    fn left_hand_side(&self) -> &Box<Expression> {
        &self.left_hand_side
    }
    fn boxed_clone(&self) -> Box<dyn Association> {
        Box::new(Self {
            left_hand_side: self.left_hand_side.clone(),
            right_hand_side: self.right_hand_side.clone(),
        })
    }
}

/**
 * Overloads minus (-) Operation
 */
impl std::ops::Sub for Expression {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Expression::Association(Box::new(Subtraction {
            left_hand_side: Box::new(self),
            right_hand_side: Box::new(other),
        }))
    }
}

impl std::ops::Sub<&Expression> for Expression {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Expression::Association(Box::new(Subtraction {
            left_hand_side: Box::new(self),
            right_hand_side: Box::new(other.clone()),
        }))
    }
}

impl std::ops::Sub<&Expression> for &Expression {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Expression::Association(Box::new(Subtraction {
            left_hand_side: Box::new(self.clone()),
            right_hand_side: Box::new(other.clone()),
        }))
    }
}

impl std::ops::Sub<Expression> for &Expression {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Expression::Association(Box::new(Subtraction {
            left_hand_side: Box::new(self.clone()),
            right_hand_side: Box::new(other),
        }))
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Subtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} - {})", self.left_hand_side, self.right_hand_side)
    }
}
