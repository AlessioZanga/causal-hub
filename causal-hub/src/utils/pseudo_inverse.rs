use ndarray::prelude::*;
use ndarray_linalg::SVD;
use ndarray_stats::QuantileExt;

use crate::types::EPSILON;

/// Moore-Penrose pseudo-inverse.
pub trait PseudoInverse {
    /// Computes the Moore-Penrose pseudo-inverse of a matrix.
    ///
    /// # Panics
    ///
    /// * Panics if the SVD computation fails.
    ///
    /// # Returns
    ///
    /// The pseudo-inverse of the matrix.
    ///
    fn pinv(&self) -> Self;
}

impl PseudoInverse for Array2<f64> {
    fn pinv(&self) -> Self {
        // Step 1: Compute the Single Value Decomposition (SVD).
        let (u, s, vt) = self.svd(true, true).expect("Failed to compute the SVD.");
        let u = u.expect("Failed to get U from the SVD.");
        let vt = vt.expect("Failed to get VT from the SVD.");
        // Step 2: Compute the pseudo-inverse of the singular values.
        let smax = s.max().unwrap_or(&0.);
        let rtol = f64::max(EPSILON, s.len() as f64 * smax * EPSILON);
        let sinv = Array2::from_diag(&s.mapv(|x| if x > rtol { 1. / x } else { 0. }));
        // Step 3: Compute the pseudo-inverse of S_zz.
        vt.t().dot(&sinv).dot(&u.t())
    }
}
