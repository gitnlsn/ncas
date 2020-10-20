use crate::arithmetics::addition::Addition;
use crate::base::expression::Expression;
use crate::manipulation::{identifiable::Identity, simplification_rules::rule::Rule};

use std::collections::HashMap;

pub struct AdditiveCommonAddend {}
impl Rule for AdditiveCommonAddend {
    fn apply(expression: &Expression) -> Expression {
        let mut alternatives: Vec<Expression> = Vec::new();
        
        match expression {
            Expression::CommutativeAssociation(a) => {
                if a.id() == Identity::Addition {
                    let mut addend_count_list: HashMap<Expression, isize> = HashMap::new();
                    for addend in a.items().iter() {
                        let opposite = &-addend;

                        if addend_count_list.contains_key(addend) {
                            let counter = addend_count_list.remove(addend).unwrap();
                            addend_count_list.insert(addend.clone(), counter + 1);
                        } else if addend_count_list.contains_key(opposite) {
                            let counter = addend_count_list.remove(opposite).unwrap();
                            addend_count_list.insert(opposite.clone(), counter - 1);
                        } else {
                            addend_count_list.insert(addend.clone(), 1);
                        }
                    }
                    let addends: Vec<Expression> = addend_count_list
                        .iter()
                        .filter(|&(_, counter)| counter != &0)
                        .map(|(expression, counter)| *counter as isize * expression)
                        .collect();

                    alternatives.push(Addition::new(addends));
                }
            }
            _ => {}
        }

        if alternatives.is_empty() {
            return vec![expression.clone()];
        }

        return alternatives;
    }
}
