use crate::base::expression::Expression;
use crate::manipulation::identifiable::Identity;

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
            Expression::CommutativeAssociation(association) => match association.id() {
                Identity::Multiplication => {
                    association.items().iter().cloned().for_each(|factor| {
                        factors.insert(factor, 1);
                    });
                    return FactorDecomposition { factors: factors };
                }
                _ => {}
            },
            Expression::AssociativeOperation(operation) => match operation.id() {
                Identity::Power => {
                    let exponent = operation.modifier().as_ref();
                    match exponent {
                        Expression::Symbol(s) => match s.id() {
                            Identity::Number => {
                                let value = s.value().unwrap();
                                if value > 0.0 && (value as usize) as f64 == value {
                                    factors.insert(
                                        operation.argument().as_ref().clone(),
                                        value as usize,
                                    );
                                    return FactorDecomposition { factors: factors };
                                }
                            }
                            _ => {}
                        },
                        _ => {}
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
        use crate::arithmetics::multiplication::Multiplication;
        return Multiplication::new(
            self.factors
                .iter()
                .map(|(factor, multiplicity)| factor * (*multiplicity as isize))
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
                        1 / factor,
                        other_factor_multiplicity - self_factor_multiplicity,
                    );
                }
            }
            factor_list.insert(1 / factor, *other_factor_multiplicity);
        }

        return FactorDecomposition {
            factors: factor_list,
        };
    }
}

impl std::ops::Div<&FactorDecomposition> for FactorDecomposition {
    type Output = FactorDecomposition;
    fn div(self, other: &FactorDecomposition) -> FactorDecomposition {
        self / other.clone()
    }
}

impl std::ops::Div<FactorDecomposition> for &FactorDecomposition {
    type Output = FactorDecomposition;
    fn div(self, other: FactorDecomposition) -> FactorDecomposition {
        self.clone() / other
    }
}

impl std::ops::Div<&FactorDecomposition> for &FactorDecomposition {
    type Output = FactorDecomposition;
    fn div(self, other: &FactorDecomposition) -> FactorDecomposition {
        self.clone() / other.clone()
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

impl std::ops::Mul<&FactorDecomposition> for FactorDecomposition {
    type Output = FactorDecomposition;
    fn mul(self, other: &FactorDecomposition) -> FactorDecomposition {
        self * other.clone()
    }
}

impl std::ops::Mul<FactorDecomposition> for &FactorDecomposition {
    type Output = FactorDecomposition;
    fn mul(self, other: FactorDecomposition) -> FactorDecomposition {
        self.clone() * other
    }
}

impl std::ops::Mul<&FactorDecomposition> for &FactorDecomposition {
    type Output = FactorDecomposition;
    fn mul(self, other: &FactorDecomposition) -> FactorDecomposition {
        self.clone() * other.clone()
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
    use crate::symbols::{number::Number, variable::Variable};

    #[test]
    fn sample_1() {
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        let trial = a * b * c;
        let test = FactorDecomposition::from_expression(trial.clone()).as_expression();

        assert_eq!(trial, test);
    }

    #[test]
    fn sample_2() {
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        let trial = a * b.pow(&c) * c.pow(&Number::new(3.0));
        let test = FactorDecomposition::from_expression(trial.clone()).as_expression();

        assert_eq!(trial, test);
    }
}

#[cfg(test)]
mod divisibility {

    use super::*;
    use crate::symbols::{number::Number, variable::Variable};

    #[test]
    fn sample_1() {
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        let dividend = FactorDecomposition::from_expression(a * b * c);
        let divisor = FactorDecomposition::from_expression(a * b);

        assert!(divisor.divides(&dividend));
    }

    #[test]
    fn sample_2() {
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        let dividend = FactorDecomposition::from_expression((a + b).pow(&(c + 1)));
        let divisor = FactorDecomposition::from_expression((a + b).pow(&(c)));

        assert!(divisor.divides(&dividend));
    }
}
