use dry::macro_for;
use ndarray::prelude::*;
use statrs::function::gamma::ln_gamma;

use crate::{
    datasets::{CatTable, CatTrj, CatTrjs, CatWtdTrj, CatWtdTrjs, Dataset},
    estimation::{CPDEstimator, CSSEstimator, ParCPDEstimator, ParCSSEstimator, SSE},
    models::{CatCIM, CatCPD},
    types::{Labels, Set, States},
};

/// A struct representing a Bayesian estimator.
#[derive(Clone, Copy, Debug)]
pub struct BE<'a, D, Pi> {
    dataset: &'a D,
    prior: Pi,
}

impl<'a, D, Pi> BE<'a, D, Pi> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to the dataset to fit the estimator to.
    /// * `prior` - The prior distribution.
    ///
    /// # Returns
    ///
    /// A new `BE` instance.
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
impl CPDEstimator<CatCPD> for BE<'_, CatTable, usize> {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }

    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
        // Get states and cardinality.
        let states = self.dataset.states();
        let cardinality = self.dataset.cardinality();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let n_xz = sse.fit(x, z);

        // Marginalize the counts.
        let n_z = n_xz.sum_axis(Axis(1)).insert_axis(Axis(1));
        // Compute the sample size.
        let n = n_z.sum();

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = *self.prior();
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");

        // Cast alpha to floating point.
        let alpha = alpha as f64;

        // Add the prior to the counts.
        let n_xz = n_xz + alpha;
        let n_z = n_z + alpha * x.iter().map(|&i| cardinality[i]).product::<usize>() as f64;
        // Compute the parameters by normalizing the counts with the prior.
        let parameters = &n_xz / &n_z;

        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some((&n_xz * parameters.ln()).sum());

        // Set the sample conditional counts.
        let sample_conditional_counts = Some(n_xz);
        // Set the sample size.
        let sample_size = Some(n);

        // Subset the conditioning labels, states and cardinality.
        let conditioning_states = z
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();
        // Get the labels of the conditioned variables.
        let states = x
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();

        // Construct the CPD.
        CatCPD::with_optionals(
            states,
            conditioning_states,
            parameters,
            sample_conditional_counts,
            sample_size,
            sample_log_likelihood,
        )
    }
}

impl BE<'_, CatTrj, (usize, f64)> {
    // Fit a CIM given sufficient statistics.
    fn fit_cim(
        states: &States,
        x: &Set<usize>,
        z: &Set<usize>,
        n_xz: Array3<f64>,
        t_xz: Array3<f64>,
        prior: (usize, f64),
    ) -> CatCIM {
        // Get the prior, as the alpha of Dirichlet and tau of Gamma.
        let (alpha, tau) = prior;
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");
        // Assert tau is positive.
        assert!(tau > 0.0, "Tau must be positive.");

        // Compute the sample size.
        let n = n_xz.sum();

        // Get the cardinality of the conditioning variables.
        let c_z = n_xz.shape()[0] as f64;
        // Scale the prior by the cardinality.
        let alpha = alpha as f64 / c_z;
        let tau = tau / c_z;

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

        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some({
            // Sum counts.
            let n_z = n_xz.sum_axis(Axis(2));
            let t_z = t_xz.sum_axis(Axis(2));
            // Compute the sample log-likelihood.
            let ll_q_xz = {
                // Compute the sample log-likelihood.
                (&n_z + 1.).mapv(ln_gamma).sum() + (alpha + 1.) * f64::ln(tau) //.
                - (ln_gamma(alpha + 1.) + ((&n_z + 1.) * &t_z.ln()).sum())
            };
            // Compute the sample log-likelihood.
            let ll_p_xz = {
                // Compute the sample log-likelihood.
                (ln_gamma(alpha) - n_z.mapv(ln_gamma).sum())     //.
                + (ln_gamma(alpha) - n_xz.mapv(ln_gamma).sum())
            };
            // Return the total log-likelihood.
            ll_q_xz + ll_p_xz
        });

        // Set the sample conditional counts.
        let sample_conditional_counts = Some(n_xz);
        // Set the sample conditional times.
        let sample_conditional_times = Some(t_xz);
        // Set the sample size.
        let sample_size = Some(n);

        // Subset the conditioning labels, states and cardinality.
        let conditioning_states = z
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();
        // Get the labels of the conditioned variables.
        let states = x
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();

        // Construct the CIM.
        CatCIM::with_optionals(
            states,
            conditioning_states,
            parameters,
            sample_conditional_counts,
            sample_conditional_times,
            sample_size,
            sample_log_likelihood,
        )
    }
}

// Implement the CIM estimator for the BE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CPDEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        #[inline]
        fn labels(&self) -> &Labels {
            self.dataset.labels()
        }

        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), *self.prior());
            // Compute sufficient statistics.
            let (n_xz, t_xz) = SSE::new(self.dataset).fit(x, z);
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit_cim(states, x, z, n_xz, t_xz, prior)
        }
    }

});

// Implement the parallel CIM estimator for the BE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCPDEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), *self.prior());
            // Compute sufficient statistics in parallel.
            let (n_xz, t_xz) = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit_cim(states, x, z, n_xz, t_xz, prior)
        }
    }

});
