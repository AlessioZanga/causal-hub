use crate::{
    data::{CategoricalDataMatrix, DataSet, GaussianDataMatrix, ZINBDataMatrix},
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

/// Bayesian Information Criterion Corrected (BICC) functor.
///
/// $BICC = LL - \frac{1}{2} \frac{n|\theta|}{n - |\theta| - 2} \log(n)$
///
#[derive(Clone, Debug)]
pub struct BayesianInformationCriterionCorrected<'a, D> {
    log_likelihood: LogLikelihood<'a, D>,
}

impl<'a, D> BayesianInformationCriterionCorrected<'a, D> {
    /// Constructor for BIC functor.
    #[inline]
    pub const fn new(d: &'a D) -> Self {
        // Initialize the log-likelihood functor.
        let log_likelihood = LogLikelihood::new(d);

        Self { log_likelihood }
    }
}

/* Implement BIC for categorical data_set. */
impl<'a, G> DecomposableScoringCriterion<CategoricalDataMatrix, G>
    for BayesianInformationCriterionCorrected<'a, CategoricalDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the sample size.
        let n = self.log_likelihood.data_set.sample_size() as f64;
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

        // Compute the BICC, enforcing positivity of the denominator.
        log_likelihood - 0.5 * ((n * theta) / f64::max(n - theta - 2., 1.)) * f64::ln(n)
    }
}

/* Implement BIC for Gaussian data_set. */
impl<'a, G> DecomposableScoringCriterion<GaussianDataMatrix, G>
    for BayesianInformationCriterionCorrected<'a, GaussianDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the sample size.
        let n = self.log_likelihood.data_set.sample_size() as f64;
        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Compute the BICC, enforcing positivity of the denominator.
        log_likelihood - 0.5 * ((n * theta) / f64::max(n - theta - 2., 1.)) * f64::ln(n)
    }
}

/* Implement BIC for ZINB data_set. */
impl<'a, G> DecomposableScoringCriterion<ZINBDataMatrix, G>
    for BayesianInformationCriterionCorrected<'a, ZINBDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the sample size.
        let n = self.log_likelihood.data_set.sample_size() as f64;
        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 * z.len() + 3) as f64;

        // Compute the BICC, enforcing positivity of the denominator.
        log_likelihood - 0.5 * ((n * theta) / f64::max(n - theta - 2., 1.)) * f64::ln(n)
    }
}

/// Alias for the BayesianInformationCriterionCorrected functor.
pub type BICC<'a, D> = BayesianInformationCriterionCorrected<'a, D>;
