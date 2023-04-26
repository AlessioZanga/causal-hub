#[cfg(test)]
mod discrete {
    use causal_hub::prelude::{skeleton, *};
    use polars::prelude::*;

    #[test]
    fn call_skel() {
        // Set labels
        let labels = [
            "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray",
        ];
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

        // Set complete graph
        let complete_graph = Graph::complete(labels);

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let alpha = 0.05;
        let test = ChiSquared::new(&d).with_significance_level(alpha);

        // Perform skeleton discovery
        let skel = skeleton(&test, complete_graph).0;

        // Perform test
        assert_eq!(skel, true_g.to_undirected());
    }

    #[test]
    fn call_orient_vstructures() {
        // Set labels
        let labels = [
            "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray",
        ];
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

        // Set complete graph
        let complete_graph = Graph::complete(labels);

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let alpha = 0.05;
        let test = ChiSquared::new(&d).with_significance_level(alpha);

        // Perform skeleton discovery
        let (skel, sepsets) = skeleton(&test, complete_graph);

        // Orient v-structures
        let g: PDGraph = orient_vstructures(skel, sepsets);

        // Perform test
        assert_eq!(g, true_g);
    }
    #[test]
    fn meek_1_base_case() {
        let mut g = PDGraph::new_partial(vec![], vec![("1", "2")], vec![("0", "1")]).unwrap();
        meek_1(&mut g);
        assert!(g.type_of_edge(1, 2) == Some('d'));
        assert!(g.type_of_edge(0, 1) == Some('d'))
    }

    #[test]
    fn meek_1_general_case() {
        let mut g = PDGraph::new_partial(
            vec![], 
            vec![("0", "2"), ("0", "3"), ("0", "4"), ("1", "4"), ("4", "5"), ("3", "5"), ],
            vec![("1", "0")]).unwrap();
        meek_1(&mut g);
        // Test for undirected edges
        assert!(g.type_of_edge(0, 4) == Some('u'));
        assert!(g.type_of_edge(4, 1) == Some('u'));
        // Test for directed edges
        assert!(g.type_of_edge(0, 2) == Some('d'));
        assert!(g.type_of_edge(0, 3) == Some('d'));
        assert!(g.type_of_edge(3, 5) == Some('d'));
        assert!(g.type_of_edge(5, 4) == Some('d'));
    }

    #[test]
    fn meek_2_base_case() {
        let mut g =
            PDGraph::new_partial(vec![], vec![("0", "2")], vec![("0", "1"), ("1", "2")]).unwrap();
        meek_2(&mut g);
        assert!(g.type_of_edge(0, 2) == Some('d'));
        assert!(g.type_of_edge(0, 1) == Some('d'));
        assert!(g.type_of_edge(1, 2) == Some('d'));
    }

    #[test]
    fn meek_2_general_case() {
        let mut g = PDGraph::new_partial(
            vec![], 
            vec![("2", "3"), ("1", "4"),("4", "2")],
            vec![("4", "0"), ("0", "1"), ("0", "2"), ("0", "3"),]).unwrap();
        meek_2(&mut g);
        // Test for undirected edges
        assert!(g.type_of_edge(2, 3) == Some('u'));
        // Test for directed edges
        assert!(g.type_of_edge(4, 1) == Some('d'));
        assert!(g.type_of_edge(4, 2) == Some('d'));
    }

    #[test]
    fn meek_3_base_case() {
        let mut g = PDGraph::new_partial(
            vec![],
            vec![("0", "1"), ("0", "2"), ("0", "3")],
            vec![("1", "2"), ("3", "2")],
        )
        .unwrap();
        meek_3(&mut g);
        assert!(g.type_of_edge(0, 1) == Some('u'));
        assert!(g.type_of_edge(0, 3) == Some('u'));

        assert!(g.type_of_edge(1, 3) == None);

        assert!(g.type_of_edge(0, 2) == Some('d'));
        assert!(g.type_of_edge(1, 2) == Some('d'));
        assert!(g.type_of_edge(3, 2) == Some('d'));
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
            meek_4(&mut g);
            dbg!("ciao");
            assert!(g.type_of_edge(0, 3) == Some('u'));

            assert!(g.type_of_edge(0, 2) == None);

            assert!(g.type_of_edge(0, 1) == Some('d'));
            assert!(g.type_of_edge(1, 2) == Some('d'));
            assert!(g.type_of_edge(3, 2) == Some('d'));
        }
    }
}
