use ndarray::prelude::*;

use super::{CPDEstimator, CSSEstimator, SSE};
use crate::{
    datasets::{CategoricalDataset, CategoricalTrj, Dataset},
    distributions::{CategoricalCIM, CategoricalCPD},
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

impl CPDEstimator<CategoricalDataset, CategoricalCPD> for MLE {
    fn fit(&self, dataset: &CategoricalDataset, x: usize, z: &[usize]) -> CategoricalCPD {
        // Get states and cardinality.
        let states = dataset.states();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new();
        // Compute sufficient statistics.
        let (n_xz, n_z, n) = sse.fit(dataset, x, z);

        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&x| x > 0.),
            "Failed to get non-zero counts for variable '{}'.",
            dataset.labels()[x]
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

impl CPDEstimator<CategoricalTrj, CategoricalCIM> for MLE {
    fn fit(&self, trj: &CategoricalTrj, x: usize, z: &[usize]) -> CategoricalCIM {
        // Get states and cardinality.
        let states = trj.states();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new();
        // Compute sufficient statistics.
        let (n_xz, t_xz, n) = sse.fit(trj, x, z);

        // Assert the conditional times counts are not zero.
        assert!(
            t_xz.iter().all(|&x| x > 0.),
            "Failed to get non-zero conditional times for variable '{}'.",
            trj.labels()[x]
        );

        // Align the dimensions of the counts and times.
        let t_xz = t_xz.insert_axis(Axis(2));
        // Estimate the parameters by normalizing the counts.
        let mut parameters = &n_xz / &t_xz;
        // Fix the diagonal.
        parameters.outer_iter_mut().for_each(|mut q| {
            // Fill the diagonal with zeros.
            q.diag_mut().fill(0.);
            // Compute the negative sum of the rows.
            let q_neg_sum = -q.sum_axis(Axis(1));
            // Assign the negative sum to the diagonal.
            q.diag_mut().assign(&q_neg_sum);
        });

        // Set the sample size.
        let sample_size = Some(n);

        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = Some({
            // Copy counts, times and parameters.
            let mut q_xz = parameters.clone();
            // Set diagonal to zero.
            q_xz.outer_iter_mut().for_each(|mut q| {
                // Fill the diagonal with zeros.
                q.diag_mut().fill(0.);
            });
            // Compute the sample log-likelihood as -t * q + n * ln(q + eps).
            (-t_xz * &q_xz).sum() + (n_xz * (q_xz + eps).mapv(f64::ln)).sum()
        });

        // Subset the conditioning labels, states and cardinality.
        let conditioning_states = z.iter().map(|&i| states.get_index(i).unwrap());
        // Get the labels of the conditioned variables.
        let states = states.get_index(x).unwrap();

        CategoricalCIM::with_sample_size(
            states,
            conditioning_states,
            parameters,
            sample_size,
            sample_log_likelihood,
        )
    }
}
