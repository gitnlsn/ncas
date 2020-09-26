use crate::base::{commutative_association::CommutativeAssociation, expression::Expression};
use crate::manipulation::identifiable::{Identifiable, Identity};

use std::collections::BinaryHeap;

#[derive(std::fmt::Debug)]
pub struct Multiplication {
    items: BinaryHeap<Expression>,
}

impl Multiplication {
    pub fn new(factors: Vec<Expression>) -> Expression {
        use crate::symbols::number::Number;

        if factors.len() == 0 {
            return Number::new(1.0);
        }

        if factors.len() == 1 {
            let single_addend = factors.iter().cloned().next().unwrap();
            return single_addend;
        }

        let mut items_vec: BinaryHeap<Expression> = BinaryHeap::new();

        for addend in factors.iter() {
            match addend.id() {
                Identity::Multiplication => {
                    if let Expression::CommutativeAssociation(multiplication) = addend {
                        items_vec.append(
                            &mut multiplication
                                .items()
                                .iter()
                                .map(|item| item.clone())
                                .collect(),
                        );
                    }
                }
                Identity::Number => {
                    if let Expression::Symbol(number) = addend {
                        if number.value() == Some(1.0) || String::from("1").eq(&number.label()) {
                            continue;
                        }
                    }
                    items_vec.push(addend.clone());
                }
                _ => {
                    items_vec.push(addend.clone());
                }
            }
        }
        return Expression::CommutativeAssociation(Box::new(Self { items: items_vec }));
    }
}

impl CommutativeAssociation for Multiplication {
    fn items(&self) -> Vec<Expression> {
        let mut expressions = self.items.clone().into_sorted_vec();
        expressions.reverse();
        return expressions;
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
