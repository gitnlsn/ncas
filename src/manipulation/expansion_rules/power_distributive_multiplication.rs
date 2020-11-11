use crate::base::{expression::Expression};
use crate::manipulation::expansion_rules::rule::Rule;

/**
 * Expands power with multiplicative base.
 * Distributes the exponent over its factors resulting in a multiplicative expression.
 */
pub struct PowerDistributiveMultiplication {}
impl Rule for PowerDistributiveMultiplication {
    fn apply(expression: &Expression) -> Expression {
        match expression {
            Expression::Power(power) => match &power.argument() {
                // Distributive: (a * b) ^ c == a ^ c * b ^ c
                Expression::Multiplication(base_factors) => {
                    return Expression::multiplication(
                        base_factors
                            .map(&|factor| Expression::power(factor.clone(), power.modifier())),
                    );
                }
                _ => {}
            },
            _ => {}
        }
        return expression.clone();
    }
}

#[cfg(test)]
mod simplify {
    use super::*;
    use crate::base::symbol::Symbol;

    #[test]
    fn power_into_multiplication() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();

        let trial = (a * b).pow(c.clone());
        let expected = a.clone().pow(c.clone()) * b.clone().pow(c.clone());

        assert_eq!(PowerDistributiveMultiplication::apply(&trial), expected);
    }
}