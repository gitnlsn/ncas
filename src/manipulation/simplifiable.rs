/**
 *  Simplication should return an smaller Expression that satisfies normal equality.
 *  
 */
pub trait Simplifiable {
    fn simplify(&self) -> Expression;
}
use crate::exponential::power::Power;

// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::base::expression::Expression;
impl Simplifiable for Expression {
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
        let mut numerical: Option<f64> = None;
        let mut opposite_list: HashMap<Expression, (Expression, usize)> = HashMap::new();
        for addend in self.items().iter().map(|item| item.simplify()) {
            let opposite = (-addend.clone()).simplify();

            if addend.id() == Identity::Number {
                /* accumulates number value */
                if let Expression::Symbol(symbolic_number) = addend {
                    if let Some(number) = numerical {
                        numerical = Some(number + symbolic_number.value().unwrap());
                    } else {
                        numerical = Some(symbolic_number.value().unwrap());
                    }
                }
            } else if opposite_list.contains_key(&addend) {
                /* cancels opposite addend  */
                let (expression, counter) = opposite_list.remove(&addend).unwrap();
                if counter > 0 {
                    opposite_list.insert(addend, (expression, counter - 1));
                }
            } else if opposite_list.contains_key(&opposite) {
                /* increases equal addend  */
                let (expression, counter) = opposite_list.remove(&opposite).unwrap();
                opposite_list.insert(opposite, (expression, counter + 1));
            } else {
                /* includes new addend  */
                opposite_list.insert(opposite, (addend, 1));
            }
        }

        let mut addends: Vec<Expression> = opposite_list
            .values()
            .filter(|(_, counter)| *counter > 0)
            .map(|(expression, counter)| {
                if *counter == 1 {
                    return expression.clone();
                } else {
                    Multiplication::new(vec![expression.clone(), Number::new(*counter as f64)])
                        .simplify()
                }
            })
            .collect();

        if let Some(number) = numerical {
            addends.push(Number::new(number));
        }

        return Addition::new(addends);
    }
}

use crate::arithmetics::multiplication::Multiplication;
impl Simplifiable for Multiplication {
    fn simplify(&self) -> Expression {
        let mut numerical: Option<f64> = None;
        let mut inverse_list: HashMap<Expression, (Expression, usize)> = HashMap::new();
        for addend in self.items().iter().map(|item| item.simplify()) {
            let inverse = (1 / addend.clone()).simplify();
            if addend.id() == Identity::Number {
                /* accumulates number value */
                if let Expression::Symbol(symbolic_number) = addend {
                    if let Some(number) = numerical {
                        numerical = Some(number * symbolic_number.value().unwrap());
                    } else {
                        numerical = Some(symbolic_number.value().unwrap());
                    }
                }
            } else if inverse_list.contains_key(&addend) {
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

        let mut factors: Vec<Expression> = inverse_list
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

        if let Some(number) = numerical {
            if number < 0.0 {
                factors.push(Number::new(number * -1.0));
                factors.push(Number::new(-1.0));
            } else if number > 0.0 {
                factors.push(Number::new(number));
            }
        }

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
        match self.argument().id() {
            Identity::Logarithm => {
                if let Expression::AssociativeOperation(logarithm) = self.argument().as_ref() {
                    if logarithm.modifier() == self.modifier() {
                        return logarithm.argument().as_ref().clone();
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
        match self.argument().id() {
            Identity::Power => {
                if let Expression::AssociativeOperation(power) = self.argument().as_ref() {
                    if power.modifier() == self.modifier() {
                        return power.argument().as_ref().clone();
                    }
                }
                return Expression::AssociativeOperation(self.boxed_clone());
            }
            _ => return Expression::AssociativeOperation(self.boxed_clone()),
        }
    }
}
