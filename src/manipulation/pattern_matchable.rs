use crate::base::expression::Expression;
use crate::symbols::variable::Variable;

pub struct Match {
    /* test expression: variable marks pattern matching ocurence */
    pub expression: Expression,

    /*
        Variable is build as Expression,
        but it should match Indentity::Variable to facilitate latter substitution
    */
    pub variable: Expression,
}

pub trait PatternMatchable {
    /**
     *  Implementation should return the equivalent expression
     *  with a variable placed where pattern match happens
     */
    fn matches(&self, pattern: &Expression, variable: &String) -> Option<Match>;
}


// =================================== //
//      Recursion on Expression        //
// =================================== //
impl PatternMatchable for Expression {
    fn matches(&self, pattern: &Expression, label: &String) -> Option<Match> {
        if self == pattern {
            return Some(Match {
                expression: self.clone(),
                variable: Variable::new(String::from("P")),
            });
        }

        match self {
            Expression::Symbol(_) => return None, /* not meant to occur */
            Expression::Operation(op) => return op.matches(pattern, label),
            Expression::Association(a) => return a.matches(pattern, label),
            Expression::AssociativeOperation(op) => return op.matches(pattern, label),
            Expression::CommutativeAssociation(a) => return a.matches(pattern, label),
        }
    }
}

use crate::base::commutative_association::CommutativeAssociation;
use crate::manipulation::identifiable::{Identifiable, Identity};


// ====================== //
//       Arithmetics      //
// ====================== //
use crate::arithmetics::addition::Addition;
impl PatternMatchable for Addition {
    fn matches(&self, pattern: &Expression, label: &String) -> Option<Match> {
        if pattern.id() == Identity::Addition {
            if let Expression::CommutativeAssociation(additive_pattern) = pattern {
                let mut pattern_addends = additive_pattern.items();
                let mut match_addends: Vec<Expression> = Vec::new();

                for self_addend in self.items().iter() {
                    if let Some(index) = pattern_addends.iter().position(|item| item == self_addend)
                    {
                        pattern_addends.remove(index);
                    } else if let Some(matching) = self_addend.matches(pattern, label) {
                        match_addends.push(matching.expression);
                    } else {
                        match_addends.push(self_addend.clone());
                    }
                }

                if pattern_addends.is_empty() {
                    match_addends.push(Variable::new(label.clone()));
                    return Some(Match {
                        expression: Addition::new(match_addends),
                        variable: Variable::new(label.clone()),
                    });
                }
            }
        }

        let mut found_match: bool = false;
        let mut addends: Vec<Expression> = Vec::new();

        for addend in self.items().iter() {
            if let Some(matching) = addend.matches(pattern, label) {
                found_match = true;
                addends.push(matching.expression);
            } else {
                addends.push(addend.clone());
            }
        }

        if found_match {
            return Some(Match {
                expression: Addition::new(addends),
                variable: Variable::new(label.clone()),
            });
        }

        return None;
    }
}

use crate::arithmetics::multiplication::Multiplication;
impl PatternMatchable for Multiplication {
    fn matches(&self, pattern: &Expression, label: &String) -> Option<Match> {
        if pattern.id() == Identity::Multiplication {
            if let Expression::CommutativeAssociation(multiplicative_patern) = pattern {
                let mut pattern_factors = multiplicative_patern.items();
                let mut match_factors: Vec<Expression> = Vec::new();

                for self_factor in self.items().iter() {
                    if let Some(index) = pattern_factors.iter().position(|item| item == self_factor)
                    {
                        pattern_factors.remove(index);
                    } else if let Some(matching) = self_factor.matches(pattern, label) {
                        match_factors.push(matching.expression);
                    } else {
                        match_factors.push(self_factor.clone());
                    }
                }

                if pattern_factors.is_empty() {
                    match_factors.push(Variable::new(label.clone()));
                    return Some(Match {
                        expression: Addition::new(match_factors),
                        variable: Variable::new(label.clone()),
                    });
                }
            }
        }

        let mut found_match: bool = false;
        let mut factors: Vec<Expression> = Vec::new();

        for factor in self.items().iter() {
            if let Some(matching) = factor.matches(pattern, label) {
                found_match = true;
                factors.push(matching.expression);
            } else {
                factors.push(factor.clone());
            }
        }

        if found_match {
            return Some(Match {
                expression: Addition::new(factors),
                variable: Variable::new(label.clone()),
            });
        }

        return None;
    }
}


// ====================== //
//       Exponential      //
// ====================== //
use crate::base::associative_operation::AssociativeOperation;
use crate::exponential::power::Power;
impl PatternMatchable for Power {
    fn matches(&self, pattern: &Expression, label: &String) -> Option<Match> {
        let mut found_match: bool = false;
        let base: Expression = match self.argument().matches(pattern, label) {
            Some(_) => {
                found_match = true;
                Variable::new(label.clone())
            }
            None => self.argument().as_ref().clone(),
        };

        let exponent: Expression = match self.modifier().matches(pattern, label) {
            Some(_) => {
                found_match = true;
                Variable::new(label.clone())
            }
            None => self.modifier().as_ref().clone(),
        };

        if found_match {
            return Some(Match {
                expression: Power::new(base, exponent),
                variable: Variable::new(label.clone()),
            });
        }

        return None;
    }
}
use crate::exponential::logarithm::Log;
impl PatternMatchable for Log {
    fn matches(&self, pattern: &Expression, label: &String) -> Option<Match> {
        let mut found_match: bool = false;
        let argument: Expression = match self.argument().matches(pattern, label) {
            Some(_) => {
                found_match = true;
                Variable::new(label.clone())
            }
            None => self.argument().as_ref().clone(),
        };

        let base: Expression = match self.modifier().matches(pattern, label) {
            Some(_) => {
                found_match = true;
                Variable::new(label.clone())
            }
            None => self.modifier().as_ref().clone(),
        };

        if found_match {
            return Some(Match {
                expression: Log::new(argument, base),
                variable: Variable::new(label.clone()),
            });
        }

        return None;
    }
}


// ============================== //
//         Trigonometrics         //
// ============================== //
use crate::base::operation::Operation;

use crate::trigonometrics::sine::Sin;
impl PatternMatchable for Sin {
    fn matches(&self, pattern: &Expression, label: &String) -> Option<Match> {
        let mut found_match: bool = false;
        let argument: Expression = match self.argument().matches(pattern, label) {
            Some(_) => {
                found_match = true;
                Variable::new(label.clone())
            }
            None => self.argument().as_ref().clone(),
        };

        if found_match {
            return Some(Match {
                expression: Sin::new(argument),
                variable: Variable::new(label.clone()),
            });
        }

        return None;
    }
}

use crate::trigonometrics::cossine::Cos;
impl PatternMatchable for Cos {
    fn matches(&self, pattern: &Expression, label: &String) -> Option<Match> {
        let mut found_match: bool = false;
        let argument: Expression = match self.argument().matches(pattern, label) {
            Some(_) => {
                found_match = true;
                Variable::new(label.clone())
            }
            None => self.argument().as_ref().clone(),
        };

        if found_match {
            return Some(Match {
                expression: Cos::new(argument),
                variable: Variable::new(label.clone()),
            });
        }

        return None;
    }
}
