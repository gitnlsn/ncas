/*
    Ordering for sorting expressions inside CommutativeAssociations

    Precedence:
        - Expression::Symbol
        - Expression::Operation
        - Expression::Association
        - Expression::AssociativeOperation
        - Expression::CommutativeAssociation

*/
use std::cmp::{Ord, Ordering, PartialOrd};

use crate::base::expression::Expression;
use crate::manipulation::identifiable::{Identifiable, Identity};

// ============================= //
//      Ordering for Identity    //
// ============================= //
impl Identity {
    fn precedence(&self) -> usize {
        match self {
            /* symbols */
            Identity::Number => 110,
            Identity::Constant => 120,
            Identity::Variable => 130,

            /* Operations */
            /* sine */
            /* cossine */
            /* tangent */
            /* exp */
            /* ln */

            /* Associations */
            Identity::Subtraction => 210,
            Identity::Division => 220,

            /* AssociativeOperations */
            Identity::Power => 310,
            /* log */
            /* derivative */
            /* integral */

            /* CommutativeAssociations */
            Identity::Multiplication => 410,
            Identity::Addition => 420,
            /* Defaults to a high number */
            // _ => panic!(format!("Precedence is not implemented for {}", self)),
        }
    }
}

impl Ord for Identity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.precedence().cmp(&other.precedence())
    }
}

impl PartialOrd for Identity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================= //
//    Ordering for Expression    //
// ============================= //
impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_id = self.id();
        let other_id = other.id();

        if self_id != other_id {
            return self_id.cmp(&other_id);
        }

        match (self, other) {
            (Expression::Symbol(s1), Expression::Symbol(s2)) => return s1.cmp(&s2),
            (Expression::Association(s1), Expression::Association(s2)) => return s1.cmp(&s2),
            (Expression::AssociativeOperation(s1), Expression::AssociativeOperation(s2)) => {
                return s1.cmp(&s2)
            }
            (Expression::CommutativeAssociation(s1), Expression::CommutativeAssociation(s2)) => {
                return s1.cmp(&s2)
            }
            _ => panic!(format!("Not implemented ordering for {}", self.id())),
        }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Expression) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

// ============================ //
//      Ordering for Symbol     //
// ============================ //
use crate::base::symbol::Symbol;
impl Ord for dyn Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.id(), other.id()) {
            (Identity::Number, Identity::Number) => {
                if self.value() < other.value() {
                    return Ordering::Less;
                } else if self.value() > other.value() {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            }
            (Identity::Number, _) => return Ordering::Less,
            (_, Identity::Number) => return Ordering::Greater,
            _ => {
                return self.label().cmp(&other.label());
            }
        }
    }
}
impl PartialOrd for dyn Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ================= //
//    Association    //
// ================= //
use crate::base::association::Association;
impl Ord for dyn Association {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.left_hand_side().cmp(&other.left_hand_side()) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {
                return self.right_hand_side().cmp(&other.right_hand_side());
            }
        }
    }
}

impl PartialOrd for dyn Association {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =========================== //
//    Associative Operation    //
// =========================== //
use crate::base::associative_operation::AssociativeOperation;
impl Ord for dyn AssociativeOperation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.argument().cmp(&other.argument()) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {
                return self.modifier().cmp(&other.modifier());
            }
        }
    }
}
impl PartialOrd for dyn AssociativeOperation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================= //
//    Commutative Association    //
// ============================= //
use crate::base::commutative_association::CommutativeAssociation;
impl Ord for dyn CommutativeAssociation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.items().len().cmp(&other.items().len()) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {}
        }

        let self_items = self.items();
        let other_items = other.items();

        let mut self_items_iter = self_items.iter();
        let mut other_items_iter = other_items.iter();

        while let (Some(self_item), Some(other_item)) =
            (self_items_iter.next(), other_items_iter.next())
        {
            match self_item.cmp(other_item) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => continue,
            }
        }

        return Ordering::Equal;
    }
}
impl PartialOrd for dyn CommutativeAssociation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
