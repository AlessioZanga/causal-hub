use std::{fmt::Debug, iter::FusedIterator};

use crate::models::Independence;

/// Conditional Independence Test (CIT) trait.
pub trait ConditionalIndependenceTest<'a>: Clone + Debug + Sync {
    /// Labels iterator type.
    type LabelsIter<'b>: Iterator<Item = &'b str> + ExactSizeIterator + FusedIterator
    where
        Self: 'b;

    /// Compute (degree-of-freedom, statistic, p-value) of $X \mathrlap{\thinspace\perp}{\perp} \thinspace Y \mid \mathbf{Z}$.
    fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64);

    /// Returns `true` whether $H_0: X \mathrlap{\thinspace\perp}{\perp}_{\mathcal{P}} \thinspace Y \mid \mathbf{Z}$ is not rejected.
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool;

    /// Set significance level $\alpha$.
    ///
    /// # Panics
    ///
    /// If $\alpha$ is not in the (0, 1) interval.
    ///
    fn with_significance_level(self, alpha: f64) -> Self;

    /// Returns data labels
    fn labels(&self) -> Self::LabelsIter<'_>;
}

impl<'a, T> Independence for T
where
    T: ConditionalIndependenceTest<'a>,
{
    #[inline]
    fn is_independent(&self, x: usize, y: usize, z: &[usize]) -> bool {
        <Self as ConditionalIndependenceTest>::call(self, x, y, z)
    }
}
