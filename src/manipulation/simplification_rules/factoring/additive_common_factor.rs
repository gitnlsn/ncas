use crate::arithmetics::addition::Addition;
use crate::base::expression::Expression;
use crate::manipulation::{identifiable::Identity, simplification_rules::rule::Rule};
use crate::manipulation::simplification_rules::factoring::factorable::Factorable;
use std::collections::HashMap;

pub struct AdditiveCommonFactor {}
impl Rule for AdditiveCommonFactor {
    fn apply(expression: &Expression) -> Vec<Expression> {
        let mut alternatives: Vec<Expression> = Vec::new();

        match expression {
            Expression::CommutativeAssociation(association) => match association.id() {
                Identity::Addition => {
                    let addends = association.items();
                    let mut common_factors: HashMap<Expression, usize> = HashMap::new();

                    for index_1 in 0..addends.len() {
                        for index_2 in (index_1 + 1)..addends.len() {
                            let mut addends = addends.clone();
                            let a2: Expression = addends.remove(index_2);
                            let a1: Expression = addends.remove(index_1);
                            let common_factors = Expression::gcd(vec![a2.clone(), a1.clone()]);
                            // let a1 = a1.into_factors();
                            // let a2 = a2.into_factors();
                            
                        }
                    }

                    // alternatives.push();
                }
                _ => {}
            },
            _ => {}
        }

        if alternatives.is_empty() {
            return vec![expression.clone()];
        }

        return alternatives;
    }
}

#[cfg(test)]
mod functional_tests {
    use super::*;
    use crate::symbols::variable::Variable;

    // #[test]
    // fn first_depth() {
    //     let a = &Variable::new(String::from("a"));
    //     let b = &Variable::new(String::from("b"));
    //     let c = &Variable::new(String::from("c"));

    //     let factorable = a * b + a * c;
    //     let expected = a * (b + c);
    //     assert!(AdditiveCommonFactor::apply(&factorable).contains(&expected));
    // }

    // #[test]
    // fn second_depth() {
    //     let a = &Variable::new(String::from("a"));
    //     let b = &Variable::new(String::from("b"));
    //     let c = &Variable::new(String::from("c"));
    //     let d = &Variable::new(String::from("d"));

    //     let factorable = a * c + a * d + b * c + b * d;
    //     let expected = (a + b) * (c + d);
    //     assert!(AdditiveCommonFactor::apply(&factorable).contains(&expected));
    // }
}
