#[cfg(test)]
mod evaluable {
    use crate::{
        base::symbols::{number::Number},
        manipulation::evaluate::Evaluable,
    };

    #[test]
    fn multiplication_precedes_addition() {
        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let three = Number::new(3.0);
        let mut sum = one + two * three;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), 7.0);

        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let three = Number::new(3.0);
        let mut sum = one * two + three;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), 5.0);
    }

    #[test]
    fn multiplication_precedes_subtraction() {
        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let three = Number::new(3.0);
        let mut sum = one - two * three;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), -5.0);

        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let three = Number::new(3.0);
        let mut sum = one * two - three;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), -1.0);
    }

    #[test]
    fn division_precedes_addition() {
        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let four = Number::new(4.0);
        let mut sum = one + two / four;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), 1.5);

        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let four = Number::new(4.0);
        let mut sum = one / two + four;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), 4.5);
    }

    #[test]
    fn division_precedes_subtraction() {
        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let four = Number::new(4.0);
        let mut sum = one - two * four;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), -7.0);

        let one = Number::new(1.0);
        let two = Number::new(2.0);
        let four = Number::new(4.0);
        let mut sum = one * two - four;

        assert!(sum.evaluate().is_ok());
        assert_eq!(sum.evaluate().unwrap(), -2.0);
    }
}
