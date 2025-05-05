use ndarray::prelude::*;
use statrs::function::gamma::ln_gamma;

use super::{CPDEstimator, CSSEstimator, SSE};
use crate::{
    datasets::{CategoricalDataset, CategoricalTrj, CategoricalTrjs},
    distributions::{CategoricalCIM, CategoricalCPD},
};

/// A struct representing a Bayesian estimator.
#[derive(Clone, Copy, Debug)]
pub struct BayesianEstimator<'a, D, Pi> {
    dataset: &'a D,
    prior: Pi,
}

/// A type alias for a bayesian estimator.
pub type BE<'a, D, Pi> = BayesianEstimator<'a, D, Pi>;

impl<'a, D, Pi> BayesianEstimator<'a, D, Pi> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to the dataset to fit the estimator to.
    /// * `prior` - The prior distribution.
    ///
    /// # Returns
    ///
    /// A new `BayesianEstimator` instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D, prior: Pi) -> Self {
        Self { dataset, prior }
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
impl CPDEstimator<CategoricalCPD> for BE<'_, CategoricalDataset, usize> {
    fn fit(&self, x: usize, z: &[usize]) -> CategoricalCPD {
        // Get states and cardinality.
        let (states, cards) = (self.dataset.states(), self.dataset.cardinality());

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, n_z, n) = sse.fit(x, z);

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        let n_z = n_z.mapv(|x| x as f64);

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = *self.prior();
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");

        // Cast alpha to floating point.
        let alpha = alpha as f64;

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

impl CPDEstimator<CategoricalCIM> for BE<'_, CategoricalTrj, (usize, f64)> {
    fn fit(&self, x: usize, z: &[usize]) -> CategoricalCIM {
        // Get states and cardinality.
        let states = self.dataset.states();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, t_xz, n) = sse.fit(x, z);

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);

        // Get the prior, as the alpha of Dirichlet and tau of Gamma.
        let (alpha, tau) = *self.prior();
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");
        // Assert tau is positive.
        assert!(tau > 0.0, "Tau must be positive.");

        // Get the cardinality of the conditioning variables.
        let c_z = n_xz.shape()[0] as f64;
        // Scale the prior by the cardinality.
        let alpha = alpha as f64 / c_z;
        let tau = tau / c_z;

        // Align the dimensions of the counts and times.
        let t_xz = t_xz.insert_axis(Axis(2));
        // Estimate the parameters by normalizing the counts.
        let mut parameters = (&n_xz + alpha) / (&t_xz + tau);
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

        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some({
            // Compute the sample log-likelihood as the sum of:
            //
            //   1. ln(tau) * (alpha + 1) - ln_gamma(alpha + 1)
            //   2. + ln_gamma(n_xz + alpha + 1) - ln(t_xz + tau) * (n_xz + alpha + 1)
            //
            let mut sll = f64::ln(tau) * (alpha + 1.) - ln_gamma(alpha + 1.);
            sll += (n_xz + alpha + 1.).mapv(ln_gamma).sum() - (t_xz + tau).mapv(ln_gamma).sum();

            sll
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

// TODO: Avoid code duplication with the above implementation.
impl CPDEstimator<CategoricalCIM> for BE<'_, CategoricalTrjs, (usize, f64)> {
    fn fit(&self, x: usize, z: &[usize]) -> CategoricalCIM {
        // Get states and cardinality.
        let states = self.dataset.states();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, t_xz, n) = sse.fit(x, z);

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);

        // Get the prior, as the alpha of Dirichlet and tau of Gamma.
        let (alpha, tau) = *self.prior();
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");
        // Assert tau is positive.
        assert!(tau > 0.0, "Tau must be positive.");

        // Get the cardinality of the conditioning variables.
        let c_z = n_xz.shape()[0] as f64;
        // Scale the prior by the cardinality.
        let alpha = alpha as f64 / c_z;
        let tau = tau / c_z;

        // Align the dimensions of the counts and times.
        let t_xz = t_xz.insert_axis(Axis(2));
        // Estimate the parameters by normalizing the counts.
        let mut parameters = (&n_xz + alpha) / (&t_xz + tau);
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

        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some({
            // Compute the sample log-likelihood as the sum of:
            //
            //   1. ln(tau) * (alpha + 1) - ln_gamma(alpha + 1)
            //   2. + ln_gamma(n_xz + alpha + 1) - ln(t_xz + tau) * (n_xz + alpha + 1)
            //
            let mut sll = f64::ln(tau) * (alpha + 1.) - ln_gamma(alpha + 1.);
            sll += (n_xz + alpha + 1.).mapv(ln_gamma).sum() - (t_xz + tau).mapv(ln_gamma).sum();

            sll
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
