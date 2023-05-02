#[cfg(test)]
mod discrete {
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call_skeleton() {
        // Set true graph
        let true_g = PDGraph::new_partial(
            ["asia", "xray"],
            [("bronc", "dysp")],
            [
                ("bronc", "smoke"),
                ("lung", "smoke"),
                ("lung", "either"),
                ("tub", "either"),
            ],
        )
        .unwrap();

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Perform skeleton discovery
        let skel = pcs.call_skeleton();

        // Perform test
        assert_eq!(skel, true_g.to_undirected());
    }

    #[test]
    fn call_orient_vstructures() {
        // Set true graph
        let true_g = PDGraph::new_partial(
            ["asia", "xray"],
            [("bronc", "dysp")],
            [
                ("bronc", "smoke"),
                ("lung", "smoke"),
                ("lung", "either"),
                ("tub", "either"),
            ],
        )
        .unwrap();

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Perform discovery
        let g = pcs.call();

        // Perform test
        assert_eq!(g, true_g);
    }
    #[test]
    fn meek_1_base_case() {
        let mut g = PDGraph::new_partial(vec![], vec![("1", "2")], vec![("0", "1")]).unwrap();
        g.meek_1();
        assert!(g.type_of_edge(1, 2) == Some('d'));
        assert!(g.type_of_edge(0, 1) == Some('d'))
    }

    #[test]
    fn meek_1_general_case() {
        let mut g = PDGraph::new_partial(
            vec![],
            vec![
                ("1", "2"),
                ("0", "3"),
                ("0", "4"),
                ("1", "4"),
                ("4", "5"),
                ("3", "5"),
            ],
            vec![("1", "0")],
        )
        .unwrap();
        g.meek_1();
        // Test for undirected edges
        assert!(g.type_of_edge(0, 4) == Some('u'));
        assert!(g.type_of_edge(4, 1) == Some('u'));
        assert!(g.type_of_edge(1, 2) == Some('u'));
        // Test for directed edges
        assert!(g.type_of_edge(0, 3) == Some('d'));
        assert!(g.type_of_edge(3, 5) == Some('d'));
        assert!(g.type_of_edge(5, 4) == Some('d'));
    }

    #[test]
    fn meek_2_base_case() {
        let mut g =
            PDGraph::new_partial(vec![], vec![("0", "2")], vec![("0", "1"), ("1", "2")]).unwrap();
        g.meek_2();
        assert!(g.type_of_edge(0, 2) == Some('d'));
        assert!(g.type_of_edge(0, 1) == Some('d'));
        assert!(g.type_of_edge(1, 2) == Some('d'));
    }

    #[test]
    fn meek_2_general_case() {
        let mut g = PDGraph::new_partial(
            vec![],
            vec![("1", "2"), ("1", "3"), ("4", "0")],
            vec![("1", "0"), ("0", "2"), ("4", "2"), ("2", "3")],
        )
        .unwrap();
        g.meek_2();
        // Test for undirected edges
        assert!(g.type_of_edge(0, 4) == Some('u'));
        // Test for directed edges
        assert!(g.type_of_edge(1, 2) == Some('d'));
        assert!(g.type_of_edge(1, 3) == Some('d'));
    }

    #[test]
    fn meek_3_base_case() {
        let mut g = PDGraph::new_partial(
            vec![],
            vec![("0", "1"), ("0", "2"), ("0", "3")],
            vec![("1", "2"), ("3", "2")],
        )
        .unwrap();
        g.meek_3();
        // Test for undirected edges
        assert!(g.type_of_edge(0, 1) == Some('u'));
        assert!(g.type_of_edge(0, 3) == Some('u'));
        // Test for directed edges
        assert!(g.type_of_edge(0, 2) == Some('d'));
        assert!(g.type_of_edge(1, 2) == Some('d'));
        assert!(g.type_of_edge(3, 2) == Some('d'));
    }

    #[test]
    fn meek_3_general_case() {
        let mut g = PDGraph::new_partial(
            vec![],
            vec![
                ("0", "1"),
                ("0", "4"),
                ("0", "5"),
                ("6", "5"),
                ("6", "2"),
                ("2", "5"),
                ("3", "1"),
                ("2", "1"),
                ("4", "1"),
                ("6", "4"),
            ],
            vec![("2", "0"), ("3", "0"), ("6", "0")],
        )
        .unwrap();
        g.meek_3();
        // Test for undirected edges
        assert!(g.type_of_edge(5, 0) == Some('u'));
        // Test for directed edges
        assert!(g.type_of_edge(1, 0) == Some('d'));
        assert!(g.type_of_edge(4, 0) == Some('d'));
    }

    #[test]
    fn meek_4_base_case() {
        let data = [
            (
                vec![],
                vec![("0", "3"), ("1", "3"), ("2", "3")],
                vec![("0", "1"), ("1", "2")],
            ),
            (
                vec![],
                vec![("0", "3"), ("2", "3")],
                vec![("0", "1"), ("1", "2")],
            ),
        ];
        for (v, ue, de) in data {
            let mut g = PDGraph::new_partial(v, ue, de).unwrap();
            g.meek_4();
            // Test for undirected edges
            assert!(g.type_of_edge(0, 3) == Some('u'));
            // Test for directed edges
            assert!(g.type_of_edge(0, 1) == Some('d'));
            assert!(g.type_of_edge(1, 2) == Some('d'));
            assert!(g.type_of_edge(3, 2) == Some('d'));
        }
    }

    #[test]
    fn meek_4_general_case() {
        let mut g = PDGraph::new_partial(
            vec![],
            vec![
                ("0", "5"),
                ("0", "2"),
                ("2", "5"),
                ("0", "7"),
                ("0", "3"),
                ("6", "7"),
                ("3", "4"),
            ],
            vec![("1", "0"), ("2", "1"), ("4", "1"), ("3", "7"), ("6", "3")],
        )
        .unwrap();
        g.meek_4();
        // Test for undirected edges
        assert!(g.type_of_edge(5, 0) == Some('u'));
        // Test for directed edges
        assert!(g.type_of_edge(7, 0) == Some('d'));
        assert!(g.type_of_edge(3, 0) == Some('d'));
    }
}
