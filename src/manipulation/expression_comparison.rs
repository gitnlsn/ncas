use crate::base::expression::Expression;
use crate::base::symbol::Symbol;

impl Expression {
    pub fn comparable(&self, other: &Expression) -> bool {
        match (self - other).simplify() {
            Expression::Integer(_) => return true,
            Expression::Real(_) => return true,
            _ => return false,
        }
    }

    pub fn equal(&self, other: &Self) -> bool {
        let simplified_difference = (self - other).simplify();
        match simplified_difference {
            Expression::Integer(n) => return n == Symbol::integer(0),
            Expression::Real(r) => return r == Symbol::real(0.0),
            _ => return false,
        }
    }

    pub fn not_equal(&self, other: &Self) -> bool {
        let simplified_difference = (self - other).simplify();
        match simplified_difference {
            Expression::Integer(n) => return n != Symbol::integer(0),
            Expression::Real(r) => return r != Symbol::real(0.0),
            _ => return false,
        }
    }

    pub fn greater(&self, other: &Self) -> bool {
        let simplified_difference = (self - other).simplify();
        match simplified_difference {
            Expression::Integer(n) => return n > Symbol::integer(0),
            Expression::Real(r) => return r > Symbol::real(0.0),
            _ => return false,
        }
    }

    pub fn lesser(&self, other: &Self) -> bool {
        let simplified_difference = (self - other).simplify();
        match simplified_difference {
            Expression::Integer(n) => return n < Symbol::integer(0),
            Expression::Real(r) => return r < Symbol::real(0.0),
            _ => return false,
        }
    }

    pub fn greater_equal(&self, other: &Self) -> bool {
        let simplified_difference = (self - other).simplify();
        match simplified_difference {
            Expression::Integer(n) => return n == Symbol::integer(0) || n > Symbol::integer(0),
            Expression::Real(r) => return r == Symbol::real(0.0) || r > Symbol::real(0.0),
            _ => return false,
        }
    }
    pub fn lesser_equal(&self, other: &Self) -> bool {
        let simplified_difference = (self - other).simplify();
        match simplified_difference {
            Expression::Integer(n) => return n == Symbol::integer(0) || n < Symbol::integer(0),
            Expression::Real(r) => return r == Symbol::real(0.0) || r < Symbol::real(0.0),
            _ => return false,
        }
    }
}
