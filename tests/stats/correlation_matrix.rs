#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use approx::*;
    use causal_hub::{polars::prelude::*, prelude::*};
    use ndarray::prelude::*;

    #[test]
    fn clone() {
        // Construct a new correlation matrix.
        let rho = CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 1.00, 0.50],
            [0.90, 0.50, 1.00]
        ]);

        let rho_prime = rho.clone();

        assert_eq!(rho.deref(), rho_prime.deref());
    }

    #[test]
    fn debug() {
        // Construct a new correlation matrix.
        let rho = CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 1.00, 0.50],
            [0.90, 0.50, 1.00]
        ]);

        assert_eq!(format!("{:?}", rho), "CorrelationMatrix { rho: [[1.0, 0.25, 0.9],\n [0.25, 1.0, 0.5],\n [0.9, 0.5, 1.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2 }");
    }

    #[test]
    fn deref() {
        // Construct a new correlation matrix.
        let rho = CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 1.00, 0.50],
            [0.90, 0.50, 1.00]
        ]);

        let rho_prime = rho.clone();

        assert_eq!(rho.deref(), rho_prime.deref());
    }

    #[test]
    fn new() {
        // Construct a new correlation matrix.
        CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 1.00, 0.50],
            [0.90, 0.50, 1.00]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_non_square() {
        // Construct a new correlation matrix.
        CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 1.00, 0.75],
            // [0.90, 0.50, 1.00]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_non_symmetric() {
        // Construct a new correlation matrix.
        CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 1.00, 0.75],
            [0.90, 0.50, 1.00]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_non_diagonal_one() {
        // Construct a new correlation matrix.
        CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 2.00, 0.50],
            [0.90, 0.50, 1.00]
        ]);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_out_of_interval() {
        // Construct a new correlation matrix.
        CorrelationMatrix::new(array![
            [1.00, 0.25, 0.90],
            [0.25, 1.00, 2.50],
            [0.90, 0.50, 1.00]
        ]);
    }

    #[test]
    fn into_array() {
        // Define a correlation matrix.
        let true_r = array![[1.00, 0.25, 0.90], [0.25, 1.00, 0.50], [0.90, 0.50, 1.00]];
        // Construct a new correlation matrix.
        let pred_r = CorrelationMatrix::new(true_r.clone());

        assert_relative_eq!(true_r, pred_r.into());
    }

    #[test]
    fn from_data() {
        // Read data from file.
        let true_r = std::fs::read_to_string("./tests/assets/correlation_matrix.json").unwrap();
        let true_r: Array2<f64> = serde_json::from_str(&true_r).unwrap();

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = GaussianDataMatrix::from(d);
        // Construct a new correlation matrix.
        let pred_r = CorrelationMatrix::from(&d);

        assert_relative_eq!(true_r, pred_r.into(), max_relative = 1e-8);
    }

    #[test]
    fn from_covariance() {
        // Read data from file.
        let true_r = std::fs::read_to_string("./tests/assets/correlation_matrix.json").unwrap();
        let true_r: Array2<f64> = serde_json::from_str(&true_r).unwrap();

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = GaussianDataMatrix::from(d);
        // Compute a new covariance matrix.
        let sigma = CovarianceMatrix::from(&d);
        // Construct a new correlation matrix.
        let pred_r = CorrelationMatrix::from(sigma);

        assert_relative_eq!(true_r, pred_r.into(), max_relative = 1e-8);
    }
}
