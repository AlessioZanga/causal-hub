use ndarray::prelude::*;
use rayon::prelude::*;

use super::LogLikelihood;
use crate::{
    data::{ConditionalCountMatrix, DiscreteDataMatrix, MarginalCountMatrix},
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    utils::{axis_chunks_size, nan_to_zero},
};

impl<'a, const PARALLEL: bool> LogLikelihood<'a, DiscreteDataMatrix, PARALLEL> {
    #[inline]
    pub(crate) fn marginal_eval(n_i: ArrayView1<usize>) -> f64 {
        // Sum over states and cast to floating point.
        let n = n_i.sum() as f64;
        let n_i = n_i.mapv(|i| i as f64);

        // Compute log-likelihood as n_i * ln(n_i  / n).
        (&n_i * (&n_i / n).mapv(f64::ln))
            // Map NaNs to zero.
            .mapv(nan_to_zero)
            // Sum each term.
            .sum()
    }

    /// Computes marginal log-likelihood given data set $\mathbf{D}$ and vertex $X$.
    #[inline]
    pub fn marginal(&self, x: usize) -> f64 {
        // Compute marginal contingency table.
        let n_i = MarginalCountMatrix::new(self.d, x);

        // Compute the log likelihood.
        Self::marginal_eval(n_i.values().view())
    }

    #[inline]
    pub(crate) fn conditional_eval(n_ij: ArrayView2<usize>) -> f64 {
        // Sum over states and cast to floating point.
        let n_j = n_ij
            .sum_axis(Axis(1))
            .insert_axis(Axis(1))
            .mapv(|j| j as f64);
        let n_ij = n_ij.mapv(|i| i as f64);

        // Compute log-likelihood as n_ij * ln(n_ij  / n_i).
        (&n_ij * (&n_ij / n_j).mapv(f64::ln))
            // Map NaNs to zero.
            .mapv(nan_to_zero)
            // Sum each term.
            .sum()
    }

    /// Computes conditional log-likelihood given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn conditional(&self, x: usize, z: &[usize]) -> f64 {
        // Compute marginal contingency table.
        let n_ij = ConditionalCountMatrix::<PARALLEL>::new(self.d, x, z);

        // Iterate over chunks.
        let n_ij = n_ij
            .values()
            .axis_chunks_iter(Axis(0), axis_chunks_size(n_ij.values()));

        // Check if parallelization is enabled.
        match PARALLEL {
            // Map each chunk and sum over in parallel.
            true => n_ij.into_par_iter().map(Self::conditional_eval).sum(),
            // Map each chunk and sum over.
            false => n_ij.map(Self::conditional_eval).sum(),
        }
    }
}

impl<'a, G, const PARALLEL: bool> DecomposableScoringCriterion<DiscreteDataMatrix, G>
    for LogLikelihood<'a, DiscreteDataMatrix, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        match z.is_empty() {
            true => self.marginal(x),
            false => self.conditional(x, z),
        }
    }
}
