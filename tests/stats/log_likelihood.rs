#[cfg(test)]
mod discrete {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/discrete.json").unwrap();
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
        let s = LL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &z),
                true_s,
                max_relative = 1e-8
            );
        }
    }

    #[test]
    fn parallel_log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/discrete.json").unwrap();
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
        let s = ParallelLL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &z),
                true_s,
                max_relative = 1e-8
            );
        }
    }
}

#[cfg(test)]
mod gaussian {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/gaussian.json").unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = ContinuousDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels());

        // Initialize the default scoring criterion.
        let s = LL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &z),
                true_s,
                max_relative = 1e-8
            );
        }
    }

    #[test]
    fn parallel_log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/gaussian.json").unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = ContinuousDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels());

        // Initialize the default scoring criterion.
        let s = ParallelLL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&s, x, &z),
                true_s,
                max_relative = 1e-8
            );
        }
    }
}
