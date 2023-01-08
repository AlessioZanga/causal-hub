#[cfg(test)]
mod tests {
    use causal_hub::utils::UnionFind;
    use itertools::Itertools;

    #[test]
    fn new() {
        let u = UnionFind::new(5);

        // Assert all items are disjoint by default.
        (0..u.len()).for_each(|x| assert_eq!(u.find(x), x));
    }

    #[test]
    #[should_panic]
    fn new_should_panic() {
        UnionFind::new(0);
    }

    #[test]
    fn len() {
        let u = UnionFind::new(5);

        assert_eq!(u.len(), 5);
    }

    #[test]
    fn contains() {
        let u = UnionFind::new(5);

        // Assert all items are disjoint by default.
        (0..u.len())
            .tuple_windows()
            .for_each(|(x, y)| assert!(!u.contains(x, y)));
    }

    #[test]
    #[should_panic]
    fn contains_should_panic() {
        let u = UnionFind::new(5);

        u.contains(u.len(), u.len() + 1);
    }

    #[test]
    fn find() {
        let mut u = UnionFind::new(5);

        // Assert all items are disjoint by default.
        (0..u.len()).for_each(|x| assert_eq!(u.find(x), x));

        u.union(0, 1);

        // Assert merged items has left item as root if same rank.
        assert_eq!(u.find(1), 0);
    }

    #[test]
    #[should_panic]
    fn find_should_panic() {
        let u = UnionFind::new(5);

        u.find(u.len() + 1);
    }

    #[test]
    fn find_mut() {
        let mut u = UnionFind::new(5);

        // Assert all items are disjoint by default.
        (0..u.len()).for_each(|x| assert_eq!(u.find_mut(x), x));

        u.union(0, 1);

        // Assert merged items has left item as root if same rank.
        assert_eq!(u.find_mut(1), 0);
    }

    #[test]
    #[should_panic]
    fn find_mut_should_panic() {
        let mut u = UnionFind::new(5);

        u.find_mut(u.len() + 1);
    }

    #[test]
    fn union() {
        let mut u = UnionFind::new(5);

        u.union(0, 1);
        u.union(1, 2);
        u.union(3, 2);
        u.union(4, 0);
        u.union(4, 4);

        assert!(u.contains(4, 0));
    }

    #[test]
    #[should_panic]
    fn union_should_panic() {
        let mut u = UnionFind::new(5);

        u.union(0, u.len() + 1);
    }

    #[test]
    fn extend() {
        let mut u = UnionFind::new(5);

        u.extend(0..5);
    }

    #[test]
    #[should_panic]
    fn extend_should_panic() {
        let mut u = UnionFind::new(5);

        u.extend(0..(u.len() + 1));
    }
}
