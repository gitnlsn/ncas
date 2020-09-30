use crate::{
    arithmetics::multiplication::Multiplication,
    base::expression::Expression,
    exponential::power::Power,
    manipulation::identifiable::{Identifiable, Identity},
    symbols::number::Number,
};

#[derive(std::fmt::Debug)]
pub struct Division {}

impl Division {
    pub fn new(left_hand_side: Expression, right_hand_side: Expression) -> Expression {
        match right_hand_side.id() {
            Identity::Number => {
                /* Avoid unnecessary power structures */
                if let Expression::Symbol(number) = right_hand_side.clone() {
                    if number.value() == Some(1.0)
                        || number.value() == Some(-1.0)
                        || number.label().eq(&String::from("1"))
                        || number.label().eq(&String::from("-1"))
                    {
                        return Multiplication::new(vec![
                            left_hand_side.clone(),
                            right_hand_side.clone(),
                        ]);
                    }
                }
            }
            _ => {}
        };
        return Multiplication::new(vec![
            left_hand_side.clone(),
            Power::new(right_hand_side.clone(), Number::new(-1.0)),
        ]);
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
