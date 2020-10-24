use crate::base::{
    commutative_association::CommutativeAssociation, expression::Expression, symbol::Symbol,
};
use num::bigint::BigInt;

impl Expression {
    /**
     * Builds a multiplicative expression
     *  - ignores neutral element
     *  - separates sign with -1 integer part
     */
    pub fn multiplication(factors: Vec<Expression>) -> Expression {
        let factors: Vec<Expression> = factors
            .iter()
            .cloned()
            .filter(|addend| match addend {
                Expression::Integer(n) => n != &Symbol::integer(1),
                _ => true,
            })
            .collect();

        let mut pending_factors: Vec<Expression> = factors.iter().cloned().collect();
        let mut items_vec: Vec<Expression> = Vec::new();
        let mut integer: Symbol<BigInt> = Symbol::integer(1);
        let mut real: Symbol<f64> = Symbol::real(1.0);

        while !pending_factors.is_empty() {
            let factor = &pending_factors.pop().unwrap();
            match factor {
                Expression::Multiplication(m) => {
                    pending_factors.append(&mut m.items());
                }
                Expression::Real(r) => {
                    if r == &Symbol::real(0.0) {
                        return Symbol::integer(0).expr();
                    } else if r.value().unwrap() < 0.0 {
                        integer = integer * Symbol::integer(-1);
                        real = real * r * Symbol::real(-1.0);
                        continue;
                    } else {
                        real = real * r;
                        continue;
                    }
                }
                Expression::Integer(n) => {
                    if n == &Symbol::integer(0) {
                        return Symbol::integer(0).expr();
                    } else {
                        integer = integer * n;
                        continue;
                    }
                }
                _ => {
                    items_vec.push(factor.clone());
                }
            }
        }

        if integer != Symbol::integer(1) {
            items_vec.push(integer.expr());
        }

        if real != Symbol::real(1.0) {
            items_vec.push(real.expr());
        }

        if items_vec.len() == 0 {
            return Symbol::integer(1).expr();
        }

        if items_vec.len() == 1 {
            return items_vec.pop().unwrap();
        }

        return Expression::Multiplication(CommutativeAssociation::new(items_vec));
    }
}

/**
 * Overloads plus (*) Operation
 */
impl std::ops::Mul for Expression {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        Expression::multiplication(vec![self, other])
    }
}

impl std::ops::Mul<&Expression> for Expression {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        self * other.clone()
    }
}

impl std::ops::Mul<&Expression> for &Expression {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        self.clone() * other.clone()
    }
}

impl std::ops::Mul<Expression> for &Expression {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        self.clone() * other
    }
}
