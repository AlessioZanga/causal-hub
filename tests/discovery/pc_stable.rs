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
        // Set true skeleton
        let true_skel = Graph::new(
            labels,
            [
                ("bronc", "dysp"),
                ("either", "dysp"),
                ("either", "lung"),
                ("either", "tub"),
                ("either", "xray"),
                ("lung", "smoke"),
                ("lung", "tub"),
                ("smoke", "bronc"),
            ],
        );

        // Set complete graph
        let complete_graph = Graph::complete(labels);

        // Plot true skeleton
        DOT::from(true_skel.clone())
            .plot("./tests/discovery/true_skel")
            .unwrap();

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
        let skel = skeleton(&test, complete_graph);

        // Plot found skeleton
        DOT::from(skel.clone())
            .plot("./tests/discovery/skel")
            .unwrap();

        assert!(skel.is_subgraph(&true_skel));
    }
}