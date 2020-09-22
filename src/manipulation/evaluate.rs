use crate::base::expression::Expression;

/**
 * Expression evaluation will perform all operations expecting a f64 numeric value.
 * It returns the Err with the Expression if the expression is not evaluable.
 */
pub trait Evaluable {
    fn evaluate(&mut self) -> Result<f64, Expression>;
}

impl Evaluable for Expression {
    fn evaluate(&mut self) -> Result<f64, Expression> {
        match self {
            Expression::Symbol(symbol) => symbol.evaluate(),
            Expression::Association(association) => association.evaluate()
        }
    }
}
