use crate::base::expression::Expression;
use crate::base::symbol::Symbol;
use crate::manipulation::simplification_rules::{rational::rationals::Rational, rule::Rule};

pub struct RationalsMultiplication {}
impl Rule for RationalsMultiplication {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Multiplication(factors) => {
                let rational: Rational = factors
                    .get_rationals()
                    .iter()
                    .cloned()
                    .map(
                        |rational_addend| match Rational::from_expression(rational_addend) {
                            Some(rational) => rational,
                            None => panic!("Unexpected value"),
                        },
                    )
                    .fold(
                        Rational::from(Symbol::integer(1), Symbol::integer(1)),
                        |acc, new_value| acc * new_value,
                    );

                let non_rational = Expression::multiplication(factors.get_non_rationals());

                return rational.expr() * non_rational;
            }
            _ => return expression.clone(),
        }
    } /* end - apply */
} /* rule implementation */

#[cfg(test)]
mod simplify {
    use super::*;

    #[test]
    fn irreducible_multiplication() {
        let multiplication = Expression::multiplication(vec![
            Symbol::integer(2).expr() / Symbol::integer(3).expr(),
            Symbol::integer(5).expr() / Symbol::integer(7).expr(),
        ]);

        let expected = Symbol::integer(10).expr() / Symbol::integer(21).expr();

        assert_eq!(RationalsMultiplication::apply(&multiplication), expected);
    }

    #[test]
    fn negative_expression() {
        let multiplication = Expression::multiplication(vec![
            Symbol::integer(-1).expr(),
            Symbol::variable("a").expr(),
        ]);

        let expected = multiplication.clone();

        assert_eq!(RationalsMultiplication::apply(&multiplication), expected);
    }

    #[test]
    fn gcd_simplifiable() {
        let multiplication = Expression::multiplication(vec![
            Symbol::integer(2).expr() / Symbol::integer(3).expr(),
            Symbol::integer(6).expr() / Symbol::integer(7).expr(),
        ]);

        let expected = Symbol::integer(4).expr() / Symbol::integer(7).expr();

        assert_eq!(RationalsMultiplication::apply(&multiplication), expected);
    }
}
