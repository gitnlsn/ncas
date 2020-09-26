#[cfg(test)]
mod all {
    use crate::{
        manipulation::expression_measure::{ExpressionMeasure, Histogram},
        symbols::{constant::Constant, number::Number, variable::Variable},
    };

    #[test]
    fn sample_1() {
        let a = Variable::new(String::from("a"));
        let b = Variable::new(String::from("b"));
        let c = Variable::new(String::from("c"));
        let d = Variable::new(String::from("d"));

        let sum = (a + b) + (c + d);
        let histogram: Histogram = sum.histogram();

        assert_eq!(histogram.expression.quantity, 7);
        assert_eq!(histogram.expression.max_depth, 2);

        assert_eq!(histogram.symbols.variables.quantity, 4);
        assert_eq!(histogram.symbols.variables.max_depth, 2);

        assert_eq!(histogram.arithmetics.addition.quantity, 3);
        assert_eq!(histogram.arithmetics.addition.max_depth, 1);
    }

    #[test]
    fn sample_2() {
        let a = Constant::new(String::from("a"), 1.0);
        let b = Constant::new(String::from("b"), 1.0);
        let c = Constant::new(String::from("c"), 1.0);
        let d = Constant::new(String::from("d"), 1.0);

        let sum = (a * b) * (c * d);
        let histogram: Histogram = sum.histogram();

        assert_eq!(histogram.expression.quantity, 7);
        assert_eq!(histogram.expression.max_depth, 2);

        assert_eq!(histogram.symbols.constant.quantity, 4);
        assert_eq!(histogram.symbols.constant.max_depth, 2);

        assert_eq!(histogram.arithmetics.multiplication.quantity, 3);
        assert_eq!(histogram.arithmetics.multiplication.max_depth, 1);
    }

    #[test]
    fn sample_3() {
        let n1 = Number::new(1.0);
        let n2 = Number::new(1.0);
        let n3 = Variable::new(String::from("n3"));
        let n4 = Number::new(1.0);

        let sum = n1 / n2 / n3 / n4;
        let histogram: Histogram = sum.histogram();

        assert_eq!(histogram.expression.quantity, 7);
        assert_eq!(histogram.expression.max_depth, 3);

        assert_eq!(histogram.symbols.numbers.quantity, 3);
        assert_eq!(histogram.symbols.numbers.max_depth, 3);

        assert_eq!(histogram.symbols.variables.quantity, 1);
        assert_eq!(histogram.symbols.variables.max_depth, 2);

        assert_eq!(histogram.arithmetics.division.quantity, 3);
        assert_eq!(histogram.arithmetics.division.max_depth, 2);
    }
}
