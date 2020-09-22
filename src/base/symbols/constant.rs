use crate::base::expression::{Expression, Symbol};

/**
 * Symbol implementation
 */
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct Constant {
    label: String,
    value: Option<f64>,
}

impl Constant {
    pub fn new(label: String, value: f64) -> Expression {
        Expression::Symbol(Box::new(Self {
            label: label,
            value: Some(value),
        }))
    }
}

/**
 * Symbol Implementation
 */
impl Symbol for Constant {
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
impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
