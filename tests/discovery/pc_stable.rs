#[cfg(test)]
mod discrete {
    use causal_hub::prelude::*;
    use ndarray::array;
    use polars::prelude::*;

    // Set ChiSquared significance level
    const ALPHA: f64 = 0.05;

    // Set base path
    const BASE_PATH: &str = "./tests/assets/PC-Stable/";

    #[test]
    fn cancer() {
        // Set dataset name
        let db_name: String = "cancer".into();

        // Set true graph
        let true_g = PDGraph::from((
            vec!["Cancer", "Dyspnoea", "Pollution", "Smoker", "Xray"],
            array![
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false]
            ],
            array![
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [true, false, false, false, false],
                [true, false, false, false, false]
            ],
        ));

        // Set true skeleton
        let true_skel = true_g.clone().to_undirected();

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, db_name))
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Perform skeleton discovery
        let skel = pcs.call_skeleton();
        let par_skel = pcs.par_call_skeleton();

        // Perform discovery
        let g = pcs.call().meek_procedure_until_3();
        let par_g = pcs.par_call().meek_procedure_until_3();

        // Perform tests
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn asia() {
        // Set dataset name
        let db_name: String = "asia".into();

        // Set true graph
        let true_g = PDGraph::from((
            vec![
                "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray",
            ],
            array![
                [false, false, false, false, false, false, false, false],
                [false, false, true, false, false, true, false, false],
                [false, true, false, false, false, false, false, false],
                [false, false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false]
            ],
            array![
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false]
            ],
        ));

        // Set true skeleton
        let true_skel = true_g.clone().to_undirected();

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, db_name))
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Perform skeleton discovery
        let skel = pcs.call_skeleton();
        let par_skel = pcs.par_call_skeleton();

        // Perform discovery
        let g = pcs.call().meek_procedure_until_3();
        let par_g = pcs.par_call().meek_procedure_until_3();

        // Perform tests
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn survey() {
        // Set dataset name
        let db_name: String = "survey".into();

        // Set true graph
        let true_g = PDGraph::from((
            vec!["A", "E", "O", "R", "S", "T"],
            array![
                [false, false, false, false, false, false],
                [false, false, false, false, false, false],
                [false, false, false, false, false, false],
                [false, false, false, false, false, true],
                [false, false, false, false, false, false],
                [false, false, false, true, false, false]
            ],
            array![
                [false, true, false, false, false, false],
                [false, false, false, false, false, false],
                [false, false, false, false, false, false],
                [false, false, false, false, false, false],
                [false, true, false, false, false, false],
                [false, false, false, false, false, false]
            ],
        ));

        // Set true skeleton
        let true_skel = true_g.clone().to_undirected();

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, db_name))
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Perform skeleton discovery
        let skel = pcs.call_skeleton();
        let par_skel = pcs.par_call_skeleton();

        // Perform discovery
        let g = pcs.call().meek_procedure_until_3();
        let par_g = pcs.par_call().meek_procedure_until_3();

        // Perform tests
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
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
