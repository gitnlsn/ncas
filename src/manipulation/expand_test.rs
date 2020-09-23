#[cfg(test)]
mod multiplication {
    use crate::{
        base::symbols::{variable::Variable},
        manipulation::{expand::Expandable},
    };

    #[test]
    fn sample_1() {
        /* chain rule */
        let a = Variable::new(String::from("a"));
        let b = Variable::new(String::from("b"));
        let c = Variable::new(String::from("c"));
        let d = Variable::new(String::from("d"));

        let expression = (a + b) * (c + d);

        let expanded = expression.expand();
        assert_eq!(
            format!("{}", expanded),
            String::from("((a * c + a * d) + (b * c + b * d))")
        );
    }

    #[test]
    fn sample_2() {
        /* single distributive */
        let a = Variable::new(String::from("a"));
        let b = Variable::new(String::from("b"));
        let c = Variable::new(String::from("c"));

        let expression = (a + b) * c;
        let expanded = expression.expand();
        assert_eq!(format!("{}", expanded), String::from("(a * c + b * c)"));

        let a = Variable::new(String::from("a"));
        let b = Variable::new(String::from("b"));
        let c = Variable::new(String::from("c"));

        let expression = a * (b + c);
        let expanded = expression.expand();
        assert_eq!(format!("{}", expanded), String::from("(a * b + a * c)"));
    }

    #[test]
    fn sample_3() {
        /* mutiplicative distribution */
        let a = Variable::new(String::from("a"));
        let b = Variable::new(String::from("b"));
        let c = Variable::new(String::from("c"));
        let d = Variable::new(String::from("d"));

        let expression = (a * b) * (c + d);

        let expanded = expression.expand();
        assert_eq!(
            format!("{}", expanded),
            String::from("(a * b * c + a * b * d)")
        );

        let a = Variable::new(String::from("a"));
        let b = Variable::new(String::from("b"));
        let c = Variable::new(String::from("c"));
        let d = Variable::new(String::from("d"));

        let expression = (a + b) * (c * d);

        let expanded = expression.expand();
        assert_eq!(
            format!("{}", expanded),
            String::from("(a * c * d + b * c * d)")
        );
    }

    #[test]
    fn sample_4() {
        /* mutiplicative distribution */
        let a = Variable::new(String::from("a"));
        let b = Variable::new(String::from("b"));

        let ab_plus = a.clone() + b.clone();
        let expression = ab_plus.clone() * ab_plus.clone() * a.clone();

        let expanded = expression.expand().expand().expand();
        assert_eq!(
            format!("{}", expanded),
            String::from("((a * a * a + a * b * a) + (b * a * a + b * b * a))")
        );
    }
}
