use crate::arithmetics::addition::Addition;
use crate::base::expression::Expression;
use crate::exponential::power::Power;
use crate::manipulation::{
    identifiable::{Identifiable, Identity},
    simplification_rules::rule::Rule,
};
use crate::symbols::number::Number;
use crate::trigonometrics::{cossine::Cos, sine::Sin};

use std::collections::HashMap;

pub struct PitagoricIdentity {}

impl Rule for PitagoricIdentity {
    fn apply(expression: &Expression) -> Vec<Expression> {
        let mut alternative_list: Vec<Expression> = Vec::new();

        match expression {
            Expression::CommutativeAssociation(association) => match association.id() {
                Identity::Addition => {
                    let mut pitagorean_counter: HashMap<Expression, isize> = HashMap::new();
                    let mut other_addend_list: Vec<Expression> = Vec::new();
                    for addend in association.items().iter() {
                        if let Some(complement) = complement(addend) {
                            if pitagorean_counter.contains_key(addend) {
                                let counter = pitagorean_counter.remove(addend).unwrap();
                                pitagorean_counter.insert(addend.clone(), counter + 1);
                            } else if pitagorean_counter.contains_key(&complement) {
                                let counter = pitagorean_counter.remove(&complement).unwrap();
                                pitagorean_counter.insert(complement.clone(), counter - 1);
                                other_addend_list.push(Number::new(1.0));
                            } else {
                                pitagorean_counter.insert(addend.clone(), 1);
                            }
                        } else {
                            other_addend_list.push(addend.clone());
                        }
                    }

                    let mut addends: Vec<Expression> = pitagorean_counter
                        .iter()
                        .map(|(expression, counter)| *counter as isize * expression)
                        .collect();

                    addends.append(&mut other_addend_list);

                    alternative_list.push(Addition::new(addends));
                }
                _ => {}
            },
            _ => {}
        }

        if alternative_list.is_empty() {
            return vec![expression.clone()];
        }

        return alternative_list;
    }
}

/**
 * Complement pitagorean expression is the addend . Eg:
 *      complement(sin^2(a + b + c)) =  cos^2(a + b + c)
 *      complement(cos^2(cos(theta))) = sin^2(cos(theta))
 */
fn complement(expression: &Expression) -> Option<Expression> {
    match expression {
        Expression::AssociativeOperation(op) => match op.id() {
            Identity::Power => {
                let base = op.argument();
                let exponent = op.modifier();

                let is_trigonometric: bool = match base.id() {
                    Identity::Sin => true,
                    Identity::Cos => true,
                    _ => false,
                };

                let is_squared: bool = match exponent.as_ref() {
                    Expression::Symbol(s) => match exponent.id() {
                        Identity::Number => s.value() == Some(2.0),
                        _ => false,
                    },
                    _ => false,
                };

                if is_trigonometric && is_squared {
                    if let Expression::Operation(op) = base.as_ref() {
                        match base.id() {
                            Identity::Cos => {
                                return Some(Power::new(
                                    Sin::new(op.argument().as_ref().clone()),
                                    Number::new(2.0),
                                ));
                            }
                            Identity::Sin => {
                                return Some(Power::new(
                                    Cos::new(op.argument().as_ref().clone()),
                                    Number::new(2.0),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        },
        _ => {}
    };

    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pitagorean_complement() {
        let angle = &Number::new(1.0);
        let cos_square = Power::new(Cos::new(angle.clone()), Number::new(2.0));
        let sin_square = Power::new(Sin::new(angle.clone()), Number::new(2.0));
        assert_eq!(complement(&cos_square.clone()), Some(sin_square.clone()));
        assert_eq!(complement(&sin_square.clone()), Some(cos_square.clone()));

        let angle = &Cos::new(Number::new(1.0));
        let cos_square = Power::new(Cos::new(angle.clone()), Number::new(2.0));
        let sin_square = Power::new(Sin::new(angle.clone()), Number::new(2.0));
        assert_eq!(complement(&cos_square.clone()), Some(sin_square.clone()));
        assert_eq!(complement(&sin_square.clone()), Some(cos_square.clone()));
    }

    #[test]
    fn rule_apply() {
        let angle = &Number::new(1.0);
        let cos_square = Power::new(Cos::new(angle.clone()), Number::new(2.0));
        let sin_square = Power::new(Sin::new(angle.clone()), Number::new(2.0));
        assert_eq!(
            PitagoricIdentity::apply(&(cos_square + sin_square))
                .pop()
                .unwrap(),
            Number::new(1.0)
        );

        let angle = &Cos::new(Number::new(1.0));
        let cos_square = Power::new(Cos::new(angle.clone()), Number::new(2.0));
        let sin_square = Power::new(Sin::new(angle.clone()), Number::new(2.0));
        assert_eq!(
            PitagoricIdentity::apply(&(cos_square + sin_square))
                .pop()
                .unwrap(),
            Number::new(1.0)
        );
    }
}
