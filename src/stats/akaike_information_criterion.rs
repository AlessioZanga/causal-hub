use crate::{
    data::{CategoricalDataMatrix, GaussianDataMatrix},
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    prelude::ZINBDataMatrix,
    stats::LogLikelihood,
};

/// Akaike Information Criterion (AIC) functor.
///
/// $AIC = LL - |\theta|$
///
#[derive(Clone, Debug)]
pub struct AkaikeInformationCriterion<'a, D> {
    log_likelihood: LogLikelihood<'a, D>,
}

impl<'a, D> AkaikeInformationCriterion<'a, D> {
    /// Constructor for AIC functor.
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        // Initialize the log-likelihood functor.
        let log_likelihood = LogLikelihood::new(data_set);

        Self { log_likelihood }
    }
}

/* Implement AIC for categorical data. */
impl<'a, G> DecomposableScoringCriterion<CategoricalDataMatrix, G>
    for AkaikeInformationCriterion<'a, CategoricalDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the cardinality.
        let cards = self.log_likelihood.data_set.cardinality();
        // Get the cardinality of vertices.
        // NOTE: If Z is empty, then the product of an empty vector is still one.
        let (card_x, card_z) = (
            cards[x] as usize,
            z.iter().map(|&z| cards[z] as usize).product::<usize>(),
        );
        // Compute the number of parameters.
        let theta = ((card_x - 1) * card_z) as f64;

        // Compute the AIC.
        log_likelihood - theta
    }
}

/* Implement AIC for Gaussian data. */
impl<'a, G> DecomposableScoringCriterion<GaussianDataMatrix, G>
    for AkaikeInformationCriterion<'a, GaussianDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Compute the AIC.
        log_likelihood - theta
    }
}

/* Implement AIC for ZINB data. */
impl<'a, G> DecomposableScoringCriterion<ZINBDataMatrix, G>
    for AkaikeInformationCriterion<'a, ZINBDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 * z.len() + 3) as f64;

        // Compute the AIC.
        log_likelihood - theta
    }
}

/// Alias for the AkaikeInformationCriterion functor.
pub type AIC<'a, D> = AkaikeInformationCriterion<'a, D>;
