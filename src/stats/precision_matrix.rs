use std::ops::Deref;

use ndarray::prelude::*;
use ndarray_linalg::InverseInto;

use super::CovarianceMatrix;

/// Precision matrix $\Omega$.
#[derive(Clone, Debug)]
pub struct PrecisionMatrix {
    omega: Array2<f64>,
}

impl Deref for PrecisionMatrix {
    type Target = Array2<f64>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.omega
    }
}

impl PrecisionMatrix {
    /// Construct a new precision matrix.
    ///
    /// # Panics
    ///
    /// Matrix must be square and symmetric.
    ///
    #[inline]
    pub fn new(omega: Array2<f64>) -> Self {
        // Assert Omega is squared ...
        assert!(omega.is_square(), "Covariance matrix must be square");
        // ... symmetric ...
        assert_eq!(omega, omega.t(), "Covariance matrix must be symmetric");

        Self { omega }
    }
}

impl From<PrecisionMatrix> for Array2<f64> {
    #[inline]
    fn from(other: PrecisionMatrix) -> Self {
        other.omega
    }
}

impl From<CovarianceMatrix> for PrecisionMatrix {
    #[inline]
    fn from(sigma: CovarianceMatrix) -> Self {
        // Get underlying data.
        let sigma: Array2<f64> = sigma.into();
        // Compute the inverse of the correlation matrix. TODO: Use SVD decomposition.
        let omega = sigma
            .inv_into()
            .expect("Failed to compute the inverse of the covariance matrix");

        Self { omega }
    }
}
