use crate::{
    base::expression::Expression,
    symbols::{integer::Integer, number::Number, variable::Variable},
};

pub trait AsExpression {
    fn as_expression(self) -> Expression;
}

impl AsExpression for f64 {
    fn as_expression(self) -> Expression {
        Number::new(self as f64)
    }
}

impl AsExpression for isize {
    fn as_expression(self) -> Expression {
        Integer::new(self)
    }
}

/*
    TODO:
        - Implement a proper expression interpreter when parsing a strings
        - this is a temporary handle: "sin(a^2)" will generate a Variable!
*/
impl AsExpression for String {
    fn as_expression(self) -> Expression {
        Variable::new(self)
    }
}

impl AsExpression for &str {
    fn as_expression(self) -> Expression {
        Variable::new(String::from(self))
    }
}

// ========================= //
//          Negation         //
// ========================= //
impl std::ops::Neg for Expression {
    type Output = Expression;
    fn neg(self) -> Self {
        Integer::new(-1) * self
    }
}

impl std::ops::Neg for &Expression {
    type Output = Expression;
    fn neg(self) -> Expression {
        Integer::new(-1) * self
    }
}

// ========================= //
//          Addition         //
// ========================= //
impl std::ops::Add<isize> for Expression {
    type Output = Expression;
    fn add(self, other: isize) -> Expression {
        self + Integer::new(other)
    }
}

impl std::ops::Add<Expression> for isize {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Integer::new(self) + other
    }
}

impl std::ops::Add<f64> for Expression {
    type Output = Expression;
    fn add(self, other: f64) -> Expression {
        self + Number::new(other)
    }
}

impl std::ops::Add<Expression> for f64 {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Number::new(self) + other
    }
}

impl std::ops::Add<&str> for Expression {
    type Output = Expression;
    fn add(self, other: &str) -> Expression {
        self + Variable::new(String::from(other))
    }
}

impl std::ops::Add<Expression> for &str {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Variable::new(String::from(self)) + other
    }
}

impl std::ops::Add<String> for Expression {
    type Output = Expression;
    fn add(self, other: String) -> Expression {
        self + Variable::new(other)
    }
}

impl std::ops::Add<Expression> for String {
    type Output = Expression;
    fn add(self, other: Expression) -> Expression {
        Variable::new(self) + other
    }
}

impl std::ops::Add<isize> for &Expression {
    type Output = Expression;
    fn add(self, other: isize) -> Expression {
        self + Integer::new(other)
    }
}

impl std::ops::Add<&Expression> for isize {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        Integer::new(self) + other
    }
}

impl std::ops::Add<f64> for &Expression {
    type Output = Expression;
    fn add(self, other: f64) -> Expression {
        self + Number::new(other)
    }
}

impl std::ops::Add<&Expression> for f64 {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        Number::new(self) + other
    }
}

impl std::ops::Add<&str> for &Expression {
    type Output = Expression;
    fn add(self, other: &str) -> Expression {
        self + Variable::new(String::from(other))
    }
}

impl std::ops::Add<&Expression> for &str {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        Variable::new(String::from(self)) + other
    }
}

impl std::ops::Add<String> for &Expression {
    type Output = Expression;
    fn add(self, other: String) -> Expression {
        self + Variable::new(other)
    }
}

impl std::ops::Add<&Expression> for String {
    type Output = Expression;
    fn add(self, other: &Expression) -> Expression {
        Variable::new(self) + other
    }
}

// ========================= //
//        Subtraction        //
// ========================= //
impl std::ops::Sub<isize> for Expression {
    type Output = Expression;
    fn sub(self, other: isize) -> Expression {
        self - Integer::new(other)
    }
}

impl std::ops::Sub<Expression> for isize {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Integer::new(self) - other
    }
}

impl std::ops::Sub<f64> for Expression {
    type Output = Expression;
    fn sub(self, other: f64) -> Expression {
        self - Number::new(other as f64)
    }
}

impl std::ops::Sub<Expression> for f64 {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Number::new(self) - other
    }
}

impl std::ops::Sub<&str> for Expression {
    type Output = Expression;
    fn sub(self, other: &str) -> Expression {
        self - Variable::new(String::from(other))
    }
}

impl std::ops::Sub<Expression> for &str {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Variable::new(String::from(self)) - other
    }
}

impl std::ops::Sub<String> for Expression {
    type Output = Expression;
    fn sub(self, other: String) -> Expression {
        self - Variable::new(other)
    }
}

impl std::ops::Sub<Expression> for String {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Variable::new(self) - other
    }
}

impl std::ops::Sub<isize> for &Expression {
    type Output = Expression;
    fn sub(self, other: isize) -> Expression {
        self - Integer::new(other)
    }
}

impl std::ops::Sub<&Expression> for isize {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Integer::new(self) - other
    }
}

impl std::ops::Sub<f64> for &Expression {
    type Output = Expression;
    fn sub(self, other: f64) -> Expression {
        self - Number::new(other as f64)
    }
}

impl std::ops::Sub<&Expression> for f64 {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Number::new(self) - other
    }
}

