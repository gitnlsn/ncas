/**
 * Overall Histogram data
 */
pub struct Histogram {
    pub expression: TreeData,
    pub symbols: Symbols,
    pub arithmetics: Arithmetics,
    pub exponential: Exponential,
}

impl std::ops::Add for Histogram {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Histogram {
            expression: self.expression + other.expression,
            symbols: self.symbols + other.symbols,
            arithmetics: self.arithmetics + other.arithmetics,
            exponential: self.exponential + other.exponential,
        }
    }
}

impl std::ops::Sub for Histogram {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Histogram {
            expression: self.expression - other.expression,
            symbols: self.symbols - other.symbols,
            arithmetics: self.arithmetics - other.arithmetics,
            exponential: self.exponential - other.exponential,
        }
    }
}

impl Histogram {
    pub fn new(symbols_tree: Symbols) -> Self {
        Self {
            expression: TreeData::new(1),
            symbols: symbols_tree,
            arithmetics: Arithmetics::new(),
            exponential: Exponential::new(),
        }
    }

    pub fn increase_depth(&mut self) {
        /* Arithmetics depth */
        if self.arithmetics.addition.quantity > 0 {
            self.arithmetics.addition.max_depth += 1;
        }
        if self.arithmetics.subtraction.quantity > 0 {
            self.arithmetics.subtraction.max_depth += 1;
        }
        if self.arithmetics.multiplication.quantity > 0 {
            self.arithmetics.multiplication.max_depth += 1;
        }
        if self.arithmetics.division.quantity > 0 {
            self.arithmetics.division.max_depth += 1;
        }

        /* Symbols depth */
        if self.symbols.constant.quantity > 0 {
            self.symbols.constant.max_depth += 1;
        }
        if self.symbols.variables.quantity > 0 {
            self.symbols.variables.max_depth += 1;
        }
        if self.symbols.numbers.quantity > 0 {
            self.symbols.numbers.max_depth += 1;
        }

        /* Symbols depth */
        if self.exponential.power.quantity > 0 {
            self.exponential.power.max_depth += 1;
        }

        /* Expression depth */
        self.expression.max_depth += 1;
        self.expression.quantity += 1;
    }
}

/**
 * Arithmetics tree
 */
pub struct Arithmetics {
    pub addition: TreeData,
    pub subtraction: TreeData,
    pub multiplication: TreeData,
    pub division: TreeData,
}

impl std::ops::Add for Arithmetics {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            addition: self.addition + other.addition,
            subtraction: self.subtraction + other.subtraction,
            multiplication: self.multiplication + other.multiplication,
            division: self.division + other.division,
        }
    }
}

impl std::ops::Sub for Arithmetics {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            addition: self.addition - other.addition,
            subtraction: self.subtraction - other.subtraction,
            multiplication: self.multiplication - other.multiplication,
            division: self.division - other.division,
        }
    }
}

impl Arithmetics {
    pub fn new() -> Self {
        Self {
            addition: TreeData::new(0),
            subtraction: TreeData::new(0),
            multiplication: TreeData::new(0),
            division: TreeData::new(0),
        }
    }
}

/**
 * Exponential tree
 */
pub struct Exponential {
    pub power: TreeData,
    // pub log: TreeData,
    // pub exp: TreeData,
}

impl std::ops::Add for Exponential {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            power: self.power + other.power,
        }
    }
}

impl std::ops::Sub for Exponential {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            power: self.power - other.power,
        }
    }
}

impl Exponential {
    pub fn new() -> Self {
        Self {
            power: TreeData::new(0),
        }
    }
}

/**
 * Symbols tree
 */
pub struct Symbols {
    pub variables: TreeData,
    pub constant: TreeData,
    pub numbers: TreeData,
}

impl std::ops::Add for Symbols {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            variables: self.variables + other.variables,
            constant: self.constant + other.constant,
            numbers: self.numbers + other.numbers,
        }
    }
}

