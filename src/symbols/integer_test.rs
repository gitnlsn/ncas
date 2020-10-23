#[cfg(test)]
mod base {
    use num::bigint::BigInt;
    use crate::base::symbol::*;

    #[test]
    fn displays_label() {
        let foo: Symbol<BigInt> = Symbol::integer(1);
        assert_eq!(foo.label(), String::from("1"));
    }

    #[test]
    fn returns_f64() {
        let foo: Symbol<BigInt> = Symbol::integer(1);
        assert_eq!(foo.value(), Some(1));
    }

    #[test]
    fn hashable() {
        use std::collections::HashSet;
        let mut set: HashSet<Symbol<BigInt>> = HashSet::new();
        set.insert(Symbol::integer(1));
        set.insert(Symbol::integer(1));
        set.insert(Symbol::integer(2));

        assert!(set.contains(&Symbol::integer(1)));
        assert!(set.contains(&Symbol::integer(2)));
        assert_eq!(set.len(), 2);
    }
}

#[cfg(test)]
mod evaluable {
    use crate::base::symbol::*;

    #[test]
    fn addition() {
        assert_eq!(Symbol::integer(1) + &Symbol::integer(1), Symbol::integer(2));
        assert_eq!(
            &Symbol::integer(1) + &Symbol::integer(1),
            Symbol::integer(2)
        );
        assert_eq!(&Symbol::integer(1) + Symbol::integer(1), Symbol::integer(2));
        assert_eq!(Symbol::integer(1) + Symbol::integer(1), Symbol::integer(2));
    }

    #[test]
    fn subtraction() {
        assert_eq!(Symbol::integer(1) - &Symbol::integer(1), Symbol::integer(0));
        assert_eq!(
            &Symbol::integer(1) - &Symbol::integer(1),
            Symbol::integer(0)
        );
        assert_eq!(&Symbol::integer(1) - Symbol::integer(1), Symbol::integer(0));
        assert_eq!(Symbol::integer(1) - Symbol::integer(1), Symbol::integer(0));
    }

    #[test]
    fn multiplication() {
        assert_eq!(Symbol::integer(2) * &Symbol::integer(2), Symbol::integer(4));
        assert_eq!(
            &Symbol::integer(2) * &Symbol::integer(2),
            Symbol::integer(4)
        );
        assert_eq!(&Symbol::integer(2) * Symbol::integer(2), Symbol::integer(4));
        assert_eq!(Symbol::integer(2) * Symbol::integer(2), Symbol::integer(4));
    }

    #[test]
    fn division() {
        Symbol::integer(1);
        assert_eq!(Symbol::integer(4) / &Symbol::integer(2), Symbol::integer(2));
        assert_eq!(
            &Symbol::integer(4) / &Symbol::integer(2),
            Symbol::integer(2)
        );
        assert_eq!(&Symbol::integer(4) / Symbol::integer(2), Symbol::integer(2));
        assert_eq!(Symbol::integer(4) / Symbol::integer(2), Symbol::integer(2));
    }
}
