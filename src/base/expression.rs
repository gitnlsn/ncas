use crate::manipulation::{
    differentiate::Differentiable, expand::Expandable, numeric_evaluation::NumericEvaluable,
};

use std::fmt::{Debug, Display};

/**
 * Minimal representative value
 */
pub trait Symbol: Debug + Display + NumericEvaluable {
    fn label(&self) -> String;
    fn value(&self) -> Option<f64>;
}

impl Clone for Box<dyn Symbol> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl Eq for dyn Symbol {}
impl PartialEq for dyn Symbol {
    fn eq(&self, other: &dyn Symbol) -> bool {
        self.label().eq(&other.label()) && self.value() == other.value()
    }
}

/**
 *  Associations between two Symbols
 *      - associativity
 */
pub trait Association: Debug + Display + NumericEvaluable + Expandable
/* + Simplifiable + Sortable */
{
    fn right_hand_side(&self) -> &Box<Expression>;
    fn left_hand_side(&self) -> &Box<Expression>;
}

impl Clone for Box<dyn Association> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

/**
 *  Operations applied on an Expression
 */
pub trait Operation /* Debug + Display + Evaluable + Expandable */ /* + Simplifiable + Sortable */ {
    fn argument(&self) -> &Box<Expression>;
}

/**
 * Operations applied on an Expression, given an Expresssion as parameter
 */
pub trait AssociativeOperation
/* : Evaluable + Display + Replaceable + Expandable + Simplifiable + Sortable */
{
    fn argument(&self) -> &Box<Expression>;
    fn modifier(&self) -> &Box<Expression>;
}

/**
 * Symbols related through composition of Associations, Operations and AssociativeOperations
 */
#[derive(std::fmt::Debug, Clone)]
pub enum Expression {
    // AssociativeOperation(Box<dyn AssociativeOperation>),
    // Operation(Box<dyn Operation>),
    Association(Box<dyn Association>),
    Symbol(Box<dyn Symbol>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Symbol(s) => std::fmt::Display::fmt(s, f),
            Expression::Association(a) => std::fmt::Display::fmt(a, f),
        }
    }
}
