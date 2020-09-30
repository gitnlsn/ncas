/**
 *  Simplication should return an smaller Expression that satisfies normal equality.
 *  
 */
pub trait Simplifiable {
    fn simplify(&self) -> Expression;
    fn simplification_alternatives(&self) -> Vec<Expression>;
    fn simplify_identity(&self) -> Expression;
}
use crate::exponential::power::Power;

// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::base::expression::Expression;
impl Simplifiable for Expression {
    fn simplification_alternatives(&self) -> Vec<Expression> {
        match self {
            Expression::Symbol(_) => Vec::new(),
            Expression::Association(_) => Vec::new(),
            Expression::AssociativeOperation(s) => s.simplification_alternatives(),
            Expression::CommutativeAssociation(s) => s.simplification_alternatives(),
        }
    }
    fn simplify_identity(&self) -> Expression {
        match self {
            Expression::Symbol(_) => self.clone(),
            Expression::Association(_) => self.clone(),
            Expression::AssociativeOperation(s) => s.simplify_identity(),
            Expression::CommutativeAssociation(s) => s.simplify_identity(),
        }
    }
    fn simplify(&self) -> Expression {
        match self {
            Expression::Symbol(_) => self.clone(),
            Expression::Association(_) => self.clone(),
            Expression::AssociativeOperation(s) => s.simplify(),
            Expression::CommutativeAssociation(s) => s.simplify(),
        }
    }
}

// =================================== //
//              Arithmetics            //
// =================================== //
use crate::base::commutative_association::CommutativeAssociation;
use std::collections::HashMap;

use crate::arithmetics::addition::Addition;
impl Simplifiable for Addition {
    fn simplify(&self) -> Expression {
        self.simplify_identity()
    }
    fn simplification_alternatives(&self) -> Vec<Expression> {
        Vec::new()
    }
    fn simplify_identity(&self) -> Expression {
        let mut opposite_list: HashMap<Expression, Expression> = HashMap::new();
        for addend in self.items().iter().map(|item| item.simplify_identity()) {
            // println!("\n{}", addend);
            // for (key, value) in opposite_list.iter() {
            //     println!("{} -> {}", key, value);
            // }
            if opposite_list.contains_key(&addend) {
                opposite_list.remove(&addend);
            } else {
                opposite_list.insert((-1 * addend.clone()).simplify(), addend.clone());
            }
        }
        // for (key, value) in opposite_list.iter() {
        //     println!("{} -> {}", key, value);
        // }
        return Addition::new(opposite_list.values().cloned().collect());
    }
}

use crate::arithmetics::multiplication::Multiplication;
impl Simplifiable for Multiplication {
    fn simplify(&self) -> Expression {
        self.simplify_identity()
    }
    fn simplification_alternatives(&self) -> Vec<Expression> {
        Vec::new()
    }
    fn simplify_identity(&self) -> Expression {
        let mut inverse_list: HashMap<Expression, (Expression, usize)> = HashMap::new();
        for addend in self.items().iter().map(|item| item.simplify_identity()) {
            let inverse = (1 / addend.clone()).simplify();
            if inverse_list.contains_key(&addend) {
                let (expression, counter) = inverse_list.remove(&addend).unwrap();
                if counter > 0 {
                    inverse_list.insert(addend, (expression, counter - 1));
                }
            } else if inverse_list.contains_key(&inverse) {
                let (expression, counter) = inverse_list.remove(&inverse).unwrap();
                inverse_list.insert(inverse, (expression, counter + 1));
            } else {
                inverse_list.insert(inverse, (addend, 1));
            }
        }
        let factors: Vec<Expression> = inverse_list
            .values()
            .filter(|(_, counter)| *counter != 0)
            .map(|(expression, counter)| {
                if *counter == 1 {
                    return expression.clone();
                } else {
                    Power::new(expression.clone(), Number::new(*counter as f64))
                }
            })
            .collect();
        return Multiplication::new(factors);
    }
}

// =================================== //
//              Exponential            //
// =================================== //
use crate::base::associative_operation::AssociativeOperation;
use crate::manipulation::identifiable::{Identifiable, Identity};

impl Simplifiable for Power {
    fn simplify(&self) -> Expression {
        self.simplify_identity()
    }
    fn simplification_alternatives(&self) -> Vec<Expression> {
        Vec::new()
    }
    fn simplify_identity(&self) -> Expression {
        match self.argument().id() {
            Identity::Logarithm => {
                if let Expression::AssociativeOperation(logarithm) = self.argument().as_ref() {
                    if logarithm.modifier() == self.modifier() {
                        return Number::new(1.0);
                    }
                }
                return Expression::AssociativeOperation(self.boxed_clone());
            }
            _ => return Expression::AssociativeOperation(self.boxed_clone()),
        }
    }
}

use crate::exponential::logarithm::Log;
use crate::symbols::number::Number;
impl Simplifiable for Log {
    fn simplify(&self) -> Expression {
        self.simplify_identity()
    }
    fn simplification_alternatives(&self) -> Vec<Expression> {
        Vec::new()
    }
    fn simplify_identity(&self) -> Expression {
        match self.argument().id() {
            Identity::Power => {
                if let Expression::AssociativeOperation(power) = self.argument().as_ref() {
                    if power.modifier() == self.modifier() {
                        return Number::new(1.0);
                    }
                }
                return Expression::AssociativeOperation(self.boxed_clone());
            }
            _ => return Expression::AssociativeOperation(self.boxed_clone()),
        }
    }
}
