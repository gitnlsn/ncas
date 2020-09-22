#[cfg(test)]
mod base {
    use crate::base::{expression::Expression, symbols::constant::Constant};

    #[test]
    fn displays_label() {
        let x: Expression = Constant::new(String::from("x"), 143.0);
        assert_eq!(format!("{}", x), String::from("x"));
    }
}

#[cfg(test)]
mod evaluable {
    use crate::{
        base::{expression::Expression, symbols::constant::Constant},
        manipulation::evaluate::Evaluable,
    };

    #[test]
    fn constant_is_evaluable() {
        let mut x: Expression = Constant::new(String::from("x"), 1.0);
        assert!(x.evaluate().is_ok());
        assert_eq!(x.evaluate().unwrap(), 1.0);

        let mut x: Expression = Constant::new(String::from("x"), 143.0);
        assert!(x.evaluate().is_ok());
        assert_eq!(x.evaluate().unwrap(), 143.0);
    }
}