impl std::ops::Sub for Symbols {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            variables: self.variables - other.variables,
            constant: self.constant - other.constant,
            numbers: self.numbers - other.numbers,
        }
    }
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            variables: TreeData::new(0),
            constant: TreeData::new(0),
            numbers: TreeData::new(0),
        }
    }
    pub fn variable() -> Self {
        Self {
            variables: TreeData::new(1),
            constant: TreeData::new(0),
            numbers: TreeData::new(0),
        }
    }
    pub fn constant() -> Self {
        Self {
            variables: TreeData::new(0),
            constant: TreeData::new(1),
            numbers: TreeData::new(0),
        }
    }
    pub fn number() -> Self {
        Self {
            variables: TreeData::new(0),
            constant: TreeData::new(0),
            numbers: TreeData::new(1),
        }
    }
}

/**
 * Counters for default data
 */
pub struct TreeData {
    pub quantity: isize,
    pub max_depth: isize,
}

impl std::ops::Add for TreeData {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            quantity: self.quantity + other.quantity,
            max_depth: std::cmp::max(self.max_depth, other.max_depth),
        }
    }
}

impl std::ops::Sub for TreeData {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            quantity: self.quantity - other.quantity,
            max_depth: self.max_depth - other.max_depth,
        }
    }
}

impl TreeData {
    pub fn new(quantity: isize) -> Self {
        Self {
            quantity: quantity,
            max_depth: 0,
        }
    }
}

// pub struct Trigonometrics {
//     pub sin: usize,
//     pub cos: usize,
//     pub tag: usize,
//     pub cosin: usize,
//     pub cosec: usize,
//     pub cotag: usize,
// }

/**
 *  
 */
pub trait ExpressionMeasure {
    fn histogram(&self) -> Histogram;
}

// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::base::expression::{
    Association, AssociativeOperation, CommutativeAssociation, Expression, SymbolType,
};
impl ExpressionMeasure for Expression {
    fn histogram(&self) -> Histogram {
        match self {
            Expression::CommutativeAssociation(a) => a.histogram(),
            Expression::AssociativeOperation(a) => a.histogram(),
            Expression::Association(a) => a.histogram(),
            Expression::Symbol(s) => match s.symbol_type() {
                SymbolType::Constant => Histogram::new(Symbols::constant()),
                SymbolType::Variable => Histogram::new(Symbols::variable()),
                SymbolType::Number => Histogram::new(Symbols::number()),
            },
        }
    }
}

// =================================== //
//              Arithmetics            //
// =================================== //
use crate::arithmetics::{
    addition::Addition, division::Division, multiplication::Multiplication,
    subtraction::Subtraction,
};
impl ExpressionMeasure for Addition {
    fn histogram(&self) -> Histogram {
        let mut histogram: Histogram = self
            .items()
            .iter()
            .map(|item| item.histogram())
            .fold(Histogram::new(Symbols::new()), |acc, new| acc + new);

        histogram.increase_depth();

        histogram.arithmetics.addition.quantity += (self.items().len() - 1) as isize;
        return histogram;
    }
}

impl ExpressionMeasure for Subtraction {
    fn histogram(&self) -> Histogram {
        let mut histogram = self.left_hand_side().histogram() + self.right_hand_side().histogram();

        histogram.increase_depth();

        histogram.arithmetics.subtraction.quantity += 1;
        return histogram;
    }
}

impl ExpressionMeasure for Multiplication {
    fn histogram(&self) -> Histogram {
        let mut histogram: Histogram = self
            .items()
            .iter()
            .map(|item| item.histogram())
            .fold(Histogram::new(Symbols::new()), |acc, new| acc + new);

        histogram.increase_depth();

        histogram.arithmetics.multiplication.quantity += (self.items().len() - 1) as isize;
        return histogram;
    }
}

impl ExpressionMeasure for Division {
    fn histogram(&self) -> Histogram {
        let mut histogram = self.left_hand_side().histogram() + self.right_hand_side().histogram();

        histogram.increase_depth();

        histogram.arithmetics.division.quantity += 1;
        return histogram;
    }
}

// =================================== //
//              Exponential            //
// =================================== //
use crate::exponential::power::Power;
impl ExpressionMeasure for Power {
    fn histogram(&self) -> Histogram {
        let mut histogram = self.argument().histogram() + self.modifier().histogram();

        histogram.increase_depth();

        histogram.exponential.power.quantity += 1;
        return histogram;
    }
}
