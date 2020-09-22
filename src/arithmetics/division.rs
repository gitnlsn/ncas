use crate::base::expression::{Association, Expression};

use crate::manipulation::{differentiate::Differentiable, evaluate::Evaluable};

use std::cell::RefCell;

#[derive(std::fmt::Debug)]
pub struct Division {
    right_hand_side: RefCell<Expression>,
    left_hand_side: RefCell<Expression>,
}

impl Association for Division {
    fn right_hand_side(&self) -> &RefCell<Expression> {
        &self.right_hand_side
    }
    fn left_hand_side(&self) -> &RefCell<Expression> {
        &self.left_hand_side
    }
}

impl Evaluable for Division {
    fn evaluate(&mut self) -> Result<f64, Expression> {
        let possible_lhs_value = match self.left_hand_side.get_mut() {
            Expression::Association(association) => association.evaluate(),
            Expression::Symbol(symbol) => symbol.evaluate(),
        };

        let possible_rhs_value = match self.right_hand_side.get_mut() {
            Expression::Association(association) => association.evaluate(),
            Expression::Symbol(symbol) => symbol.evaluate(),
        };

        if possible_lhs_value.is_ok() && possible_rhs_value.is_ok() {
            return Ok(possible_lhs_value.unwrap() / possible_rhs_value.unwrap());
        }

        if possible_lhs_value.is_err() {
            return possible_lhs_value;
        } else {
            return possible_rhs_value;
        }
    }
}

/**
 * Overloads plus (/) Operation
 */
impl std::ops::Div for Expression {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Expression::Association(Box::new(Division {
            left_hand_side: RefCell::new(self),
            right_hand_side: RefCell::new(other),
        }))
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Division {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} / {}",
            self.left_hand_side.borrow(),
            self.right_hand_side.borrow()
        )
    }
}
