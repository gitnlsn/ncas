use crate::base::{expression::Expression, operation::Operation};

#[derive(std::fmt::Debug)]
pub struct Sin {
    angle: Box<Expression>,
}

impl Sin {
    pub fn new(angle: Expression) -> Expression {
        Expression::Operation(Box::new(Self {
            angle: Box::new(angle),
        }))
    }
}

impl Operation for Sin {
    fn argument(&self) -> &Box<Expression> {
        &self.angle
    }
    fn boxed_clone(&self) -> Box<dyn Operation> {
        Box::new(Self {
            angle: self.angle.clone(),
        })
    }
}

/*
    Debug implementation
*/
impl std::fmt::Display for Sin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sin({})", self.angle)
    }
}
