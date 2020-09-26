#[cfg(test)]
mod evaluable {
    use crate::{
        exponential::power::Power,
        manipulation::numeric_evaluation::NumericEvaluable,
        symbols::{number::Number, variable::Variable},
    };

    #[test]
    fn sample_1() {
        /* Must work constructor */
        let two_to_two = Power::new(Number::new(2.0), Number::new(2.0));
        assert_eq!(two_to_two.into_num(), Ok(4.0));
    }

    #[test]
    fn sample_2() {
        /* Operator overloading */
        let two_to_two = &(Number::new(2.0) ^ Number::new(2.0));
        assert_eq!(two_to_two.into_num(), Ok(4.0));
    }

    #[test]
    fn sample_3() {
        /* Primitives interface */
        let two_to_two = &(2.0 ^ Number::new(2.0));
        assert_eq!(two_to_two.into_num(), Ok(4.0));

        let two_to_two = &(2 ^ Number::new(2.0));
        assert_eq!(two_to_two.into_num(), Ok(4.0));

        let two_to_two = &(Number::new(2.0) ^ 2.0);
        assert_eq!(two_to_two.into_num(), Ok(4.0));

        let two_to_two = &(Number::new(2.0) ^ 2);
        assert_eq!(two_to_two.into_num(), Ok(4.0));
    }

    #[test]
    fn sample_4() {
        /* Incorporates expressions from string */
        let two_to_two = &("a" ^ Number::new(2.0));
        assert_eq!(
            two_to_two,
            &(Variable::new(String::from("a")) ^ Number::new(2.0))
        );

        let two_to_two = &(Number::new(2.0) ^ "a");
        assert_eq!(
            two_to_two,
            &(Number::new(2.0) ^ Variable::new(String::from("a")))
        );
    }
}

#[cfg(test)]
mod evaluable_against_other_operations {
    use crate::{manipulation::numeric_evaluation::NumericEvaluable, symbols::number::Number};

    #[test]
    fn sample_1() {
        /* Chains agains addition commutation */
        let three = &Number::new(3.0);

        assert_eq!(
            (three ^ (three + three)).into_num(), /* 3 ^ (3 * 3) == 729 */
            Ok(729.0)
        );

        assert_eq!(
            ((three + three) ^ three).into_num(), /* (3 + 3) ^ 3 == 216 */
            Ok(216.0)
        );
    }

    #[test]
    fn sample_2() {
        /* Chains agains multiplication commutation */
        let three = &Number::new(3.0);

        assert_eq!(
            (three ^ (three * three)).into_num(), /* 3 ^ (3 * 3) == 19683 */
            Ok(19683.0)
        );

        assert_eq!(
            ((three * three) ^ three).into_num(), /* (3 * 3) ^ 3 == 729 */
            Ok(729.0)
        );
    }

    #[test]
    fn sample_3() {
        /* Chains agains subtraction commutation */
        let three = &Number::new(3.0);
        let one = &Number::new(1.0);

        assert_eq!(
            (three ^ (three - one)).into_num(), /* 3 ^ (3 - 1) == 9 */
            Ok(9.0)
        );

        assert_eq!(
            ((three - one) ^ three).into_num(), /* (3 - 1) ^ 3 == 8 */
            Ok(8.0)
        );
    }

    #[test]
    fn sample_4() {
        /* Chains agains subtraction commutation */
        let three = &Number::new(3.0);

        assert_eq!(
            (three ^ (three / three)).into_num(), /* 3 ^ (3 / 3) == 3 */
            Ok(3.0)
        );

        assert_eq!(
            ((three / three) ^ three).into_num(), /* (3 / 3) ^ 3 == 1 */
            Ok(1.0)
        );
    }
}