impl std::ops::Sub<&str> for &Expression {
    type Output = Expression;
    fn sub(self, other: &str) -> Expression {
        self - Variable::new(String::from(other))
    }
}

impl std::ops::Sub<&Expression> for &str {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Variable::new(String::from(self)) - other
    }
}

impl std::ops::Sub<String> for &Expression {
    type Output = Expression;
    fn sub(self, other: String) -> Expression {
        self - Variable::new(other)
    }
}

impl std::ops::Sub<&Expression> for String {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        Variable::new(self) - other
    }
}

// =========================== //
//        Mutiplication        //
// =========================== //
impl std::ops::Mul<isize> for Expression {
    type Output = Expression;
    fn mul(self, other: isize) -> Expression {
        self * Integer::new(other)
    }
}

impl std::ops::Mul<Expression> for isize {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        Integer::new(self) * other
    }
}

impl std::ops::Mul<f64> for Expression {
    type Output = Expression;
    fn mul(self, other: f64) -> Expression {
        self * Number::new(other)
    }
}

impl std::ops::Mul<Expression> for f64 {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        Number::new(self) * other
    }
}

impl std::ops::Mul<&str> for Expression {
    type Output = Expression;
    fn mul(self, other: &str) -> Expression {
        self * Variable::new(String::from(other))
    }
}

impl std::ops::Mul<Expression> for &str {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        Variable::new(String::from(self)) * other
    }
}

impl std::ops::Mul<String> for Expression {
    type Output = Expression;
    fn mul(self, other: String) -> Expression {
        self * Variable::new(other)
    }
}

impl std::ops::Mul<Expression> for String {
    type Output = Expression;
    fn mul(self, other: Expression) -> Expression {
        Variable::new(self) * other
    }
}
impl std::ops::Mul<isize> for &Expression {
    type Output = Expression;
    fn mul(self, other: isize) -> Expression {
        self * Integer::new(other)
    }
}

impl std::ops::Mul<&Expression> for isize {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        Integer::new(self) * other
    }
}

impl std::ops::Mul<f64> for &Expression {
    type Output = Expression;
    fn mul(self, other: f64) -> Expression {
        self * Number::new(other)
    }
}

impl std::ops::Mul<&Expression> for f64 {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        Number::new(self) * other
    }
}

impl std::ops::Mul<&str> for &Expression {
    type Output = Expression;
    fn mul(self, other: &str) -> Expression {
        self * Variable::new(String::from(other))
    }
}

impl std::ops::Mul<&Expression> for &str {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        Variable::new(String::from(self)) * other
    }
}

impl std::ops::Mul<String> for &Expression {
    type Output = Expression;
    fn mul(self, other: String) -> Expression {
        self * Variable::new(other)
    }
}

impl std::ops::Mul<&Expression> for String {
    type Output = Expression;
    fn mul(self, other: &Expression) -> Expression {
        Variable::new(self) * other
    }
}

// ====================== //
//        Division        //
// ====================== //
impl std::ops::Div<isize> for Expression {
    type Output = Expression;
    fn div(self, other: isize) -> Expression {
        self / Integer::new(other)
    }
}

impl std::ops::Div<Expression> for isize {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Integer::new(self) / other
    }
}

impl std::ops::Div<f64> for Expression {
    type Output = Expression;
    fn div(self, other: f64) -> Expression {
        self / Number::new(other)
    }
}

impl std::ops::Div<Expression> for f64 {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Number::new(self) / other
    }
}

impl std::ops::Div<&str> for Expression {
    type Output = Expression;
    fn div(self, other: &str) -> Expression {
        self / Variable::new(String::from(other))
    }
}

impl std::ops::Div<Expression> for &str {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Variable::new(String::from(self)) / other
    }
}

impl std::ops::Div<String> for Expression {
    type Output = Expression;
    fn div(self, other: String) -> Expression {
        self / Variable::new(other)
    }
}

impl std::ops::Div<Expression> for String {
    type Output = Expression;
    fn div(self, other: Expression) -> Expression {
        Variable::new(self) / other
    }
}

impl std::ops::Div<isize> for &Expression {
    type Output = Expression;
    fn div(self, other: isize) -> Expression {
        self / Integer::new(other)
    }
}

impl std::ops::Div<&Expression> for isize {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        Integer::new(self) / other
    }
}

impl std::ops::Div<f64> for &Expression {
    type Output = Expression;
    fn div(self, other: f64) -> Expression {
        self / Number::new(other)
    }
}

impl std::ops::Div<&Expression> for f64 {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        Number::new(self) / other
    }
}

impl std::ops::Div<&str> for &Expression {
    type Output = Expression;
    fn div(self, other: &str) -> Expression {
        self / Variable::new(String::from(other))
    }
}

impl std::ops::Div<&Expression> for &str {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        Variable::new(String::from(self)) / other
    }
}

impl std::ops::Div<String> for &Expression {
    type Output = Expression;
    fn div(self, other: String) -> Expression {
        self / Variable::new(other)
    }
}

impl std::ops::Div<&Expression> for String {
    type Output = Expression;
    fn div(self, other: &Expression) -> Expression {
        Variable::new(self) / other
    }
}
