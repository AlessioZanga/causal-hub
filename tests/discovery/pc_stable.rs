#[cfg(test)]
mod discrete {
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call_skeleton() {
        // Set true graph
        let true_g = PDGraph::new_pagraph(
            ["asia", "xray"],
            [("bronc", "dysp")],
            [
                ("bronc", "smoke"),
                ("lung", "smoke"),
                ("lung", "either"),
                ("tub", "either"),
            ],
        );

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
    fn call() {
        // Set true graph
        let true_g = PDGraph::new_pagraph(
            ["asia", "xray"],
            [("bronc", "dysp")],
            [
                ("bronc", "smoke"),
                ("lung", "smoke"),
                ("lung", "either"),
                ("tub", "either"),
            ],
        );

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
        let mut g = PDGraph::new_pagraph(vec![], vec![("1", "2")], vec![("0", "1")]);
        g.meek_1();
        assert!(g.has_directed_edge_by_index(1, 2));
        assert!(g.has_directed_edge_by_index(0, 1))
    }

    #[test]
    fn meek_1_general_case() {
        let mut g = PDGraph::new_pagraph(
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
        );
        g.meek_1();
        // Test for undirected edges
        assert!(g.has_undirected_edge_by_index(0, 4));
        assert!(g.has_undirected_edge_by_index(4, 1));
        assert!(g.has_undirected_edge_by_index(1, 2));
        // Test for directed edges
        assert!(g.has_directed_edge_by_index(0, 3));
        assert!(g.has_directed_edge_by_index(3, 5));
        assert!(g.has_directed_edge_by_index(5, 4));
    }

    #[test]
    fn meek_2_base_case() {
        let mut g = PDGraph::new_pagraph(vec![], vec![("0", "2")], vec![("0", "1"), ("1", "2")]);
        g.meek_2();
        assert!(g.has_directed_edge_by_index(0, 2));
        assert!(g.has_directed_edge_by_index(0, 1));
        assert!(g.has_directed_edge_by_index(1, 2));
    }

    #[test]
    fn meek_2_general_case() {
        let mut g = PDGraph::new_pagraph(
            vec![],
            vec![("1", "2"), ("1", "3"), ("4", "0")],
            vec![("1", "0"), ("0", "2"), ("4", "2"), ("2", "3")],
        );
        g.meek_2();
        // Test for undirected edges
        assert!(g.has_undirected_edge_by_index(0, 4));
        // Test for directed edges
        assert!(g.has_directed_edge_by_index(1, 2));
        assert!(g.has_directed_edge_by_index(1, 3));
    }

    #[test]
    fn meek_3_base_case() {
        let mut g = PDGraph::new_pagraph(
            vec![],
            vec![("0", "1"), ("0", "2"), ("0", "3")],
            vec![("1", "2"), ("3", "2")],
        );
        g.meek_3();
        // Test for undirected edges
        assert!(g.has_undirected_edge_by_index(0, 1));
        assert!(g.has_undirected_edge_by_index(0, 3));
        // Test for directed edges
        assert!(g.has_directed_edge_by_index(0, 2));
        assert!(g.has_directed_edge_by_index(1, 2));
        assert!(g.has_directed_edge_by_index(3, 2));
    }

    #[test]
    fn meek_3_general_case() {
        let mut g = PDGraph::new_pagraph(
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
        );
        g.meek_3();
        // Test for undirected edges
        assert!(g.has_undirected_edge_by_index(5, 0));
        // Test for directed edges
        assert!(g.has_directed_edge_by_index(1, 0));
        assert!(g.has_directed_edge_by_index(4, 0));
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
            let mut g = PDGraph::new_pagraph(v, ue, de);
            g.meek_4();
            // Test for undirected edges
            assert!(g.has_undirected_edge_by_index(0, 3));
            // Test for directed edges
            assert!(g.has_directed_edge_by_index(0, 1));
            assert!(g.has_directed_edge_by_index(1, 2));
            assert!(g.has_directed_edge_by_index(3, 2));
        }
    }

    #[test]
    fn meek_4_general_case() {
        let mut g = PDGraph::new_pagraph(
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
        );
        g.meek_4();
        // Test for undirected edges
        assert!(g.has_undirected_edge_by_index(5, 0));
        // Test for directed edges
        assert!(g.has_directed_edge_by_index(7, 0));
        assert!(g.has_directed_edge_by_index(3, 0));
    }
}
