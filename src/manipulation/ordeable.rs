use std::cmp::Ordering;

use crate::base::expression::Expression;

// ============================= //
//    Ordering for Expression    //
// ============================= //
impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expression::Power(p1), Expression::Power(p2)) => return p1.cmp(&p2),
            (Expression::Logarithm(l1), Expression::Logarithm(l2)) => return l1.cmp(&l2),
            (Expression::Multiplication(m1), Expression::Multiplication(m2)) => return m1.cmp(&m2),
            (Expression::Addition(a1), Expression::Addition(a2)) => return a1.cmp(&a2),
            (Expression::Variable(s1), Expression::Variable(s2)) => return s1.cmp(&s2),
            (Expression::Integer(s1), Expression::Integer(s2)) => return s1.cmp(&s2),
            (Expression::Real(s1), Expression::Real(s2)) => return s1.cmp(&s2),

            /* Single Symbols first */
            (Expression::Integer(_), _) => return Ordering::Less,
            (Expression::Real(_), _) => return Ordering::Less,
            (Expression::Variable(_), _) => return Ordering::Less,
            
            /* Associative Operations */
            (Expression::Power(_), _) => return Ordering::Less,
            (Expression::Logarithm(_), _) => return Ordering::Less,
            
            /* Addition and Multiplication should be merged inside commutation */
            (Expression::Addition(_), _) => return Ordering::Less,
            (Expression::Multiplication(_), _) => return Ordering::Less,
        }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Expression) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

// ============== //
//    Operation    //
// ============== //
use crate::base::operation::Operation;
impl Ord for Operation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.argument().cmp(&other.argument())
    }
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// // ================= //
// //    Association    //
// // ================= //
// use crate::base::association::Association;
// impl Ord for dyn Association {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match self.left_hand_side().cmp(&other.left_hand_side()) {
//             Ordering::Greater => return Ordering::Greater,
//             Ordering::Less => return Ordering::Less,
//             Ordering::Equal => {
//                 return self.right_hand_side().cmp(&other.right_hand_side());
//             }
//         }
//     }
// }

// impl PartialOrd for dyn Association {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// =========================== //
//    Associative Operation    //
// =========================== //
use crate::base::associative_operation::AssociativeOperation;
impl Ord for AssociativeOperation {
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
impl PartialOrd for AssociativeOperation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================= //
//    Commutative Association    //
// ============================= //
use crate::base::commutative_association::CommutativeAssociation;
impl Ord for CommutativeAssociation {
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
impl PartialOrd for CommutativeAssociation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
