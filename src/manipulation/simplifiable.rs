/**
 *  Simplication should return an smaller Expression that satisfies normal equality.
 */
pub trait Simplifiable {
    fn simplify(&self) -> Expression;
}

// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::base::expression::Expression;
impl Simplifiable for Expression {
    fn simplify(&self) -> Expression {
        let mut simplified_expression: Expression;
        match self {
            Expression::Symbol(_) => self.clone(),
            Expression::Association(s) => {
                simplified_expression = s.simplify();
                if self == &simplified_expression {
                    return simplified_expression;
                }
                loop {
                    let possible_simplification = simplified_expression.simplify();
                    if possible_simplification == simplified_expression {
                        break;
                    }
                    simplified_expression = possible_simplification;
                }
                return simplified_expression;
            }
            Expression::AssociativeOperation(s) => {
                simplified_expression = s.simplify();
                if self == &simplified_expression {
                    return simplified_expression;
                }
                loop {
                    let possible_simplification = simplified_expression.simplify();
                    if possible_simplification == simplified_expression {
                        break;
                    }
                    simplified_expression = possible_simplification;
                }
                return simplified_expression;
            }
            Expression::CommutativeAssociation(s) => {
                simplified_expression = s.simplify();
                if self == &simplified_expression {
                    return simplified_expression;
                }
                loop {
                    let possible_simplification = simplified_expression.simplify();
                    if possible_simplification == simplified_expression {
                        break;
                    }
                    simplified_expression = possible_simplification;
                }
                return simplified_expression;
            }
        }
    }
}

impl Expression {
    fn simplify_inner(&self) -> Expression {
        match self {
            Expression::Symbol(_) => self.clone(),
            Expression::Association(s) => {
                /* TODO: handle future association identities */
                // let lhs = s.left_hand_side().simplify();
                // let rhs = s.right_hand_side().simplify();
                // match self.id() {
                //     Identity::*** => {
                //     }
                // }
                return s.simplify();
            }
            Expression::AssociativeOperation(operation) => match self.id() {
                Identity::Power => {
                    let base = operation.argument().simplify();
                    let expo = operation.modifier().simplify();
                    return Power::new(base, expo);
                }
                Identity::Logarithm => {
                    let argument = operation.argument().simplify();
                    let base = operation.modifier().simplify();
                    return Log::new(argument, base);
                }
                _ => {
                    panic!("Not expected identity at associative operation");
                }
            },
            Expression::CommutativeAssociation(association) => match self.id() {
                Identity::Addition => {
                    return Addition::new(
                        association
                            .items()
                            .iter()
                            .map(|addend| addend.simplify())
                            .collect(),
                    );
                }
                Identity::Multiplication => {
                    return Multiplication::new(
                        association
                            .items()
                            .iter()
                            .map(|factor| factor.simplify())
                            .collect(),
                    );
                }
                _ => {
                    panic!("Not expected identity at associative operation");
                }
            },
        }
    }
}

// =================================== //
//              Arithmetics            //
// =================================== //
use crate::base::commutative_association::CommutativeAssociation;
use crate::manipulation::identifiable::{Identifiable, Identity};
use crate::manipulation::simplification_rules::{
    identities::{
        additive_common_factor::AdditiveCommonFactor,
        multiplicative_common_factor::MultiplicativeCommonFactor,
    },
    rule::Rule,
};

use crate::arithmetics::addition::Addition;
impl Simplifiable for Addition {
    fn simplify(&self) -> Expression {
        let mut alternative_list =
            AdditiveCommonFactor::apply(&Expression::CommutativeAssociation(self.boxed_clone()))
                .iter()
                .map(|possible_simplification| possible_simplification.simplify_inner())
                .collect::<HashSet<Expression>>()
                .iter()
                .cloned()
                .collect::<BinaryHeap<Expression>>();

        if alternative_list.is_empty() {
            return Expression::CommutativeAssociation(self.boxed_clone());
        }

        return alternative_list.pop().unwrap();
    }
}

use crate::arithmetics::multiplication::Multiplication;
impl Simplifiable for Multiplication {
    fn simplify(&self) -> Expression {
        let mut alternative_list = MultiplicativeCommonFactor::apply(
            &Expression::CommutativeAssociation(self.boxed_clone()),
        )
        .iter()
        .map(|possible_expression| possible_expression.simplify_inner())
        .collect::<HashSet<Expression>>()
        .iter()
        .cloned()
        .collect::<BinaryHeap<Expression>>();

        if alternative_list.is_empty() {
            return Expression::CommutativeAssociation(self.boxed_clone());
        }

        return alternative_list.pop().unwrap();
    }
}

// =================================== //
//              Exponential            //
// =================================== //
use crate::base::associative_operation::AssociativeOperation;
use crate::manipulation::simplification_rules::identities::inverse_power_log::InversePowerLog;
use std::collections::{BinaryHeap, HashSet};

use crate::exponential::power::Power;
impl Simplifiable for Power {
    fn simplify(&self) -> Expression {
        let mut alternative_list =
            InversePowerLog::apply(&Expression::AssociativeOperation(self.boxed_clone()))
                .iter()
                .map(|possible_expression| possible_expression.simplify_inner())
                .collect::<HashSet<Expression>>()
                .iter()
                .cloned()
                .collect::<BinaryHeap<Expression>>();

        if alternative_list.is_empty() {
            return Expression::AssociativeOperation(self.boxed_clone());
        }

        return alternative_list.pop().unwrap();
    }
}

use crate::exponential::logarithm::Log;
impl Simplifiable for Log {
    fn simplify(&self) -> Expression {
        let mut alternative_list =
            InversePowerLog::apply(&Expression::AssociativeOperation(self.boxed_clone()))
                .iter()
                .map(|possible_expression| possible_expression.simplify_inner())
                .collect::<HashSet<Expression>>()
                .iter()
                .cloned()
                .collect::<BinaryHeap<Expression>>();

        if alternative_list.is_empty() {
            return Expression::AssociativeOperation(self.boxed_clone());
        }

        return alternative_list.pop().unwrap();
    }
}
