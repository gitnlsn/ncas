use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::base::{
    association::Association, associative_operation::AssociativeOperation,
    commutative_association::CommutativeAssociation, symbol::Symbol,
};

/**
 * Symbols related through composition of Associations, Operations and AssociativeOperations
 */
#[derive(Debug, Clone, Hash)]
pub enum Expression {
    // Operation(Box<dyn Operation>),
    CommutativeAssociation(Box<dyn CommutativeAssociation>),
    AssociativeOperation(Box<dyn AssociativeOperation>),
    Association(Box<dyn Association>),
    Symbol(Box<dyn Symbol>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Symbol(s) => std::fmt::Display::fmt(s, f),
            Expression::Association(a) => std::fmt::Display::fmt(a, f),
            Expression::AssociativeOperation(op) => std::fmt::Display::fmt(op, f),
            Expression::CommutativeAssociation(ca) => std::fmt::Display::fmt(ca, f),
        }
    }
}
