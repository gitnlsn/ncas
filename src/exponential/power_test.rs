#[cfg(test)]
mod evaluable {
    use crate::base::{expression::Expression, symbol::Symbol};

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
