use crate::base::expression::Expression;
use crate::base::symbol::Symbol;
use crate::manipulation::simplification_rules::rule::Rule;

use std::collections::HashMap;

pub struct AdditiveCommonAddend {}
impl Rule for AdditiveCommonAddend {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Addition(addends) => {
                let mut addend_count_list: HashMap<Expression, isize> = HashMap::new();

                fn update_addend_list(
                    mapping: &mut HashMap<Expression, isize>,
                    addend: &Expression,
                    counter: isize,
                ) {
                    let opposite = &-addend;

                    if mapping.contains_key(addend) {
                        let current_counter = mapping.remove(addend).unwrap();
                        mapping.insert(addend.clone(), current_counter + counter);
                    } else if mapping.contains_key(opposite) {
                        let current_counter = mapping.remove(opposite).unwrap();
                        mapping.insert(opposite.clone(), current_counter - counter);
                    } else {
                        mapping.insert(addend.clone(), counter);
                    }
                }

                for addend in addends.items().iter() {
                    match addend {
                        Expression::Multiplication(factors) => {
                            match factors.get_one(&|factor| match factor {
                                Expression::Integer(_) => true,
                                _ => false,
                            }) {
                                /* Integer factor in multiplication */
                                Some(Expression::Integer(integer)) => {
                                    let remaining_factors = factors.get(&|factor| match factor {
                                        Expression::Integer(_) => false,
                                        _ => true,
                                    });
                                    update_addend_list(
                                        &mut addend_count_list,
                                        &Expression::multiplication(remaining_factors),
                                        integer.value().unwrap(),
                                    );
                                }

                                /* No Integer factor in multiplication */
                                _ => {
                                    update_addend_list(&mut addend_count_list, addend, 1);
                                }
                            };
                        }
                        /* Not multiplication */
                        _ => {
                            update_addend_list(&mut addend_count_list, addend, 1);
                        }
                    }
                }
                let addends: Vec<Expression> = addend_count_list
                    .iter()
                    .filter(|&(_, counter)| counter != &0)
                    .map(|(expression, counter)| Symbol::integer(*counter).expr() * expression)
                    .collect();

                return Expression::addition(addends);
            }
            _ => {}
        }

        return expression.clone();
    }
}

#[cfg(test)]
mod simplify {
    use super::*;

    #[test]
    fn factors_symbols() {
        let expression = Expression::addition(vec![
            Symbol::variable("a").expr(),
            Symbol::variable("a").expr(),
            Symbol::variable("a").expr(),
            Symbol::variable("b").expr(),
        ]);

        let expected = Expression::addition(vec![
            Symbol::integer(3).expr() * Symbol::variable("a").expr(),
            Symbol::variable("b").expr(),
        ]);

        let factored = AdditiveCommonAddend::apply(&expression);

        assert_eq!(factored, expected);
    }

    #[test]
    fn factors_expressions() {
        let expression = Expression::addition(vec![
            Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("a").expr(),
            Symbol::variable("b").expr(),
        ]);

        let expected = Expression::addition(vec![
            Symbol::integer(3).expr() * Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("b").expr(),
            Symbol::variable("a").expr(),
        ]);

        let factored = AdditiveCommonAddend::apply(&expression);

        assert_eq!(factored, expected);
    }

    #[test]
    fn simplifies_integer_multiplied_addends() {
        let expression = Expression::addition(vec![
            Symbol::integer(2).expr() * Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::integer(-2).expr()
                * Symbol::variable("a").expr()
                * Symbol::variable("b").expr(),
            Symbol::variable("b").expr(),
        ]);

        let expected = Expression::addition(vec![Symbol::variable("b").expr()]);

        let factored = AdditiveCommonAddend::apply(&expression);

        assert_eq!(factored, expected);
    }

    #[test]
    fn subtracts_opposite() {
        let expression = Expression::addition(vec![
            -Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("a").expr() * Symbol::variable("b").expr(),
        ]);

        let expected = Expression::addition(vec![
            Symbol::integer(2).expr() * Symbol::variable("a").expr() * Symbol::variable("b").expr(),
        ]);

        let factored = AdditiveCommonAddend::apply(&expression);

        assert_eq!(factored, expected);
    }

    #[test]
    fn nullifies_opposite() {
        let expression = Expression::addition(vec![
            -Symbol::variable("a").expr() * Symbol::variable("b").expr(),
            Symbol::variable("a").expr() * Symbol::variable("b").expr(),
        ]);

        let expected = Symbol::integer(0).expr();

        let factored = AdditiveCommonAddend::apply(&expression);

        assert_eq!(factored, expected);
    }
}
