use crate::base::{expression::Expression, symbol::Symbol};
use crate::manipulation::expansion_rules::{
    multiplicative_distributive::MultiplicativeDistributive, rule::Rule,
};

pub struct PowerDistributive {}
impl Rule for PowerDistributive {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Power(power) => match power.argument() {
                Expression::Addition(_) => match power.modifier() {
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
                    Expression::Multiplication(exponent_factors) => {
                        let exponent_numerator =
                            exponent_factors /* expecting one numerator in multiplication */
                                .get_one(&|factor| match factor {
                                    Expression::Integer(_) => true,
                                    _ => false,
                                });

                        let exponent_other = exponent_factors.get(&|factor| match factor {
                            Expression::Integer(_) => false,
                            _ => true,
                        });

                        if let Some(numerator) = exponent_numerator {
                            return Expression::power(
                                PowerDistributive::apply(&Expression::power(
                                    power.argument(),
                                    numerator,
                                )),
                                Expression::multiplication(exponent_other),
                            );
                        }
                    }
                    _ => {}
                },
                Expression::Multiplication(factors) => {
                    let additive_factors = factors.get(&|factor| match factor {
                        Expression::Addition(_) => true,
                        _ => false,
                    });

                    let other_factors: Vec<Expression> = factors
                        .get(&|factor| match factor {
                            Expression::Addition(_) => false,
                            Expression::Integer(_) => false,
                            Expression::Real(_) => false,
                            _ => true,
                        })
                        .iter()
                        .map(|factor| Expression::power(factor.clone(), power.modifier()))
                        .collect();

                    return Expression::multiplication(vec![
                        PowerDistributive::apply(&Expression::power(
                            MultiplicativeDistributive::apply(&Expression::multiplication(
                                additive_factors,
                            )),
                            power.modifier(),
                        )),
                        Expression::multiplication(other_factors),
                    ]);
                }
                _ => {}
            },
            _ => {}
        }
        return expression.clone();
    }
}
