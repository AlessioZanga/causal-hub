use std::ops::Deref;

use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;

use crate::data::{ContinuousDataMatrix, DataSet};

/// (Sample) Covariance matrix $\Sigma$.
#[derive(Clone, Debug)]
pub struct CovarianceMatrix {
    sigma: Array2<f64>,
}

impl CovarianceMatrix {
    /// Construct a new covariance matrix.
    ///
    /// # Panics
    ///
    /// The matrix must be squared, symmetric and non-negative.
    ///
    #[inline]
    pub fn new(sigma: Array2<f64>) -> Self {
        // Assert Sigma is square ...
        assert!(sigma.is_square(), "Covariance matrix must be square");
        // ... symmetric.
        assert_eq!(sigma, sigma.t(), "Covariance matrix must be symmetric");

        Self { sigma }
    }
}

impl Deref for CovarianceMatrix {
    type Target = Array2<f64>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.sigma
    }
}

impl From<CovarianceMatrix> for Array2<f64> {
    #[inline]
    fn from(other: CovarianceMatrix) -> Self {
        other.sigma
    }
}

impl From<&ContinuousDataMatrix> for CovarianceMatrix {
    #[inline]
    fn from(d: &ContinuousDataMatrix) -> Self {
        // Compute the (sample) covariance matrix.
        let sigma = d
            .values()
            .t()
            .cov(1.)
            .expect("Failed to compute the correlation matrix");

        Self { sigma }
    }
}
