#[cfg(test)]
mod multiplication_common_factors {
    use crate::arithmetics::multiplication::Multiplication;
    use crate::manipulation::simplifiable::Simplifiable;
    use crate::symbols::{number::Number, variable::Variable};

    #[test]
    fn kepts_sign_separated() {
        let a = &Variable::new(String::from("a"));
        assert_eq!(
            (-a).simplify(),
            Multiplication::new(vec![Number::new(-1.0), Variable::new(String::from("a"))])
        );
        assert_eq!((-(-a)).simplify(), Variable::new(String::from("a")));
        assert_eq!(
            (-(-(-a))).simplify(),
            Multiplication::new(vec![Number::new(-1.0), Variable::new(String::from("a"))])
        );
    }

    #[test]
    fn simplifies_opposite_factor() {
        let minus_one = &Number::new(-1.0);
        assert_eq!((minus_one * minus_one).simplify(), Number::new(1.0));

        let a = &Variable::new(String::from("a"));
        assert_eq!((-a * -a).simplify(), a ^ 2);
        assert_eq!((-a * -a * -a).simplify(), -(a ^ 3));

        assert_eq!((-a ^ 2).simplify(), a ^ 2);
        assert_eq!((-a ^ 3).simplify(), -(a ^ 3));
    }

    #[test]
    fn simplifies_after_distribution() {
        let a = &Variable::new(String::from("a"));

        let test = ((a + a) * (a + a)).simplify();
        let expected = 4 * (a ^ 2);
        assert_eq!(test, expected);
    }
}

#[cfg(test)]
mod addition_common_addends {
    use crate::manipulation::simplifiable::Simplifiable;
    use crate::symbols::{number::Number, variable::Variable};

    #[test]
    fn accumulates_numbers_in_addition() {
        /*
            case: 1 + 1 = 2
                - merges numbers inside Addition CommutativeAssociation
        */
        assert_eq!(
            (Number::new(1.0) + Number::new(1.0)).simplify(),
            Number::new(2.0)
        );
    }

    #[test]
    fn separates_sign_at_multiplication() {
        let a = &Variable::new(String::from("a"));
        assert_eq!((-a - a).simplify(), -1 * 2 * a);
    }

    #[test]
    fn simplifies_opposite_sum() {
        let a = &Variable::new(String::from("a"));
        assert_eq!((a - a).simplify(), Number::new(0.0));

        let n1 = &Number::new(1.0);
        assert_eq!((n1 - n1).simplify(), Number::new(0.0));
    }
}

#[cfg(test)]
mod power_log {
    use crate::exponential::{logarithm::Log, power::Power};
    use crate::manipulation::simplifiable::Simplifiable;
    use crate::symbols::variable::Variable;

    #[test]
    fn simplifies_through_power_log() {
        let a = &Variable::new(String::from("a"));

        let test = Power::new(a + 1, Log::new(a + a, a + 1)).simplify();
        let expected = 2 * a;
        assert_eq!(test, expected);

        let test = Power::new(a + 1, Log::new((a + a) * (a + a), a + 1)).simplify();
        let expected = 4 * (a ^ 2);
        assert_eq!(test, expected);

        let test =
            Power::new(a + 1, Log::new(Power::new((a + a) * (a + a), a + a), a + 1)).simplify();
        let expected = Power::new(4 * (a ^ 2), 2 * a);
        assert_eq!(test, expected);
    }
}
