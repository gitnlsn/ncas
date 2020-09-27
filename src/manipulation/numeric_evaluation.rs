/**
 * Expression evaluation
 */
pub trait NumericEvaluable {
    fn into_num(&self) -> Result<f64, Expression>;
}

// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::base::{
    association::Association, associative_operation::AssociativeOperation,
    commutative_association::CommutativeAssociation, expression::Expression, symbol::Symbol,
};
impl NumericEvaluable for Expression {
    fn into_num(&self) -> Result<f64, Expression> {
        match self {
            Expression::Symbol(symbol) => symbol.into_num(),
            Expression::Association(association) => association.into_num(),
            Expression::AssociativeOperation(op) => op.into_num(),
            Expression::CommutativeAssociation(op) => op.into_num(),
        }
    }
}

// =============================== //
//              Symbols            //
// =============================== //
use crate::symbols::{constant::Constant, number::Number, variable::Variable};

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
use crate::arithmetics::{
    addition::Addition, division::Division, multiplication::Multiplication,
    subtraction::Subtraction,
};

impl NumericEvaluable for Addition {
    fn into_num(&self) -> Result<f64, Expression> {
        let results: Vec<Result<f64, Expression>> = self
            .items()
            .iter()
            .map(|item| item.into_num()) /* Recursion: numeric evaluation */
            .collect();

        for res in results.iter() {
            if res.is_err() {
                return res.clone();
            }
        }

        return Ok(results
            .iter()
            .cloned()
            .map(|res| res.unwrap())
            .fold(0.0, |acc, new| acc + new));
    }
}

impl NumericEvaluable for Multiplication {
    fn into_num(&self) -> Result<f64, Expression> {
        let results: Vec<Result<f64, Expression>> = self
            .items()
            .iter()
            .map(|item| item.into_num()) /* Recursion: numeric evaluation */
            .collect();

        for res in results.iter() {
            if res.is_err() {
                return res.clone();
            }
        }

        return Ok(results
            .iter()
            .cloned()
            .map(|res| res.unwrap())
            .fold(1.0, |acc, new| acc * new));
    }
}

// =================================== //
//              Exponential            //
// =================================== //
use crate::exponential::power::Power;
impl NumericEvaluable for Power {
    fn into_num(&self) -> Result<f64, Expression> {
        let base = self.argument().into_num();
        let exp = self.modifier().into_num();

        if base.is_ok() && exp.is_ok() {
            return Ok(base.unwrap().powf(exp.unwrap()));
        }

        if base.is_err() {
            return base;
        } else {
            return exp;
        }
    }
}
