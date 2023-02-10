use std::fmt::Debug;

use crate::models::Independence;

/// Conditional Independence Test (CIT) trait.
pub trait ConditionalIndependenceTest: Clone + Debug + Sync {
    /// Compute (degree-of-freedom, statistic, p-value) of $X \mathrlap{\thinspace\perp}{\perp} \thinspace Y \mid \mathbf{Z}$.
    fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64);

    /// Check whether $X \mathrlap{\thinspace\perp}{\perp}_{\mathcal{P}} \thinspace Y \mid \mathbf{Z}$ holds or not.
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool;

    /// Set significance level $\alpha$.
    fn with_significance_level(self, alpha: f64) -> Self;
}

impl<T> Independence for T
where
    T: ConditionalIndependenceTest,
{
    #[inline]
    fn is_independent(&self, x: usize, y: usize, z: &[usize]) -> bool {
        <Self as ConditionalIndependenceTest>::call(self, x, y, z)
    }
}