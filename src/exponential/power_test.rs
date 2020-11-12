#[cfg(test)]
mod identities_simplification {
    use crate::base::{expression::Expression, symbol::Symbol};

    /*
        Base identities
    */
    #[test]
    fn zero_integer_base() {
        let zero = &Symbol::integer(0).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(zero.clone().pow(whatever), zero.clone());
    }

    #[test]
    fn zero_real_base() {
        let zero = &Symbol::real(0.0).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(zero.clone().pow(whatever), zero.clone());
    }

    #[test]
    fn one_integer_base() {
        let one = &Symbol::integer(1).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(one.clone().pow(whatever), one.clone());
    }

    #[test]
    fn one_real_base() {
        let one = &Symbol::real(1.0).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(one.clone().pow(whatever), one.clone());
    }

    /*
       Exponent identities
    */
    #[test]
    fn zero_integer_exponent() {
        let zero = &Symbol::integer(0).expr();
        let one = &Symbol::integer(1).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(whatever.clone().pow(zero.clone()), one.clone());
    }

    #[test]
    fn zero_real_exponent() {
        let zero = &Symbol::real(0.0).expr();
        let one = &Symbol::integer(1).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(whatever.clone().pow(zero.clone()), one.clone());
    }

    #[test]
    fn one_integer_exponent() {
        let one = &Symbol::integer(1).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(whatever.clone().pow(one.clone()), whatever.clone());
    }

    #[test]
    fn one_real_exponent() {
        let one = &Symbol::real(1.0).expr();
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        assert_eq!(whatever.clone().pow(one.clone()), whatever.clone());
    }
}

#[cfg(test)]
mod constructor {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn simplifies_identity_power_log() {
        /* Must work constructor */
        let whatever = Symbol::variable("a").expr() + Symbol::variable("b").expr();
        let trial = Expression::power(
            whatever.clone(),
            Expression::logarithm(Symbol::integer(4).expr(), whatever.clone()),
        );

        assert_eq!(trial, Symbol::integer(4).expr());
    }

    #[test]
    fn evaluates_integer_to_real() {
        let two = Symbol::integer(2).expr();
        let three = Symbol::real(3.0).expr();
        let two_to_three = two.clone().pow(three.clone());

        if let Expression::Real(r) = two_to_three {
            assert_eq!(r, Symbol::real(8.0));
        }
    }

    #[test]
    fn evaluates_real_to_real() {
        let two = Symbol::real(2.0).expr();
        let three = Symbol::real(3.0).expr();
        let two_to_three = two.clone().pow(three.clone());

        if let Expression::Real(r) = two_to_three {
            assert_eq!(r, Symbol::real(8.0));
        }
    }

    #[test]
    fn evaluates_real_to_integer() {
        let two = Symbol::real(2.0).expr();
        let three = Symbol::integer(3).expr();
        let two_to_three = two.clone().pow(three.clone());

        if let Expression::Real(r) = two_to_three {
            assert_eq!(r, Symbol::real(8.0));
        }
    }

    #[test]
    fn evaluates_integer_to_integer() {
        let two = Symbol::integer(2).expr();
        let three = Symbol::integer(3).expr();
        let two_to_three = two.clone().pow(three.clone());

        if let Expression::Integer(n) = two_to_three {
            assert_eq!(n, Symbol::integer(8));
        }
    }

    #[test]
    fn addition_to_integer() {
        let arg = Symbol::variable("a").expr() + Symbol::real(2.0).expr();
        let three = Symbol::integer(3).expr();
        let two_to_three = arg.clone().pow(three.clone());

        if let Expression::Power(p) = two_to_three {
            assert_eq!(p.argument(), arg);
            assert_eq!(p.modifier(), three);
        }
    }

    #[test]
    fn multiplication_to_integer() {
        let arg = Symbol::variable("a").expr() * Symbol::integer(2).expr();
        let three = Symbol::real(3.0).expr();
        let two_to_three = arg.clone().pow(three.clone());

        if let Expression::Power(p) = two_to_three {
            assert_eq!(p.argument(), arg);
            assert_eq!(p.modifier(), three);
        }
    }
}
