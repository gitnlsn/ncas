#[cfg(test)]
mod evaluable {
    use crate::{
        exponential::logarithm::Log, manipulation::numeric_evaluation::NumericEvaluable,
        symbols::number::Number,
    };

    #[test]
    fn sample_1() {
        /* Must work constructor */
        let log_4_2 = Log::new(Number::new(4.0), Number::new(2.0));
        assert_eq!(log_4_2.into_num(), Ok(2.0));

        let log_8_2 = Log::new(Number::new(8.0), Number::new(2.0));
        assert_eq!(log_8_2.into_num(), Ok(3.0));
    }
}

#[cfg(test)]
mod evaluable_against_other_operations {
    use crate::{
        exponential::logarithm::Log, manipulation::numeric_evaluation::NumericEvaluable,
        symbols::number::Number,
    };

    #[test]
    fn addition() {
        /* Chains agains Addition commutation */
        assert_eq!(
            Log::new(Number::new(16.0), Number::new(1.0) + Number::new(1.0)).into_num(), /* log(16, 1 + 1) == 4 */
            Ok(4.0)
        );

        assert_eq!(
            Log::new(Number::new(15.0) + Number::new(1.0), Number::new(2.0)).into_num(), /* log(15 + 1, 2) == 4 */
            Ok(4.0)
        );
    }

    #[test]
    fn multiplication() {
        /* Chains agains multiplication commutation */
        assert_eq!(
            Log::new(Number::new(16.0), Number::new(2.0) * Number::new(2.0)).into_num(), /* log(16, 4) = 2.0 */
            Ok(2.0)
        );

        assert_eq!(
            Log::new(Number::new(4.0) * Number::new(4.0), Number::new(2.0)).into_num(), /* log(4 * 4, 2) = 4.0 */
            Ok(4.0)
        );
    }
}
