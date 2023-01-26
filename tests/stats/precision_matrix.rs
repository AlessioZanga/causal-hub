#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;
    use polars::prelude::*;

    #[test]
    fn clone() {
        // Construct a new precision matrix.
        let omega = PrecisionMatrix::new(array![
            [7.31707317, 0.48780488, -0.8401084],
            [0.48780488, 0.11585366, -0.07452575],
            [-0.8401084, -0.07452575, 0.1129178]
        ]);

        let omega_prime = omega.clone();

        assert_eq!(omega.deref(), omega_prime.deref());
    }

    #[test]
    fn debug() {
        // Construct a new precision matrix.
        let omega = PrecisionMatrix::new(array![
            [7.31707317, 0.48780488, -0.8401084],
            [0.48780488, 0.11585366, -0.07452575],
            [-0.8401084, -0.07452575, 0.1129178]
        ]);

        assert_eq!(format!("{:?}", omega), "PrecisionMatrix { omega: [[7.31707317, 0.48780488, -0.8401084],\n [0.48780488, 0.11585366, -0.07452575],\n [-0.8401084, -0.07452575, 0.1129178]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2 }");
    }

    #[test]
    fn deref() {
        // Construct a new precision matrix.
        let omega = PrecisionMatrix::new(array![
            [7.31707317, 0.48780488, -0.8401084],
            [0.48780488, 0.11585366, -0.07452575],
            [-0.8401084, -0.07452575, 0.1129178]
        ]);

        let omega_prime = omega.clone();

        assert_eq!(omega.deref(), omega_prime.deref());
    }

    #[test]
    fn new() {
        // Construct a new precision matrix.
        PrecisionMatrix::new(array![
            [7.31707317, 0.48780488, -0.8401084],
            [0.48780488, 0.11585366, -0.07452575],
            [-0.8401084, -0.07452575, 0.1129178]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_non_square() {
        // Construct a new precision matrix.
        PrecisionMatrix::new(array![
            [7.31707317, 0.48780488, -0.8401084],
            [0.48780488, 0.11585366, -0.07452575],
            // [-0.8401084, -0.07452575, 0.1129178]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_non_symmetric() {
        // Construct a new precision matrix.
        PrecisionMatrix::new(array![
            [7.31707317, 0.48780488, -0.8401084],
            [0.48780488, 0.11585366, -4.07452575],
            [-0.8401084, -0.07452575, 0.1129178]
        ]);
    }

    #[test]
    fn into_array() {
        // Define a precision matrix.
        let true_s = array![
            [7.31707317, 0.48780488, -0.8401084],
            [0.48780488, 0.11585366, -0.07452575],
            [-0.8401084, -0.07452575, 0.1129178]
        ];
        // Construct a new precision matrix.
        let pred_s = PrecisionMatrix::new(true_s.clone());

        assert_relative_eq!(true_s, pred_s.into());
    }

    #[test]
    fn from_precision() {
        // Read data from file.
        let true_o = std::fs::read_to_string("./tests/assets/precision_matrix.json").unwrap();
        let true_o: Array2<f64> = serde_json::from_str(&true_o).unwrap();

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = ContinuousDataMatrix::from(d);
        // Compute a new covariance matrix.
        let sigma = CovarianceMatrix::from(&d);
        // Construct a new precision matrix.
        let pred_o = PrecisionMatrix::from(sigma);

        assert_relative_eq!(true_o, pred_o.into(), max_relative = 1e-8);
    }
}
