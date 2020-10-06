use crate::manipulation::{
    differentiate::Differentiable, expand::Expandable, identifiable::Identifiable,
    numeric_evaluation::NumericEvaluable, pattern_matchable::PatternMatchable,
    simplifiable::Simplifiable,
};

use crate::base::expression::Expression;
use std::fmt::{Debug, Display};

/**
 *  Associations between two Expressions.
 *      - associativity
 *      - no commutativity: left and right order matters
 *      
 *  Eg:
 *      - matrix multiplication
 *      - vector associations (inner product, cross product, )
 */
pub trait Association:
    Debug + Display + NumericEvaluable + Expandable + Identifiable + Simplifiable + PatternMatchable
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

use std::hash::{Hash, Hasher};
impl Hash for dyn Association {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
        self.left_hand_side().hash(state);
        self.right_hand_side().hash(state);
    }
}
