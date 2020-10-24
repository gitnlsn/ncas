use crate::base::expression::Expression;
use crate::manipulation::expansion_rules::rule::Rule;

pub struct MultiplicativeDistributive {}
impl Rule for MultiplicativeDistributive {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Multiplication(factor_list) => {
                let distributable_addends: Vec<Expression> =
                    factor_list.get(&|factor| match factor {
                        Expression::Addition(_) => true,
                        _ => false,
                    });

                let non_distributable: Vec<Expression> = factor_list.get(&|factor| match factor {
                    Expression::Addition(_) => false,
                    _ => true,
                });

                let mut distributive_sum: Expression =
                    Expression::multiplication(non_distributable);

                for addition in distributable_addends.iter() {
                    distributive_sum = distribute(distributive_sum, addition.clone());
                }

                return distributive_sum;
            }
            _ => {
                return expression.clone();
            }
        }
    }
}

fn distribute(left: Expression, right: Expression) -> Expression {
    match (&left, &right) {
        (Expression::Addition(addend_list_1), Expression::Addition(addend_list_2)) => {
            let mut new_addends: Vec<Expression> = Vec::new();
            for addend_1 in addend_list_1.items().iter() {
                for addend_2 in addend_list_2.items().iter() {
                    new_addends.push(Expression::multiplication(vec![
                        addend_1.clone(),
                        addend_2.clone(),
                    ]));
                }
            }
            return Expression::addition(new_addends);
        }
        (Expression::Addition(addend_list), _) => {
            let mut new_addends: Vec<Expression> = Vec::new();
            for addend in addend_list.items().iter() {
                new_addends.push(Expression::multiplication(vec![
                    addend.clone(),
                    right.clone(),
                ]));
            }
            return Expression::addition(new_addends);
        }
        (_, Expression::Addition(addend_list)) => {
            let mut new_addends: Vec<Expression> = Vec::new();
            for addend in addend_list.items().iter() {
                new_addends.push(Expression::multiplication(vec![
                    addend.clone(),
                    left.clone(),
                ]));
            }
            return Expression::addition(new_addends);
        }
        (_, _) => {
            return Expression::multiplication(vec![left, right]);
        }
    }
}

#[cfg(test)]
mod distribute {
    use super::*;
    use crate::base::symbol::Symbol;

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let d = &Symbol::variable("d").expr();

        let result = a * c + a * d + b * c + b * d;

        assert_eq!(distribute(a + b, c + d), result);
    }

    #[test]
    fn sample_2() {
        let a = &Symbol::variable("a").expr();
        let c = &Symbol::variable("c").expr();
        let d = &Symbol::variable("d").expr();

        let result = a * c + a * d;

        assert_eq!(distribute(a.clone(), c + d), result);
    }

    #[test]
    fn sample_3() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();

        let result = a * c + b * c;

        assert_eq!(distribute(a + b, c.clone()), result);
    }
}
