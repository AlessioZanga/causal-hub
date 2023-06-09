#[cfg(test)]
mod discrete {
    use causal_hub::prelude::*;
    use polars::prelude::*;

    // Set ChiSquared significance level
    const ALPHA: f64 = 0.05;

    #[test]
    fn pcstable_cancer() {
        // Set dataset name
        let db_name: String = "cancer".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(g.clone()).plot("./cancer.pdf").unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_earthquake() {
        // Set dataset name
        let db_name: String = "earthquake".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(skel.clone())
            .plot("./skel_earthquake.pdf")
            .unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_asia() {
        // Set dataset name
        let db_name: String = "asia".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(skel.clone()).plot("./skel_asia.pdf").unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_survey() {
        // Set dataset name
        let db_name: String = "survey".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(g.clone()).plot("./survey.pdf").unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_sachs() {
        // Set dataset name
        let db_name: String = "sachs".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(skel.clone()).plot("./skel_sachs.pdf").unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_child() {
        // Set dataset name
        let db_name: String = "child".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(skel.clone()).plot("./skel_child.pdf").unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_alarm() {
        // Set dataset name
        let db_name: String = "alarm".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(skel.clone()).plot("./skel_alarm.pdf").unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_win95pts() {
        // Set dataset name
        let db_name: String = "win95pts".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(g.clone()).plot("./win95pts.pdf").unwrap();

        // Perform tests
        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn pcstable_insurance() {
        // Set dataset name
        let db_name: String = "insurance".into();
        // Set dataset base path
        let base_path: String = format!("./tests/assets/PC-Stable/{}/", db_name);

        // Set true skeleton
        let true_skel =
            Graph::from(DOT::read(format!("{}skeleton-{}.dot", base_path, db_name)).unwrap());

        // Set true graph
        let true_g =
            PDGraph::from(DOT::read(format!("{}cpdag-{}.dot", base_path, db_name)).unwrap());

        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", base_path, db_name))
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

        // Perform discovery
        let mut g = pcs.call();
        g = g.meek_procedure_until_3();

        // Plot found skeleton
        DOT::from(g.clone()).plot("./insurance.pdf").unwrap();

        // Perform tests
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
