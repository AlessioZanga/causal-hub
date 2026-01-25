use dry::macro_for;
use ndarray::prelude::*;
use statrs::function::gamma::ln_gamma;

use crate::{
    datasets::{CatTrj, CatTrjs, CatWtdTrj, CatWtdTrjs},
    estimators::{BE, CIMEstimator, CSSEstimator, ParCIMEstimator, ParCSSEstimator, SSE},
    models::{CatCIM, CatCIMS},
    types::{Error, Result, Set, States},
};

impl BE<'_, CatTrj, (usize, f64)> {
    // Fit a CIM given sufficient statistics.
    fn fit(
        states: &States,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: CatCIMS,
        prior: (usize, f64),
    ) -> Result<CatCIM> {
        // Get the prior, as the alpha of Dirichlet and tau of Gamma.
        let (alpha, tau) = prior;
        // Assert alpha is positive.
        if alpha == 0 {
            return Err(Error::InvalidParameter(
                "alpha".into(),
                "must be positive".into(),
            ));
        }
        // Assert tau is positive.
        if tau <= 0.0 {
            return Err(Error::InvalidParameter(
                "tau".into(),
                "must be positive".into(),
            ));
        }

        // Get the conditional counts and times.
        let n_xz = sample_statistics.sample_conditional_counts();
        let t_xz = sample_statistics.sample_conditional_times();

        // Insert axis to align the dimensions.
        let t_xz = &t_xz.clone().insert_axis(Axis(2));

        // Get the shape of the conditioning variables.
        let s_z = n_xz.shape()[0] as f64;
        // Scale the prior by the shape.
        let alpha = alpha as f64 / s_z;
        let tau = tau / s_z;

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

        // Subset the conditioning labels, states and shape.
        let conditioning_states = z
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).ok_or(Error::VertexOutOfBounds(i))?;
                Ok((k.clone(), v.clone()))
            })
            .collect::<Result<_>>()?;
        // Get the labels of the conditioned variables.
        let states = x
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).ok_or(Error::VertexOutOfBounds(i))?;
                Ok((k.clone(), v.clone()))
            })
            .collect::<Result<_>>()?;

        // Wrap the sufficient statistics in an option.
        let sample_statistics = Some(sample_statistics);

        // Construct the CIM.
        Ok(CatCIM::with_optionals(
            states,
            conditioning_states,
            parameters,
            sample_statistics,
            sample_log_likelihood,
        ))
    }
}

// Implement the CIM estimator for the BE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CIMEstimator<CatCIM> for BE<'_, $type, ()> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCIM> {
            // Default to uniform prior.
            self.clone().with_prior((1, 1.)).fit(x, z)
        }
    }

    impl CIMEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCIM> {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), self.prior);
            // Set sufficient statistics estimator.
            let sample_statistics = SSE::new(self.dataset);
            // Set missing handling method, if any.
            let sample_statistics = sample_statistics.with_missing_method(
                self.missing_method,
                self.missing_mechanism.clone()
            );
            // Compute sufficient statistics.
            let sample_statistics = sample_statistics.fit(x, z)?;
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit(states, x, z, sample_statistics, prior)
        }
    }

});

// Implement the parallel CIM estimator for the BE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCIMEstimator<CatCIM> for BE<'_, $type, ()> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCIM> {
            // Default to uniform prior.
            self.clone().with_prior((1, 1.)).fit(x, z)
        }
    }

    impl ParCIMEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCIM> {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), self.prior);
            // Set sufficient statistics estimator.
            let sample_statistics = SSE::new(self.dataset);
            // Set missing handling method, if any.
            let sample_statistics = sample_statistics.with_missing_method(
                self.missing_method,
                self.missing_mechanism.clone()
            );
            // Compute sufficient statistics in parallel.
            let sample_statistics = sample_statistics.par_fit(x, z)?;
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit(states, x, z, sample_statistics, prior)
        }
    }

});
