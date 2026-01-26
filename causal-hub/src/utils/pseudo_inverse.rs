use ndarray::prelude::*;
use ndarray_linalg::SVD;
use ndarray_stats::QuantileExt;

use crate::types::{EPSILON, Error, Result};

/// Moore-Penrose pseudo-inverse.
pub trait PseudoInverse {
    /// Computes the Moore-Penrose pseudo-inverse of a matrix.
    ///
    /// # Returns
    ///
    /// The pseudo-inverse of the matrix.
    ///
    fn pinv(&self) -> Result<Self>
    where
        Self: Sized;
}

impl PseudoInverse for Array2<f64> {
    fn pinv(&self) -> Result<Self> {
        // Step 0: Scale the matrix to improve numerical stability.
        let a = *self.abs().max().unwrap_or(&1.);
        let m = self / a;
        // Step 1: Compute the Single Value Decomposition (SVD).
        let (u, s, vt) = m
            .svd(true, true)
            .map_err(|e| Error::Linalg(format!("Failed to compute SVD: {e}")))?;
        let u = u.ok_or_else(|| Error::Linalg("Failed to get U from the SVD.".to_string()))?;
        let vt = vt.ok_or_else(|| Error::Linalg("Failed to get VT from the SVD.".to_string()))?;
        // Step 2: Compute the pseudo-inverse of the singular values.
        let s_max = s.max().unwrap_or(&0.);
        let r_tol = f64::max(EPSILON, s.len() as f64 * s_max * EPSILON);
        let s_inv = Array2::from_diag(&s.mapv(|x| if x > r_tol { 1. / x } else { 0. }));
        // Step 3: Compute the pseudo-inverse of S_zz.
        Ok(vt.t().dot(&s_inv).dot(&u.t()) / a)
    }
}
