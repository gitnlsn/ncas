#[cfg(test)]
mod base {
    use crate::base::symbol::*;

    #[test]
    fn displays_label() {
        let foo: Symbol<f64> = Symbol::real(1.0);
        assert_eq!(foo.label(), String::from("1"));

        let foo: Symbol<f64> = Symbol::real(1.01);
        assert_eq!(foo.label(), String::from("1.01"));
    }

    #[test]
    fn returns_f64() {
        let foo: Symbol<f64> = Symbol::real(1.0);
        assert_eq!(foo.value(), Some(1.0));
    }

    #[test]
    fn hashable() {
        use std::collections::HashSet;
        let mut set: HashSet<Symbol<f64>> = HashSet::new();
        set.insert(Symbol::real(1.0));
        set.insert(Symbol::real(1.0));
        set.insert(Symbol::real(2.0));

        assert!(set.contains(&Symbol::real(1.0)));
        assert!(set.contains(&Symbol::real(2.0)));
        assert_eq!(set.len(), 2);
    }
}

#[cfg(test)]
mod evaluable {
    use crate::base::symbol::*;

    #[test]
    fn addition() {
        assert_eq!(Symbol::real(1.0) + &Symbol::real(1.0), Symbol::real(2.0));
        assert_eq!(&Symbol::real(1.0) + &Symbol::real(1.0), Symbol::real(2.0));
        assert_eq!(&Symbol::real(1.0) + Symbol::real(1.0), Symbol::real(2.0));
        assert_eq!(Symbol::real(1.0) + Symbol::real(1.0), Symbol::real(2.0));
    }

    #[test]
    fn subtraction() {
        assert_eq!(Symbol::real(1.0) - &Symbol::real(1.0), Symbol::real(0.0));
        assert_eq!(&Symbol::real(1.0) - &Symbol::real(1.0), Symbol::real(0.0));
        assert_eq!(&Symbol::real(1.0) - Symbol::real(1.0), Symbol::real(0.0));
        assert_eq!(Symbol::real(1.0) - Symbol::real(1.0), Symbol::real(0.0));
    }

    #[test]
    fn multiplication() {
        assert_eq!(Symbol::real(2.0) * &Symbol::real(2.0), Symbol::real(4.0));
        assert_eq!(&Symbol::real(2.0) * &Symbol::real(2.0), Symbol::real(4.0));
        assert_eq!(&Symbol::real(2.0) * Symbol::real(2.0), Symbol::real(4.0));
        assert_eq!(Symbol::real(2.0) * Symbol::real(2.0), Symbol::real(4.0));
    }

    #[test]
    fn division() {
        Symbol::real(1.0);
        assert_eq!(Symbol::real(4.0) / &Symbol::real(2.0), Symbol::real(2.0));
        assert_eq!(&Symbol::real(4.0) / &Symbol::real(2.0), Symbol::real(2.0));
        assert_eq!(&Symbol::real(4.0) / Symbol::real(2.0), Symbol::real(2.0));
        assert_eq!(Symbol::real(4.0) / Symbol::real(2.0), Symbol::real(2.0));
    }
}
