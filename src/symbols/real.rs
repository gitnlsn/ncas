use crate::base::{expression::Expression, symbol::Symbol};

impl Symbol<f64> {
    pub fn real(value: f64) -> Self {
        Self { data: value }
    }
    pub fn value(&self) -> Option<f64> {
        Some(self.data)
    }
    pub fn label(&self) -> String {
        format!("{}", self.data)
    }
    pub fn expr(self) -> Expression {
        Expression::Real(self)
    }
}

use std::hash::{Hash, Hasher};
impl Hash for Symbol<f64> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (m, e, s) = num::Float::integer_decode(self.data);
        m.hash(state);
        e.hash(state);
        s.hash(state);
    }
}

// ================= //
//      Equality     //
// ================= //
impl Eq for Symbol<f64> {}
impl PartialEq for Symbol<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Ord for Symbol<f64> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.data > other.data {
            return std::cmp::Ordering::Greater;
        } else if self.data < other.data {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
}
impl PartialOrd for Symbol<f64> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let both_nan = self.data == std::f64::NAN || other.data == std::f64::NAN;
        let both_ninf = self.data == std::f64::NEG_INFINITY && other.data == std::f64::NEG_INFINITY;
        let both_inf = self.data == std::f64::INFINITY && other.data == std::f64::INFINITY;

        let first_ninf =
            self.data == std::f64::NEG_INFINITY && other.data != std::f64::NEG_INFINITY;
        let second_ninf =
            other.data == std::f64::NEG_INFINITY && self.data != std::f64::NEG_INFINITY;

        let first_inf = self.data == std::f64::INFINITY && other.data != std::f64::INFINITY;
        let second_inf = other.data == std::f64::INFINITY && self.data != std::f64::INFINITY;

        if both_nan || both_inf || both_ninf {
            return None;
        }

        if first_ninf || second_inf {
            return Some(std::cmp::Ordering::Less);
        }

        if first_inf || second_ninf {
            return Some(std::cmp::Ordering::Less);
        }

        Some(self.cmp(other))
    }
}

// ================= //
//      Addition     //
// ================= //
impl std::ops::Add for Symbol<f64> {
    type Output = Symbol<f64>;
    fn add(self, other: Symbol<f64>) -> Symbol<f64> {
        Self {
            data: self.data + other.data,
        }
    }
}

impl std::ops::Add<&Symbol<f64>> for Symbol<f64> {
    type Output = Symbol<f64>;
    fn add(self, other: &Symbol<f64>) -> Symbol<f64> {
        self + other.clone()
    }
}

impl std::ops::Add for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn add(self, other: &Symbol<f64>) -> Symbol<f64> {
        self.clone() + other.clone()
    }
}

impl std::ops::Add<Symbol<f64>> for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn add(self, other: Symbol<f64>) -> Symbol<f64> {
        self.clone() + other
    }
}

// ================= //
//      Subtract     //
// ================= //
impl std::ops::Sub for Symbol<f64> {
    type Output = Symbol<f64>;
    fn sub(self, other: Symbol<f64>) -> Symbol<f64> {
        Self {
            data: self.data - other.data,
        }
    }
}

impl std::ops::Sub<&Symbol<f64>> for Symbol<f64> {
    type Output = Symbol<f64>;
    fn sub(self, other: &Symbol<f64>) -> Symbol<f64> {
        self - other.clone()
    }
}

impl std::ops::Sub for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn sub(self, other: &Symbol<f64>) -> Symbol<f64> {
        self.clone() - other.clone()
    }
}

impl std::ops::Sub<Symbol<f64>> for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn sub(self, other: Symbol<f64>) -> Symbol<f64> {
        self.clone() - other
    }
}

// ======================= //
//      Multiplication     //
// ======================= //
impl std::ops::Mul for Symbol<f64> {
    type Output = Symbol<f64>;
    fn mul(self, other: Symbol<f64>) -> Symbol<f64> {
        Self {
            data: self.data * other.data,
        }
    }
}

impl std::ops::Mul<&Symbol<f64>> for Symbol<f64> {
    type Output = Symbol<f64>;
    fn mul(self, other: &Symbol<f64>) -> Symbol<f64> {
        self * other.clone()
    }
}

impl std::ops::Mul for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn mul(self, other: &Symbol<f64>) -> Symbol<f64> {
        self.clone() * other.clone()
    }
}

impl std::ops::Mul<Symbol<f64>> for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn mul(self, other: Symbol<f64>) -> Symbol<f64> {
        self.clone() * other
    }
}

// ================= //
//      Division     //
// ================= //
impl std::ops::Div for Symbol<f64> {
    type Output = Symbol<f64>;
    fn div(self, other: Symbol<f64>) -> Symbol<f64> {
        Self {
            data: self.data / other.data,
        }
    }
}

impl std::ops::Div<&Symbol<f64>> for Symbol<f64> {
    type Output = Symbol<f64>;
    fn div(self, other: &Symbol<f64>) -> Symbol<f64> {
        self / other.clone()
    }
}

impl std::ops::Div for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn div(self, other: &Symbol<f64>) -> Symbol<f64> {
        self.clone() / other.clone()
    }
}

impl std::ops::Div<Symbol<f64>> for &Symbol<f64> {
    type Output = Symbol<f64>;
    fn div(self, other: Symbol<f64>) -> Symbol<f64> {
        self.clone() / other
    }
}
