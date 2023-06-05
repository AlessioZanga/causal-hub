use crate::{
    data::{ContinuousDataMatrix, DiscreteDataMatrix},
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

/// Akaike Information Criterion (AIC) functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct AkaikeInformationCriterion<'a, D, const PARALLEL: bool> {
    ll: LogLikelihood<'a, D, PARALLEL>,
}

impl<'a, D, const PARALLEL: bool> AkaikeInformationCriterion<'a, D, PARALLEL> {
    /// Constructor for AIC functor.
    #[inline]
    pub const fn new(d: &'a D) -> Self {
        // Initialize the log-likelihood functor.
        let ll = LogLikelihood::new(d);

        Self { ll }
    }
}

/* Implement AIC for discrete data. */
impl<'a, G, const PARALLEL: bool> DecomposableScoringCriterion<DiscreteDataMatrix, G>
    for AkaikeInformationCriterion<'a, DiscreteDataMatrix, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let ll = DecomposableScoringCriterion::<_, G>::call(&self.ll, x, z);

        // Get the cardinality.
        let cards = self.ll.d.cardinality();
        // Get the cardinality of vertices.
        // NOTE: If Z is empty, then the product of an empty vector is still one.
        let (card_x, card_z) = (cards[x], z.iter().map(|&z| cards[z]).product::<usize>());
        // Compute the number of parameters.
        let theta = ((card_x - 1) * card_z) as f64;

        // Compute the AIC.
        ll - theta
    }
}

/* Implement AIC for Gaussian data. */
impl<'a, G, const PARALLEL: bool> DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for AkaikeInformationCriterion<'a, ContinuousDataMatrix, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let ll = DecomposableScoringCriterion::<_, G>::call(&self.ll, x, z);

        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Compute the AIC.
        ll - theta
    }
}

/// Alias for single-thread AIC functor.
pub type AIC<'a, D> = AkaikeInformationCriterion<'a, D, false>;
/// Alias for multi-thread AIC functor.
pub type ParallelAIC<'a, D> = AkaikeInformationCriterion<'a, D, true>;
