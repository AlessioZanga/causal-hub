use std::ops::Deref;

use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;

use crate::data::{DataSet, GaussianDataSet};

#[derive(Clone, Debug)]
pub struct CovarianceMatrix {
    sigma: Array2<f64>,
}

impl CovarianceMatrix {
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

impl From<&GaussianDataSet> for CovarianceMatrix {
    #[inline]
    fn from(d: &GaussianDataSet) -> Self {
        // Compute the (sample) covariance matrix.
        let sigma = d
            .data()
            .t()
            .cov(1.)
            .expect("Failed to compute the correlation matrix");

        Self { sigma }
    }
}
