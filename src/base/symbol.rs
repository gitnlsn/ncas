use crate::manipulation::{identifiable::Identifiable, numeric_evaluation::NumericEvaluable};

use std::fmt::{Debug, Display};

/**
 * Minimal representative value
 */
pub trait Symbol: Debug + Display + NumericEvaluable + Identifiable {
    fn label(&self) -> String;
    fn value(&self) -> Option<f64>;
    fn boxed_clone(&self) -> Box<dyn Symbol>;
}

impl Clone for Box<dyn Symbol> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
    }
}
