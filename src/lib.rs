pub mod base {
    pub mod expression;

    /*
        Symbols
            - Numbers
            - Constants
            - Variables
    */
    pub mod symbols {
        pub mod constant;
        mod constant_test;
        pub mod number;
        mod number_test;
        pub mod variable;
        mod variable_test;
    }
}

pub mod manipulation {
    pub mod differentiate;
    pub mod expand;
    pub mod expand_test;
    pub mod expression_measure;
    pub mod expression_measure_test;
    pub mod numeric_evaluation;
    pub mod identifiable;
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
    pub mod primitives_interface_test;
    pub mod subtraction;
    mod subtraction_test;
}

pub mod exponential {
    pub mod power;
}
