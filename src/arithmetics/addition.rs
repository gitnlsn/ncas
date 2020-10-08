use crate::base::{commutative_association::CommutativeAssociation, expression::Expression};
use crate::manipulation::identifiable::{Identifiable, Identity};

use std::collections::BinaryHeap;

#[derive(std::fmt::Debug)]
pub struct Addition {
    items: BinaryHeap<Expression>,
}

impl Addition {
    pub fn new(addends: Vec<Expression>) -> Expression {
        use crate::symbols::number::Number;
        let addends: Vec<Expression> = addends
            .iter()
            .cloned()
            .filter(|addend| addend != &Number::new(0.0))
            .collect();

        if addends.len() == 0 {
            return Number::new(0.0);
        }

        if addends.len() == 1 {
            let single_addend = addends.iter().cloned().next().unwrap();
            return single_addend;
        }

        let mut pending_addends: Vec<Expression> = addends.iter().cloned().collect();
        let mut items_vec: BinaryHeap<Expression> = BinaryHeap::new();

        while !pending_addends.is_empty() {
            let addend = &pending_addends.pop().unwrap();
            match addend.id() {
                Identity::Addition => {
                    if let Expression::CommutativeAssociation(addition) = addend {
                        pending_addends.append(&mut addition.items().iter().cloned().collect());
                    }
                }
                Identity::Number => {
                    if let Expression::Symbol(number) = addend {
                        if number.value() == Some(0.0) || String::from("0").eq(&number.label()) {
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

impl CommutativeAssociation for Addition {
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
impl std::ops::Add for Expression {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Addition::new(vec![self, other])
    }
}

impl std::ops::Add<&Expression> for Expression {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        Addition::new(vec![self, other.clone()])
    }
}

impl std::ops::Add<&Expression> for &Expression {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        Addition::new(vec![self.clone(), other.clone()])
    }
}

impl std::ops::Add<Expression> for &Expression {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Addition::new(vec![self.clone(), other])
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Addition {
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
            write!(f, " + {}", item).expect("");
        }
        write!(f, ")")
    }
}
