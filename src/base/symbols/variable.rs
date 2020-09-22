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

/*
    Debug implementation
*/
impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
