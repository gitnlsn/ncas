use crate::base::expression::{Association, Expression};

#[derive(std::fmt::Debug)]
pub struct Addition {
    right_hand_side: Box<Expression>,
    left_hand_side: Box<Expression>,
}

impl Addition {
    pub fn new(left_hand_side: Expression, right_hand_side: Expression) -> Expression {
        Expression::Association(Box::new(Self {
            left_hand_side: Box::new(left_hand_side),
            right_hand_side: Box::new(right_hand_side),
        }))
    }
}

impl Association for Addition {
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
 * Overloads plus (+) Operation
 */
impl std::ops::Add for Expression {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Expression::Association(Box::new(Addition {
            left_hand_side: Box::new(self),
            right_hand_side: Box::new(other),
        }))
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Addition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.left_hand_side, self.right_hand_side)
    }
}
