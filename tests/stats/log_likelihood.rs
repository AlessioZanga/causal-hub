#[cfg(test)]
mod categorical {
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
        let d = CategoricalDataMatrix::from(d);

        // Build an empty the graph.
        let g = DGraph::empty(d.labels_iter());

        // Initialize the default scoring criterion.
        let s = LL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.label_to_vertex(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.label_to_vertex(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &z),
                true_s,
                max_relative = 1e-8
            );
        }
    }

    #[test]
    fn par_log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/discrete.json").unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Build an empty the graph.
        let g = DGraph::empty(d.labels_iter());

        // Initialize the default scoring criterion.
        let s = LL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.label_to_vertex(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.label_to_vertex(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &z),
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
        let d = GaussianDataMatrix::from(d);

        // Build an empty the graph.
        let g = DGraph::empty(d.labels_iter());

        // Initialize the default scoring criterion.
        let s = LL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.label_to_vertex(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.label_to_vertex(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &z),
                true_s,
                max_relative = 1e-8
            );
        }
    }

    #[test]
    fn par_log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/gaussian.json").unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = GaussianDataMatrix::from(d);

        // Build an empty the graph.
        let g = DGraph::empty(d.labels_iter());

        // Initialize the default scoring criterion.
        let s = LL::new(&d);

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&s, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, true_s) in data {
            let x = g.label_to_vertex(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.label_to_vertex(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DGraph>::call(&s, x, &z),
                true_s,
                max_relative = 1e-8
            );
        }
    }
}
