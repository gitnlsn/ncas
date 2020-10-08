use crate::base::{commutative_association::CommutativeAssociation, expression::Expression};
use crate::manipulation::identifiable::{Identifiable, Identity};

use crate::symbols::number::Number;
use std::collections::BinaryHeap;

#[derive(std::fmt::Debug)]
pub struct Multiplication {
    items: BinaryHeap<Expression>,
}

impl Multiplication {
    /**
     * Builds a multiplicative expression
     *  - ignores neutral element
     *  - separates sign with single -1.0 Number
     */
    pub fn new(factors: Vec<Expression>) -> Expression {
        let factors: Vec<Expression> = factors
            .iter()
            .cloned()
            .filter(|addend| addend != &Number::new(1.0))
            .collect();

        if factors.len() == 0 {
            return Number::new(1.0);
        }

        if factors.len() == 1 {
            let single_addend = factors.iter().cloned().next().unwrap();
            return single_addend;
        }

        let mut pending_factors: Vec<Expression> = factors.iter().cloned().collect();
        let mut items_vec: BinaryHeap<Expression> = BinaryHeap::new();
        let mut is_negative: bool = false;

        while !pending_factors.is_empty() {
            let factor = &pending_factors.pop().unwrap();
            match factor.id() {
                Identity::Multiplication => {
                    if let Expression::CommutativeAssociation(multiplication) = factor {
                        pending_factors
                            .append(&mut multiplication.items().iter().cloned().collect());
                    }
                }
                Identity::Number => {
                    if let Expression::Symbol(number) = factor {
                        if number.value() == Some(0.0) || String::from("0").eq(&number.label()) {
                            return Number::new(0.0);
                        } else if number.value() == Some(1.0)
                            || String::from("1").eq(&number.label())
                        {
                            continue;
                        } else if number.value() == Some(-1.0)
                            || String::from("-1").eq(&number.label())
                        {
                            is_negative = !is_negative;
                            continue;
                        } else if number.value().unwrap() < 0.0 {
                            is_negative = !is_negative;
                            items_vec.push(Number::new(number.value().unwrap() * -1.0));
                            continue;
                        } else {
                            items_vec.push(factor.clone());
                            continue;
                        }
                    }
                }
                _ => {
                    items_vec.push(factor.clone());
                }
            }
        }

        if is_negative {
            items_vec.push(Number::new(-1.0));
        }

        if items_vec.len() == 0 {
            return Number::new(1.0);
        }

        if items_vec.len() == 1 {
            return items_vec.pop().unwrap();
        }

        return Expression::CommutativeAssociation(Box::new(Self { items: items_vec }));
    }
}

impl CommutativeAssociation for Multiplication {
    fn items(&self) -> Vec<Expression> {
        self.items.clone().into_sorted_vec()
    }
    fn boxed_clone(&self) -> Box<dyn CommutativeAssociation> {
        Box::new(Self {
            items: self.items.clone(),
        })
    }
}

/**
 * Overloads plus (*) Operation
 */
impl std::ops::Mul for Expression {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        Multiplication::new(vec![self, other])
    }
}

impl std::ops::Mul<&Expression> for Expression {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        Multiplication::new(vec![self, other.clone()])
    }
}

impl std::ops::Mul<&Expression> for &Expression {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        Multiplication::new(vec![self.clone(), other.clone()])
    }
}

impl std::ops::Mul<Expression> for &Expression {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        Multiplication::new(vec![self.clone(), other])
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Multiplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.items().is_empty() {
            return write!(f, "");
        }
        let self_items = self.items();
        let mut iterator = self_items.iter();
        if let Some(first_item) = iterator.next() {
            write!(f, "({}", first_item).expect("");
        }
        while let Some(item) = iterator.next() {
            write!(f, " * {}", item).expect("");
        }
        write!(f, ")")
    }
}
