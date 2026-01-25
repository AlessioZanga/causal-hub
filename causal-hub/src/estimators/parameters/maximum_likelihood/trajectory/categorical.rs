use dry::macro_for;
use ndarray::prelude::*;

use crate::{
    datasets::{CatTrj, CatTrjs, CatWtdTrj, CatWtdTrjs},
    estimators::{CIMEstimator, CSSEstimator, MLE, ParCIMEstimator, ParCSSEstimator, SSE},
    models::{CatCIM, CatCIMS},
    types::{Error, Result, Set, States},
};

impl MLE<'_, CatTrj> {
    // Fit a CIM given sufficient statistics.
    fn fit(
        states: &States,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: CatCIMS,
    ) -> Result<CatCIM> {
        // Get the conditional counts and times.
        let n_xz = sample_statistics.sample_conditional_counts();
        let t_xz = sample_statistics.sample_conditional_times();

        // Assert the conditional times counts are not zero.
        if !t_xz.iter().all(|&x| x > 0.) {
            return Err(Error::Stats(
                "Failed to get non-zero conditional times.".into(),
            ));
        }

        // Insert axis to align the dimensions.
        let t_xz = &t_xz.clone().insert_axis(Axis(2));

        // Estimate the parameters by normalizing the counts.
        let mut parameters = n_xz / t_xz;
        // Fix the diagonal.
        parameters.outer_iter_mut().for_each(|mut q| {
            // Fill the diagonal with zeros.
            q.diag_mut().fill(0.);
            // Compute the negative sum of the rows.
            let q_neg_sum = -q.sum_axis(Axis(1));
            // Assign the negative sum to the diagonal.
            q.diag_mut().assign(&q_neg_sum);
        });

        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = {
            // Compute the sample log-likelihood.
            let ll_q_xz = {
                // Sum counts, aligning the dimensions.
                let n_z = n_xz.sum_axis(Axis(2));
                let t_z = t_xz.sum_axis(Axis(2));
                // Clone the parameters.
                let mut q_z = Array::zeros(n_z.dim());
                // Get the diagonals.
                parameters
                    .outer_iter()
                    .zip(q_z.outer_iter_mut())
                    .for_each(|(p, mut q)| {
                        q.assign(&(-&p.diag()));
                    });
                // Compute the sample log-likelihood.
                (&n_z * (&q_z + eps).ln()).sum() + (-&q_z * &t_z).sum()
            };
            // Compute the sample log-likelihood.
            let ll_p_xz = {
                // Clone the parameters.
                let mut p_xz = parameters.clone();
                // Set diagonal to zero.
                p_xz.outer_iter_mut().for_each(|mut p| {
                    // Fill the diagonal with zeros.
                    p.diag_mut().fill(0.);
                });
                // Normalize the parameters, align the dimensions.
                p_xz /= &p_xz.sum_axis(Axis(2)).insert_axis(Axis(2));
                // Compute the sample log-likelihood.
                (n_xz * (p_xz + eps).ln()).sum()
            };
            // Return the total log-likelihood.
            ll_q_xz + ll_p_xz
        };

        // Subset the conditioning labels, states and shape.
        let conditioning_states = z
            .iter()
            .map(|&i| {
                let (k, v) = states
                    .get_index(i)
                    .ok_or_else(|| Error::VertexOutOfBounds(i))?;
                Ok((k.clone(), v.clone()))
            })
            .collect::<Result<_>>()?;
        // Get the labels of the conditioned variables.
        let states = x
            .iter()
            .map(|&i| {
                let (k, v) = states
                    .get_index(i)
                    .ok_or_else(|| Error::VertexOutOfBounds(i))?;
                Ok((k.clone(), v.clone()))
            })
            .collect::<Result<_>>()?;

        // Wrap the sufficient statistics in an option.
        let sample_statistics = Some(sample_statistics);
        // Wrap the sample log-likelihood in an option.
        let sample_log_likelihood = Some(sample_log_likelihood);

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

// Implement the CatCIM estimator for the MLE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CIMEstimator<CatCIM> for MLE<'_, $type> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCIM> {
            // Get states.
            let states = self.dataset.states();
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
            MLE::<'_, CatTrj>::fit(states, x, z, sample_statistics)
        }
    }

});

// Implement the parallel version of the CIM estimator for the MLE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCIMEstimator<CatCIM> for MLE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCIM> {
            // Get states.
            let states = self.dataset.states();
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
            MLE::<'_, CatTrj>::fit(states, x, z, sample_statistics)
        }
    }

});
