use crate::manipulation::{
    differentiate::Differentiable,
    expand::Expandable,
    expression_measure::ExpressionMeasure,
    identifiable::{Identifiable},
    numeric_evaluation::NumericEvaluable,
};

use std::fmt::{Debug, Display};

/**
 * Minimal representative value
 */
pub trait Symbol: Debug + Display + NumericEvaluable + Identifiable {
    fn symbol_type(&self) -> SymbolType;
    fn label(&self) -> String;
    fn value(&self) -> Option<f64>;
    fn boxed_clone(&self) -> Box<dyn Symbol>;
}

pub enum SymbolType {
    Variable,
    Constant,
    Number,
}

impl Clone for Box<dyn Symbol> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
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
pub trait Association:
    Debug + Display + NumericEvaluable + Expandable + ExpressionMeasure + Identifiable
/* + Simplifiable + Sortable */
{
    fn right_hand_side(&self) -> &Box<Expression>;
    fn left_hand_side(&self) -> &Box<Expression>;
    fn boxed_clone(&self) -> Box<dyn Association>;
}

impl Clone for Box<dyn Association> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
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

impl Eq for Expression {}
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Symbol(s1), Expression::Symbol(s2)) => {
                return s1 == s2;
            }
            (Expression::Association(a1), Expression::Association(a2)) => {
                if a1.id() == a2.id() {
                    let a1_rhs = a1.right_hand_side();
                    let a1_lhs = a1.left_hand_side();
                    let a2_rhs = a2.right_hand_side();
                    let a2_lhs = a2.left_hand_side();

                    return (a1_lhs == a2_lhs && a1_rhs == a2_rhs)
                        || (a1_lhs == a2_rhs && a1_rhs == a2_lhs);
                }
                return false;
            }
            _ => {
                return false;
            }
        }
    }
}
