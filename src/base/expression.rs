use num::bigint::BigInt;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::base::{
    associative_operation::AssociativeOperation, commutative_association::CommutativeAssociation,
    /* operation::Operation, */
};

use crate::base::symbol::Symbol;

/**
 * Symbols related through composition of Associations, Operations and AssociativeOperations
 */
#[derive(Debug, Clone, Hash)]
pub enum Expression {
    /* Symbols */
    Variable(Symbol<String>),
    Real(Symbol<f64>),
    Integer(Symbol<BigInt>),

    /* Commutative associations */
    Multiplication(CommutativeAssociation),
    Addition(CommutativeAssociation),

    /* Associative Operations */
    Power(AssociativeOperation),
    Logarithm(AssociativeOperation),
    
    /* Single Operations */
}

/**
 * Display implementation for Expression
 */
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Integer(n) => {
                return write!(f, "{}", n.label());
            }
            Expression::Real(r) => {
                return write!(f, "{}", r.label());
            }
            Expression::Variable(v) => {
                return write!(f, "{}", v.label());
            }
            Expression::Power(p) => {
                return write!(f, "pow({}, {})", p.argument(), p.modifier());
            }
            Expression::Logarithm(l) => {
                return write!(f, "log({}, {})", l.argument(), l.modifier());
            }
            Expression::Addition(addition) => {
                if addition.items().is_empty() {
                    return write!(f, "");
                }
                let addition_items = addition.items();
                let mut iterator = addition_items.iter();
                if let Some(first_item) = iterator.next() {
                    write!(f, "({}", first_item).expect("");
                }
                while let Some(item) = iterator.next() {
                    write!(f, " + {}", item).expect("");
                }
                write!(f, ")")
            }
            Expression::Multiplication(multiplication) => {
                if multiplication.items().is_empty() {
                    return write!(f, "");
                }
                let multiplication_items = multiplication.items();
                let mut iterator = multiplication_items.iter();
                if let Some(first_item) = iterator.next() {
                    write!(f, "({}", first_item).expect("");
                }
                while let Some(item) = iterator.next() {
                    write!(f, " * {}", item).expect("");
                }
                write!(f, ")")
            }
        }
    }
}
