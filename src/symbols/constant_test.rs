#[cfg(test)]
mod base {
    use crate::{base::expression::Expression, symbols::constant::Constant};

    #[test]
    fn displays_label() {
        let x: Expression = Constant::new(String::from("x"), 143.0);
        assert_eq!(format!("{}", x), String::from("x"));
    }
}

#[cfg(test)]
mod evaluable {
    use crate::{
        base::expression::Expression, manipulation::numeric_evaluation::NumericEvaluable,
        symbols::constant::Constant,
    };

    #[test]
    fn constant_is_evaluable() {
        let x: Expression = Constant::new(String::from("x"), 1.0);
        assert!(x.into_num().is_ok());
        assert_eq!(x.into_num().unwrap(), 1.0);

        let x: Expression = Constant::new(String::from("x"), 143.0);
        assert!(x.into_num().is_ok());
        assert_eq!(x.into_num().unwrap(), 143.0);
    }
}
