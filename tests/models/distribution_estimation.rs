#[cfg(test)]
mod variable_elimination {
    use approx::*;
    use causal_hub::prelude::*;
    use itertools::Itertools;
    use ndarray::prelude::*;

    #[test]
    fn call() {
        // Load data from file.
        let text =
            std::fs::read_to_string("./tests/assets/distribution_estimation/discrete.json")
                .expect("Failed to read file to string");
        let data: Vec<(&str, Vec<&str>, Vec<usize>, Vec<Option<f64>>)> =
            serde_json::from_str(&text).expect("Failed to deserialize string to struct");
        // Initialize Bayesian network.
        let b: CategoricalBN = BIF::read("tests/assets/bif/asia.bif").unwrap().into();

        // Construct estimator.
        let estimator = VE::new(&b);

        // Test for each query type.
        for (t, x, shape, values) in data {
            // Map None values to NaN.
            let values = values
                .into_iter()
                .map(|x| x.unwrap_or(f64::NAN))
                .collect_vec();
            // Construct factor values.
            let true_query = ArrayD::from_shape_vec(shape, values).unwrap();
            // Perform the specified query.
            let pred_query: CategoricalFactor = match t {
                "marginal" => estimator.marginal(x[0]).into(),
                "joint" => estimator.joint(x).into(),
                "conditional" => estimator.conditional(x[0], x.into_iter().skip(1)).into(),
                _ => unreachable!(),
            };

            // Assert relative equal by handling NaN values accordingly.
            assert!(true_query
                .into_iter()
                .zip(pred_query.values().into_iter())
                .all(|(x, y)| { x.relative_eq(y, 1e-16, 1e-15) || (x.is_nan() && y.is_nan()) }));
        }
    }

    #[test]
    fn par_call() {
        // Load data from file.
        let text =
            std::fs::read_to_string("./tests/assets/distribution_estimation/discrete.json")
                .expect("Failed to read file to string");
        let data: Vec<(&str, Vec<&str>, Vec<usize>, Vec<Option<f64>>)> =
            serde_json::from_str(&text).expect("Failed to deserialize string to struct");
        // Initialize Bayesian network.
        let b: CategoricalBN = BIF::read("tests/assets/bif/asia.bif").unwrap().into();

        // Construct estimator.
        let estimator = ParallelVE::new(&b);

        // Test for each query type.
        for (t, x, shape, values) in data {
            // Map None values to NaN.
            let values = values
                .into_iter()
                .map(|x| x.unwrap_or(f64::NAN))
                .collect_vec();
            // Construct factor values.
            let true_query = ArrayD::from_shape_vec(shape, values).unwrap();
            // Perform the specified query.
            let pred_query: CategoricalFactor = match t {
                "marginal" => estimator.marginal(x[0]).into(),
                "joint" => estimator.joint(x).into(),
                "conditional" => estimator.conditional(x[0], x.into_iter().skip(1)).into(),
                _ => unreachable!(),
            };

            // Assert relative equal by handling NaN values accordingly.
            assert!(true_query
                .into_iter()
                .zip(pred_query.values().into_iter())
                .all(|(x, y)| { x.relative_eq(y, 1e-16, 1e-15) || (x.is_nan() && y.is_nan()) }));
        }
    }
}
