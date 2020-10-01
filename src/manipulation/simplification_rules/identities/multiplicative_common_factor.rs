use crate::arithmetics::multiplication::Multiplication;
use crate::base::expression::Expression;
use crate::exponential::power::Power;
use crate::manipulation::{
    identifiable::{Identifiable, Identity},
    simplification_rules::rule::Rule,
};
use crate::symbols::number::Number;

use std::collections::HashMap;

pub struct MultiplicativeCommonFactor {}
impl Rule for MultiplicativeCommonFactor {
    fn apply(expression: &Expression) -> Vec<Expression> {
        let mut alternatives = Vec::new();
        match expression {
            Expression::CommutativeAssociation(association) => {
                if association.id() == Identity::Multiplication {
                    let mut power_list: HashMap<Expression, Expression> = HashMap::new();
                    for factor in association.items().iter() {
                        match factor.id() {
                            Identity::Power => {
                                if let Expression::AssociativeOperation(power) = factor {
                                    let base = power.argument().as_ref();
                                    let exponent = power.modifier().as_ref();

                                    let opposite = &(1 / base);
                                    if power_list.contains_key(base) {
                                        let current_exponent = power_list.remove(base).unwrap();
                                        power_list
                                            .insert(base.clone(), current_exponent + exponent);
                                    } else if power_list.contains_key(opposite) {
                                        let counter = power_list.remove(opposite).unwrap();
                                        power_list.insert(opposite.clone(), counter - exponent);
                                    } else {
                                        power_list.insert(base.clone(), exponent.clone());
                                    }
                                }
                            }
                            _ => {
                                let opposite = &-factor;
                                if power_list.contains_key(factor) {
                                    let counter = power_list.remove(factor).unwrap();
                                    power_list.insert(factor.clone(), counter + 1);
                                } else if power_list.contains_key(opposite) {
                                    let counter = power_list.remove(opposite).unwrap();
                                    power_list.insert(opposite.clone(), counter - 1);
                                } else {
                                    power_list.insert(factor.clone(), Number::new(1.0));
                                }
                            }
                        }
                    }

                    let addends: Vec<Expression> = power_list
                        .iter()
                        .filter(|(_, exponent)| match exponent {
                            Expression::Symbol(s) => match exponent.id() {
                                Identity::Number => {
                                    s.value() != Some(0.0) && !s.label().eq(&String::from("0"))
                                }
                                _ => true,
                            },
                            _ => true,
                        })
                        .map(|(base, exponent)| Power::new(base.clone(), exponent.clone()))
                        .collect();

                    alternatives.push(Multiplication::new(addends));
                }
            }
            _ => {}
        }
        return alternatives;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn factors_symbols() {
        use crate::symbols::variable::Variable;

        let expression = Multiplication::new(vec![
            Variable::new(String::from("a")),
            Variable::new(String::from("a")),
            Variable::new(String::from("a")),
            Variable::new(String::from("b")),
        ]);

        let expected = Multiplication::new(vec![
            Variable::new(String::from("a"))
                ^ (Number::new(1.0) + Number::new(1.0) + Number::new(1.0)),
            Variable::new(String::from("b")),
        ]);

        let factored = MultiplicativeCommonFactor::apply(&expression)
            .pop()
            .unwrap();

        assert_eq!(factored, expected);
    }

    #[test]
    fn factors_expressions() {
        use crate::symbols::variable::Variable;
        let n1 = &Number::new(1.0);
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));

        let expression = Multiplication::new(vec![a * b, a * b, a * b, a.clone(), b.clone()]);

        let expected = Multiplication::new(vec![a ^ (n1 + n1 + n1 + n1), b ^ (n1 + n1 + n1 + n1)]);

        let factored = MultiplicativeCommonFactor::apply(&expression)
            .pop()
            .unwrap();

        assert_eq!(factored, expected);
    }

    #[test]
    fn subtracts_inverse() {
        use crate::symbols::variable::Variable;

        let n1 = &Number::new(1.0);
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));

        let expression = Multiplication::new(vec![a / b, a / b, a / b]);

        let expected = Multiplication::new(vec![a ^ (n1 + n1 + n1), b ^ (-n1 - n1 - n1)]);

        let factored = MultiplicativeCommonFactor::apply(&expression)
            .pop()
            .unwrap();

        assert_eq!(factored, expected);
    }

    #[test]
    fn zero_power_wont_go_to_one() {
        use crate::symbols::variable::Variable;

        let n1 = &Number::new(1.0);
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));

        let expression = Multiplication::new(vec![a / b, b / a]);

        let expected = Multiplication::new(vec![a ^ (n1 - n1), b ^ (n1 - n1)]);

        let factored = MultiplicativeCommonFactor::apply(&expression)
            .pop()
            .unwrap();

        assert_eq!(factored, expected);
    }
}
