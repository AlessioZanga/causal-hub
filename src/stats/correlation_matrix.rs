use std::ops::Deref;

use approx::*;
use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;

use super::CovarianceMatrix;
use crate::data::{ContinuousDataMatrix, DataSet};

/// Correlation matrix $\Rho$.
#[derive(Clone, Debug)]
pub struct CorrelationMatrix {
    rho: Array2<f64>,
}

impl CorrelationMatrix {
    /// Construct a new correlation matrix.
    ///
    /// # Panics
    ///
    /// The matrix must be squared, symmetric, diagonal is +1 and
    /// all other values are in the [-1, +1] interval.
    ///
    #[inline]
    pub fn new(rho: Array2<f64>) -> Self {
        // Assert Rho is square ...
        assert!(rho.is_square(), "Correlation matrix must be square");
        // ... symmetric ...
        assert_eq!(rho, rho.t(), "Correlation matrix must be symmetric");
        // ... all values on the diagonal are +1. ...
        assert!(rho
            .diag()
            .iter()
            .all(|r| r.relative_eq(&1., f64::EPSILON, 1e-8)));
        // ... and all other values are in the [-1., +1.] interval.
        assert!(
            rho.iter().all(|r| (-1. ..=1.).contains(r)),
            "Correlation matrix must be in the [-1, +1] interval"
        );

        Self { rho }
    }
}

impl Deref for CorrelationMatrix {
    type Target = Array2<f64>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.rho
    }
}

impl From<CorrelationMatrix> for Array2<f64> {
    #[inline]
    fn from(other: CorrelationMatrix) -> Self {
        other.rho
    }
}

impl From<&ContinuousDataMatrix> for CorrelationMatrix {
    #[inline]
    fn from(d: &ContinuousDataMatrix) -> Self {
        // Compute the correlation matrix.
        let rho = d
            .values()
            .t()
            .pearson_correlation()
            .expect("Failed to compute the correlation matrix");

        Self { rho }
    }
}

impl From<CovarianceMatrix> for CorrelationMatrix {
    #[inline]
    fn from(sigma: CovarianceMatrix) -> Self {
        // Get underlying data.
        let sigma: Array2<f64> = sigma.into();
        // Compute the variance vector.
        let d = sigma.diag().mapv(|s| 1. / f64::sqrt(s));
        // Cast to column vector.
        let d = d.insert_axis(Axis(1));
        // Compute the correlation matrix.
        let rho = &d * sigma * d.t();
        // Clip the values to [-1., +1.].
        let rho = rho.mapv(|r| r.clamp(-1., 1.));

        Self { rho }
    }
}
