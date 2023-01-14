#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood.json").unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels());

        // Initialize the default scoring criterion.
        let score = LogLikelihood::<false, false>::new();

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&score, &d, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, s) in data {
            let x = g.vertex(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.vertex(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &z),
                s,
                max_relative = 1e-8
            );
        }
    }

    #[test]
    fn parallel_log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood.json").unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels());

        // Initialize the default scoring criterion.
        let score = LogLikelihood::<true, true>::new();

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&score, &d, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, s) in data {
            let x = g.vertex(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.vertex(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &z),
                s,
                max_relative = 1e-8
            );
        }
    }
}
