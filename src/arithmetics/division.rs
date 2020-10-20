use crate::base::{expression::Expression, symbol::Symbol};

impl Expression {
    pub fn division(dividend: Expression, divisor: Expression) -> Expression {
        match &divisor {
            Expression::Integer(integer_divisor) => {
                /* Avoid unnecessary power structures */
                if integer_divisor == &Symbol::integer(1) {
                    return divisor;
                }
                if integer_divisor == &Symbol::integer(-1) {
                    return Expression::multiplication(vec![dividend, Symbol::integer(-1).expr()]);
                }

                match &dividend {
                    Expression::Integer(integer_dividend) => {
                        let gcd = &Symbol::gcd(integer_divisor, integer_dividend);
                        if integer_divisor == gcd {
                            return (integer_dividend / gcd).expr();
                        } else if integer_dividend == gcd {
                            return Expression::power(
                                (integer_divisor / integer_dividend).expr(),
                                Symbol::integer(-1).expr(),
                            );
                        } else {
                            return Expression::multiplication(vec![
                                (integer_dividend / gcd).expr(),
                                Expression::power(
                                    (integer_divisor / gcd).expr(),
                                    Symbol::integer(-1).expr(),
                                ),
                            ]);
                        }
                    }
                    Expression::Multiplication(factors) => {
                        let possible_interger_factor: Option<Expression> =
                            factors.items().iter().cloned().find(|factor| match factor {
                                Expression::Integer(_) => true,
                                _ => false,
                            });

                        match possible_interger_factor {
                            Some(integer_factor) => match integer_factor {
                                Expression::Integer(_) => {
                                    let mut factors: Vec<Expression> = factors
                                        .items()
                                        .iter()
                                        .filter(|factor| match factor {
                                            Expression::Integer(_) => false,
                                            _ => true,
                                        })
                                        .cloned()
                                        .collect();

                                    factors.push(Expression::division(integer_factor, divisor));

                                    return Expression::multiplication(factors);
                                }
                                _ => {}
                            },
                            None => {
                                let mut factors: Vec<Expression> = factors.items();
                                factors
                                    .push(Expression::power(divisor, Symbol::integer(-1).expr()));
                                return Expression::multiplication(factors);
                            }
                        }
                    }
                    _ => { /* nothing to do */ }
                }
            }
            Expression::Real(real_divisor) => {
                if real_divisor == &Symbol::real(1.0) {
                    return divisor;
                }
                if real_divisor == &Symbol::real(-1.0) {
                    return Expression::multiplication(vec![dividend, Symbol::real(-1.0).expr()]);
                }

                match &dividend {
                    Expression::Real(real_dividend) => {
                        return (real_dividend / real_divisor).expr();
                    }
                    Expression::Multiplication(factors) => {
                        let possible_real_factor: Option<Expression> =
                            factors.items().iter().cloned().find(|factor| match factor {
                                Expression::Real(_) => true,
                                _ => false,
                            });

                        match possible_real_factor {
                            Some(real_factor) => match real_factor {
                                Expression::Real(_) => {
                                    let mut factors: Vec<Expression> = factors
                                        .items()
                                        .iter()
                                        .filter(|factor| match factor {
                                            Expression::Real(_) => false,
                                            _ => true,
                                        })
                                        .cloned()
                                        .collect();

                                    factors.push(Expression::division(real_factor, divisor));

                                    return Expression::multiplication(factors);
                                }
                                _ => {}
                            },
                            None => {
                                let mut factors: Vec<Expression> = factors.items();
                                factors
                                    .push(Expression::power(divisor, Symbol::integer(-1).expr()));
                                return Expression::multiplication(factors);
                            }
                        }
                    }
                    _ => { /* nothing to do */ }
                }
            }
            _ => { /* nothing to do */ }
        };
        return Expression::multiplication(vec![
            dividend,
            Expression::power(divisor, Symbol::integer(-1).expr()),
        ]);
    }
}

/**
 * Overloads plus (/) Operation
 */
impl std::ops::Div for Expression {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Expression::division(self, other)
    }
}

impl std::ops::Div<&Expression> for Expression {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        self / other.clone()
    }
}

impl std::ops::Div<&Expression> for &Expression {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        self.clone() / other.clone()
    }
}

impl std::ops::Div<Expression> for &Expression {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        self.clone() / other
    }
}
