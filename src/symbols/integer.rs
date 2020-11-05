use crate::base::{expression::Expression, symbol::Symbol};
use num::bigint::BigInt;

impl Symbol<BigInt> {
    /**
     * Constructor for integer as Symbol<BigInt>
     */
    pub fn integer(value: isize) -> Self {
        Self {
            data: BigInt::from(value),
        }
    }
    pub fn value(&self) -> Option<isize> {
        num::traits::ToPrimitive::to_isize(&self.data)
    }
    pub fn label(&self) -> String {
        format!("{}", self.data)
    }
    pub fn expr(self) -> Expression {
        Expression::Integer(self)
    }
}

impl std::hash::Hash for Symbol<BigInt> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

// =================== //
//      Comparison     //
// =================== //
impl Eq for Symbol<BigInt> {}
impl PartialEq for Symbol<BigInt> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Ord for Symbol<BigInt> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}
impl PartialOrd for Symbol<BigInt> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ================= //
//      Addition     //
// ================= //
impl std::ops::Add for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn add(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        Self {
            data: self.data + other.data,
        }
    }
}

impl std::ops::Add<&Symbol<BigInt>> for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn add(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self + other.clone()
    }
}

impl std::ops::Add for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn add(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() + other.clone()
    }
}

impl std::ops::Add<Symbol<BigInt>> for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn add(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() + other
    }
}

// ================= //
//      Subtract     //
// ================= //
impl std::ops::Sub for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn sub(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        Self {
            data: self.data - other.data,
        }
    }
}

impl std::ops::Sub<&Symbol<BigInt>> for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn sub(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self - other.clone()
    }
}

impl std::ops::Sub for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn sub(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() - other.clone()
    }
}

impl std::ops::Sub<Symbol<BigInt>> for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn sub(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() - other
    }
}

// ======================= //
//      Multiplication     //
// ======================= //
impl std::ops::Mul for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn mul(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        Self {
            data: self.data * other.data,
        }
    }
}

impl std::ops::Mul<&Symbol<BigInt>> for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn mul(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self * other.clone()
    }
}

impl std::ops::Mul for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn mul(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() * other.clone()
    }
}

impl std::ops::Mul<Symbol<BigInt>> for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn mul(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() * other
    }
}

// ================= //
//      Division     //
// ================= //
impl std::ops::Div for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn div(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        Self {
            data: self.data / other.data,
        }
    }
}

impl std::ops::Div<&Symbol<BigInt>> for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn div(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self / other.clone()
    }
}

impl std::ops::Div for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn div(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() / other.clone()
    }
}

impl std::ops::Div<Symbol<BigInt>> for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn div(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() / other
    }
}

// ================== //
//      Remainder     //
// ================== //
impl std::ops::Rem for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn rem(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        Self {
            data: self.data % other.data,
        }
    }
}

impl std::ops::Rem<&Symbol<BigInt>> for Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn rem(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self % other.clone()
    }
}

impl std::ops::Rem for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn rem(self, other: &Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() % other.clone()
    }
}

impl std::ops::Rem<Symbol<BigInt>> for &Symbol<BigInt> {
    type Output = Symbol<BigInt>;
    fn rem(self, other: Symbol<BigInt>) -> Symbol<BigInt> {
        self.clone() % other
    }
}

// ================== //
//      Utilities     //
// ================== //
impl Symbol<BigInt> {
    pub fn gcd(x: &Self, y: &Self) -> Self {
        Self {
            data: num::integer::gcd(x.data.clone(), y.data.clone()),
        }
    }
    pub fn lcm(x: &Self, y: &Self) -> Self {
        Self {
            data: num::integer::lcm(x.data.clone(), y.data.clone()),
        }
    }

    pub fn is_negative(&self) -> bool {
        use num::bigint::Sign;
        return self.data.sign() == Sign::Minus;
    }

    pub fn opposite(&self) -> Self {
        Self {
            data: self.data.clone() * BigInt::from(-1)
        }
    }

    pub fn pow(&self, exponent: &Self) -> Option<Self> {
        if !exponent.is_negative() {
            let base_value = self.data.clone();
            let exponent_value = exponent.data.to_biguint().unwrap();
            let power_value: BigInt =
                num::traits::Pow::<num::BigUint>::pow(base_value, exponent_value);
            return Some(Self { data: power_value });
        }

        return None;
    }
}
