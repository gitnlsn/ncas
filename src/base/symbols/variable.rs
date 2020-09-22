use crate::manipulation::evaluate::Evaluable;

use crate::base::expression::{Expression, Symbol};

/**
 * Symbol implementation
 */
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct Variable {
    label: String,
    value: Option<f64>,
}

impl Variable {
    pub fn new(label: String) -> Expression {
        Expression::Symbol(Box::new(Self {
            label: label,
            value: None,
        }))
    }
}

/**
 * Symbol Implementation
 */
impl Symbol for Variable {
    fn label(&self) -> String {
        self.label.clone()
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
}

impl Evaluable for Variable {
    fn evaluate(&mut self) -> Result<f64, Expression> {
        match self.value {
            Some(value) => return Ok(value),
            None => return Err(Expression::Symbol(Box::new(self.clone()))),
        }
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
