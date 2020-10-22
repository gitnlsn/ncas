use crate::base::{expression::Expression, symbol::Symbol};

impl Symbol<String> {
    /**
     * Constructor for variable as Symbol<String>
     */
    pub fn variable(label: &str) -> Self {
        Self {
            data: String::from(label),
        }
    }
    pub fn value(&self) -> Option<f64> {
        None
    }
    pub fn label(&self) -> String {
        format!("{}", self.data)
    }
    pub fn expr(self) -> Expression {
        Expression::Variable(self)
    }
}

impl std::hash::Hash for Symbol<String> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

// ================= //
//      Equality     //
// ================= //
impl Eq for Symbol<String> {}
impl PartialEq for Symbol<String> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Ord for Symbol<String> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label().cmp(&other.label())
    }
}
impl PartialOrd for Symbol<String> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
