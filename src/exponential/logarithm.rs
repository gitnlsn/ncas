use crate::base::{
    associative_operation::AssociativeOperation, expression::Expression, symbol::Symbol,
};

impl Expression {
    pub fn logarithm(argument: Expression, base: Expression) -> Expression {
        match &argument {
            Expression::Power(power) => {
                if base == power.argument() {
                    return power.modifier();
                }
            }
            Expression::Integer(integer_argument) => match &base {
                Expression::Integer(integer_base) => {
                    let log_result: f64 =
                        (integer_argument.value().unwrap() as f64).log(integer_base.value().unwrap() as f64);
                    if (log_result as isize) as f64 == log_result {
                        return Symbol::integer(log_result as isize).expr();
                    }
                }
                _ => {}
            },
            Expression::Real(real_argument) => match &base {
                Expression::Real(real_base) => {
                    let log_result: f64 =
                        (real_argument.value().unwrap()).log(real_base.value().unwrap());
                    return Symbol::real(log_result).expr();
                }
                _ => {}
            },
            _ => {}
        }

        return Expression::Logarithm(AssociativeOperation::new(argument, base));
    }
    pub fn log(self, base: Expression) -> Self {
        Self::logarithm(self, base)
    }
}
