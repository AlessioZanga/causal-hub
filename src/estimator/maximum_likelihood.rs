use ndarray::prelude::*;

use super::{CPDEstimator, CSSEstimator, SSE};
use crate::{
    data::{CategoricalData, Data},
    distribution::CategoricalCPD,
};

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Copy, Debug, Default)]
pub struct MaximumLikelihoodEstimator;

/// A type alias for a maximum likelihood estimator.
pub type MLE = MaximumLikelihoodEstimator;

impl MaximumLikelihoodEstimator {
    /// Creates a new maximum likelihood estimator.
    ///
    /// # Returns
    ///
    /// A new `MaximumLikelihoodEstimator` instance.
    ///
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl CPDEstimator<CategoricalData, CategoricalCPD> for MLE {
    fn fit(&self, data: &CategoricalData, x: usize, z: &[usize]) -> CategoricalCPD {
        // Get states and cardinality.
        let states = data.states();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new();
        // Compute sufficient statistics.
        let (n_xz, n_z, n) = sse.fit(data, x, z);

        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&x| x > 0.),
            "Failed to get non-zero counts for variable '{}'.",
            data.labels()[x]
        );

        // Compute the parameters by normalizing the counts.
        let parameters = &n_xz / n_z.insert_axis(Axis(1));

        // Set the sample size.
        let sample_size = Some(n);
        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = Some((n_xz * (&parameters + eps).mapv(f64::ln)).sum());

        // Subset the conditioning labels, states and cardinality.
        let conditioning_states = z.iter().map(|&i| states.get_index(i).unwrap());
        // Get the labels of the conditioned variables.
        let states = states.get_index(x).unwrap();

        CategoricalCPD::with_sample_size(
            states,
            conditioning_states,
            parameters,
            sample_size,
            sample_log_likelihood,
        )
    }
}
