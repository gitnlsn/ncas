#[cfg(test)]
mod constructor {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn multiplies_symbols() {
        let one = &Symbol::real(2.0).expr();
        let three = &Symbol::integer(3).expr();
        let x = &Symbol::variable("x").expr();
        let sum: Expression = one * x * three;

        if let Expression::Multiplication(m) = sum {
            let addends = m.items();
            assert_eq!(addends.len(), 3);
            assert!(addends.contains(one));
            assert!(addends.contains(three));
            assert!(addends.contains(x));
        } else {
            panic!();
        }
    }

    #[test]
    fn multiply_merge_integers() {
        let mut sum: Expression = Symbol::integer(1).expr() * Symbol::integer(3).expr();
        for _ in 0..10 {
            sum = sum * Symbol::integer(2).expr();
        }

        assert_eq!(sum, Symbol::integer(3072).expr());
    }

    #[test]
    fn multiply_merge_reals() {
        let mut sum: Expression = Symbol::real(1.0).expr() * Symbol::real(3.0).expr();
        for _ in 0..10 {
            sum = sum * Symbol::real(2.0).expr();
        }

        assert_eq!(sum, Symbol::real(3072.0).expr());
    }

    #[test]
    fn opposed_of_opposed() {
        assert_eq!(
            Symbol::integer(-1).expr() * Symbol::integer(-1).expr() * Symbol::variable("x").expr(),
            Symbol::variable("x").expr()
        );

        assert_eq!(
            Symbol::integer(-1).expr()
                * Symbol::integer(-1).expr()
                * Symbol::integer(-1).expr()
                * Symbol::variable("x").expr(),
            Symbol::integer(-1).expr() * Symbol::variable("x").expr()
        );
    }
} /* end - constructor tests */
