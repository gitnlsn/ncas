use crate::base::expression::Expression;
use crate::base::symbol::Symbol;
use crate::manipulation::simplification_rules::{rational::rationals::Rational, rule::Rule};

pub struct RationalsAddition {}
impl Rule for RationalsAddition {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Addition(addends) => {
                let rational: Rational = addends
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
                        Rational::from(Symbol::integer(0), Symbol::integer(1)),
                        |acc, new_value| acc + new_value,
                    );

                let non_rational = Expression::addition(addends.get_non_rationals());

                return rational.expr() + non_rational;
            }
            _ => return expression.clone(),
        }
    } /* end - apply */
} /* rule implementation */

#[cfg(test)]
mod simplify {
    use super::*;

    #[test]
    fn irreducible_fraction() {
        let addition = Expression::addition(vec![
            Symbol::integer(2).expr() / Symbol::integer(3).expr(),
            Symbol::integer(5).expr() / Symbol::integer(7).expr(),
        ]);

        let expected = Symbol::integer(29).expr() / Symbol::integer(21).expr();

        assert_eq!(RationalsAddition::apply(&addition), expected);
    }

    #[test]
    fn gcd_simplifiable() {
        let addition = Expression::addition(vec![
            Symbol::integer(1).expr() / Symbol::integer(2).expr(),
            Symbol::integer(1).expr() / Symbol::integer(3).expr(),
            Symbol::integer(1).expr() / Symbol::integer(6).expr(),
        ]);

        let expected = Symbol::integer(1).expr();

        assert_eq!(RationalsAddition::apply(&addition), expected);
    }
}
