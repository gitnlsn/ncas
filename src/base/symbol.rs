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

use std::hash::{Hash, Hasher};
impl Hash for dyn Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
        self.label().hash(state);
        if let Some(value) = self.value() {
            let (m, e, s) = num::Float::integer_decode(value);
            m.hash(state);
            e.hash(state);
            s.hash(state);
        }
    }
}
