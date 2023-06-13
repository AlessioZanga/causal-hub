use crate::{
    data::{ContinuousDataMatrix, DataSet, DiscreteDataMatrix},
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

/// Bayesian Information Criterion (BIC) functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct BayesianInformationCriterion<'a, D, const PARALLEL: bool> {
    log_likelihood: LogLikelihood<'a, D, PARALLEL>,
}

impl<'a, D, const PARALLEL: bool> BayesianInformationCriterion<'a, D, PARALLEL> {
    /// Constructor for BIC functor.
    #[inline]
    pub const fn new(d: &'a D) -> Self {
        // Initialize the log-likelihood functor.
        let log_likelihood = LogLikelihood::new(d);

        Self { log_likelihood }
    }
}

/* Implement BIC for discrete data. */
impl<'a, G, const PARALLEL: bool> DecomposableScoringCriterion<DiscreteDataMatrix, G>
    for BayesianInformationCriterion<'a, DiscreteDataMatrix, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the sample size.
        let n = self.log_likelihood.d.values().nrows() as f64;
        // Get the cardinality.
        let cards = self.log_likelihood.d.cardinality();
        // Get the cardinality of vertices.
        // NOTE: If Z is empty, then the product of an empty vector is still one.
        let (card_x, card_z) = (cards[x], z.iter().map(|&z| cards[z]).product::<usize>());
        // Compute the number of parameters.
        let theta = ((card_x - 1) * card_z) as f64;

        // Compute the BIC.
        log_likelihood - 0.5 * theta * f64::ln(n)
    }

    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        // Get the sample size.
        let n = self.log_likelihood.d.values().nrows() as f64;

        // Compute the maximum number of parents given the sample size.
        let n = f64::ceil(1. + f64::log2(n) - f64::log2(f64::ln(n)));

        Some(n as usize)
    }
}

/* Implement BIC for Gaussian data. */
impl<'a, G, const PARALLEL: bool> DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for BayesianInformationCriterion<'a, ContinuousDataMatrix, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the sample size.
        let n = self.log_likelihood.d.values().nrows() as f64;
        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Compute the BIC.
        log_likelihood - 0.5 * theta * f64::ln(n)
    }

    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        // Get the sample size.
        let n = self.log_likelihood.d.values().nrows() as f64;

        // Compute the maximum number of parents given the sample size.
        let n = f64::ceil(1. + f64::log2(n) - f64::log2(f64::ln(n)));

        Some(n as usize)
    }
}

/// Alias for single-thread BIC functor.
pub type BIC<'a, D> = BayesianInformationCriterion<'a, D, false>;
/// Alias for multi-thread BIC functor.
pub type ParallelBIC<'a, D> = BayesianInformationCriterion<'a, D, true>;
