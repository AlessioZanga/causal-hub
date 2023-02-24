use itertools::Itertools;
use ndarray::prelude::*;

use super::{CovarianceMatrix, PrecisionMatrix};

/// Partial correlation functor.
#[derive(Clone, Debug)]
pub struct PartialCorrelation {
    sigma: CovarianceMatrix,
}

impl PartialCorrelation {
    /// Compute partial correlation of $X$ and $Y$ given $\mathbf{Z}$.
    pub fn call(&self, x: usize, y: usize, z: &[usize]) -> f64 {
        // Get size of the sub-covariance matrix.
        let n = 2 + z.len();
        // Allocate the sub-covariance matrix.
        let mut sigma = Array2::<f64>::zeros((n, n));

        // Get the space of indices.
        let idx = [&[x, y], z].concat();
        let idx = idx.into_iter().enumerate();
        let idx = idx.clone().cartesian_product(idx);

        // Fill the sub-covariance matrix.
        idx.for_each(|((i, a), (j, b))| sigma[[i, j]] = self.sigma[[a, b]]);

        // Construct the covariance matrix.
        let sigma = CovarianceMatrix::new(sigma);

        // Compute the precision matrix.
        let omega = PrecisionMatrix::from(sigma);

        // Compute the partial correlation of X and Y given Z.
        -omega[[0, 1]] / f64::sqrt(omega[[0, 0]] * omega[[1, 1]])
    }
}

impl From<CovarianceMatrix> for PartialCorrelation {
    #[inline]
    fn from(sigma: CovarianceMatrix) -> Self {
        Self { sigma }
    }
}
