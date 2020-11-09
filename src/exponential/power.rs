use crate::base::associative_operation::AssociativeOperation;
use crate::base::{expression::Expression, symbol::Symbol};

impl Expression {
    /**
     * Builds power associative operation
     *  - ignores neutral exponent
     *  - keeps signal separated
     */
    pub fn power(base: Expression, exponent: Expression) -> Expression {
        match &exponent {
            Expression::Integer(integer_exponent) => {
                /* Identity (power 1 exponent) */
                if integer_exponent == &Symbol::integer(1) {
                    return base;
                }
                /* Identity (power 0 exponent) */
                if integer_exponent == &Symbol::integer(0) {
                    return Symbol::integer(1).expr();
                }
            }
            Expression::Real(real_exponent) => {
                /* Identity (power 1.0 exponent) */
                if real_exponent == &Symbol::real(1.0) {
                    return base;
                }
                /* Identity (power 0.0 exponent) */
                if real_exponent == &Symbol::real(0.0) {
                    return Symbol::integer(1).expr();
                }
            }
            _ => {}
        }

        match &base {
            // Identity (power to power):  (a ^ b) ^ c == a ^ (b * c)
            Expression::Power(power) => {
                return Expression::power(
                    power.argument(),
                    Expression::multiplication(vec![power.modifier(), exponent]),
                );
            }
            // Distributive: (a * b) ^ c == a ^ c * b ^ c
            Expression::Multiplication(base_factors) => {
                return Expression::multiplication(
                    base_factors.map(&|factor| Expression::power(factor.clone(), exponent.clone())),
                );
            }
            _ => {}
        }

        match &exponent {
            /* Logarithm inverse operation */
            Expression::Logarithm(log) => {
                if base == log.modifier() {
                    return log.argument();
                }
            }
            _ => {}
        }

        match &base {
            Expression::Integer(integer_base) => {
                match &exponent {
                    /* Simplification (Integer to Integer) */
                    Expression::Integer(integer_exponent) => {
                        if integer_exponent.is_negative() {
                            /* Rational Denominator */
                            return Expression::Power(AssociativeOperation::new(
                                integer_base
                                    .pow(&integer_exponent.opposite())
                                    .unwrap()
                                    .expr(),
                                Symbol::integer(-1).expr(),
                            ));
                        } else {
                            return integer_base.pow(integer_exponent).unwrap().expr();
                        }
                    }
                    /* Simplification (Integer to Real) */
                    Expression::Real(real_exponent) => {
                        let base_value: f64 = integer_base.value().unwrap() as f64;
                        let exp_value: f64 = real_exponent.value().unwrap();
                        return Symbol::real(base_value.powf(exp_value)).expr();
                    }
                    /* Simplification (Integer to multiplication with Integer or Real) */
                    Expression::Multiplication(exponent_factors) => {
                        if let Some(real_exponent_factor) =
                            exponent_factors.get_one(&|factor| match factor {
                                Expression::Real(_) => true,
                                _ => false,
                            })
                        {
                            let other_factors = exponent_factors.get(&|factor| match factor {
                                Expression::Real(_) => false,
                                _ => true,
                            });
                            return Expression::power(
                                Expression::power(base, real_exponent_factor), /* Recursion: int to real */
                                Expression::multiplication(other_factors),
                            );
                        }
                        if let Some(integer_exponent_factor) =
                            exponent_factors.get_one(&|factor| match factor {
                                Expression::Integer(_) => true,
                                _ => false,
                            })
                        {
                            let other_factors = exponent_factors.get(&|factor| match factor {
                                Expression::Integer(_) => false,
                                _ => true,
                            });
                            return Expression::power(
                                Expression::power(base, integer_exponent_factor), /* Recursion: int to int */
                                Expression::multiplication(other_factors),
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match &base {
            Expression::Real(real_base) => {
                match &exponent {
                    /* Simplification (Real to Real) */
                    Expression::Real(real_exponent) => {
                        let base_value = real_base.value().unwrap();
                        let exponent_value = real_exponent.value().unwrap();
                        let power_value = (base_value).powf(exponent_value);
                        return Symbol::real(power_value).expr();
                    }
                    /* Simplification (Real to Integer) */
                    Expression::Integer(integer_exponent) => {
                        let base_value: f64 = real_base.value().unwrap();
                        let exp_value: f64 = integer_exponent.value().unwrap() as f64;
                        return Symbol::real(base_value.powf(exp_value)).expr();
                    }
                    /* Simplification (Real to multiplication with Integer or Real) */
                    Expression::Multiplication(exponent_factors) => {
                        if let Some(real_exponent_factor) =
                            exponent_factors.get_one(&|factor| match factor {
                                Expression::Real(_) => true,
                                _ => false,
                            })
                        {
                            let other_factors = exponent_factors.get(&|factor| match factor {
                                Expression::Real(_) => false,
                                _ => true,
                            });
                            return Expression::power(
                                Expression::power(base, real_exponent_factor), /* Recursion: real to real */
                                Expression::multiplication(other_factors),
                            );
                        }
                        if let Some(integer_exponent_factor) =
                            exponent_factors.get_one(&|factor| match factor {
                                Expression::Integer(_) => true,
                                _ => false,
                            })
                        {
                            let other_factors = exponent_factors.get(&|factor| match factor {
                                Expression::Integer(_) => false,
                                _ => true,
                            });
                            return Expression::power(
                                Expression::power(base, integer_exponent_factor), /* Recursion: real to int */
                                Expression::multiplication(other_factors),
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        /* default constructor */
        return Expression::Power(AssociativeOperation::new(base, exponent));
    }

    pub fn pow(self, exponent: Expression) -> Expression {
        Self::power(self, exponent)
    }
} /* end - power expression */
