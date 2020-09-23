#[cfg(test)]
mod base {
    use crate::base::{expression::Expression, symbols::number::Number};

    #[test]
    fn displays_label() {
        let n: Expression = Number::new(143.0);
        assert_eq!(format!("{}", n), String::from("143"));

        let n: Expression = Number::new(1.0);
        assert_eq!(format!("{}", n), String::from("1"));

        let n: Expression = Number::new(1.01);
        assert_eq!(format!("{}", n), String::from("1.01"));
    }
}

#[cfg(test)]
mod evaluable {
    use crate::{
        base::{expression::Expression, symbols::number::Number},
        manipulation::numeric_evaluation::NumericEvaluable,
    };

    #[test]
    fn not_evaluable() {
        let x: Expression = Number::new(1.0);
        assert!(x.into_num().is_ok());
        assert_eq!(x.into_num().unwrap(), 1.0);

        let x: Expression = Number::new(143.0);
        assert_eq!(x.into_num().unwrap(), 143.0);
    }
}
