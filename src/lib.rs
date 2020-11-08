pub mod base {
    /* General recursive structure */
    pub mod expression;
    mod expression_test;

    /* Minimal value representation - leaf nodes */
    pub mod symbol;

    /* Operation traits on symbols - intermediary nodes */
    // pub mod association;
    pub mod commutative_association;
    pub mod associative_operation;
    pub mod operation;
}

pub mod symbols {
    pub mod integer;
    mod integer_test;
    pub mod real;
    mod real_test;
    pub mod variable;
    mod variable_test;
}

pub mod manipulation {
//     pub mod differentiate;
    pub mod expand;
    mod expand_test;
    pub mod expression_equality;
    pub mod expression_comparison;
    pub mod expression_comparison_test;
//     pub mod numeric_evaluation;
    pub mod ordeable;
//     mod ordeable_test;
//     pub mod pattern_matchable;
    pub mod simplifiable;
    mod simplifiable_test;
    pub mod simplification_rules {
        pub mod rule;
        pub mod identities {
            pub mod inverse_power_log;
            // pub mod pitagorean_identity;
        }
        pub mod factoring {
            // pub mod factorable;
            pub mod additive_common_addend;
            // pub mod additive_common_factor;
            pub mod multiplicative_common_factor;
            pub mod single_algebraic_denominator;
        }
        pub mod rational {
            pub mod rationals;
            pub mod rationals_addition;
            pub mod rationals_multiplication;
        }
    }
    pub mod expansion_rules {
        pub mod rule;
        pub mod multiplicative_distributive;
        mod multiplicative_distributive_test;
        pub mod power_distributive;
        mod power_distributive_test;
    }
//     // pub mod replaceable;
}

pub mod arithmetics {
    pub mod addition;
    mod addition_test;
    pub mod division;
    mod division_test;
    pub mod multiplication;
    mod multiplication_test;
//     pub mod primitives_interface;
//     mod primitives_interface_test;
    pub mod subtraction;
    mod subtraction_test;
}

pub mod exponential {
    pub mod logarithm;
    pub mod logarithm_test;
    pub mod power;
    pub mod power_test;
}

// pub mod trigonometrics {
//     pub mod cossine;
//     pub mod sine;
// }
