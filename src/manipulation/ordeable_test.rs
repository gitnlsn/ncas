#[cfg(test)]
mod relative_precedence {

    #[test]
    fn between_symbols() {
        use crate::symbols::{constant::Constant, number::Number, variable::Variable};

        /* Number < Constant */
        assert!(Number::new(1.0) < Constant::new(String::from("a"), 1.0));

        /* Constant < Variable */
        assert!(Constant::new(String::from("a"), 1.0) < Variable::new(String::from("a")));

        /* Number < Variable */
        assert!(Number::new(1.0) < Variable::new(String::from("a")));
    }
}

#[cfg(test)]
mod symbol {

    #[test]
    fn numbers_by_value() {
        use crate::symbols::number::Number;
        /* Numbers ordeable by value */
        assert!(Number::new(-0.05) < Number::new(0.0));
        assert!(Number::new(0.05) < Number::new(1.0));
        assert!(Number::new(0.0) < Number::new(1.0));
        assert!(Number::new(1.0) < Number::new(2.0));

        /* interesting */
        assert!(Number::new(2.0) > Number::new(10.0));
    }

    #[test]
    fn constants_by_label() {
        use crate::symbols::constant::Constant;
        /* Constants and Variables ordeable by label */
        assert!(Constant::new(String::from("a"), 1.0) < Constant::new(String::from("b"), 2.0));
        assert!(Constant::new(String::from("a"), 2.0) < Constant::new(String::from("b"), 1.0));
        assert!(Constant::new(String::from("a"), 2.0) < Constant::new(String::from("c"), 1.0));

        /* interesting! */
        assert!(Constant::new(String::from("A"), 2.0) < Constant::new(String::from("a"), 1.0));
        assert!(Constant::new(String::from("Z"), 2.0) < Constant::new(String::from("a"), 1.0));
    }

    #[test]
    fn variable_by_label() {
        use crate::symbols::variable::Variable;
        /* Constants and Variables ordeable by label */
        assert!(Variable::new(String::from("a")) < Variable::new(String::from("b")));
        assert!(Variable::new(String::from("a")) < Variable::new(String::from("b")));
        assert!(Variable::new(String::from("a")) < Variable::new(String::from("c")));
        assert!(Variable::new(String::from("a")) < Variable::new(String::from("d")));

        /* interesting! */
        assert!(Variable::new(String::from("A")) < Variable::new(String::from("a")));
        assert!(Variable::new(String::from("Z")) < Variable::new(String::from("a")));
    }

    #[test]
    fn symbolic_numeric_comparable() {
        use crate::symbols::{number::Number, variable::Variable};
        let a = &Variable::new(String::from("a"));

        /* Number < Constant */
        assert!(a + Number::new(1.0) > a.clone());
        assert!(a - Number::new(1.0) < a.clone()); // Todo: requires normal comparison
    }
}

#[cfg(test)]
mod association {

    #[test]
    fn subtraction() {
        use crate::symbols::variable::Variable;
        /* Orders by left hand side first. Later by right hand side */

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        assert!((a - b) < (b - a));
        assert!((a - b) < (c - b));
        assert!((a - b) < (a - c));
    }

    #[test]
    fn division() {
        use crate::symbols::variable::Variable;
        /* Orders by left hand side first. Later by right hand side */

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        assert!((a / b) < (b / a));
        assert!((a / b) < (c / b));
        assert!((a / b) < (a / c));
    }
}

#[cfg(test)]
mod commutative_association {

    #[test]
    fn addition() {
        use crate::symbols::variable::Variable;
        /* Evaluates by quanity of commutative elements */

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        let d = &Variable::new(String::from("d"));

        /* First evaluation by length */
        assert!((a + b + c) < (a + b + c + d));

        /* Second elements precedes */
        assert!((a + a + a) < (a + b + c));
    }

    #[test]
    fn multiplication() {
        use crate::symbols::variable::Variable;
        /* Evaluates by quanity of commutative elements */

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        let d = &Variable::new(String::from("d"));

        /* First evaluation by length */
        assert!((a * b * c) < (a * b * c * d));

        /* Second elements precedes */
        assert!((a * a * a) < (a * b * c));
    }

    #[test]
    fn addition_ordeable_during_creation() {
        use crate::symbols::number::Number;
        /* Evaluates by quanity of commutative elements */

        let n1 = &Number::new(1.0);
        let n2 = &Number::new(2.0);
        let n3 = &Number::new(3.0);

        /* First evaluation by length */
        assert_eq!((n1 + n2 + n3), (n3 + n2 + n1));
        assert_eq!((n1 + n2 + n3), (n3 + n2 + n1));

        use crate::symbols::variable::Variable;
        /* Evaluates by quanity of commutative elements */

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        /* First evaluation by length */
        assert_eq!(a + c + b, a + b + c);
        assert_eq!(b + c + a, a + b + c);
    }

    #[test]
    fn multiplication_ordeable_during_creation() {
        use crate::symbols::number::Number;
        /* Evaluates by quanity of commutative elements */

        let n1 = &Number::new(1.0);
        let n2 = &Number::new(2.0);
        let n3 = &Number::new(3.0);

        /* First evaluation by length */
        assert_eq!((n1 * n2 * n3), (n3 * n2 * n1));
        assert_eq!((n1 * n2 * n3), (n3 * n2 * n1));

        use crate::symbols::variable::Variable;
        /* Evaluates by quanity of commutative elements */

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));

        /* First evaluation by length */
        assert_eq!(a + c + b, a + b + c);
        assert_eq!(b + c + a, a + b + c);
    }
}

#[cfg(test)]
mod associative_operation {

    #[test]
    fn power() {
        use crate::symbols::variable::Variable;
        /* Evaluates by quanity of commutative elements */

        let a = &Variable::new(String::from("a"));
        let b = &Variable::new(String::from("b"));
        let c = &Variable::new(String::from("c"));
        let d = &Variable::new(String::from("d"));

        assert!(a.pow(&(a + a + a)) < b.pow(&(a + a + a + a)));
        assert!(a.pow(&(a + b + c)) < a.pow(&(a + b + c + d)));
    }
}
