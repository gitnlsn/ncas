#[cfg(test)]
mod hash {
    use crate::base::expression::Expression;
    use crate::base::symbol::Symbol;
    use std::collections::HashSet;

    #[test]
    fn real_sample1() {
        let mut expression_set: HashSet<Expression> = HashSet::new();
        let testing_expression = Symbol::real(1.0).expr();

        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());

        assert_eq!(expression_set.len(), 1);
    }

    #[test]
    fn real_sample2() {
        let mut expression_set: HashSet<Expression> = HashSet::new();
        let testing_expression =
            Expression::power(Symbol::real(1.0).expr(), Symbol::variable("a").expr());

        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());

        assert_eq!(expression_set.len(), 1);
    }

    #[test]
    fn real_sample3() {
        let mut expression_set: HashSet<Expression> = HashSet::new();

        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let three = &Symbol::real(3.0).expr();

        let testing_expression = a * b.clone().pow(c.clone()) * c.clone().pow(three.clone());

        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());

        assert_eq!(expression_set.len(), 1);
    }


    #[test]
    fn integer_sample1() {
        let mut expression_set: HashSet<Expression> = HashSet::new();
        let testing_expression = Symbol::integer(1).expr();

        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());

        assert_eq!(expression_set.len(), 1);
    }


    #[test]
    fn integer_sample3() {
        let mut expression_set: HashSet<Expression> = HashSet::new();

        let a = &Symbol::variable("a").expr();
        let b = &Symbol::variable("b").expr();
        let c = &Symbol::variable("c").expr();
        let three = &Symbol::integer(1).expr();

        let testing_expression = a * b.clone().pow(c.clone()) * c.clone().pow(three.clone());

        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());
        expression_set.insert(testing_expression.clone());

        assert_eq!(expression_set.len(), 1);
    }
}
