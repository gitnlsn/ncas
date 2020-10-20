#[cfg(test)]
mod constructor {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn subtracts_symbols() {
        let one = &Symbol::real(1.0).expr();
        let three = &Symbol::integer(3).expr();
        let x = &Symbol::variable("x").expr();
        let sum: Expression = one - x - three;

        if let Expression::Addition(a) = sum {
            let addends = a.items();
            assert_eq!(addends.len(), 3);
            assert!(addends.contains(one));
            assert!(addends.contains(&-three));
            assert!(addends.contains(&-x));
        } else {
            panic!();
        }
    }

    #[test]
    fn subtraction_merges_integers() {
        let mut sum: Expression = Symbol::integer(0).expr() - Symbol::integer(3).expr();
        for _ in 0..10 {
            sum = sum - Symbol::integer(1).expr();
        }

        assert_eq!(sum, Symbol::integer(-13).expr());
    }

    #[test]
    fn subtraction_wont_merges_reals() {
        let mut sum: Expression = Symbol::real(0.0).expr() - Symbol::real(0.3).expr();
        for _ in 0..10 {
            sum = sum - Symbol::real(0.1).expr();
        }

        if let Expression::Addition(addition) = sum {
            /* subtraction will include addition of negated addents */
            assert_eq!(addition.items().len(), 11);
            assert!(addition.items().contains(&-Symbol::real(0.3).expr()));
            assert!(addition.items().contains(&-Symbol::real(0.1).expr()));
        } else {
            panic!();
        }
    }
}
