#[cfg(test)]
mod constructor {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn evaluates_real() {
        /* Must work constructor */
        let log_4_2 = Expression::logarithm(Symbol::real(4.0).expr(), Symbol::real(2.0).expr());
        assert_eq!(log_4_2, Symbol::real(2.0).expr());

        let log_8_2 = Expression::logarithm(Symbol::real(8.0).expr(), Symbol::real(2.0).expr());
        assert_eq!(log_8_2, Symbol::real(3.0).expr());

        let log_1024_2 =
            Expression::logarithm(Symbol::real(1024.0).expr(), Symbol::real(2.0).expr());
        assert_eq!(log_1024_2, Symbol::real(10.0).expr());
    }

    #[test]
    fn evaluates_integer() {
        /* Must work constructor */
        let log_4_2 = Expression::logarithm(Symbol::integer(4).expr(), Symbol::integer(2).expr());
        assert_eq!(log_4_2, Symbol::integer(2).expr());

        let log_8_2 = Expression::logarithm(Symbol::integer(8).expr(), Symbol::integer(2).expr());
        assert_eq!(log_8_2, Symbol::integer(3).expr());

        let log_1024_2 =
            Expression::logarithm(Symbol::integer(1024).expr(), Symbol::integer(2).expr());
        assert_eq!(log_1024_2, Symbol::integer(10).expr());
    }

    #[test]
    fn simplifies_identity_log_power() {
        /* Must work constructor */
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        let trial = Expression::logarithm(
            Expression::power(whatever.clone(), Symbol::integer(4).expr()),
            whatever.clone(),
        );

        assert_eq!(trial, Symbol::integer(4).expr());
    }
} /* end - constructor test */
