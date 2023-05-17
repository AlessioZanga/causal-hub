#[cfg(test)]
mod tests {
    use causal_hub::{prelude::DiscreteDataMatrix, stats::AIC};

    #[test]
    fn clone() {
        // Construct a new scoring criterion functor.
        let s = AIC::<DiscreteDataMatrix>::default();
        // Clone the functor.
        let s = s.clone();

        dbg!(&s);
    }

    #[test]
    fn debug() {
        // Construct a new scoring criterion functor.
        let s = AIC::<DiscreteDataMatrix>::default();
        // Debug the functor.
        let s = format!("{:?}", s);

        assert_eq!(&s, "AkaikeInformationCriterion { _d: PhantomData<causal_hub::data::data_matrix::DiscreteDataMatrix>, k: 1.0 }");
    }

    #[test]
    fn default() {
        // Construct the default scoring criterion functor.
        let s = AIC::<DiscreteDataMatrix>::default();

        dbg!(&s);
    }

    #[test]
    fn with_penalty_coeff() {
        // Construct the default scoring criterion functor.
        let s = AIC::<DiscreteDataMatrix>::default();
        // Set penalty coefficient.
        let s = s.with_penalty_coeff(2.);

        dbg!(&s);
    }
}

#[cfg(test)]
mod discrete {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn akaike_information_criterion() {
        // Read test database from file.
        let data =
            std::fs::read_to_string("./tests/assets/akaike_information_criterion/discrete.json")
                .unwrap();
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
        let score = AIC::new();

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&score, &d, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &z),
                s,
                max_relative = 1e-8
            );
        }
    }

    #[test]
    fn parallel_akaike_information_criterion() {
        // Read test database from file.
        let data =
            std::fs::read_to_string("./tests/assets/akaike_information_criterion/discrete.json")
                .unwrap();
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
        let score = ParallelAIC::new();

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&score, &d, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

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
    fn akaike_information_criterion() {
        // Read test database from file.
        let data =
            std::fs::read_to_string("./tests/assets/akaike_information_criterion/gaussian.json")
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
        let score = AIC::new();

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&score, &d, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &z),
                s,
                max_relative = 1e-8
            );
        }
    }

    #[test]
    fn parallel_akaike_information_criterion() {
        // Read test database from file.
        let data =
            std::fs::read_to_string("./tests/assets/akaike_information_criterion/gaussian.json")
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
        let score = ParallelAIC::new();

        // Compute global score.
        assert_relative_eq!(
            ScoringCriterion::call(&score, &d, &g),
            V!(g)
                .map(|x| DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &[]))
                .sum(),
            max_relative = 1e-8
        );

        for (x, z, s) in data {
            let x = g.get_vertex_index(&x);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(
                DecomposableScoringCriterion::<_, DiGraph>::call(&score, &d, x, &z),
                s,
                max_relative = 1e-8
            );
        }
    }
}
