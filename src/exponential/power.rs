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
            Expression::Real(real_exponent) => {
                if real_exponent == &Symbol::real(1.0) {
                    return base;
                }
                if real_exponent == &Symbol::real(0.0) {
                    return Symbol::integer(1).expr();
                }

                match &base {
                    Expression::Real(real_base) => {
                        let base_value = real_base.value().unwrap();
                        let exponent_value = real_exponent.value().unwrap();
                        let power_value = (base_value).powf(exponent_value);
                        return Symbol::real(power_value).expr();
                    }
                    Expression::Multiplication(base_factors) => {
                        let mut factor_vec: Vec<Expression> = Vec::new();

                        /* Sign */
                        let odd_exponent: bool = real_exponent.value().unwrap() as isize % 2 == 1;

                        let is_negative: bool =
                            match base_factors.items().iter().find(|factor| match factor {
                                Expression::Integer(n) => n == &Symbol::integer(-1),
                                _ => false,
                            }) {
                                Some(_) => true,
                                None => false,
                            };

                        if !odd_exponent && is_negative {
                            factor_vec.push(Symbol::integer(-1).expr());
                        }

                        /* real */
                        let possible_real_factor: Option<Expression> = base_factors
                            .items()
                            .iter()
                            .cloned()
                            .find(|factor| match factor {
                                Expression::Real(_) => true,
                                _ => false,
                            });

                        match possible_real_factor {
                            Some(expression) => match expression {
                                Expression::Real(real_base) => {
                                    let base_value = real_base.value().unwrap();
                                    let exponent_value = real_exponent.value().unwrap();
                                    let power_value = (base_value).powf(exponent_value);
                                    factor_vec.push(Symbol::real(power_value).expr());
                                }
                                _ => {}
                            },
                            _ => {}
                        };

                        /* remaining factors */
                        let other_factors: Vec<Expression> = base_factors
                            .items()
                            .iter()
                            .filter(|factor| match factor {
                                Expression::Real(_) => false,
                                _ => true,
                            })
                            .cloned()
                            .collect();

                        if !other_factors.is_empty() {
                            factor_vec.push(Expression::Power(AssociativeOperation::new(
                                Expression::multiplication(other_factors),
                                exponent,
                            )));
                        }

                        return Expression::multiplication(factor_vec);
                    } /* end - match multiplication */

                    _ => { /* nothing to do */ }
                }
            }
            Expression::Integer(integer_exponent) => {
                if integer_exponent == &Symbol::integer(1) {
                    return base;
                }
                if integer_exponent == &Symbol::integer(0) {
                    return Symbol::integer(1).expr();
                }

                match &base {
                    Expression::Integer(integer_base) => {
                        if integer_exponent.is_negative() {
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
                    Expression::Multiplication(base_factors) => {
                        let mut factor_vec: Vec<Expression> = Vec::new();

                        /* Sign */
                        let odd_exponent: bool =
                            integer_exponent.value().unwrap() as isize % 2 == 1;

                        let is_negative: bool =
                            match base_factors.items().iter().find(|factor| match factor {
                                Expression::Integer(n) => n == &Symbol::integer(-1),
                                _ => false,
                            }) {
                                Some(_) => true,
                                None => false,
                            };

                        if !odd_exponent && is_negative {
                            factor_vec.push(Symbol::integer(-1).expr());
                        }

                        /* integer */
                        let possible_integer_factor: Option<Expression> = base_factors
                            .items()
                            .iter()
                            .cloned()
                            .find(|factor| match factor {
                                Expression::Integer(n) => n != &Symbol::integer(-1),
                                _ => false,
                            });

                        match possible_integer_factor {
                            Some(expression) => match expression {
                                Expression::Integer(integer_base) => {
                                    let base_value = integer_base.value().unwrap();
                                    let exponent_value = integer_exponent.value().unwrap();
                                    let power_value = (base_value).powf(exponent_value) as isize;
                                    factor_vec.push(Symbol::integer(power_value).expr());
                                }
                                _ => { /* nothing to do */ }
                            },
                            _ => { /* nothing to do */ }
                        };

                        /* remaining factors */
                        let other_factors: Vec<Expression> = base_factors
                            .items()
                            .iter()
                            .filter(|factor| match factor {
                                Expression::Integer(_) => false,
                                _ => true,
                            })
                            .cloned()
                            .collect();

                        if !other_factors.is_empty() {
                            factor_vec.push(Expression::Power(AssociativeOperation::new(
                                Expression::multiplication(other_factors),
                                exponent,
                            )));
                        }

                        return Expression::multiplication(factor_vec);
                    } /* end - match multiplication */
                    _ => { /* nothing to do */ }
                }
            }
            _ => { /* nothing to do */ }
        } /* matcher for exponent */
        return Expression::Power(AssociativeOperation::new(base, exponent));
    }

    pub fn pow(self, exponent: Expression) -> Expression {
        Self::power(self, exponent)
    }
} /* end - power expression */
