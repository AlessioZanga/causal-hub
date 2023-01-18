use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    data::{ConditionalCountMatrix, DiscreteDataMatrix, MarginalCountMatrix},
    discovery::{score_types, DecomposableScoringCriterion, ScoringCriterion},
    graphs::{directions, DirectedGraph},
    utils::axis_chunks_size,
    Pa, V,
};

/// Marginal log-likelihood functor.
#[derive(Clone, Debug, Default)]
pub struct MarginalLogLikelihood {}

impl MarginalLogLikelihood {
    /// Constructor for marginal log-likelihood functor.
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }

    #[inline]
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
    #[inline]
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize) -> f64 {
        // Compute marginal contingency table.
        let n_i = MarginalCountMatrix::new(d, x);

        // Compute the log likelihood.
        Self::eval(n_i.view())
    }
}

/// Conditional log-likelihood functor.
///
/// # Generics
///
/// - `PARALLEL_CCM`: Enables parallel computation of conditional count matrix.
/// - `PARALLEL_CCL`: Enables parallel computation of conditional log-likelihood.
///
#[derive(Clone, Debug, Default)]
pub struct ConditionalLogLikelihood<const PARALLEL_CCM: bool, const PARALLEL_CLL: bool> {}

impl<const PARALLEL_CCM: bool, const PARALLEL_CLL: bool>
    ConditionalLogLikelihood<PARALLEL_CCM, PARALLEL_CLL>
{
    /// Constructor for conditional log-likelihood functor.
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }

    #[inline]
    pub(crate) fn eval(n_ij: ArrayView2<usize>) -> f64 {
        // Sum over levels and cast to floating point.
        let n_j = n_ij
            .sum_axis(Axis(1))
            .insert_axis(Axis(1))
            .mapv(|j| j as f64);
        let n_ij = n_ij.mapv(|i| i as f64);

        // Compute log-likelihood as n_ij * ln(n_ij  / n_i).
        let ll = &n_ij * (&n_ij / n_j).mapv(f64::ln);

        // Map NaNs to zero.
        let ll = ll.mapv(|i| f64::min(i, 0.));

        ll.sum()
    }

    /// Computes conditional log-likelihood given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> f64 {
        // Compute marginal contingency table.
        let n_ij = ConditionalCountMatrix::<PARALLEL_CCM>::new(d, x, z);

        // Iterate over chunks.
        let n_ij = n_ij.axis_chunks_iter(Axis(0), axis_chunks_size(&n_ij));

        // Check if parallelization is enabled.
        match PARALLEL_CLL {
            // Map each chunk and sum over in parallel.
            true => n_ij.into_par_iter().map(Self::eval).sum(),
            // Map each chunk and sum over.
            false => n_ij.map(Self::eval).sum(),
        }
    }
}

/// Log-Likelihood (LL) functor.
///
/// # Generics
///
/// - `PARALLEL_CCM`: Enables parallel computation of conditional count matrix.
/// - `PARALLEL_CCL`: Enables parallel computation of conditional log-likelihood.
///
#[derive(Clone, Debug, Default)]
pub struct LogLikelihood<const PARALLEL_CCM: bool, const PARALLEL_CLL: bool> {}

impl<const PARALLEL_CCM: bool, const PARALLEL_CLL: bool> LogLikelihood<PARALLEL_CCM, PARALLEL_CLL> {
    /// Constructor for LL functor.
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }

    /// Computes LL given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> f64 {
        match z.is_empty() {
            true => MarginalLogLikelihood::new().call(d, x),
            false => ConditionalLogLikelihood::<PARALLEL_CCM, PARALLEL_CLL>::new().call(d, x, z),
        }
    }
}

impl<G, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool> ScoringCriterion<DiscreteDataMatrix, G>
    for LogLikelihood<PARALLEL_CCM, PARALLEL_CLL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    type ScoreType = score_types::Decomposable;

    #[inline]
    fn call(&self, d: &DiscreteDataMatrix, g: &G) -> f64 {
        V!(g)
            .map(|x| (x, Pa!(g, x).collect::<Vec<_>>()))
            .map(|(x, z)| self.call(d, x, &z))
            .sum()
    }
}

impl<G, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool>
    DecomposableScoringCriterion<DiscreteDataMatrix, G>
    for LogLikelihood<PARALLEL_CCM, PARALLEL_CLL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> f64 {
        self.call(d, x, z)
    }
}

/// Alias for single-thread LL functor.
pub type LL = LogLikelihood<false, false>;
