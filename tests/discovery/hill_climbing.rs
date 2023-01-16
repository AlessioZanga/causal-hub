#[cfg(test)]
mod tests {
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn hill_climbing() {
        // Set true graph.
        let true_g = DiGraph::new(
            ["A", "B", "D", "E", "L", "S", "T", "X"],
            [
                ("B", "D"),
                ("E", "D"),
                ("E", "L"),
                ("E", "T"),
                ("E", "X"),
                ("L", "S"),
                ("L", "T"),
                ("S", "B"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(d.labels(), [], []);

        // Initialize score functor.
        let s = BIC::new();

        // Initialize discovery functor.
        let hc = HC::new(s);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }

    #[test]
    fn parallel_hill_climbing() {
        // Set true graph.
        let true_g = DiGraph::new(
            ["A", "B", "D", "E", "L", "S", "T", "X"],
            [
                ("B", "D"),
                ("E", "D"),
                ("E", "L"),
                ("E", "T"),
                ("E", "X"),
                ("L", "S"),
                ("L", "T"),
                ("S", "B"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(d.labels(), [], []);

        // Initialize score functor.
        let s = BIC::new();

        // Initialize discovery functor.
        let hc = ParallelHC::new(s);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }
}
