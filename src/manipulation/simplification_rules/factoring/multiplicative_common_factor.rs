use crate::base::expression::Expression;
use crate::base::symbol::Symbol;
use crate::manipulation::simplification_rules::rule::Rule;

use std::collections::HashMap;

pub struct MultiplicativeCommonFactor {}
impl Rule for MultiplicativeCommonFactor {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Multiplication(factors) => {
                let mut power_list: HashMap<Expression, Expression> = HashMap::new();
                for factor in factors.items().iter() {
                    match factor {
                        Expression::Power(power) => {
                            let base = power.argument();
                            let exponent = power.modifier();

                            let inverse = &(Symbol::integer(1).expr() / base.clone());
                            if power_list.contains_key(&base) {
                                let current_exponent = power_list.remove(&base).unwrap();
                                power_list.insert(base.clone(), current_exponent + exponent);
                            } else if power_list.contains_key(inverse) {
                                let counter = power_list.remove(inverse).unwrap();
                                power_list.insert(inverse.clone(), counter - exponent);
                            } else {
                                power_list.insert(base.clone(), exponent.clone());
                            }
                        }
                        _ => {
                            let inverse = &(Symbol::integer(1).expr() / factor);
                            if power_list.contains_key(factor) {
                                let counter = power_list.remove(factor).unwrap();
                                power_list
                                    .insert(factor.clone(), counter + Symbol::integer(1).expr());
                            } else if power_list.contains_key(inverse) {
                                let counter = power_list.remove(inverse).unwrap();
                                power_list
                                    .insert(inverse.clone(), counter - Symbol::integer(1).expr());
                            } else {
                                power_list.insert(factor.clone(), Symbol::integer(1).expr());
                            }
                        }
                    }
                }

                let factors: Vec<Expression> = power_list
                    .iter()
                    .filter(|&(_, exponent)| {
                        exponent != &Symbol::integer(0).expr()
                            && exponent != &Symbol::real(0.0).expr()
                    })
                    .map(|(base, exponent)| Expression::power(base.clone(), exponent.clone()))
                    .collect();

                return Expression::multiplication(factors);
            }
            _ => {}
        }

        return expression.clone();
    } /* end - apply */
}

#[cfg(test)]
mod simplify {
    use super::*;

    #[test]
    fn factors_symbols() {
        let expression = Expression::multiplication(vec![
            Symbol::variable("a").expr(),
            Symbol::variable("a").expr(),
            Symbol::variable("a").expr(),
            Symbol::variable("b").expr(),
        ]);

        let expected = Expression::multiplication(vec![
            Symbol::variable("a").expr().pow(
                Symbol::integer(1).expr() + Symbol::integer(1).expr() + Symbol::integer(1).expr(),
            ),
            Symbol::variable("b").expr(),
        ]);

        let factored = MultiplicativeCommonFactor::apply(&expression);

        assert_eq!(factored, expected);
    }

    #[test]
    fn factors_expressions() {
        let n1 = &Symbol::integer(1).expr();
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();

        let expression =
            Expression::multiplication(vec![a * b, a * b, a * b, a.clone(), b.clone()]);

        let expected = Expression::multiplication(vec![
            a.clone().pow(n1 + n1 + n1 + n1),
            b.clone().pow(n1 + n1 + n1 + n1),
        ]);

        let factored = MultiplicativeCommonFactor::apply(&expression);

        assert_eq!(factored, expected);
    }

    #[test]
    fn subtracts_inverse() {
        let n1 = &Symbol::integer(1).expr();
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();

        let expression = Expression::multiplication(vec![a / b, a / b, a / b]);

        let expected = Expression::multiplication(vec![
            a.clone().pow(n1 + n1 + n1),
            b.clone().pow(-n1 - n1 - n1),
        ]);

        let factored = MultiplicativeCommonFactor::apply(&expression);

        assert_eq!(factored, expected);
    }

    #[test]
    fn nullifies_inverse() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();

        let expression = Expression::multiplication(vec![a / b, b / a]);

        let expected = Symbol::integer(1).expr();

        let factored = MultiplicativeCommonFactor::apply(&expression);

        assert_eq!(factored, expected);
    }
}
