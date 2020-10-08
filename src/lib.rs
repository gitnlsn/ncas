pub mod base {
    /* General recursive structure */
    pub mod expression;

    /* Minimal value representation - leaf nodes */
    pub mod symbol;

    /* Operation traits on symbols - intermediary nodes */
    pub mod association;
    pub mod associative_operation;
    pub mod commutative_association;
    pub mod operation;

    /*
        Symbols
            - Numbers
            - Constants
            - Variables
    */
}

pub mod symbols {
    pub mod constant;
    mod constant_test;
    pub mod number;
    mod number_test;
    pub mod variable;
    mod variable_test;
}

pub mod manipulation {
    pub mod differentiate;
    pub mod expand;
    mod expand_test;
    pub mod expression_equality; /* TODO (Equality): implement normal form comparison instead of canonical form comparison */
    pub mod identifiable;
    pub mod numeric_evaluation;
    pub mod ordeable;
    mod ordeable_test;
    pub mod pattern_matchable;
    pub mod simplifiable;
    mod simplifiable_test;
    pub mod simplification_rules {
        pub mod rule;
        pub mod identities {
            pub mod additive_common_factor;
            pub mod inverse_power_log;
            pub mod multiplicative_common_factor;
            pub mod pitagorean_identity;
        }
    }
    // pub mod replaceable;
    // pub mod simplifiable;
    // pub mod sortable;
}

pub mod arithmetics {
    pub mod addition;
    mod addition_test;
    pub mod division;
    mod division_test;
    pub mod multiplication;
    mod multiplication_test;
    mod precedence_test;
    pub mod primitives_interface;
    mod primitives_interface_test;
    pub mod subtraction;
    mod subtraction_test;
}

pub mod exponential {
    pub mod logarithm;
    pub mod logarithm_test;
    pub mod power;
    pub mod power_test;
}

pub mod trigonometrics {
    pub mod cossine;
    pub mod sine;
}
