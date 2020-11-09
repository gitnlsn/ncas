use crate::base::expression::Expression;
use crate::base::symbol::Symbol;
use crate::manipulation::simplification_rules::rule::Rule;

pub struct SingleAlgebraicDenominator {}
impl Rule for SingleAlgebraicDenominator {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Multiplication(factors) => {
                let denominators: Vec<Expression> = factors
                    .get(&|factor| match factor {
                        Expression::Power(power) => match power.modifier() {
                            Expression::Integer(n) => n < Symbol::integer(0),
                            Expression::Multiplication(power_factors) => {
                                match power_factors.get_one(&|factor| match factor {
                                    Expression::Integer(n) => return n < &Symbol::integer(0),
                                    _ => return false,
                                }) {
                                    Some(_) => return true,
                                    None => return false,
                                }
                            }
                            _ => false,
                        },
                        _ => false,
                    })
                    .iter()
                    .map(|denominator| match denominator {
                        Expression::Power(power) => match power.modifier() {
                            Expression::Integer(n) => {
                                return power.argument().pow((n * Symbol::integer(-1)).expr())
                            }
                            Expression::Multiplication(power_factors) => {
                                return Expression::multiplication(power_factors.map(&|factor| {
                                    match factor {
                                        Expression::Integer(n) => {
                                            return power
                                                .argument()
                                                .pow((n * Symbol::integer(-1)).expr())
                                        }
                                        _ => return factor.clone(),
                                    }
                                }))
                            }
                            _ => return denominator.clone(),
                        },
                        _ => denominator.clone(),
                    })
                    .collect();

                let mut other_factors = factors.get(&|factor| match factor {
                    Expression::Power(power) => match power.modifier() {
                        Expression::Integer(n) => n > Symbol::integer(0),
                        Expression::Multiplication(power_factors) => {
                            match power_factors.get_one(&|factor| match factor {
                                Expression::Integer(n) => return n < &Symbol::integer(0),
                                _ => return false,
                            }) {
                                Some(_) => return false,
                                None => return true,
                            }
                        }
                        _ => true,
                    },
                    _ => true,
                });

                let mut factors: Vec<Expression> = Vec::new();
                factors.append(&mut other_factors);
                factors.push(Expression::power(
                    Expression::multiplication(denominators),
                    Symbol::integer(-1).expr(),
                ));

                return Expression::multiplication(factors);
            }
            _ => return expression.clone(),
        }
    }
}

#[cfg(test)]
mod simplify {
    use super::*;

    #[test]
    fn merges_variable_inverses() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let one = &Symbol::integer(1).expr();

        let trial = one / a * one / b;
        let expected = one / (a * b);

        assert_eq!(SingleAlgebraicDenominator::apply(&trial), expected);
    }

    #[test]
    fn merges_inserve_powers() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let one = &Symbol::integer(1).expr();
        let two = &Symbol::integer(2).expr();

        let trial = one / a * one / b.clone().pow(two.clone());
        let expected = one / (a * b.clone().pow(two.clone()));

        assert_eq!(SingleAlgebraicDenominator::apply(&trial), expected);
    }

    #[test]
    fn merges_inserve_expressions() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let one = &Symbol::integer(1).expr();
        let two = &Symbol::integer(2).expr();

        let trial = one / a * one / (b + c).pow(two.clone());
        let expected = one / (a * (b + c).pow(two.clone()));

        assert_eq!(SingleAlgebraicDenominator::apply(&trial), expected);
    }
}
