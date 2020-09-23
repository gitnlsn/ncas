#[cfg(test)]
mod as_expression {
    use crate::{
        arithmetics::primitives_interface::AsExpression,
        base::symbols::{number::Number, variable::Variable},
    };

    #[test]
    fn sample_1() {
        assert_eq!(1.as_expression(), Number::new(1.0));
        assert_eq!((-1).as_expression(), Number::new(-1.0));
        assert_eq!(-1.as_expression(), Number::new(-1.0) * Number::new(1.0));
        assert_eq!(1.01.as_expression(), Number::new(1.01));
        assert_eq!("a".as_expression(), Variable::new(String::from("a")));
        assert_eq!(
            String::from("a").as_expression(),
            Variable::new(String::from("a"))
        );
    }
}

#[cfg(test)]
mod negation {
    use crate::{
        base::symbols::{constant::Constant, number::Number},
        manipulation::numeric_evaluation::NumericEvaluable,
    };

    #[test]
    fn negate_number_sign() {
        let one = Number::new(1.0);
        let minus_one = -one;
        assert!(minus_one.into_num().is_ok());
        assert_eq!(minus_one.into_num().unwrap(), -1.0);
    }

    #[test]
    fn negate_constant() {
        let one = Constant::new(String::from("one"), 1.0);
        let minus_one = -one;

        assert!(minus_one.into_num().is_ok());
        assert_eq!(minus_one.into_num().unwrap(), -1.0);
    }

    #[test]
    fn formats_with_minus_one_multiply() {
        let one = Number::new(1.0);
        let minus_one = -one;
        assert_eq!(format!("{}", minus_one), String::from("-1 * 1"));
    }
}

#[cfg(test)]
mod arithmetics {
    use crate::base::symbols::{number::Number, variable::Variable};

    #[test]
    fn addition() {
        /* numeric */
        assert_eq!(1 + Number::new(1.0), Number::new(1.0) + Number::new(1.0));
        assert_eq!(1.0 + Number::new(1.0), Number::new(1.0) + Number::new(1.0));
        assert_eq!(Number::new(1.0) + 1, Number::new(1.0) + Number::new(1.0));
        assert_eq!(Number::new(1.0) + 1.0, Number::new(1.0) + Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" + Number::new(1.0),
            Variable::new(String::from("a")) + Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) + "a",
            Number::new(1.0) + Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") + Number::new(1.0),
            Variable::new(String::from("a")) + Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) + String::from("a"),
            Number::new(1.0) + Variable::new(String::from("a"))
        );
    }

    #[test]
    fn subtraction() {
        /* numeric */
        assert_eq!(1 - Number::new(1.0), Number::new(1.0) - Number::new(1.0));
        assert_eq!(1.0 - Number::new(1.0), Number::new(1.0) - Number::new(1.0));
        assert_eq!(Number::new(1.0) - 1, Number::new(1.0) - Number::new(1.0));
        assert_eq!(Number::new(1.0) - 1.0, Number::new(1.0) - Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" - Number::new(1.0),
            Variable::new(String::from("a")) - Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) - "a",
            Number::new(1.0) - Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") - Number::new(1.0),
            Variable::new(String::from("a")) - Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) - String::from("a"),
            Number::new(1.0) - Variable::new(String::from("a"))
        );
    }

    #[test]
    fn multiplication() {
        /* numeric */
        assert_eq!(1 * Number::new(1.0), Number::new(1.0) * Number::new(1.0));
        assert_eq!(1.0 * Number::new(1.0), Number::new(1.0) * Number::new(1.0));
        assert_eq!(Number::new(1.0) * 1, Number::new(1.0) * Number::new(1.0));
        assert_eq!(Number::new(1.0) * 1.0, Number::new(1.0) * Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" * Number::new(1.0),
            Variable::new(String::from("a")) * Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) * "a",
            Number::new(1.0) * Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") * Number::new(1.0),
            Variable::new(String::from("a")) * Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) * String::from("a"),
            Number::new(1.0) * Variable::new(String::from("a"))
        );
    }

    #[test]
    fn division() {
        /* numeric */
        assert_eq!(1 / Number::new(1.0), Number::new(1.0) / Number::new(1.0));
        assert_eq!(1.0 / Number::new(1.0), Number::new(1.0) / Number::new(1.0));
        assert_eq!(Number::new(1.0) / 1, Number::new(1.0) / Number::new(1.0));
        assert_eq!(Number::new(1.0) / 1.0, Number::new(1.0) / Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" / Number::new(1.0),
            Variable::new(String::from("a")) / Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) / "a",
            Number::new(1.0) / Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") / Number::new(1.0),
            Variable::new(String::from("a")) / Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) / String::from("a"),
            Number::new(1.0) / Variable::new(String::from("a"))
        );
    }
}
