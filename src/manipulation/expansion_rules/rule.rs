use crate::base::expression::Expression;

/**
 *  Non recursive Expansion Rule
 *      - it won't expand further than 1 Node deep
 */
pub trait Rule {
    fn apply(expression: Expression) -> Expression;
}
