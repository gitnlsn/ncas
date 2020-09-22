use crate::base::{
    expression::{Association, Expression, Symbol},
    symbols::{constant::Constant, number::Number, variable::Variable},
};

use crate::arithmetics::{
    addition::Addition, division::Division, multiplication::Multiplication,
    subtraction::Subtraction,
};

/**
 * Expression evaluation
 */
pub trait NumericEvaluable {
    fn into_num(&self) -> Result<f64, Expression>;
}

// =================================== //
//      Recursion on Expression        //
// =================================== //
impl NumericEvaluable for Expression {
    fn into_num(&self) -> Result<f64, Expression> {
        match self {
            Expression::Symbol(symbol) => symbol.into_num(),
            Expression::Association(association) => association.into_num(),
        }
    }
}

// =============================== //
//              Symbols            //
// =============================== //
impl NumericEvaluable for Constant {
    fn into_num(&self) -> Result<f64, Expression> {
        match self.value() {
            Some(value) => return Ok(value),
            None => return Err(Expression::Symbol(Box::new(self.clone()))),
        }
    }
}

impl NumericEvaluable for Number {
    fn into_num(&self) -> Result<f64, Expression> {
        Ok(self.value().expect("Expected number to hold a f64 value"))
    }
}

impl NumericEvaluable for Variable {
    fn into_num(&self) -> Result<f64, Expression> {
        match self.value() {
            Some(value) => return Ok(value),
            None => return Err(Expression::Symbol(Box::new(self.clone()))),
        }
    }
}


// =================================== //
//              Arithmetics            //
// =================================== //
impl NumericEvaluable for Addition {
    fn into_num(&self) -> Result<f64, Expression> {
        let possible_lhs_value = match self.left_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        let possible_rhs_value = match self.right_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        if possible_lhs_value.is_ok() && possible_rhs_value.is_ok() {
            return Ok(possible_lhs_value.unwrap() + possible_rhs_value.unwrap());
        }

        if possible_lhs_value.is_err() {
            return possible_lhs_value;
        } else {
            return possible_rhs_value;
        }
    }
}

impl NumericEvaluable for Subtraction {
    fn into_num(&self) -> Result<f64, Expression> {
        let possible_lhs_value = match self.left_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        let possible_rhs_value = match self.right_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        if possible_lhs_value.is_ok() && possible_rhs_value.is_ok() {
            return Ok(possible_lhs_value.unwrap() - possible_rhs_value.unwrap());
        }

        if possible_lhs_value.is_err() {
            return possible_lhs_value;
        } else {
            return possible_rhs_value;
        }
    }
}

impl NumericEvaluable for Multiplication {
    fn into_num(&self) -> Result<f64, Expression> {
        let possible_lhs_value = match self.left_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        let possible_rhs_value = match self.right_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        if possible_lhs_value.is_ok() && possible_rhs_value.is_ok() {
            return Ok(possible_lhs_value.unwrap() * possible_rhs_value.unwrap());
        }

        if possible_lhs_value.is_err() {
            return possible_lhs_value;
        } else {
            return possible_rhs_value;
        }
    }
}

impl NumericEvaluable for Division {
    fn into_num(&self) -> Result<f64, Expression> {
        let possible_lhs_value = match self.left_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        let possible_rhs_value = match self.right_hand_side().as_ref() {
            Expression::Association(association) => association.into_num(),
            Expression::Symbol(symbol) => symbol.into_num(),
        };

        if possible_lhs_value.is_ok() && possible_rhs_value.is_ok() {
            return Ok(possible_lhs_value.unwrap() / possible_rhs_value.unwrap());
        }

        if possible_lhs_value.is_err() {
            return possible_lhs_value;
        } else {
            return possible_rhs_value;
        }
    }
}
