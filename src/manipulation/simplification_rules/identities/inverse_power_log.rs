use crate::base::expression::Expression;

use crate::manipulation::{
    identifiable::{Identifiable, Identity},
    simplification_rules::rule::Rule,
};

pub struct InversePowerLog {}
impl Rule for InversePowerLog {
    fn apply(expression: &Expression) -> Vec<Expression> {
        let mut alternatives: Vec<Expression> = Vec::new();

        match expression {
            Expression::AssociativeOperation(outer_operation) => match expression.id() {
                Identity::Power => {
                    /* Outer operator is Power */
                    match outer_operation.modifier().as_ref() {
                        Expression::AssociativeOperation(inner_operation) => {
                            match inner_operation.id() {
                                Identity::Logarithm => {
                                    /* Inner operator is Logarithm */
                                    let exponential_base: &Expression =
                                        outer_operation.argument().as_ref();
                                    let logarithmic_base: &Expression =
                                        inner_operation.modifier().as_ref();
                                    if exponential_base == logarithmic_base {
                                        let logarithmic_argument: &Expression =
                                            inner_operation.argument().as_ref();
                                        alternatives.push(logarithmic_argument.clone());
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Identity::Logarithm => {
                    /* Outer operator is Logarithm */
                    match outer_operation.argument().as_ref() {
                        Expression::AssociativeOperation(inner_operation) => {
                            match inner_operation.id() {
                                Identity::Power => {
                                    /* Inner operator is Power */
                                    let logarithmic_base: &Expression =
                                        outer_operation.modifier().as_ref();
                                    let exponential_base: &Expression =
                                        inner_operation.argument().as_ref();
                                    if exponential_base == logarithmic_base {
                                        let exponential_exponent: &Expression =
                                            inner_operation.modifier().as_ref();
                                        alternatives.push(exponential_exponent.clone());
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
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
    use crate::exponential::{logarithm::Log, power::Power};

    #[test]
    fn power_of_log() {
        use crate::symbols::variable::Variable;

        let expression = Power::new(
            Variable::new(String::from("a")),
            Log::new(
                Variable::new(String::from("x")),
                Variable::new(String::from("a")),
            ),
        );

        let expected = Variable::new(String::from("x"));

        let factored = InversePowerLog::apply(&expression).pop().unwrap();

        assert_eq!(factored, expected);
    }

    #[test]
    fn log_of_power() {
        use crate::symbols::variable::Variable;

        let expression = Log::new(
            Power::new(
                Variable::new(String::from("a")),
                Variable::new(String::from("x")),
            ),
            Variable::new(String::from("a")),
        );

        let expected = Variable::new(String::from("x"));

        let factored = InversePowerLog::apply(&expression).pop().unwrap();

        assert_eq!(factored, expected);
    }
}
