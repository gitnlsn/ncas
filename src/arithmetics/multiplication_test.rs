#[cfg(test)]
mod evaluable {
    use crate::{
        base::symbols::{constant::Constant, number::Number, variable::Variable},
        manipulation::evaluate::Evaluable,
    };

    #[test]
    fn multiply_two_numbers() {
        let three = Number::new(3.0);
        let two = Number::new(2.0);
        let mut sum = three * two;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), 6.0);
    }

    #[test]
    fn multiply_several_numbers() {
        let mut sum = Number::new(1.0);
        for _ in 0..10 {
            sum = sum * Number::new(2.0);
        }

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), 1024.0);
    }

    #[test]
    fn multiply_constants() {
        let constant_c = Constant::new(String::from("C"), 3.0);
        let two = Number::new(2.0);
        let mut sum = constant_c * two;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), 6.0);
    }

    #[test]
    fn do_not_multiply_variable() {
        let var_x = Variable::new(String::from("x"));
        let two = Number::new(2.0);
        let mut sum = var_x * two;

        assert!(sum.evaluate().is_err());
    }
}

#[cfg(test)]
mod formatable {
    use crate::base::symbols::number::Number;

    #[test]
    fn formats_with_plus_symbol() {
        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let sum = one * two;
        assert_eq!(format!("{}", sum), String::from("1 * 2"));

        let mut sum = Number::new(0.0);
        for _ in 0..10 {
            sum = sum * Number::new(1.0);
        }
        assert_eq!(
            format!("{}", sum),
            String::from("0 * 1 * 1 * 1 * 1 * 1 * 1 * 1 * 1 * 1 * 1")
        );
    }
}