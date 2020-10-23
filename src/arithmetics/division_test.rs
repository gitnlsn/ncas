#[cfg(test)]
mod constructor {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn divides_integer_by_integer() {
        let div = Symbol::integer(6).expr() / Symbol::integer(3).expr();

        if let Expression::Integer(n) = div {
            assert_eq!(n, Symbol::integer(2));
        } else {
            panic!();
        }
    }

    #[test]
    fn irreducible_fraction_is_kept_given_gcd() {
        let div = Symbol::integer(9).expr() / Symbol::integer(6).expr();

        if let Expression::Multiplication(factors) = div {
            assert!(factors.items().contains(&Symbol::integer(3).expr()));
            assert!(factors.items().contains(&Expression::power(
                Symbol::integer(2).expr(),
                Symbol::integer(-1).expr(),
            )));
        } else {
            panic!();
        }
    }

    #[test]
    fn divides_real_by_real() {
        let div = Symbol::real(6.0).expr() / Symbol::real(3.0).expr();

        if let Expression::Real(r) = div {
            assert_eq!(r, Symbol::real(2.0));
        } else {
            panic!();
        }
    }

    #[test]
    fn donot_divide_real_by_integer() {
        let div = Symbol::real(6.0).expr() / Symbol::integer(3).expr();

        if let Expression::Multiplication(factors) = div {
            assert!(factors.items().contains(&Symbol::real(6.0).expr()));
            assert!(factors.items().contains(&Expression::power(
                Symbol::integer(3).expr(),
                Symbol::integer(-1).expr(),
            )));
        } else {
            panic!();
        }
    }

    #[test]
    fn finds_integer_simplification_in_multiplication() {
        let div = Symbol::real(6.0).expr() * Symbol::integer(9).expr() / Symbol::integer(3).expr();

        if let Expression::Multiplication(factors) = div {
            assert!(factors.items().contains(&Symbol::real(6.0).expr()));
            assert!(factors.items().contains(&Symbol::integer(3).expr()));
        } else {
            panic!();
        }
    }

    #[test]
    fn finds_real_simplification_in_multiplication() {
        let div = Symbol::real(6.0).expr() * Symbol::integer(9).expr() / Symbol::real(3.0).expr();

        if let Expression::Multiplication(factors) = div {
            assert!(factors.items().contains(&Symbol::real(2.0).expr()));
            assert!(factors.items().contains(&Symbol::integer(9).expr()));
        } else {
            panic!();
        }
    }

    #[test]
    fn divides_several_numbers() {
        let mut sum = Symbol::real(1024.0).expr();
        for _ in 0..10 {
            sum = sum / Symbol::real(2.0).expr();
        }

        if let Expression::Real(r) = sum {
            assert_eq!(r, Symbol::real(1.0));
        } else {
            panic!();
        }
    }

    #[test]
    fn inverse_of_inverse() {
        let a = &Symbol::variable("a").expr();
        let one = &Symbol::integer(1).expr();

        assert_eq!(one / (one / a), *a);
        assert_eq!(one / (one / (one / a)), one / a);
        assert_eq!(
            one / (one / (one / (one / a))),
            Symbol::variable("a").expr()
        );
    }
}
