#[cfg(test)]
mod normal_comparable {
    use crate::base::symbol::Symbol;

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let two = &Symbol::integer(2).expr();

        let trial = (a + b).pow(two.clone());
        let expected = a.clone().pow(two.clone()) + two * a * b + b.clone().pow(two.clone());

        assert!(trial.comparable(&expected));
    }
}

#[cfg(test)]
mod normal_equality {
    use crate::base::symbol::Symbol;

    #[test]
    fn sample_1() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let two = &Symbol::integer(2).expr();

        let trial = (a + b).pow(two.clone());
        let expected = a.clone().pow(two.clone()) + two * a * b + b.clone().pow(two.clone());

        assert!(trial.equal(&expected));
    }
}

#[cfg(test)]
mod normal_inequality {
    use crate::base::symbol::Symbol;

    #[test]
    fn compares_integer_result() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let two = &Symbol::integer(2).expr();

        let left_hand_side = (a + b).pow(two.clone()) + Symbol::integer(1).expr();
        let right_hand_side = a.clone().pow(two.clone()) + two * a * b + b.clone().pow(two.clone());

        assert!(left_hand_side.greater(&right_hand_side));
        assert!(right_hand_side.lesser(&left_hand_side));
    }

    #[test]
    fn compares_real_result() {
        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let two = &Symbol::integer(2).expr();

        let left_hand_side = (a + b).pow(two.clone()) + Symbol::real(1.0).expr();
        let right_hand_side = a.clone().pow(two.clone()) + two * a * b + b.clone().pow(two.clone());

        assert!(left_hand_side.greater(&right_hand_side));
        assert!(right_hand_side.lesser(&left_hand_side));
    }
}
