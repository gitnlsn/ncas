use crate::base::{expression::Expression, symbol::Symbol};

use std::collections::HashMap;

// ============================= //
//      Expression Factoring     //
// ============================= //
pub struct FactorDecomposition {
    // TODO: Depends on expression comparison
    pub factors: HashMap<Expression, usize>,
}

impl FactorDecomposition {
    pub fn new() -> Self {
        FactorDecomposition {
            factors: HashMap::new(),
        }
    }

    pub fn from_expression(expression: Expression) -> Self {
        let mut factors: HashMap<Expression, usize> = HashMap::new();

        match &expression {
            Expression::Multiplication(factor_list) => {
                factor_list.items().iter().cloned().for_each(|factor| {
                    factors.insert(factor, 1);
                });
                return FactorDecomposition { factors: factors };
            }
            Expression::Power(power) => match power.modifier() {
                Expression::Integer(n) => {
                    let value = n.value().unwrap();
                    if value > 0 {
                        factors.insert(power.argument(), value as usize);
                    } else {
                        factors.insert(power.argument(), (value * -1) as usize);
                    }
                }
                _ => {}
            },
            _ => {}
        }

        factors.insert(expression.clone(), 1);
        return FactorDecomposition { factors: factors };
    }

    pub fn as_expression(self) -> Expression {
        return Expression::multiplication(
            self.factors
                .iter()
                .map(|(factor, multiplicity)| {
                    factor * Symbol::integer(*multiplicity as isize).expr()
                })
                .collect(),
        );
    }

    pub fn lcm(elements: Vec<Self>) -> Self {
        elements
            .iter()
            .fold(Self::new(), |mut common_multiples, new_factors| {
                for (new_factor, new_factor_multiplicity) in new_factors.factors.iter() {
                    if common_multiples.factors.contains_key(new_factor) {
                        let exiting_multiplicity =
                            common_multiples.factors.remove(new_factor).unwrap();

                        common_multiples.factors.insert(
                            new_factor.clone(),
                            std::cmp::max(exiting_multiplicity, *new_factor_multiplicity),
                        );
                    } else {
                        common_multiples
                            .factors
                            .insert(new_factor.clone(), *new_factor_multiplicity);
                    }
                }
                return common_multiples;
            })
    }

    pub fn divides(&self, other: &FactorDecomposition) -> bool {
        for (factor, self_factor_multiplicity) in self.factors.iter() {
            if let Some(other_factor_multiplicity) = other.factors.get(factor) {
                if other_factor_multiplicity >= self_factor_multiplicity {
                    continue;
                }
                return false;
            }
            return false;
        }

        return true;
    }
}

impl std::ops::Div for FactorDecomposition {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut factor_list: HashMap<Expression, usize> = self.factors.clone();

        for (factor, other_factor_multiplicity) in other.factors.iter() {
            if let Some(self_factor_multiplicity) = self.factors.get(factor) {
                if other_factor_multiplicity == self_factor_multiplicity {
                    factor_list.remove(factor);
                } else if other_factor_multiplicity >= self_factor_multiplicity {
                    factor_list.insert(
                        factor.clone(),
                        self_factor_multiplicity - other_factor_multiplicity,
                    );
                } else {
                    factor_list.insert(
                        Symbol::integer(1).expr() / factor,
                        other_factor_multiplicity - self_factor_multiplicity,
                    );
                }
            }
            factor_list.insert(
                Symbol::integer(1).expr() / factor,
                *other_factor_multiplicity,
            );
        }

        return FactorDecomposition {
            factors: factor_list,
        };
    }
}

impl std::ops::Mul for FactorDecomposition {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut new_factors: HashMap<Expression, usize> = self.factors.clone();

        for (factor, other_factor_multiplicity) in other.factors.iter() {
            if let Some(self_factor_multiplicity) = self.factors.get(factor) {
                new_factors.insert(
                    factor.clone(),
                    self_factor_multiplicity + other_factor_multiplicity,
                );
            }
            new_factors.insert(factor.clone(), *other_factor_multiplicity);
        }

        return FactorDecomposition {
            factors: new_factors,
        };
    }
}

// ======================== //
//      Factorable Trait    //
// ======================== //
pub trait Factorable {
    fn gcd(items: Vec<Expression>) -> FactorDecomposition;
    fn lcm(items: Vec<Expression>) -> FactorDecomposition;
}

impl Factorable for Expression {
    fn gcd(items: Vec<Expression>) -> FactorDecomposition {
        let product = items
            .iter()
            .cloned()
            .map(|item| FactorDecomposition::from_expression(item))
            .fold(FactorDecomposition::new(), |acc, new| acc * new);

        let least_commmon_multiple = FactorDecomposition::lcm(
            items
                .iter()
                .cloned()
                .map(|item| FactorDecomposition::from_expression(item))
                .collect(),
        );

        return product / least_commmon_multiple;
    }

    fn lcm(items: Vec<Expression>) -> FactorDecomposition {
        FactorDecomposition::lcm(
            items
                .iter()
                .cloned()
                .map(|item| FactorDecomposition::from_expression(item))
                .collect(),
        )
    }
}

#[cfg(test)]
mod lossless_convertion_to_expression {
    use super::*;
    use crate::base::symbol::Symbol;

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();

        let trial = a * b * c;
        let test = FactorDecomposition::from_expression(trial.clone()).as_expression();

        assert_eq!(trial, test);
    }

    #[test]
    fn sample_2() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let three = &Symbol::real(3.0).expr();

        let trial = a * b.clone().pow(c.clone()) * c.clone().pow(three.clone());
        let test = FactorDecomposition::from_expression(trial.clone()).as_expression();

        assert_eq!(trial, test);
    }
}

#[cfg(test)]
mod divisibility {
    use super::*;

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();

        let dividend = FactorDecomposition::from_expression(a * b * c);
        let divisor = FactorDecomposition::from_expression(a * b);

        assert!(divisor.divides(&dividend));
    }

    #[test]
    fn sample_2() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let one = &Symbol::integer(1).expr();

        let dividend = FactorDecomposition::from_expression((a + b).pow(c + one));
        let divisor = FactorDecomposition::from_expression((a + b).pow(c.clone()));

        assert!(divisor.divides(&dividend));
    }
}
