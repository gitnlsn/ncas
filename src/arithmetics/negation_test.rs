#[cfg(test)]
mod evaluable {
    use crate::{
        base::symbols::{constant::Constant, number::Number},
        manipulation::numeric_evaluation::NumericEvaluable,
    };

    #[test]
    fn negate_number_sign() {
        let one = Number::new(1.0);
        let mut minus_one = -one;
        assert!(minus_one.into_num().is_ok());
        assert_eq!(minus_one.into_num().unwrap(), -1.0);
    }

    #[test]
    fn negate_constant() {
        let one = Constant::new(String::from("one"), 1.0);
        let mut minus_one = -one;

        assert!(minus_one.into_num().is_ok());
        assert_eq!(minus_one.into_num().unwrap(), -1.0);
    }

    #[test]
    fn formats_with_minus_one_multiply() {
        let one = Number::new(1.0);
        let minus_one = -one;
        assert_eq!(format!("{}", minus_one), String::from("-1 * 1"));
    }
}
