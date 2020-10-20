use crate::base::expression::Expression;
use std::collections::BinaryHeap;

/**
 *  Associations between several Expressions where elements satisfy:
 *      - associativity
 *      - commutativity
 */
#[derive(Debug, Clone)]
pub struct CommutativeAssociation {
    items: BinaryHeap<Expression>,
}

impl CommutativeAssociation {
    /**
     * Constructor
     */
    pub fn new(items: Vec<Expression>) -> Self {
        Self {
            items: items.iter().cloned().collect(),
        }
    }

    /**
     * Getter for item list
     */
    pub fn items(&self) -> Vec<Expression> {
        self.items.clone().into_sorted_vec()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for CommutativeAssociation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.items().iter() {
            item.hash(state);
        }
    }
}
