#[cfg(test)]
mod evaluable {
    use crate::{
        base::symbols::{constant::Constant, number::Number, variable::Variable},
        manipulation::numeric_evaluation::NumericEvaluable,
    };

    #[test]
    fn divides_two_numbers() {
        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let sum = one / two;

        assert!(sum.into_num().is_ok());
        assert_eq!(sum.into_num().unwrap(), 0.5);
    }

    #[test]
    fn divides_several_numbers() {
        let mut sum = Number::new(1024.0);
        for _ in 0..10 {
            sum = sum / Number::new(2.0);
        }

        assert!(sum.into_num().is_ok());
        assert_eq!(sum.into_num().unwrap(), 1.0);
    }

    #[test]
    fn divides_constants() {
        let constant_c = Constant::new(String::from("C"), 1.0);
        let two = Number::new(2.0);
        let sum = constant_c / two;

        assert!(sum.into_num().is_ok());
        assert_eq!(sum.into_num().unwrap(), 0.5);
    }

    #[test]
    fn do_not_add_variable() {
        let var_x = Variable::new(String::from("x"));
        let two = Number::new(2.0);
        let sum = var_x / two;

        assert!(sum.into_num().is_err());
    }
}
