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
                for addend in addends.items().iter() {
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
