/**
 * Identity for Expression Nodes
 */
#[derive(Debug, Eq, PartialEq)]
pub enum Identity {
    Number,
    Variable,
    Constant,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
}

pub trait Identifiable {
    fn id(&self) -> Identity;
}

use crate::base::expression::Expression;
impl Identifiable for Expression {
    fn id(&self) -> Identity {
        match self {
            Expression::Symbol(s) => s.id(),
            Expression::Association(a) => a.id(),
            Expression::AssociativeOperation(op) => op.id(),
            Expression::CommutativeAssociation(op) => op.id(),
        }
    }
}

// ======================= //
//          Symbols        //
// ======================= //
use crate::base::symbols::{constant::Constant, number::Number, variable::Variable};

impl Identifiable for Constant {
    fn id(&self) -> Identity {
        Identity::Constant
    }
}
impl Identifiable for Number {
    fn id(&self) -> Identity {
        Identity::Number
    }
}
impl Identifiable for Variable {
    fn id(&self) -> Identity {
        Identity::Variable
    }
}

// =========================== //
//          Arithmetics        //
// =========================== //
use crate::arithmetics::{
    addition::Addition, division::Division, multiplication::Multiplication,
    subtraction::Subtraction,
};

impl Identifiable for Addition {
    fn id(&self) -> Identity {
        Identity::Addition
    }
}
impl Identifiable for Subtraction {
    fn id(&self) -> Identity {
        Identity::Subtraction
    }
}
impl Identifiable for Multiplication {
    fn id(&self) -> Identity {
        Identity::Multiplication
    }
}
impl Identifiable for Division {
    fn id(&self) -> Identity {
        Identity::Division
    }
}

// =========================== //
//          Exponential        //
// =========================== //
use crate::exponential::power::Power;

impl Identifiable for Power {
    fn id(&self) -> Identity {
        Identity::Power
    }
}
