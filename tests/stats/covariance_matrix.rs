#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;
    use polars::prelude::*;

    #[test]
    fn clone() {
        // Construct a new covariance matrix.
        let sigma = CovarianceMatrix::new(array![
            [1.00, 1.00, 8.10],
            [1.00, 16.0, 18.0],
            [8.10, 18.0, 81.0]
        ]);

        let sigma_prime = sigma.clone();

        assert_eq!(sigma.deref(), sigma_prime.deref());
    }

    #[test]
    fn debug() {
        // Construct a new covariance matrix.
        let sigma = CovarianceMatrix::new(array![
            [1.00, 1.00, 8.10],
            [1.00, 16.0, 18.0],
            [8.10, 18.0, 81.0]
        ]);

        assert_eq!(format!("{:?}", sigma), "CovarianceMatrix { sigma: [[1.0, 1.0, 8.1],\n [1.0, 16.0, 18.0],\n [8.1, 18.0, 81.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2 }");
    }

    #[test]
    fn deref() {
        // Construct a new covariance matrix.
        let sigma = CovarianceMatrix::new(array![
            [1.00, 1.00, 8.10],
            [1.00, 16.0, 18.0],
            [8.10, 18.0, 81.0]
        ]);

        let sigma_prime = sigma.clone();

        assert_eq!(sigma.deref(), sigma_prime.deref());
    }

    #[test]
    fn new() {
        // Construct a new covariance matrix.
        CovarianceMatrix::new(array![
            [1.00, 1.00, 8.10],
            [1.00, 16.0, 18.0],
            [8.10, 18.0, 81.0]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_non_square() {
        // Construct a new covariance matrix.
        CovarianceMatrix::new(array![
            [1.00, 1.00, 8.10],
            [1.00, 16.0, 18.0],
            // [8.10, 18.0, 81.0]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_non_symmetric() {
        // Construct a new covariance matrix.
        CovarianceMatrix::new(array![
            [1.00, 1.00, 8.10],
            [1.00, 16.0, 20.0],
            [8.10, 18.0, 81.0]
        ]);
    }

    #[test]
    fn into_array() {
        // Define a covariance matrix.
        let true_s = array![[1.0, 1.0, 8.1], [1.0, 16.0, 18.0], [8.1, 18.0, 81.0]];
        // Construct a new covariance matrix.
        let pred_s = CovarianceMatrix::new(true_s.clone());

        assert_relative_eq!(true_s, pred_s.into());
    }

    #[test]
    fn from_data() {
        // Read data from file.
        let true_s = std::fs::read_to_string("./tests/assets/covariance_matrix.json").unwrap();
        let true_s: Array2<f64> = serde_json::from_str(&true_s).unwrap();

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = ContinuousDataSet::from(d);
        // Construct a new covariance matrix.
        let pred_s = CovarianceMatrix::from(&d);

        assert_relative_eq!(true_s, pred_s.into(), max_relative = 1e-8);
    }
}
