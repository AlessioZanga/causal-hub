use ndarray::prelude::*;

use super::{CPDEstimator, CSSEstimator, SSE};
use crate::{dataset::CategoricalDataset, distribution::CategoricalCPD};

/// A struct representing a Bayesian estimator.
#[derive(Clone, Copy, Debug, Default)]
pub struct BayesianEstimator<Pi> {
    prior: Pi,
}

/// A type alias for a bayesian estimator.
pub type BE<Pi> = BayesianEstimator<Pi>;

impl<Pi> BayesianEstimator<Pi> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `prior` - The prior distribution.
    ///
    /// # Returns
    ///
    /// A new `BayesianEstimator` instance.
    ///
    #[inline]
    pub const fn new(prior: Pi) -> Self {
        Self { prior }
    }

    /// Returns the prior distribution.
    ///
    /// # Returns
    ///
    /// A reference to the prior.
    ///
    #[inline]
    pub const fn prior(&self) -> &Pi {
        &self.prior
    }
}

// NOTE: The prior is expressed as a scalar, which is the alpha for the Dirichlet distribution.
impl CPDEstimator<CategoricalDataset, CategoricalCPD> for BE<f64> {
    fn fit(&self, dataset: &CategoricalDataset, x: usize, z: &[usize]) -> CategoricalCPD {
        // Get states and cardinality.
        let (states, cards) = (dataset.states(), dataset.cardinality());

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new();
        // Compute sufficient statistics.
        let (n_xz, n_z, n) = sse.fit(dataset, x, z);

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = *self.prior();
        // Compute the parameters by normalizing the counts with the prior.
        let parameters = (&n_xz + alpha) / (n_z.insert_axis(Axis(1)) + alpha * cards[x] as f64);

        // Set the sample size.
        let sample_size = Some(n);
        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some((n_xz * parameters.mapv(f64::ln)).sum());

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
