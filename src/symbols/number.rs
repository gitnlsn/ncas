use crate::base::{expression::Expression, symbol::Symbol};

/**
 * Symbol implementation
 */
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct Number {
    label: String,
    value: Option<f64>,
}

impl Number {
    pub fn new(value: f64) -> Expression {
        Expression::Symbol(Box::new(Self {
            label: format!("{}", value),
            value: Some(value),
        }))
    }
}

/**
 * Symbol Implementation
 */
impl Symbol for Number {
    fn label(&self) -> String {
        self.label.clone()
    }
    fn value(&self) -> Option<f64> {
        self.value
    }
    fn boxed_clone(&self) -> Box<dyn Symbol> {
        Box::new(Self {
            label: self.label.clone(),
            value: self.value.clone(),
        })
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
