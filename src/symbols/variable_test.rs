#[cfg(test)]
mod api {
    use crate::base::symbol::*;

    #[test]
    fn displays_label() {
        let foo: Symbol<String> = Symbol::variable("a");
        assert_eq!(foo.label(), String::from("a"));
    }

    #[test]
    fn returns_none_value() {
        let foo: Symbol<String> = Symbol::variable("a");
        assert_eq!(foo.value(), None);
    }

    #[test]
    fn hashable() {
        use std::collections::HashSet;
        let mut set: HashSet<Symbol<String>> = HashSet::new();

        set.insert(Symbol::variable("a"));
        set.insert(Symbol::variable("a"));
        set.insert(Symbol::variable("b"));

        assert!(set.contains(&Symbol::variable("a")));
        assert!(set.contains(&Symbol::variable("b")));
        assert_eq!(set.len(), 2);
    }
}
