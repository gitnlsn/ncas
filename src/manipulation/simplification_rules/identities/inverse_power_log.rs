use crate::base::expression::Expression;

use crate::manipulation::simplification_rules::rule::Rule;

pub struct InversePowerLog {}
impl Rule for InversePowerLog {
    fn apply(expression: &Expression) -> Vec<Expression> {
        let mut alternatives: Vec<Expression> = Vec::new();

        match expression {
            Expression::Power(power) => {
                /* Outer operator is Power */
                match power.modifier() {
                    Expression::Logarithm(log) => {
                        /* Inner operator is Logarithm */
                        let exponential_base: Expression = power.argument();
                        let logarithmic_base: Expression = log.modifier();
                        if exponential_base == logarithmic_base {
                            let logarithmic_argument: Expression = log.argument();
                            alternatives.push(logarithmic_argument.clone());
                        }
                    }
                    _ => {}
                }
            }
            Expression::Logarithm(log) => {
                /* Outer operator is Logarithm */
                match log.argument() {
                    Expression::Power(power) => {
                        /* Inner operator is Power */
                        let logarithmic_base: Expression = log.modifier();
                        let exponential_base: Expression = power.argument();
                        if exponential_base == logarithmic_base {
                            let exponential_exponent: Expression = power.modifier();
                            alternatives.push(exponential_exponent.clone());
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        };

        if alternatives.is_empty() {
            return vec![expression.clone()];
        }

        return alternatives;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::base::expression::Expression;

    #[test]
    fn power_of_log() {
        use crate::base::symbol::Symbol;

        let expression = Expression::power(
            Symbol::variable("a").expr(),
            Expression::logarithm(Symbol::variable("x").expr(), Symbol::variable("a").expr()),
        );

        let expected = Symbol::variable("x").expr();

        let factored = InversePowerLog::apply(&expression).pop().unwrap();

        assert_eq!(factored, expected);
    }

    #[test]
    fn log_of_power() {
        use crate::base::symbol::Symbol;

        let expression = Expression::logarithm(
            Expression::power(Symbol::variable("a").expr(), Symbol::variable("x").expr()),
            Symbol::variable("a").expr(),
        );

        let expected = Symbol::variable("x").expr();

        let factored = InversePowerLog::apply(&expression).pop().unwrap();

        assert_eq!(factored, expected);
    }
}
