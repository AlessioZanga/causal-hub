#[cfg(test)]
mod variable_elimination {
    use approx::assert_relative_eq;
    use causal_hub::prelude::*;
    use itertools::Itertools;
    use ndarray::ArrayD;

    #[test]
    fn call() {
        // Load data from file.
        let text = std::fs::read_to_string("./tests/assets/distribution_estimation/discrete.json")
            .expect("Failed to read file to string");
        let data: Vec<(&str, Vec<&str>, Vec<usize>, Vec<Option<f64>>)> =
            serde_json::from_str(&text).expect("Failed to deserialize string to struct");
        // Initialize Bayesian network.
        let b: DiscreteBN = BIF::read("tests/assets/bif/asia.bif").unwrap().into();

        // Construct estimator.
        let estimator = VariableElimination::new(&b);

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
            let pred_query: DiscreteFactor = match t {
                "marginal" => estimator.marginal(x[0]).into(),
                "joint" => estimator.joint(x).into(),
                "conditional" => continue, // FIXME: estimator.conditional(x[0], x.into_iter().skip(1)).into(),
                _ => unreachable!(),
            };

            assert_relative_eq!(true_query, pred_query.values());
        }
    }
}
