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
        rational::{
            rationals_addition::RationalsAddition,
            rationals_multiplication::RationalsMultiplication,
        },
        rule::Rule,
    },
};

impl Expression {
    pub fn simplify(self) -> Expression {
        /* recursive expansion */
        match self.clone().expand() {
            Expression::Multiplication(factors) => {
                let simplified_inner_factors =
                    Expression::multiplication(factors.map(&|factor| factor.clone().simplify()));

                let simplified_rationals =
                    RationalsMultiplication::apply(&simplified_inner_factors);

                let simplified_common_factors =
                    MultiplicativeCommonFactor::apply(&simplified_rationals);

                return simplified_common_factors;
            }
            Expression::Addition(addends) => {
                let simplified_inner_addends =
                    Expression::addition(addends.map(&|addend| addend.clone().simplify()));

                let simplified_rationals = RationalsAddition::apply(&simplified_inner_addends);

                let simplified_common_addends = AdditiveCommonAddend::apply(&simplified_rationals);

                return simplified_common_addends;
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
