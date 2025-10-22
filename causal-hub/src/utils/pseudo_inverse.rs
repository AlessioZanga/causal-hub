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
        // Step 0: Scale the matrix to improve numerical stability.
        let a = *self.abs().max().unwrap_or(&1.);
        let m = self / a;
        // Step 1: Compute the Single Value Decomposition (SVD).
        let (u, s, vt) = m.svd(true, true).unwrap_or_else(|e| {
            panic!(
                "Failed to compute the SVD \n\
                \t for the matrix: \n\
                \t {m:?} \n\
                \t with error: \n\
                \t {e:?}."
            )
        });
        let u = u.expect("Failed to get U from the SVD.");
        let vt = vt.expect("Failed to get VT from the SVD.");
        // Step 2: Compute the pseudo-inverse of the singular values.
        let s_max = s.max().unwrap_or(&0.);
        let r_tol = f64::max(EPSILON, s.len() as f64 * s_max * EPSILON);
        let s_inv = Array2::from_diag(&s.mapv(|x| if x > r_tol { 1. / x } else { 0. }));
        // Step 3: Compute the pseudo-inverse of S_zz.
        vt.t().dot(&s_inv).dot(&u.t()) / a
    }
}
