/*
    TODO:
        - this is a temporary definition
        - Implement a proper expression comparison, considering simplification
        - assert_eq!(Number::new(1) + Number::new(1), Number::new(2)) fails
*/
use crate::base::expression::Expression;
impl Eq for Expression {}
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Integer(s1), Expression::Integer(s2)) => s1 == s2,
            (Expression::Real(s1), Expression::Real(s2)) => s1 == s2,
            (Expression::Variable(s1), Expression::Variable(s2)) => s1 == s2,

            (Expression::Addition(a1), Expression::Addition(a2)) => a1 == a2,
            (Expression::Multiplication(m1), Expression::Multiplication(m2)) => m1 == m2,

            (Expression::Power(m1), Expression::Power(m2)) => m1 == m2,
            (Expression::Logarithm(m1), Expression::Logarithm(m2)) => m1 == m2,
            _ => false,
        }
    }
}

// // ============================================= //
// //      Operations and Associations Equality     //
// // ============================================= //
// use crate::base::association::Association;
// impl Eq for dyn Association {}
// impl PartialEq for dyn Association {
//     fn eq(&self, other: &dyn Association) -> bool {
//         if self.id() != other.id() {
//             return false;
//         }
//         let same_rhs = self.right_hand_side() == other.right_hand_side();
//         let same_lhs = self.left_hand_side() == other.left_hand_side();
//         return same_rhs && same_lhs;
//     }
// }

use crate::base::operation::Operation;
impl Eq for Operation {}
impl PartialEq for Operation {
    fn eq(&self, other: &Operation) -> bool {
        let same_argument = self.argument() == other.argument();
        return same_argument;
    }
}

use crate::base::associative_operation::AssociativeOperation;
impl Eq for AssociativeOperation {}
impl PartialEq for AssociativeOperation {
    fn eq(&self, other: &AssociativeOperation) -> bool {
        let same_argument = self.argument() == other.argument();
        let same_modifier = self.modifier() == other.modifier();
        return same_argument && same_modifier;
    }
}

use crate::base::commutative_association::CommutativeAssociation;
impl Eq for CommutativeAssociation {}
impl PartialEq for CommutativeAssociation {
    fn eq(&self, other: &CommutativeAssociation) -> bool {
        if self.items().len() != other.items().len() {
            return false;
        }

        let self_items = self.items();
        let other_items = other.items();

        let mut self_items_iter = self_items.iter();
        let mut other_items_iter = other_items.iter();

        while let (Some(self_item), Some(other_item)) =
            (self_items_iter.next(), other_items_iter.next())
        {
            if self_item != other_item {
                return false;
            }
        }

        return true;
    }
}
