use ndarray::prelude::*;

use crate::{
    data::DiscreteDataMatrix,
    discovery::{score_types, DecomposableScoringCriterion, ScoringCriterion},
    graphs::{directions, BaseGraph, DirectedGraph},
    stats::LogLikelihood,
    Pa, V,
};

/// Akaike Information Criterion (AIC) functor.
///
/// # Generics
///
/// - `RESCALED`: Rescale by -2, allowing for direct comparison with log-likelihood.
/// - `PARALLEL_CCM`: Enables parallel computation of conditional count matrix.
/// - `PARALLEL_CCL`: Enables parallel computation of conditional log-likelihood.
///
#[derive(Clone, Debug)]
pub struct AkaikeInformationCriterion<
    const RESCALED: bool,
    const PARALLEL_CCM: bool,
    const PARALLEL_CLL: bool,
> {
    k: f64,
}

impl<const RESCALED: bool, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool> Default
    for AkaikeInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const RESCALED: bool, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool>
    AkaikeInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
{
    /// Constructor for AIC functor.
    pub fn new() -> Self {
        Self { k: 1. }
    }

    /// Computes AIC given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> f64 {
        // Get the cardinality.
        let cards = d.cardinality();
        // Get the cardinality of vertices.
        // NOTE: If Z is empty, then the product of an empty vector is still one.
        let (card_x, card_z) = (cards[x], cards.select(Axis(0), z).product());
        // Compute the number of parameters.
        let theta = ((card_x - 1) * card_z) as f64;

        // Initialize the log-likelihood functor.
        let ll = LogLikelihood::<PARALLEL_CCM, PARALLEL_CLL>::new();
        // Compute the log-likelihood.
        let ll = ll.call(d, x, z);

        // Check if AIC must be scaled.
        match RESCALED {
            // Rescale AIC by -2, coherently with LL.
            true => ll - self.k * theta,
            // Otherwise, compute original AIC.
            false => 2. * self.k * theta - 2. * ll,
        }
    }

    /// Sets penalty coefficient.
    pub fn with_penalty_coeff(mut self, k: f64) -> Self {
        // Set penalty coefficient.
        self.k = k;

        self
    }
}

impl<G, const RESCALED: bool, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool>
    ScoringCriterion<DiscreteDataMatrix, G>
    for AkaikeInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
where
    G: BaseGraph<Direction = directions::Directed> + DirectedGraph,
{
    type ScoreType = score_types::Decomposable;

    fn call(&self, d: &DiscreteDataMatrix, g: &G) -> f64 {
        V!(g)
            .map(|x| (x, Pa!(g, x).collect::<Vec<_>>()))
            .map(|(x, z)| self.call(d, x, &z))
            .sum()
    }
}

impl<G, const RESCALED: bool, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool>
    DecomposableScoringCriterion<DiscreteDataMatrix, G>
    for AkaikeInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
where
    G: BaseGraph<Direction = directions::Directed> + DirectedGraph,
{
    fn call(&self, d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> f64 {
        self.call(d, x, z)
    }
}

/// Alias for (rescaled) single-thread AIC functor.
pub type AIC = AkaikeInformationCriterion<true, false, false>;
