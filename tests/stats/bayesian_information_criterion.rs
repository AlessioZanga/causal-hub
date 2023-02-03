#[cfg(test)]
mod tests {
    use causal_hub::{prelude::CategoricalDataMatrix, stats::BIC};

    #[test]
    fn clone() {
        // Construct a new scoring criterion functor.
        let s = BIC::<CategoricalDataMatrix>::default();
        // Clone the functor.
        let s = s.clone();

        dbg!(&s);
    }

    #[test]
    fn debug() {
        // Construct a new scoring criterion functor.
        let s = BIC::<CategoricalDataMatrix>::default();
        // Debug the functor.
        let s = format!("{:?}", s);

        assert_eq!(&s, "BayesianInformationCriterion { _d: PhantomData<causal_hub::data::data_matrix::CategoricalDataMatrix>, k: 1.0 }");
    }

    #[test]
    fn default() {
        // Construct the default scoring criterion functor.
        let s = BIC::<CategoricalDataMatrix>::default();

        dbg!(&s);
    }

    #[test]
    fn with_penalty_coeff() {
        // Construct the default scoring criterion functor.
        let s = BIC::<CategoricalDataMatrix>::default();
        // Set penalty coefficient.
        let s = s.with_penalty_coeff(2.);

        dbg!(&s);
    }
}

#[cfg(test)]
mod categorical {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn bayesian_information_criterion() {
        // Read test database from file.
        let data = std::fs::read_to_string(
            "./tests/assets/bayesian_information_criterion/categorical.json",
        )
        .unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels());

        // Initialize the default scoring criterion.
        let score = BIC::new();

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
    fn parallel_bayesian_information_criterion() {
        // Read test database from file.
        let data = std::fs::read_to_string(
            "./tests/assets/bayesian_information_criterion/categorical.json",
        )
        .unwrap();
        let data: Vec<(String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels());

        // Initialize the default scoring criterion.
        let score = ParallelBIC::new();

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

#[cfg(test)]
mod gaussian {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn bayesian_information_criterion() {
        // Read test database from file.
        let data =
            std::fs::read_to_string("./tests/assets/bayesian_information_criterion/gaussian.json")
                .unwrap();
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
        let score = BIC::new();

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
    fn parallel_bayesian_information_criterion() {
        // Read test database from file.
        let data =
            std::fs::read_to_string("./tests/assets/bayesian_information_criterion/gaussian.json")
                .unwrap();
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
        let score = ParallelBIC::new();

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
