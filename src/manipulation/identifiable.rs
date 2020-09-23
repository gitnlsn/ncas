use crate::{
    arithmetics::{
        addition::Addition, division::Division, multiplication::Multiplication,
        subtraction::Subtraction,
    },
    base::symbols::{constant::Constant, number::Number, variable::Variable},
};

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
}

pub trait Identifiable {
    fn id(&self) -> Identity;
}

// ======================= //
//          Symbols        //
// ======================= //
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
