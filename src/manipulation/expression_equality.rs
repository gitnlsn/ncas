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
            (Expression::Symbol(s1), Expression::Symbol(s2)) => {
                return s1 == s2;
            }
            (Expression::Association(a1), Expression::Association(a2)) => {
                return a1 == a2;
            }
            (Expression::AssociativeOperation(o1), Expression::AssociativeOperation(o2)) => {
                return o1 == o2;
            }
            (Expression::CommutativeAssociation(o1), Expression::CommutativeAssociation(o2)) => {
                return o1 == o2;
            }
            _ => {
                return false;
            }
        }
    }
}

// ========================== //
//      Symbolic Equality     //
// ========================== //
use crate::base::symbol::Symbol;
impl Eq for dyn Symbol {}
impl PartialEq for dyn Symbol {
    fn eq(&self, other: &dyn Symbol) -> bool {
        self.label().eq(&other.label()) && self.value() == other.value()
    }
}

// ============================================= //
//      Operations and Associations Equality     //
// ============================================= //
use crate::base::{
    association::Association, associative_operation::AssociativeOperation,
    commutative_association::CommutativeAssociation,
};

impl Eq for dyn Association {}
impl PartialEq for dyn Association {
    fn eq(&self, other: &dyn Association) -> bool {
        self.left_hand_side() == other.left_hand_side()
            && self.right_hand_side() == other.right_hand_side()
    }
}

impl Eq for dyn AssociativeOperation {}
impl PartialEq for dyn AssociativeOperation {
    fn eq(&self, other: &dyn AssociativeOperation) -> bool {
        self.argument() == other.argument() && self.modifier() == other.modifier()
    }
}

impl Eq for dyn CommutativeAssociation {}
impl PartialEq for dyn CommutativeAssociation {
    fn eq(&self, other: &dyn CommutativeAssociation) -> bool {
        if self.id() != other.id() {
            return false;
        }

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
