#[cfg(test)]
mod evaluable {
    use crate::{
        base::symbols::{constant::Constant, number::Number, variable::Variable},
        manipulation::numeric_evaluation::NumericEvaluable,
    };

    #[test]
    fn multiply_two_numbers() {
        let three = Number::new(3.0);
        let two = Number::new(2.0);
        let sum = three * two;

        assert!(sum.into_num().is_ok());
        assert_eq!(sum.into_num().unwrap(), 6.0);
    }

    #[test]
    fn multiply_several_numbers() {
        let mut sum = Number::new(1.0);
        for _ in 0..10 {
            sum = sum * Number::new(2.0);
        }

        assert!(sum.into_num().is_ok());
        assert_eq!(sum.into_num().unwrap(), 1024.0);
    }

    #[test]
    fn multiply_constants() {
        let constant_c = Constant::new(String::from("C"), 3.0);
        let two = Number::new(2.0);
        let sum = constant_c * two;

        assert!(sum.into_num().is_ok());
        assert_eq!(sum.into_num().unwrap(), 6.0);
    }

    #[test]
    fn do_not_multiply_variable() {
        let var_x = Variable::new(String::from("x"));
        let two = Number::new(2.0);
        let sum = var_x * two;

        assert!(sum.into_num().is_err());
    }
}
