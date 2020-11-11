use crate::base::{expression::Expression, symbol::Symbol};
use crate::manipulation::expansion_rules::{
    multiplicative_distributive::MultiplicativeDistributive, rule::Rule,
};

/**
 * Expands power with integer exponent and additive base
 *  - reduces algorithm to multiplication expantion
 */
pub struct PowerDistributiveAddition {}
impl Rule for PowerDistributiveAddition {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Power(power) => match power.argument() {
                Expression::Addition(_) => match power.modifier() {
                    /* Single Integer exponent */
                    Expression::Integer(integer_exponent) => {
                        let exponent = integer_exponent.value().unwrap();
                        let mut factors: Vec<Expression> = Vec::new();

                        for _ in 0..exponent.abs() {
                            factors.push(power.argument());
                        }

                        if exponent > 0 {
                            return MultiplicativeDistributive::apply(&Expression::multiplication(
                                factors,
                            ));
                        } else {
                            return Expression::power(
                                MultiplicativeDistributive::apply(&Expression::multiplication(
                                    factors,
                                )),
                                Symbol::integer(-1).expr(),
                            );
                        }
                    }
                    /* Integer Exponent inside multiplication */
                    Expression::Multiplication(exponent_factors) => {
                        let integer_factor = exponent_factors.get_one(&|factor| match factor {
                            Expression::Integer(_) => true,
                            _ => false,
                        });

                        let other_factors = exponent_factors.get(&|factor| match factor {
                            Expression::Integer(_) => false,
                            _ => true,
                        });

                        if let Some(integer) = integer_factor {
                            return Expression::power(
                                PowerDistributiveAddition::apply(&Expression::power(
                                    power.argument(),
                                    integer,
                                )),
                                Expression::multiplication(other_factors),
                            );
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        return expression.clone();
    }
}

#[cfg(test)]
mod rule_testing {
    use super::*;

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let two = &Symbol::integer(2).expr();

        let expandable = (a + b).pow(two.clone());
        let result = a * a + a * b + a * b + b * b;

        assert_eq!(PowerDistributiveAddition::apply(&expandable), result);
    }

    #[test]
    fn sample_2() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let two = &Symbol::integer(2).expr();

        let expandable = (a + b).pow(c * two);
        let result = (a * a + a * b + a * b + b * b).pow(c.clone());

        assert_eq!(PowerDistributiveAddition::apply(&expandable), result);
    }
}
