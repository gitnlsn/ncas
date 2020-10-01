// #[cfg(test)]
// mod identity {
//     use crate::arithmetics::{addition::Addition, multiplication::Multiplication};
//     use crate::manipulation::simplifiable::Simplifiable;
//     use crate::symbols::{number::Number, variable::Variable};

//     #[test]
//     fn sample_1() {
//         let a = &Variable::new(String::from("a"));
//         assert_eq!(
//             (-a).simplify(),
//             Multiplication::new(vec![Number::new(-1.0), Variable::new(String::from("a")),])
//         );
//     }

//     #[test]
//     fn simplifies_opposite_factor() {
//         let minus_one = &Number::new(-1.0);
//         assert_eq!((minus_one * minus_one).simplify(), Number::new(1.0));

//         let a = &Variable::new(String::from("a"));
//         assert_eq!((-a * -a).simplify(), a ^ 2);
//     }

//     #[test]
//     fn accumulates_numbers_in_addition() {
//         /*
//             case: 1 + 1 = 2
//                 - merges numbers inside Addition CommutativeAssociation
//         */
//         assert_eq!(
//             (Number::new(1.0) + Number::new(1.0)).simplify(),
//             Number::new(2.0)
//         );
//     }

//     // #[test]
//     // fn separates_sign_at_multiplication() {
//     //     let a = &Variable::new(String::from("a"));
//     //     assert_eq!((-a - a).simplify(), -1 * 2 * a);
//     // }

//     #[test]
//     fn simplifies_opposite_sum() {
//         let a = &Variable::new(String::from("a"));
//         assert_eq!((a - a).simplify(), Number::new(0.0));

//         let n1 = &Number::new(1.0);
//         assert_eq!((n1 - n1).simplify(), Number::new(0.0));
//     }
// }
