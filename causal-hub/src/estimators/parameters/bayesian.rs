use dry::macro_for;
use ndarray::prelude::*;
use statrs::function::gamma::ln_gamma;

use super::{CPDEstimator, CSSEstimator, ParCPDEstimator, ParCSSEstimator, SSE};
use crate::{
    datasets::{CatData, CatTrj, CatTrjs, CatWtdTrj, CatWtdTrjs},
    distributions::{CPD, CatCIM, CatCPD},
    types::{FxIndexMap, FxIndexSet},
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
impl CPDEstimator<CatCPD> for BE<'_, CatData, usize> {
    fn fit_transform(&self, x: usize, z: &[usize]) -> (<CatCPD as CPD>::SS, CatCPD) {
        // Get states and cardinality.
        let (states, cards) = (self.dataset.states(), self.dataset.cardinality());

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, n_z, n) = sse.fit(x, z);

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = *self.prior();
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");

        // Cast alpha to floating point.
        let alpha = alpha as f64;

        // Align the dimensions of the counts.
        let n_z = n_z.insert_axis(Axis(1));
        // Add the prior to the counts.
        let n_xz = n_xz + alpha;
        let n_z = n_z + alpha * cards[x] as f64;
        // Compute the parameters by normalizing the counts with the prior.
        let parameters = &n_xz / &n_z;

        // Set the sample size.
        let sample_size = Some(n);
        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some((&n_xz * parameters.ln()).sum());

        // Subset the conditioning labels, states and cardinality.
        let conditioning_states = z.iter().map(|&i| states.get_index(i).unwrap());
        // Get the labels of the conditioned variables.
        let states = states.get_index(x).unwrap();
        // Construct the CPD.
        let cpd_xz = CatCPD::with_sample_size(
            states,
            conditioning_states,
            parameters,
            sample_size,
            sample_log_likelihood,
        );

        // Remove the last axis of the counts.
        let n_z = n_z.remove_axis(Axis(1));

        // Return the sufficient statistics and the CPD.
        ((n_xz, n_z, n), cpd_xz)
    }
}

impl BE<'_, CatTrj, (usize, f64)> {
    // Fit a CIM given sufficient statistics.
    fn fit_transform_cim(
        x: usize,
        z: &[usize],
        n_xz: Array3<f64>,
        t_xz: Array2<f64>,
        n: f64,
        prior: (usize, f64),
        states: &FxIndexMap<String, FxIndexSet<String>>,
    ) -> ((Array3<f64>, Array2<f64>, f64), CatCIM) {
        // Get the prior, as the alpha of Dirichlet and tau of Gamma.
        let (alpha, tau) = prior;
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
        // Add the prior to the counts and times.
        let n_xz = n_xz + alpha;
        let t_xz = t_xz + tau;
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
        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some({
            //
            // Compute the sample log-likelihood as the sum of:
            //
            //   1. ln(tau) * (alpha + 1) - ln_gamma(alpha + 1)
            //   2. + ln_gamma(n_xz + alpha + 1) - ln(t_xz + tau) * (n_xz + alpha + 1)
            //
            ({
                // Compute marginal sufficient statistics.
                let n_z = n_xz.sum_axis(Axis(2));
                let t_z = t_xz.sum_axis(Axis(2));
                // Compute the sample log-likelihood.
                f64::ln(tau) * (alpha + 1.)
                    - ln_gamma(alpha + 1.)                  // .
                    + ((&n_z + 1.).mapv(ln_gamma)           // .
                    - t_z.ln() * (n_z + 1.))
            })
            .sum()
        });

        // Subset the conditioning labels, states and cardinality.
        let conditioning_states = z.iter().map(|&i| states.get_index(i).unwrap());
        // Get the labels of the conditioned variables.
        let states = states.get_index(x).unwrap();
        // Construct the CIM.
        let cim_xz = CatCIM::with_sample_size(
            states,
            conditioning_states,
            parameters,
            sample_size,
            sample_log_likelihood,
        );

        // Remove the last axis of the times.
        let t_xz = t_xz.remove_axis(Axis(2));

        // Return the sufficient statistics and the CIM.
        ((n_xz, t_xz, n), cim_xz)
    }
}

// Implement the CIM estimator for the BE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CPDEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        fn fit_transform(&self, x: usize, z: &[usize]) -> (<CatCIM as CPD>::SS, CatCIM) {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), *self.prior());
            // Compute sufficient statistics.
            let (n_xz, t_xz, n) = SSE::new(self.dataset).fit(x, z);
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit_transform_cim(x, z, n_xz, t_xz, n, prior, states)
        }
    }

});

// Implement the parallel CIM estimator for the BE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCPDEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        // (conditional counts, conditional time spent, sample size)
        type SS = (Array3<f64>, Array2<f64>, f64);

        fn par_fit_transform(&self, x: usize, z: &[usize]) -> (Self::SS, CatCIM) {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), *self.prior());
            // Compute sufficient statistics in parallel.
            let (n_xz, t_xz, n) = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit_transform_cim(x, z, n_xz, t_xz, n, prior, states)
        }
    }

});
