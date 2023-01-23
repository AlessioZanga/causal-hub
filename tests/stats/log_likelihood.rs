#[cfg(test)]
mod tests {
    mod marginal_log_likelihood {
        use causal_hub::{prelude::DiscreteDataMatrix, stats::MarginalLogLikelihood};

        #[test]
        fn clone() {
            // Construct a new scoring criterion functor.
            let s = MarginalLogLikelihood::<DiscreteDataMatrix, false>::default();
            // Clone the functor.
            let s = s.clone();

            dbg!(&s);
        }

        #[test]
        fn debug() {
            // Construct a new scoring criterion functor.
            let s = MarginalLogLikelihood::<DiscreteDataMatrix, false>::default();
            // Debug the functor.
            let s = format!("{:?}", s);

            assert_eq!(&s, "MarginalLogLikelihood { _d: PhantomData<causal_hub::data::data_matrix::DiscreteDataMatrix> }");
        }

        #[test]
        fn default() {
            // Construct the default scoring criterion functor.
            let s = MarginalLogLikelihood::<DiscreteDataMatrix, false>::default();

            dbg!(&s);
        }
    }

    mod conditional_log_likelihood {
        use causal_hub::{prelude::DiscreteDataMatrix, stats::ConditionalLogLikelihood};

        #[test]
        fn clone() {
            // Construct a new scoring criterion functor.
            let s = ConditionalLogLikelihood::<DiscreteDataMatrix, false>::default();
            // Clone the functor.
            let s = s.clone();

            dbg!(&s);
        }

        #[test]
        fn debug() {
            // Construct a new scoring criterion functor.
            let s = ConditionalLogLikelihood::<DiscreteDataMatrix, false>::default();
            // Debug the functor.
            let s = format!("{:?}", s);

            assert_eq!(&s, "ConditionalLogLikelihood { _d: PhantomData<causal_hub::data::data_matrix::DiscreteDataMatrix> }");
        }

        #[test]
        fn default() {
            // Construct the default scoring criterion functor.
            let s = ConditionalLogLikelihood::<DiscreteDataMatrix, false>::default();

            dbg!(&s);
        }
    }

    mod log_likelihood {
        use causal_hub::{prelude::DiscreteDataMatrix, stats::LogLikelihood};

        #[test]
        fn clone() {
            // Construct a new scoring criterion functor.
            let s = LogLikelihood::<DiscreteDataMatrix, false>::default();
            // Clone the functor.
            let s = s.clone();

            dbg!(&s);
        }

        #[test]
        fn debug() {
            // Construct a new scoring criterion functor.
            let s = LogLikelihood::<DiscreteDataMatrix, false>::default();
            // Debug the functor.
            let s = format!("{:?}", s);

            assert_eq!(&s, "LogLikelihood { _d: PhantomData<causal_hub::data::data_matrix::DiscreteDataMatrix> }");
        }

        #[test]
        fn default() {
            // Construct the default scoring criterion functor.
            let s = LogLikelihood::<DiscreteDataMatrix, false>::default();

            dbg!(&s);
        }
    }
}

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
