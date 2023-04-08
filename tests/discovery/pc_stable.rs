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
        let (skel, sepsets, triples) = skeleton(&test, complete_graph);

        // Orient v-structures
        let g: PDGraph = orient_vstructures(skel, sepsets, triples);

        // Perform test
        assert_eq!(g, true_g);
    }
}
