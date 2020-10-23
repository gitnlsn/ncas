#[cfg(test)]
mod rule_testing {
    use crate::base::symbol::Symbol;
    use crate::manipulation::expansion_rules::{
        multiplicative_distributive::MultiplicativeDistributive, rule::Rule,
    };

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let d = &Symbol::variable("d").expr();

        let expandable = (a + b) * (c + d);
        let result = a * c + a * d + b * c + b * d;

        assert_eq!(MultiplicativeDistributive::apply(expandable), result);
    }

    #[test]
    fn sample_2() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let d = &Symbol::variable("d").expr();
        let e = &Symbol::variable("e").expr();

        let expandable = (a + b) * (c + d) * e;
        let result = a * c * e + a * d * e + b * c * e + b * d * e;

        assert_eq!(MultiplicativeDistributive::apply(expandable), result);
    }
}
