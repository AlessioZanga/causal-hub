#[cfg(test)]
mod tests {
    mod marginal_log_likelihood {
        use causal_hub::{prelude::CategoricalDataMatrix, stats::MarginalLogLikelihood};

        #[test]
        fn clone() {
            // Construct a new scoring criterion functor.
            let s = MarginalLogLikelihood::<CategoricalDataMatrix, false>::default();
            // Clone the functor.
            let s = s.clone();

            dbg!(&s);
        }

        #[test]
        fn debug() {
            // Construct a new scoring criterion functor.
            let s = MarginalLogLikelihood::<CategoricalDataMatrix, false>::default();
            // Debug the functor.
            let s = format!("{:?}", s);

            assert_eq!(&s, "MarginalLogLikelihood { _d: PhantomData<causal_hub::data::data_matrix::CategoricalDataMatrix> }");
        }

        #[test]
        fn default() {
            // Construct the default scoring criterion functor.
            let s = MarginalLogLikelihood::<CategoricalDataMatrix, false>::default();

            dbg!(&s);
        }
    }

    mod conditional_log_likelihood {
        use causal_hub::{prelude::CategoricalDataMatrix, stats::ConditionalLogLikelihood};

        #[test]
        fn clone() {
            // Construct a new scoring criterion functor.
            let s = ConditionalLogLikelihood::<CategoricalDataMatrix, false>::default();
            // Clone the functor.
            let s = s.clone();

            dbg!(&s);
        }

        #[test]
        fn debug() {
            // Construct a new scoring criterion functor.
            let s = ConditionalLogLikelihood::<CategoricalDataMatrix, false>::default();
            // Debug the functor.
            let s = format!("{:?}", s);

            assert_eq!(&s, "ConditionalLogLikelihood { _d: PhantomData<causal_hub::data::data_matrix::CategoricalDataMatrix> }");
        }

        #[test]
        fn default() {
            // Construct the default scoring criterion functor.
            let s = ConditionalLogLikelihood::<CategoricalDataMatrix, false>::default();

            dbg!(&s);
        }
    }

    mod log_likelihood {
        use causal_hub::{prelude::CategoricalDataMatrix, stats::LogLikelihood};

        #[test]
        fn clone() {
            // Construct a new scoring criterion functor.
            let s = LogLikelihood::<CategoricalDataMatrix, false>::default();
            // Clone the functor.
            let s = s.clone();

            dbg!(&s);
        }

        #[test]
        fn debug() {
            // Construct a new scoring criterion functor.
            let s = LogLikelihood::<CategoricalDataMatrix, false>::default();
            // Debug the functor.
            let s = format!("{:?}", s);

            assert_eq!(&s, "LogLikelihood { _d: PhantomData<causal_hub::data::data_matrix::CategoricalDataMatrix> }");
        }

        #[test]
        fn default() {
            // Construct the default scoring criterion functor.
            let s = LogLikelihood::<CategoricalDataMatrix, false>::default();

            dbg!(&s);
        }
    }
}

#[cfg(test)]
mod categorical {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn log_likelihood() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/categorical.json").unwrap();
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
        let score = LL::new();

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
        let data = std::fs::read_to_string("./tests/assets/log_likelihood/categorical.json").unwrap();
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
        let score = ParallelLL::new();

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
        let score = LL::new();

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
        let score = ParallelLL::new();

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
