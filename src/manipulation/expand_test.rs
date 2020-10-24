#[cfg(test)]
mod multiplication {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn sample_1() {
        /* mutiplicative distribution */
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let d = &Symbol::variable("d").expr();

        let expanded = ((a + b) * (c + d)).expand();
        let expected = (a * c) + (a * d) + (b * c) + (b * d);

        assert_eq!(expanded, expected);
    }

    #[test]
    fn sample_2() {
        /* single distributive */
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();

        let expanded = ((a + b) * c).expand();
        let expected = a * c + b * c;
        assert_eq!(expanded, expected);

        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();

        let expanded = (a * (b + c)).expand();
        let expected = a * b + a * c;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn sample_3() {
        /* mutiplicative distribution */
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();

        let expression = (a + b) * (a + b) * a;
        let expanded = expression.expand();

        assert_eq!(expanded, (a * a * a + a * b * a) + (b * a * a + b * b * a));
    }

    #[test]
    fn sample_4() {
        /* mutiplicative distribution */
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();

        let expression = Expression::power(a.clone(), (a + b) * (a + b) * a);
        let expanded = expression.expand();
        let expected =
            Expression::power(a.clone(), (a * a * a + a * b * a) + (b * a * a + b * b * a));

        assert_eq!(expanded, expected);
    }
}

#[cfg(test)]
mod power {
    use crate::base::symbol::Symbol;

    #[test]
    fn sample_1() {
        /* mutiplicative distribution */
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let d = &Symbol::integer(2).expr();

        let expanded = ((a + b).pow(d.clone())).expand();
        let expected = (a * a) + (a * b) + (a * b) + (b * b);

        assert_eq!(expanded, expected);
    }

    #[test]
    fn sample_2() {
        /* mutiplicative distribution */
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let d = &Symbol::integer(2).expr();

        let expression = a.clone().pow(d * (a + b));
        let expanded = expression.expand();

        assert_eq!(expanded, a.clone().pow(a * d + b * d));
    }
}
