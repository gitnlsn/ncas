#[cfg(test)]
mod multiplication_common_factors {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn keeps_sign_separated() {
        let a = &Symbol::variable("a").expr();
        assert_eq!(
            (-a).simplify(),
            Expression::multiplication(vec![
                Symbol::integer(-1).expr(),
                Symbol::variable("a").expr()
            ])
        );

        assert_eq!((-(-a)).simplify(), Symbol::variable("a").expr());

        assert_eq!(
            (-(-(-a))).simplify(),
            Expression::multiplication(vec![
                Symbol::integer(-1).expr(),
                Symbol::variable("a").expr()
            ])
        );
    }

    #[test]
    fn simplifies_opposite_factor() {
        let minus_one = &Symbol::integer(-1).expr();
        let a = &Symbol::variable("a").expr();

        assert_eq!(
            (minus_one * minus_one).simplify(),
            Symbol::integer(1).expr()
        );

        assert_eq!(
            (-a * -a).simplify(),
            a.clone().pow(Symbol::integer(2).expr())
        );

        assert_eq!(
            (-a * -a * -a).simplify(),
            -(a.clone().pow(Symbol::integer(3).expr()))
        );

        assert_eq!(
            ((-a).pow(Symbol::integer(2).expr())).simplify(),
            a.clone().pow(Symbol::integer(2).expr())
        );

        assert_eq!(
            (-a.clone().pow(Symbol::integer(3).expr())).simplify(),
            -(a.clone().pow(Symbol::integer(3).expr()))
        );
    }

    #[test]
    fn simplifies_after_distribution() {
        let a = &Symbol::variable("a").expr();

        let test = ((a + a) * (a + a)).simplify();
        let expected = Symbol::integer(4).expr() * (a.clone().pow(Symbol::integer(2).expr()));
        assert_eq!(test, expected);
    }

    #[test]
    fn simplifies_division() {
        let two = &Symbol::integer(2).expr();
        let a = &Symbol::variable("a").expr();

        let test = ((a + a) / (a.clone()).pow(two.clone())).simplify();
        let expected = Symbol::integer(2).expr() / a;
        assert_eq!(test, expected);
    }

    #[test]
    fn simplifies_division_2() {
        let two = &Symbol::integer(2).expr();
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();

        let test = (((a + a) * b) / (a * b).pow(two.clone())).simplify();
        let expected = Symbol::integer(2).expr() / (a * b);
        println!("{}\n{}",test, expected);
        assert_eq!(test, expected);
    }
}

#[cfg(test)]
mod addition_common_addends {
    use crate::base::symbol::Symbol;

    #[test]
    fn separates_sign_at_multiplication() {
        let a = &Symbol::variable("a").expr();
        let one = &Symbol::integer(1).expr();
        let two = &Symbol::integer(2).expr();
        assert_eq!((-a - a).simplify(), -one * two * a);
    }

    #[test]
    fn simplifies_opposite_sum() {
        let a = &Symbol::variable("a").expr();
        assert_eq!((a - a).simplify(), Symbol::integer(0).expr());

        let n1 = &Symbol::integer(1).expr();
        assert_eq!((n1 - n1).simplify(), Symbol::integer(0).expr());
    }
}

#[cfg(test)]
mod power_log {
    use crate::base::{expression::Expression, symbol::Symbol};

    #[test]
    fn simplifies_through_power_log() {
        let a = &Symbol::variable("a").expr();
        let one = &Symbol::integer(1).expr();
        let two = &Symbol::integer(2).expr();
        let four = &Symbol::integer(4).expr();

        let test = Expression::power(a + one, Expression::logarithm(a + a, a + one)).simplify();
        let expected = two * a;
        assert_eq!(test, expected);

        let test = Expression::power(a + one, Expression::logarithm((a + a) * (a + a), a + one))
            .simplify();
        let expected = four * (a.clone().pow(two.clone()));
        assert_eq!(test, expected);

        let test = Expression::power(
            a + one,
            Expression::logarithm(Expression::power((a + a) * (a + a), a + a), a + one),
        )
        .simplify();
        let expected = Expression::power(four * (a.clone().pow(two.clone())), two * a);
        assert_eq!(test, expected);
    }
}

#[cfg(test)]
mod integer_rational {
    use crate::base::symbol::Symbol;

    #[test]
    fn simplifies_integer_rationals() {
        let quarter = &(Symbol::integer(1).expr() / Symbol::integer(4).expr());
        let third = &(Symbol::integer(1).expr() / Symbol::integer(3).expr());
        let seven_twelvth = &(Symbol::integer(7).expr() / Symbol::integer(12).expr());
        let twelvth = &(Symbol::integer(1).expr() / Symbol::integer(12).expr());

        let trial = quarter + third;
        let expected = seven_twelvth.clone();
        assert_eq!(trial.simplify(), expected);

        let trial = third - quarter;
        let expected = twelvth.clone();
        assert_eq!(trial.simplify(), expected);
    }
}
