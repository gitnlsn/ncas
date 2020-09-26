/**
 *
 */
pub trait Expandable {
    fn expand(&self) -> Expression;
}

// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::base::{
    association::Association, associative_operation::AssociativeOperation,
    commutative_association::CommutativeAssociation, expression::Expression,
};
use crate::manipulation::identifiable::{Identifiable, Identity};

impl Expandable for Expression {
    fn expand(&self) -> Expression {
        match self {
            Expression::CommutativeAssociation(comu) => comu.expand(),
            Expression::AssociativeOperation(assoc) => assoc.expand(),
            Expression::Association(assoc) => assoc.expand(),
            Expression::Symbol(symbol) => Expression::Symbol(symbol.clone()),
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
impl Expandable for Addition {
    fn expand(&self) -> Expression {
        Addition::new(
            self.items()
                .iter()
                .map(|item| item.expand() /* Recursion: expands addends */)
                .collect(),
        )
    }
}

impl Expandable for Subtraction {
    /* Subtractions are converted into additions */
    fn expand(&self) -> Expression {
        let lhs = self.left_hand_side().expand();
        let rhs = self.right_hand_side().expand();
        return lhs + (-1 * rhs);
    }
}

/**
 *  Cross product expects Addition expression only
 */
fn cross(e1: Expression, e2: Expression) -> Expression {
    if e2.id() != Identity::Addition {
        panic!("Cannot apply cross product to expression, with factor different than Addition");
    }

    match e1.id() {
        Identity::Addition => {
            if let (
                Expression::CommutativeAssociation(a1),
                Expression::CommutativeAssociation(a2),
            ) = (e1.clone(), e2.clone())
            {
                let mut addends: Vec<Expression> = Vec::new();
                // ============================= //
                //      Distributive property    //
                // ============================= //
                for a1_item in a1.items().iter() {
                    for a2_item in a2.items().iter() {
                        addends.push(a1_item.clone() * a2_item.clone());
                    }
                }
                return Addition::new(addends);
            }
        }
        _ => {
            if let Expression::CommutativeAssociation(a2) = e2.clone() {
                let mut addends: Vec<Expression> = Vec::new();
                // ============================= //
                //      Distributive property    //
                // ============================= //
                // for a1_item in a1.items().iter() {
                // }
                for a2_item in a2.items().iter() {
                    addends.push(e1.clone() * a2_item.clone());
                }
                return Addition::new(addends);
            }
        }
    }

    panic!("Cannot apply cross product to expression, with factor different than Addition");
}

impl Expandable for Multiplication {
    fn expand(&self) -> Expression {
        let expanded_fators: Vec<Expression> = self
            .items()
            .iter()
            .map(|expression| expression.expand()) /* Recursion: expands factors */
            .collect();

        let non_expandable: Vec<Expression> = expanded_fators
            .iter()
            .filter(|expression| match expression.id() {
                Identity::Addition => false,
                _ => true,
            })
            .cloned()
            .collect();

        let expandables: Vec<Expression> = expanded_fators
            .iter()
            .filter(|expression| match expression.id() {
                Identity::Addition => true,
                _ => false,
            })
            .map(|expression| match expression.id() {
                Identity::Addition => expression.expand(),
                _ => panic!("Not expected expression different than addition"),
            })
            .collect();

        if expandables.is_empty() {
            return Multiplication::new(non_expandable);
        }

        return expandables
            .iter()
            .cloned()
            .fold(Multiplication::new(non_expandable), |acc, new| {
                cross(acc, new)
            });
    } /* end - expand method */
} /* end - Expandable Multiplication */

impl Expandable for Division {
    fn expand(&self) -> Expression {
        let lhs = self.left_hand_side();
        let rhs = self.right_hand_side();
        match (lhs.id(), rhs.id()) {
            (Identity::Addition, _) => {
                /* (a+b)/d = a/d + b/d */
                if let (Expression::Association(nominator), Expression::Association(demoninator)) =
                    (lhs.as_ref(), rhs.as_ref())
                {
                    let t1 = nominator.left_hand_side().as_ref();
                    let t2 = nominator.right_hand_side().as_ref();

                    let expanded_t1 = t1.expand();
                    let expanded_t2 = t2.expand();
                    let denominator = demoninator.expand();

                    return (expanded_t1 / denominator.clone())
                        + (expanded_t2 / denominator.clone());
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
                return Division::new(lhs.expand(), rhs.expand());
            }
        } /* match expandability */
    }
} /* end - Expandable Division */

// =================================== //
//              Exponential            //
// =================================== //
use crate::exponential::power::Power;
impl Expandable for Power {
    fn expand(&self) -> Expression {
        return self.argument().expand() ^ self.modifier().expand();
    }
}
