#[cfg(test)]
mod multiplication {
    use crate::{manipulation::expand::Expandable, symbols::variable::Variable};

    #[test]
    fn sample_1() {
        /* chain rule */
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        let d = &Variable::new(String::from("d"));

        let expanded = ((a + b) * (c + d)).expand();
        let expected = (a * c) + (a * d) + (b * c) + (b * d);

        assert_eq!(expanded, expected);
    }

    #[test]
    fn sample_2() {
        /* single distributive */
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        let expanded = ((a + b) * c).expand();
        let expected = a * c + b * c;
        assert_eq!(expanded, expected);

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        let expanded = (a * (b + c)).expand();
        let expected = a * b + a * c;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn sample_3() {
        /* mutiplicative distribution */
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        let d = &Variable::new(String::from("d"));

        let expanded = ((a * b) * (c + d)).expand();
        let expected = a * b * c + a * b * d;
        assert_eq!(expanded, expected);

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        let d = &Variable::new(String::from("d"));

        let expanded = ((a + b) * (c * d)).expand();
        let expected = a * c * d + b * c * d;
        assert_eq!(expanded, expected);
    }

    #[test]
    fn sample_4() {
        /* mutiplicative distribution */
        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));

        let expression = (a + b) * (a + b) * a;
        let expanded = expression.expand();

        assert_eq!(expanded, (a * a * a + a * b * a) + (b * a * a + b * b * a));
    }
}
