
use crate::base::expression::Expression;

/**
 *  Non recursive Transformation Rule
 *      - each possible transformation is included in the output Vec
 *      - if the rule is commutative, it will apply as much as possible leading to a single result
 *      - it won't search further than 1 Expression Node deep
 */
pub trait Rule {
    fn apply(expression: Expression) -> Vec<Expression>;
}
