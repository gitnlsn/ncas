use crate::{
    arithmetics::{addition::Addition, multiplication::Multiplication},
    base::expression::Expression,
    manipulation::identifiable::{Identifiable, Identity},
    symbols::number::Number,
};

#[derive(std::fmt::Debug)]
pub struct Subtraction {}

impl Subtraction {
    pub fn new(left_hand_side: Expression, right_hand_side: Expression) -> Expression {
        let mut addends: Vec<Expression> = Vec::new();
        match left_hand_side.id() {
            Identity::Number => {
                if let Expression::Symbol(number) = left_hand_side.clone() {
                    if number.value() != Some(0.0) && !number.label().eq(&String::from("0")) {
                        addends.push(left_hand_side.clone());
                    }
                }
            }
            _ => {
                addends.push(left_hand_side.clone());
            }
        }
        match right_hand_side.id() {
            Identity::Number => {
                if let Expression::Symbol(number) = right_hand_side.clone() {
                    if number.value() != Some(0.0) && !number.label().eq(&String::from("0")) {
                        addends.push(Multiplication::new(vec![
                            Number::new(-1.0),
                            right_hand_side.clone(),
                        ]));
                    }
                }
            }
            _ => {
                addends.push(Multiplication::new(vec![
                    Number::new(-1.0),
                    right_hand_side.clone(),
                ]));
            }
        }
        if addends.len() == 0 {
            return Number::new(0.0);
        }
        if addends.len() == 1 {
            return addends.pop().unwrap();
        }
        return Addition::new(addends);
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
