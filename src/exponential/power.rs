use crate::arithmetics::multiplication::Multiplication;
use crate::base::{associative_operation::AssociativeOperation, expression::Expression};
use crate::manipulation::identifiable::{Identifiable, Identity};
use crate::symbols::number::Number;

#[derive(std::fmt::Debug)]
pub struct Power {
    base: Box<Expression>,
    exp: Box<Expression>,
}

impl Power {
    /**
     *  Builds power associative operation
     *      - ignores neutral exponent
     */
    pub fn new(base: Expression, exp: Expression) -> Expression {
        match &exp {
            Expression::Symbol(s) => match (&exp).id() {
                Identity::Number => {
                    if s.value() == Some(1.0) || s.label().eq(&String::from("1")) {
                        return base.clone();
                    }
                    if s.value() == Some(0.0) || s.label().eq(&String::from("0")) {
                        return Number::new(1.0);
                    }
                    let exp_int_value = s.value().unwrap();
                    if exp_int_value.fract() == 0.0
                        && exp_int_value == (exp_int_value as isize) as f64
                    {
                        match &base {
                            Expression::CommutativeAssociation(factors_product) => match (&base)
                                .id()
                            {
                                Identity::Multiplication => {
                                    let produit: f64 = factors_product
                                        .items()
                                        .iter()
                                        .filter(|factor| factor.id() == Identity::Number)
                                        .fold(1.0, |acc, factor| {
                                            if let Expression::Symbol(s) = factor {
                                                return acc * s.value().unwrap();
                                            } else {
                                                panic!("Not expected to multiply non numeric");
                                            }
                                        });

                                    let produit = produit.powf(exp_int_value);

                                    let other_factors: Vec<Expression> = factors_product
                                        .items()
                                        .iter()
                                        .filter(|factor| factor.id() != Identity::Number)
                                        .cloned()
                                        .collect();

                                    if produit < 0.0 {
                                        return Multiplication::new(vec![
                                            Number::new(-1.0),
                                            Number::new(-produit),
                                            Expression::AssociativeOperation(Box::new(Self {
                                                base: Box::new(Multiplication::new(other_factors)),
                                                exp: Box::new(exp),
                                            })),
                                        ]);
                                    } else {
                                        return Multiplication::new(vec![
                                            Number::new(produit),
                                            Expression::AssociativeOperation(Box::new(Self {
                                                base: Box::new(Multiplication::new(other_factors)),
                                                exp: Box::new(exp),
                                            })),
                                        ]);
                                    }
                                }
                                _ => { /* nothing to do */ }
                            }, /* end - match commutativeOperation */
                            Expression::Symbol(s) => match (&base).id() {
                                Identity::Number => {
                                    /* Number against Number */
                                    let base_int_value = s.value().unwrap();
                                    return Number::new(base_int_value.powf(exp_int_value));
                                }
                                _ => {}
                            },
                            _ => { /* nothing to do */ }
                        }
                    }
                }
                _ => { /* nothing to do */ }
            },
            _ => { /* nothing to do */ }
        }
        return Expression::AssociativeOperation(Box::new(Self {
            base: Box::new(base),
            exp: Box::new(exp),
        }));
    }
}

impl AssociativeOperation for Power {
    fn argument(&self) -> &Box<Expression> {
        &self.base
    }
    fn modifier(&self) -> &Box<Expression> {
        &self.exp
    }
    fn boxed_clone(&self) -> Box<dyn AssociativeOperation> {
        Box::new(Self {
            base: self.base.clone(),
            exp: self.exp.clone(),
        })
    }
}

/**
 * Overloads plus (+) Operation
 */
impl std::ops::BitXor for Expression {
    type Output = Expression;
    fn bitxor(self, other: Expression) -> Expression {
        Power::new(self, other)
    }
}

impl std::ops::BitXor<&Expression> for Expression {
    type Output = Expression;
    fn bitxor(self, other: &Expression) -> Expression {
        Power::new(self, other.clone())
    }
}

impl std::ops::BitXor<&Expression> for &Expression {
    type Output = Expression;
    fn bitxor(self, other: &Expression) -> Expression {
        Power::new(self.clone(), other.clone())
    }
}

impl std::ops::BitXor<Expression> for &Expression {
    type Output = Expression;
    fn bitxor(self, other: Expression) -> Expression {
        Power::new(self.clone(), other.clone())
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ^ {})", self.base, self.exp)
    }
}
