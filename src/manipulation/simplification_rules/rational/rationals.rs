use crate::base::expression::Expression;
use crate::base::symbol::Symbol;
use num::bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Rational {
    numerator: Symbol<BigInt>,
    denominator: Symbol<BigInt>,
}

impl Rational {
    /**
     * Constructor from expression. Expression must be composed of integers only
     */
    pub fn from_expression(expression: Expression) -> Option<Self> {
        match expression {
            Expression::Integer(n) => return Some(Self::from(n, Symbol::integer(1))),
            Expression::Power(power) => match (power.argument(), power.modifier()) {
                (Expression::Integer(base), Expression::Integer(exponent)) => {
                    if exponent == Symbol::integer(-1) {
                        return Some(Rational::from(Symbol::integer(1), base));
                    }
                    return None;
                }
                _ => return None,
            },
            Expression::Multiplication(factors) => {
                let numerator = factors
                    .get_rational_numerators()
                    .iter()
                    .cloned()
                    .map(|factor| match factor {
                        Expression::Integer(n) => n,
                        _ => panic!("Unexpected value"),
                    })
                    .fold(Symbol::integer(1), |acc, value| acc * value);

                let denominator = factors
                    .get_rational_denominators()
                    .iter()
                    .cloned()
                    .map(|factor| match factor {
                        Expression::Integer(n) => n,
                        _ => panic!("Unexpected value"),
                    })
                    .fold(Symbol::integer(1), |acc, value| acc * value);

                return Some(Self::from(numerator, denominator));
            }
            _ => return None,
        }
    }

    /**
     * Constructor with gcd simplification
     */
    pub fn from(numerator: Symbol<BigInt>, denominator: Symbol<BigInt>) -> Self {
        if numerator == Symbol::integer(0) {
            return Self {
                numerator: numerator,
                denominator: Symbol::integer(1),
            };
        }

        let gcd: &Symbol<BigInt> = &Symbol::gcd(&numerator, &denominator);

        return Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        };
    }

    /**
     * Constructor for Expression from Self
     */
    pub fn expr(self) -> Expression {
        self.numerator.expr() / self.denominator.expr()
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} / {}",
            self.numerator.label(),
            self.denominator.label()
        )
    }
}

// ===================================== //
//          Arithmetics Overloading      //
// ===================================== //
impl std::ops::Add for Rational {
    type Output = Rational;
    fn add(self, other: Rational) -> Rational {
        let numerator = self.numerator.clone() * other.denominator.clone()
            + other.numerator.clone() * self.denominator.clone();
        let denominator = self.denominator.clone() * other.denominator.clone();

        return Rational::from(numerator, denominator);
    }
}

impl std::ops::Sub for Rational {
    type Output = Rational;
    fn sub(self, other: Rational) -> Rational {
        let numerator = self.numerator.clone() * other.denominator.clone()
            - other.numerator.clone() * self.denominator.clone();
        let denominator = self.denominator.clone() * other.denominator.clone();

        return Rational::from(numerator, denominator);
    }
}

impl std::ops::Mul for Rational {
    type Output = Rational;
    fn mul(self, other: Rational) -> Rational {
        let numerator = self.numerator.clone() * other.numerator.clone();
        let denominator = self.denominator.clone() * other.denominator.clone();

        return Rational::from(numerator, denominator);
    }
}

impl std::ops::Div for Rational {
    type Output = Rational;
    fn div(self, other: Rational) -> Rational {
        let numerator = self.numerator.clone() * other.denominator.clone();
        let denominator = self.denominator.clone() * other.numerator.clone();

        return Rational::from(numerator, denominator);
    }
}

impl Eq for Rational {}
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            == other.numerator.clone() * self.denominator.clone()
    }
}

#[cfg(test)]
mod constructor {
    use super::*;

    #[test]
    fn simplifies_gcd() {
        assert_eq!(
            Rational::from(Symbol::integer(6), Symbol::integer(9)),
            Rational {
                numerator: Symbol::integer(2),
                denominator: Symbol::integer(3),
            }
        )
    }

    #[test]
    fn negative_value() {
        assert_eq!(
            Rational::from(Symbol::integer(-1), Symbol::integer(1)).expr(),
            Symbol::integer(-1).expr()
        )
    }
}

#[cfg(test)]
mod addition {
    use super::*;

    #[test]
    fn irreducible_addition() {
        let r1 = Rational::from(Symbol::integer(2), Symbol::integer(3));
        let r2 = Rational::from(Symbol::integer(5), Symbol::integer(7));

        let r3 = Rational::from(Symbol::integer(29), Symbol::integer(21));

        assert_eq!(r1 + r2, r3);
    }

    #[test]
    fn gcd_simplification() {
        let r1 = Rational::from(Symbol::integer(1), Symbol::integer(2));
        let r2 = Rational::from(Symbol::integer(1), Symbol::integer(3));
        let r3 = Rational::from(Symbol::integer(1), Symbol::integer(6));

        let r4 = Rational::from(Symbol::integer(1), Symbol::integer(1));

        assert_eq!(r1 + r2 + r3, r4);
    }
}

#[cfg(test)]
mod subtraction {
    use super::*;

    #[test]
    fn irreducible_subtraction() {
        let r1 = Rational::from(Symbol::integer(2), Symbol::integer(3));
        let r2 = Rational::from(Symbol::integer(5), Symbol::integer(7));

        let r3 = Rational::from(Symbol::integer(-1), Symbol::integer(21));

        assert_eq!(r1 - r2, r3);
    }

    #[test]
    fn gcd_simplification() {
        let r1 = Rational::from(Symbol::integer(1), Symbol::integer(6));
        let r2 = Rational::from(Symbol::integer(1), Symbol::integer(3));
        let r3 = Rational::from(Symbol::integer(1), Symbol::integer(2));

        let r4 = Rational::from(Symbol::integer(0), Symbol::integer(1));

        assert_eq!(r1 + r2 - r3, r4);
    }
}

#[cfg(test)]
mod multiplication {
    use super::*;

    #[test]
    fn irreducible_multiplication() {
        let r1 = Rational::from(Symbol::integer(2), Symbol::integer(3));
        let r2 = Rational::from(Symbol::integer(5), Symbol::integer(7));

        let r3 = Rational::from(Symbol::integer(10), Symbol::integer(21));

        assert_eq!(r1 * r2, r3);
    }

    #[test]
    fn gcd_simplification() {
        let r1 = Rational::from(Symbol::integer(2), Symbol::integer(3));
        let r2 = Rational::from(Symbol::integer(6), Symbol::integer(7));

        let r3 = Rational::from(Symbol::integer(4), Symbol::integer(7));

        assert_eq!(r1 * r2, r3);
    }
}

#[cfg(test)]
mod division {
    use super::*;

    #[test]
    fn irreducible_multiplication() {
        let r1 = Rational::from(Symbol::integer(2), Symbol::integer(3));
        let r2 = Rational::from(Symbol::integer(5), Symbol::integer(7));

        let r3 = Rational::from(Symbol::integer(14), Symbol::integer(15));

        assert_eq!(r1 / r2, r3);
    }

    #[test]
    fn gcd_simplification() {
        let r1 = Rational::from(Symbol::integer(2), Symbol::integer(3));
        let r2 = Rational::from(Symbol::integer(6), Symbol::integer(7));

        let r3 = Rational::from(Symbol::integer(7), Symbol::integer(9));

        assert_eq!(r1 / r2, r3);
    }
}
