// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::{
    base::expression::Expression,
    manipulation::expansion_rules::{
        multiplicative_distributive::MultiplicativeDistributive,
        power_distributive::PowerDistributive, rule::Rule,
    },
};

impl Expression {
    pub fn expand(self) -> Expression {
        /* Rules expansion */
        let expanded = MultiplicativeDistributive::apply(&self);
        let expanded = PowerDistributive::apply(&expanded);

        /* recursive expansion */
        match &expanded {
            Expression::Multiplication(factors) => {
                return Expression::multiplication(factors.map(&|factor| factor.clone().expand()));
            }
            Expression::Addition(addends) => {
                return Expression::addition(addends.map(&|addend| addend.clone().expand()));
            }

            Expression::Power(power) => {
                return Expression::power(power.argument().expand(), power.modifier().expand());
            }
            Expression::Logarithm(log) => {
                return Expression::logarithm(log.argument().expand(), log.modifier().expand())
            }

            _ => return self,
        }
    }
}
