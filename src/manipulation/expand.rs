use crate::base::expression::{Association, Expression};

use crate::arithmetics::{
    addition::Addition, division::Division, multiplication::Multiplication,
    subtraction::Subtraction,
};

pub enum Expandability {
    Symbolic,
    Additive,
    Multiplicative,
    // Exponential,
    // Trigonometric
    // Derivative
}

pub trait Expandable {
    fn expandability(&self) -> Expandability;
    fn expand(&self) -> Expression;
}

// =================================== //
//      Recursion on Expression        //
// =================================== //
impl Expandable for Expression {
    fn expandability(&self) -> Expandability {
        match self {
            Expression::Association(assoc) => assoc.expandability(),
            Expression::Symbol(_) => Expandability::Symbolic,
        }
    }

    fn expand(&self) -> Expression {
        match self {
            Expression::Association(assoc) => assoc.expand(),
            Expression::Symbol(symbol) => Expression::Symbol(symbol.clone()),
        }
    }
}

// =================================== //
//              Arithmetics            //
// =================================== //
impl Expandable for Addition {
    fn expandability(&self) -> Expandability {
        Expandability::Additive
    }
    fn expand(&self) -> Expression {
        Addition::new(
            self.left_hand_side().expand(),
            self.right_hand_side().expand(),
        )
    }
}

impl Expandable for Subtraction {
    fn expandability(&self) -> Expandability {
        Expandability::Additive
    }
    fn expand(&self) -> Expression {
        Subtraction::new(
            self.left_hand_side().expand(),
            self.right_hand_side().expand(),
        )
    }
}

impl Expandable for Multiplication {
    fn expandability(&self) -> Expandability {
        Expandability::Multiplicative
    }
    fn expand(&self) -> Expression {
        let lhs = self.left_hand_side();
        let rhs = self.right_hand_side();
        match (lhs.expandability(), rhs.expandability()) {
            (Expandability::Additive, Expandability::Additive) => {
                /* (a+b) * (c+d) = ac + ad + bc + bd */
                if let (Expression::Association(lhs), Expression::Association(rhs)) =
                    (lhs.as_ref(), rhs.as_ref())
                {
                    let t1 = lhs.left_hand_side().as_ref();
                    let t2 = lhs.left_hand_side().as_ref();
                    let t3 = rhs.left_hand_side().as_ref();
                    let t4 = rhs.left_hand_side().as_ref();

                    let expanded_t1 = t1.expand();
                    let expanded_t2 = t2.expand();
                    let expanded_t3 = t3.expand();
                    let expanded_t4 = t4.expand();

                    return Addition::new(
                        Addition::new(
                            Multiplication::new(expanded_t1.clone(), expanded_t3.clone()),
                            Multiplication::new(expanded_t1.clone(), expanded_t4.clone()),
                        ),
                        Addition::new(
                            Multiplication::new(expanded_t2.clone(), expanded_t3.clone()),
                            Multiplication::new(expanded_t2.clone(), expanded_t4.clone()),
                        ),
                    );
                } else {
                    panic!("Expected additive terms to hold on association expressions");
                }
            }
            (Expandability::Multiplicative, Expandability::Additive) => {
                /* (a*b) * (c+d) = a*b*c + a*b*d */
                if let (
                    Expression::Association(multiplicative_term),
                    Expression::Association(additive_term),
                ) = (lhs.as_ref(), rhs.as_ref())
                {
                    let t1 = additive_term.left_hand_side().as_ref();
                    let t2 = additive_term.left_hand_side().as_ref();

                    let expanded_multiplicative = multiplicative_term.expand();
                    let expanded_t1 = t1.expand();
                    let expanded_t2 = t2.expand();

                    return Addition::new(
                        Multiplication::new(expanded_multiplicative.clone(), expanded_t1),
                        Multiplication::new(expanded_multiplicative.clone(), expanded_t2),
                    );
                } else {
                    panic!("Expected multiplicative and additive terms to hold on association expressions");
                }
            }
            (Expandability::Additive, Expandability::Multiplicative) => {
                /* (a+b) * (c*d) = a*c*d + b*c*d */
                if let (
                    Expression::Association(additive_term),
                    Expression::Association(multiplicative_term),
                ) = (lhs.as_ref(), rhs.as_ref())
                {
                    let t1 = additive_term.left_hand_side().as_ref();
                    let t2 = additive_term.left_hand_side().as_ref();

                    let expanded_multiplicative = multiplicative_term.expand();
                    let expanded_t1 = t1.expand();
                    let expanded_t2 = t2.expand();

                    return Addition::new(
                        Multiplication::new(expanded_t1, expanded_multiplicative.clone()),
                        Multiplication::new(expanded_t2, expanded_multiplicative.clone()),
                    );
                } else {
                    panic!("Expected multiplicative and additive terms to hold on association expressions");
                }
            }

            _ => {
                /*
                    Default matcher holds among {
                        symbols,
                        multiplicative expressions
                    }
                */
                return Multiplication::new(
                    self.left_hand_side().expand(),
                    self.right_hand_side().expand(),
                );
            }
        } /* match expandability */
    } /* end - expand method */
}

impl Expandable for Division {
    fn expandability(&self) -> Expandability {
        Expandability::Multiplicative
    }
    fn expand(&self) -> Expression {
        let lhs = self.left_hand_side();
        let rhs = self.right_hand_side();
        match (lhs.expandability(), rhs.expandability()) {
            (Expandability::Additive, _) => {
                if let (
                    Expression::Association(nominator),
                    Expression::Association(demoninator),
                ) = (lhs.as_ref(), rhs.as_ref())
                {
                    let t1 = nominator.left_hand_side().as_ref();
                    let t2 = nominator.left_hand_side().as_ref();

                    let expanded_t1 = t1.expand();
                    let expanded_t2 = t2.expand();
                    let denominator = demoninator.expand();

                    return Addition::new(
                        Division::new(expanded_t1, denominator.clone()),
                        Division::new(expanded_t2, denominator.clone()),
                    );
                } else {
                    panic!("Expected multiplicative and additive terms to hold on association expressions");
                }
            }

            _ => {
                /*
                    Default matcher holds among {
                        symbols,
                        multiplicative expressions
                    }
                */
                return Multiplication::new(lhs.expand(), rhs.expand());
            }
        } /* match expandability */
    }
}
