#[cfg(test)]
mod categorical {
    use causal_hub::{polars::prelude::*, prelude::*};

    // Set ChiSquared significance level
    const ALPHA: f64 = 0.05;

    // Set base path
    const BASE_PATH: &str = "./tests/assets/pc_stable/";

    #[test]
    fn cancer() {
        // Set dataset name
        let d: String = "cancer".into();
        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, d))
            .unwrap()
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Set true graph
        let mut true_g = PGraph::empty(d.labels_iter());
        // Add directed edge.
        true_g.add_directed_edge(3, 0);
        true_g.add_directed_edge(4, 0);
        // Set true skeleton
        let true_skel = true_g.to_undirected();

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d, ALPHA);

        // Create PC-Stable functor
        let pc_stable = PCStable::new(&test);

        // Perform skeleton discovery
        let (skel, _): (UGraph, _) = pc_stable.skeleton();
        let (par_skel, _) = pc_stable.par_skeleton();

        // Perform discovery
        let g: PGraph = pc_stable.call();
        let par_g = pc_stable.par_call();

        // Perform tests
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn asia() {
        // Set dataset name
        let d: String = "asia".into();
        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, d))
            .unwrap()
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Set true graph
        let mut true_g = PGraph::empty(d.labels_iter());
        // Add undirected edge.
        true_g.add_undirected_edge(1, 2);
        true_g.add_undirected_edge(1, 5);
        true_g.add_undirected_edge(3, 4);
        // Set true skeleton
        let true_skel = true_g.to_undirected();

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d, ALPHA);

        // Create PC-Stable functor
        let pc_stable = PCStable::new(&test);

        // Perform skeleton discovery
        let (skel, _): (UGraph, _) = pc_stable.skeleton();
        let (par_skel, _) = pc_stable.par_skeleton();

        // Perform discovery
        let g: PGraph = pc_stable.call();
        let par_g = pc_stable.par_call();

        // Perform tests
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    /* FIXME:
    #[test]
    fn survey() {
        // Set dataset name
        let db_name: String = "survey".into();

        // Set true graph
        let true_g = PGraph::from((
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
        let d = CategoricalDataMatrix::from(d);

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
        let mut g = PGraph::new_pagraph(vec![], vec![("1", "2")], vec![("0", "1")]);
        g.meek_1();
        assert!(g.has_directed_edge(1, 2));
        assert!(g.has_directed_edge(0, 1))
    }

    #[test]
    fn meek_1_general_case() {
        let mut g = PGraph::new_pagraph(
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
        assert!(g.has_undirected_edge(0, 4));
        assert!(g.has_undirected_edge(4, 1));
        assert!(g.has_undirected_edge(1, 2));
        // Test for directed edges
        assert!(g.has_directed_edge(0, 3));
        assert!(g.has_directed_edge(3, 5));
        assert!(g.has_directed_edge(5, 4));
    }

    #[test]
    fn meek_2_base_case() {
        let mut g = PGraph::new_pagraph(vec![], vec![("0", "2")], vec![("0", "1"), ("1", "2")]);
        g.meek_2();
        assert!(g.has_directed_edge(0, 2));
        assert!(g.has_directed_edge(0, 1));
        assert!(g.has_directed_edge(1, 2));
    }

    #[test]
    fn meek_2_general_case() {
        let mut g = PGraph::new_pagraph(
            vec![],
            vec![("1", "2"), ("1", "3"), ("4", "0")],
            vec![("1", "0"), ("0", "2"), ("4", "2"), ("2", "3")],
        );
        g.meek_2();
        // Test for undirected edges
        assert!(g.has_undirected_edge(0, 4));
        // Test for directed edges
        assert!(g.has_directed_edge(1, 2));
        assert!(g.has_directed_edge(1, 3));
    }

    #[test]
    fn meek_3_base_case() {
        let mut g = PGraph::new_pagraph(
            vec![],
            vec![("0", "1"), ("0", "2"), ("0", "3")],
            vec![("1", "2"), ("3", "2")],
        );
        g.meek_3();
        // Test for undirected edges
        assert!(g.has_undirected_edge(0, 1));
        assert!(g.has_undirected_edge(0, 3));
        // Test for directed edges
        assert!(g.has_directed_edge(0, 2));
        assert!(g.has_directed_edge(1, 2));
        assert!(g.has_directed_edge(3, 2));
    }

    #[test]
    fn meek_3_general_case() {
        let mut g = PGraph::new_pagraph(
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
        assert!(g.has_undirected_edge(5, 0));
        // Test for directed edges
        assert!(g.has_directed_edge(1, 0));
        assert!(g.has_directed_edge(4, 0));
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
            let mut g = PGraph::new_pagraph(v, ue, de);
            g.meek_4();
            // Test for undirected edges
            assert!(g.has_undirected_edge(0, 3));
            // Test for directed edges
            assert!(g.has_directed_edge(0, 1));
            assert!(g.has_directed_edge(1, 2));
            assert!(g.has_directed_edge(3, 2));
        }
    }

    #[test]
    fn meek_4_general_case() {
        let mut g = PGraph::new_pagraph(
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
        assert!(g.has_undirected_edge(5, 0));
        // Test for directed edges
        assert!(g.has_directed_edge(7, 0));
        assert!(g.has_directed_edge(3, 0));
    }
    */
}
