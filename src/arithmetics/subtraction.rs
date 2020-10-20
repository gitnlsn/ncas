use crate::base::{expression::Expression, symbol::Symbol};

impl Expression {
    pub fn subtraction(left_hand_side: Expression, right_hand_side: Expression) -> Expression {
        let mut addends: Vec<Expression> = Vec::new();

        match &left_hand_side {
            Expression::Integer(n) => {
                if n != &Symbol::integer(0) {
                    addends.push(left_hand_side);
                }
            }
            Expression::Real(r) => {
                if r != &Symbol::real(0.0) {
                    addends.push(left_hand_side);
                }
            }
            _ => {
                addends.push(left_hand_side);
            }
        }

        match &right_hand_side {
            Expression::Integer(n) => {
                if n != &Symbol::integer(0) {
                    addends.push(Expression::multiplication(vec![
                        Symbol::integer(-1).expr(),
                        right_hand_side,
                    ]));
                }
            }
            Expression::Real(r) => {
                if r != &Symbol::real(0.0) {
                    addends.push(Expression::multiplication(vec![
                        Symbol::integer(-1).expr(),
                        right_hand_side,
                    ]));
                }
            }
            _ => {
                addends.push(Expression::multiplication(vec![
                    Symbol::integer(-1).expr(),
                    right_hand_side,
                ]));
            }
        }

        if addends.len() == 0 {
            return Symbol::integer(0).expr();
        }

        if addends.len() == 1 {
            return addends.pop().unwrap();
        }

        return Expression::addition(addends);
    }
}

/**
 * Overloads minus (-) Operation
 */
impl std::ops::Sub for Expression {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        Expression::subtraction(self, other)
    }
}

impl std::ops::Sub<&Expression> for Expression {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        self - other.clone()
    }
}

impl std::ops::Sub<&Expression> for &Expression {
    type Output = Expression;
    fn sub(self, other: &Expression) -> Expression {
        self.clone() - other.clone()
    }
}

impl std::ops::Sub<Expression> for &Expression {
    type Output = Expression;
    fn sub(self, other: Expression) -> Expression {
        self.clone() - other
    }
}

// ========================= //
//          Negation         //
// ========================= //
impl std::ops::Neg for Expression {
    type Output = Expression;
    fn neg(self) -> Self {
        Symbol::integer(-1).expr() * self
    }
}

impl std::ops::Neg for &Expression {
    type Output = Expression;
    fn neg(self) -> Expression {
        Symbol::integer(-1).expr() * self
    }
}
