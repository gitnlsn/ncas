use crate::manipulation::{
    differentiate::Differentiable, expand::Expandable, expression_measure::ExpressionMeasure,
    identifiable::Identifiable, numeric_evaluation::NumericEvaluable,
};

use crate::base::expression::Expression;
use std::fmt::{Debug, Display};

/**
 *  Associations between two Expressions.
 *      - associativity
 *      - no commuativity: left and right order matters
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
