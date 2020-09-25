use crate::base::expression::{CommutativeAssociation, Expression};
use crate::manipulation::identifiable::{Identifiable, Identity};

#[derive(std::fmt::Debug)]
pub struct Addition {
    items: Vec<Expression>,
}

impl Addition {
    pub fn new(addends: Vec<Expression>) -> Expression {
        use crate::base::symbols::number::Number;
        if addends.len() == 0 {
            return Number::new(0.0);
        }

        if addends.len() == 1 {
            let single_addend = addends.iter().cloned().next().unwrap();
            return single_addend;
        }

        let mut items_vec: Vec<Expression> = Vec::new();

        for addend in addends.iter() {
            match addend.id() {
                Identity::Addition => {
                    if let Expression::CommutativeAssociation(addition) = addend {
                        items_vec.append(&mut addition.items().iter().cloned().collect());
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
            } /* match  */
        }
        return Expression::CommutativeAssociation(Box::new(Self { items: items_vec }));
    }
}

impl CommutativeAssociation for Addition {
    fn items(&self) -> &Vec<Expression> {
        &self.items
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
        let mut iterator = self.items.iter();
        if let Some(first_item) = iterator.next() {
            write!(f, "({}", first_item).expect("");
        }
        while let Some(item) = iterator.next() {
            write!(f, " + {}", item).expect("");
        }
        write!(f, ")")
    }
}
