use crate::{
    data::{CategoricalDataSet, DataSet, GaussianDataSet, ZINBDataSet},
    discovery::DecomposableScoringCriterion,
    graphs::{Directed, DirectedGraph},
    stats::LogLikelihood,
};

/// Bayesian Information Criterion (BIC) functor.
///
/// $BIC = LL - \frac{1}{2} |\theta| \log(n)$
///
#[derive(Clone, Debug)]
pub struct BayesianInformationCriterion<'a, D> {
    log_likelihood: LogLikelihood<'a, D>,
}

impl<'a, D> BayesianInformationCriterion<'a, D> {
    #[inline]
    pub const fn new(d: &'a D) -> Self {
        // Initialize the log-likelihood functor.
        let log_likelihood = LogLikelihood::new(d);

        Self { log_likelihood }
    }
}

/* Implement BIC for categorical data_set. */
impl<'a, G> DecomposableScoringCriterion<CategoricalDataSet, G>
    for BayesianInformationCriterion<'a, CategoricalDataSet>
where
    G: DirectedGraph<Direction = Directed>,
{
    type LabelsIter<'b> = <CategoricalDataSet as DataSet>::LabelsIter<'b> where Self: 'b;

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

        // Compute the BIC.
        log_likelihood - 0.5 * theta * f64::ln(n)
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.log_likelihood.data_set.labels_iter()
    }

    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        // Get the sample size.
        let n = self.log_likelihood.data_set.sample_size() as f64;

        // Compute the maximum number of parents given the sample size.
        let n = f64::ceil(1. + f64::log2(n) - f64::log2(f64::ln(n)));

        Some(n as usize)
    }
}

/* Implement BIC for Gaussian data_set. */
impl<'a, G> DecomposableScoringCriterion<GaussianDataSet, G>
    for BayesianInformationCriterion<'a, GaussianDataSet>
where
    G: DirectedGraph<Direction = Directed>,
{
    type LabelsIter<'b> = <GaussianDataSet as DataSet>::LabelsIter<'b> where Self: 'b;

    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the sample size.
        let n = self.log_likelihood.data_set.sample_size() as f64;
        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Compute the BIC.
        log_likelihood - 0.5 * theta * f64::ln(n)
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.log_likelihood.data_set.labels_iter()
    }

    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        // Get the sample size.
        let n = self.log_likelihood.data_set.sample_size() as f64;

        // Compute the maximum number of parents given the sample size.
        let n = f64::ceil(1. + f64::log2(n) - f64::log2(f64::ln(n)));

        Some(n as usize)
    }
}

/* Implement BIC for ZINB data_set. */
impl<'a, G> DecomposableScoringCriterion<ZINBDataSet, G>
    for BayesianInformationCriterion<'a, ZINBDataSet>
where
    G: DirectedGraph<Direction = Directed>,
{
    type LabelsIter<'b> = <ZINBDataSet as DataSet>::LabelsIter<'b> where Self: 'b;

    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the log-likelihood.
        let log_likelihood = DecomposableScoringCriterion::<_, G>::call(&self.log_likelihood, x, z);

        // Get the sample size.
        let n = self.log_likelihood.data_set.sample_size() as f64;
        // Compute the number of parameters as intercept, standard deviation
        // and each regression coefficient per parent.
        let theta = (2 * z.len() + 3) as f64;

        // Compute the BIC.
        log_likelihood - 0.5 * theta * f64::ln(n)
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.log_likelihood.data_set.labels_iter()
    }
}

pub type BIC<'a, D> = BayesianInformationCriterion<'a, D>;
