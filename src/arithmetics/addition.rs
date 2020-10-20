use crate::base::{
    commutative_association::CommutativeAssociation, expression::Expression, symbol::Symbol,
};
use num::bigint::BigInt;

impl Expression {
    /**
     * Builds a additive expression
     *  - ignores neutral element
     */
    pub fn addition(addends: Vec<Expression>) -> Expression {
        let addends: Vec<Expression> = addends
            .iter()
            .cloned()
            .filter(|addend| match addend {
                Expression::Real(r) => r != &Symbol::real(0.0),
                Expression::Integer(n) => n != &Symbol::integer(0),
                _ => true,
            })
            .collect();

        let mut pending_addends: Vec<Expression> = addends.iter().cloned().collect();
        let mut items_vec: Vec<Expression> = Vec::new();
        let mut integer: Symbol<BigInt> = Symbol::integer(0);
        let mut real: Symbol<f64> = Symbol::real(0.0);

        while !pending_addends.is_empty() {
            let addend = &pending_addends.pop().unwrap();
            match addend {
                Expression::Addition(a) => {
                    pending_addends.append(&mut a.items());
                }
                Expression::Real(r) => {
                    if r == &Symbol::real(0.0) {
                        continue;
                    }
                    real = real + r;
                }
                Expression::Integer(n) => {
                    integer = integer + n;
                }
                _ => {
                    items_vec.push(addend.clone());
                }
            }
        }

        if integer < Symbol::integer(0) {
            items_vec.push(Expression::multiplication(vec![
                Symbol::integer(-1).expr(),
                (Symbol::integer(-1) * integer).expr(),
            ]));
        } else if integer > Symbol::integer(0) {
            items_vec.push(integer.expr());
        }

        if real < Symbol::real(0.0) {
            items_vec.push(Expression::multiplication(vec![
                Symbol::integer(-1).expr(),
                (Symbol::real(-1.0) * real).expr(),
            ]));
        } else if real > Symbol::real(0.0) {
            items_vec.push(real.expr());
        }

        if items_vec.len() == 0 {
            return Symbol::integer(0).expr();
        }

        if items_vec.len() == 1 {
            return items_vec.pop().unwrap();
        }

        return Expression::Addition(CommutativeAssociation::new(items_vec));
    }
}

/**
 * Overloads plus (*) Operation
 */
impl std::ops::Add for Expression {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Expression::addition(vec![self, other])
    }
}

impl std::ops::Add<&Expression> for Expression {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        self + other.clone()
    }
}

impl std::ops::Add<&Expression> for &Expression {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        self.clone() + other.clone()
    }
}

impl std::ops::Add<Expression> for &Expression {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        self.clone() + other
    }
}
