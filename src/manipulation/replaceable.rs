use crate::base::expression::Symbol;

pub trait Replaceable {
    fn substitute(&mut self, variable: &dyn Symbol, value: f64);
}
