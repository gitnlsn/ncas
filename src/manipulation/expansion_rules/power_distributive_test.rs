#[cfg(test)]
mod rule_testing {
    use crate::base::symbol::Symbol;
    use crate::manipulation::expansion_rules::{power_distributive::PowerDistributive, rule::Rule};

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let two = &Symbol::integer(2).expr();

        let expandable = (a + b).pow(two.clone());
        let result = a * a + a * b + a * b + b * b;

        assert_eq!(PowerDistributive::apply(&expandable), result);
    }

    #[test]
    fn sample_2() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let two = &Symbol::integer(2).expr();

        let expandable = (a + b).pow(c * two);
        let result = (a * a + a * b + a * b + b * b).pow(c.clone());

        assert_eq!(PowerDistributive::apply(&expandable), result);
    }
}
