use crate::{
    data::DiscreteDataMatrix,
    discovery::{score_types, DecomposableScoringCriterion, ScoringCriterion},
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
    Pa, V,
};

/// Bayesian Information Criterion (BIC) functor.
///
/// # Generics
///
/// - `RESCALED`: Rescale by -2, allowing for direct comparison with log-likelihood.
/// - `PARALLEL_CCM`: Enables parallel computation of conditional count matrix.
/// - `PARALLEL_CCL`: Enables parallel computation of conditional log-likelihood.
///
#[derive(Clone, Debug)]
pub struct BayesianInformationCriterion<
    const RESCALED: bool,
    const PARALLEL_CCM: bool,
    const PARALLEL_CLL: bool,
> {
    k: f64,
}

impl<const RESCALED: bool, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool> Default
    for BayesianInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const RESCALED: bool, const PARALLEL_CCM: bool, const PARALLEL_CLL: bool>
    BayesianInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
{
    /// Constructor for BIC functor.
    pub fn new() -> Self {
        Self { k: 1. }
    }

    /// Computes BIC given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    pub fn call(&self, d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> f64 {
        // Get the sample size.
        let n = d.nrows() as f64;

        // Get the cardinality.
        let cards = d.cardinality();
        // Get the cardinality of vertices.
        // NOTE: If Z is empty, then the product of an empty vector is still one.
        let (card_x, card_z) = (cards[x], z.iter().map(|&z| cards[z]).product::<usize>());
        // Compute the number of parameters.
        let theta = ((card_x - 1) * card_z) as f64;

        // Initialize the log-likelihood functor.
        let ll = LogLikelihood::<PARALLEL_CCM, PARALLEL_CLL>::new();
        // Compute the log-likelihood.
        let ll = ll.call(d, x, z);

        // Check if BIC must be scaled.
        match RESCALED {
            // Rescale BIC by -2, coherently with LL.
            true => ll - 0.5 * self.k * theta * f64::ln(n),
            // Otherwise, compute original BIC.
            false => self.k * theta * f64::ln(n) - 2. * ll,
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
    for BayesianInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
where
    G: DirectedGraph<Direction = directions::Directed>,
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
    for BayesianInformationCriterion<RESCALED, PARALLEL_CCM, PARALLEL_CLL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    fn call(&self, d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> f64 {
        self.call(d, x, z)
    }
}

/// Alias for (rescaled) single-thread BIC functor.
pub type BIC = BayesianInformationCriterion<true, false, false>;
