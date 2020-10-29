// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::{
    base::expression::Expression,
    manipulation::simplification_rules::{
        factoring::{
            additive_common_addend::AdditiveCommonAddend,
            multiplicative_common_factor::MultiplicativeCommonFactor,
        },
        rule::Rule,
    },
};

impl Expression {
    pub fn simplify(self) -> Expression {
        /* recursive expansion */
        match self.clone().expand() {
            Expression::Multiplication(factors) => {
                return MultiplicativeCommonFactor::apply(&Expression::multiplication(
                    factors.map(&|factor| factor.clone().simplify()),
                ));
            }
            Expression::Addition(addends) => {
                return AdditiveCommonAddend::apply(&Expression::addition(
                    addends.map(&|addend| addend.clone().simplify()),
                ));
            }

            Expression::Power(power) => {
                return Expression::power(power.argument().simplify(), power.modifier().simplify());
            }
            Expression::Logarithm(log) => {
                return Expression::logarithm(log.argument().simplify(), log.modifier().simplify())
            }

            _ => return self,
        }
    }
}
