use crate::base::expression::{Expression, Symbol, SymbolType};

/**
 * Symbol implementation
 */
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct Variable {
    label: String,
    value: Option<f64>,
    expression: Option<Expression>
}

impl Variable {
    pub fn new(label: String) -> Expression {
        Expression::Symbol(Box::new(Self {
            label: label,
            value: None,
            expression: None,
        }))
    }
    pub fn set_expression(&mut self, e: Expression) {
        self.expression = Some(e);
    }
    pub fn get_expression(&self) -> Option<Expression> {
        self.expression.clone()
    }
}

/**
 * Symbol Implementation
 */
impl Symbol for Variable {
    fn symbol_type(&self) -> SymbolType {
        SymbolType::Variable
    }
    fn label(&self) -> String {
        self.label.clone()
    }
    fn value(&self) -> Option<f64> {
        use crate::manipulation::numeric_evaluation::NumericEvaluable;
        if let Some(expression) = self.expression.clone() {
            if let Ok(value) = expression.clone().into_num() {
                return Some(value);
            }
        }
        self.value
    }
    fn boxed_clone(&self) -> Box<dyn Symbol> {
        Box::new(Self {
            label: self.label.clone(),
            value: self.value.clone(),
            expression: self.expression.clone()
        })
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
