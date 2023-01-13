use ndarray::prelude::*;

use crate::{
    data::{ConditionalCountMatrix, DiscreteDataMatrix, MarginalCountMatrix},
    discovery::{score_types, DecomposableScoringCriterion, ScoringCriterion},
    graphs::{directions, BaseGraph, DirectedGraph},
    utils::axis_chunks_size,
    Pa, V,
};

/// Marginal log-likelihood functor.
pub struct MarginalLogLikelihood {}

impl MarginalLogLikelihood {
    /// Constructor for marginal log-likelihood functor.
    pub const fn new() -> Self {
        Self {}
    }

    pub(crate) fn eval(n_i: ArrayView1<usize>) -> f64 {
        // Sum over levels and cast to floating point.
        let n = n_i.sum() as f64;
        let n_i = n_i.mapv(|i| i as f64);

        // Compute log-likelihood as n_i * ln(n_i  / n).
        let ll = &n_i * (&n_i / n).mapv(f64::ln);

        // Map NaNs to zero.
        let ll = ll.mapv(|i| f64::min(i, 0.));

        ll.sum()
    }

    /// Computes marginal log-likelihood given data set $\mathbf{D}$ and vertex $X$.
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize) -> f64 {
        // Compute marginal contingency table.
        let n_i = MarginalCountMatrix::new(d, x);

        // Compute the log likelihood.
        Self::eval(n_i.view())
    }
}

/// Conditional log-likelihood functor.
pub struct ConditionalLogLikelihood {}

impl ConditionalLogLikelihood {
    /// Constructor for conditional log-likelihood functor.
    pub const fn new() -> Self {
        Self {}
    }

    pub(crate) fn eval(n_ij: ArrayView2<usize>) -> f64 {
        // Sum over levels and cast to floating point.
        let n_j = n_ij.sum_axis(Axis(1)).insert_axis(Axis(1)).mapv(|j| j as f64);
        let n_ij = n_ij.mapv(|i| i as f64);

        // Compute log-likelihood as n_ij * ln(n_ij  / n_i).
        let ll = &n_ij * (&n_ij / n_j).mapv(f64::ln);

        // Map NaNs to zero.
        let ll = ll.mapv(|i| f64::min(i, 0.));

        ll.sum()
    }

    /// Computes conditional log-likelihood given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize, z: Vec<usize>) -> f64 {
        // Compute marginal contingency table.
        let n_ij = ConditionalCountMatrix::new(d, x, z);

        // Iterate over chunks.
        n_ij.axis_chunks_iter(Axis(0), axis_chunks_size(&n_ij))
            // Map each chunk.
            .map(Self::eval)
            // Sum over chunks.
            .sum()
    }
}

/// Constructor for log-likelihood functor.
pub struct LogLikelihood {}

impl LogLikelihood {
    /// Constructor for log-likelihood functor.
    pub const fn new() -> Self {
        Self {}
    }

    /// Computes log-likelihood given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize, z: Vec<usize>) -> f64 {
        match z.is_empty() {
            true => MarginalLogLikelihood::new().call(d, x),
            false => ConditionalLogLikelihood::new().call(d, x, z),
        }
    }
}

impl<G> ScoringCriterion<DiscreteDataMatrix, G> for LogLikelihood
where
    G: BaseGraph<Direction = directions::Directed> + DirectedGraph,
{
    type ScoreType = score_types::Decomposable;

    fn call(&self, d: &DiscreteDataMatrix, g: &G) -> f64 {
        V!(g).map(|x| self.call(d, x, Pa!(g, x).collect())).sum()
    }
}

impl<G> DecomposableScoringCriterion<DiscreteDataMatrix, G> for LogLikelihood
where
    G: BaseGraph<Direction = directions::Directed> + DirectedGraph,
{
    fn call(&self, d: &DiscreteDataMatrix, x: usize, z: Vec<usize>) -> f64 {
        self.call(d, x, z)
    }
}

/// Alias for log-likelihood functor.
pub type LL = LogLikelihood;
