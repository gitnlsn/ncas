use crate::base::{associative_operation::AssociativeOperation, expression::Expression};

#[derive(std::fmt::Debug)]
pub struct Log {
    argument: Box<Expression>,
    base: Box<Expression>,
}

impl Log {
    pub fn new(argument: Expression, base: Expression) -> Expression {
        Expression::AssociativeOperation(Box::new(Self {
            argument: Box::new(argument),
            base: Box::new(base),
        }))
    }
}

impl AssociativeOperation for Log {
    fn argument(&self) -> &Box<Expression> {
        &self.argument
    }
    fn modifier(&self) -> &Box<Expression> {
        &self.base
    }
    fn boxed_clone(&self) -> Box<dyn AssociativeOperation> {
        Box::new(Self {
            argument: self.argument.clone(),
            base: self.base.clone(),
        })
    }
}

impl Expression {
    pub fn log(argument: Expression, base: Expression) -> Expression {
        Log::new(argument.clone(), base.clone())
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "log({}, {})", self.argument, self.base)
    }
}
