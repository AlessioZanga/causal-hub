use dry::macro_for;
use ndarray::prelude::*;

use crate::{
    datasets::{CatTable, CatWtdTable},
    estimators::{CPDEstimator, CSSEstimator, MLE, ParCPDEstimator, ParCSSEstimator, SSE},
    models::{CatCPD, CatCPDS},
    types::{Set, States},
};

impl MLE<'_, CatTable> {
    fn fit(states: &States, x: &Set<usize>, z: &Set<usize>, sample_statistics: CatCPDS) -> CatCPD {
        // Get the conditional counts.
        let n_xz = sample_statistics.sample_conditional_counts();
        // Marginalize the counts.
        let n_z = &n_xz.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&x| x > 0.),
            "Failed to get non-zero counts.",
        );

        // Compute the parameters by normalizing the counts.
        let parameters = n_xz / n_z;

        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = (n_xz * (&parameters + eps).ln()).sum();

        // Subset the conditioning labels, states and shape.
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

        // Wrap the sample statistics in an option.
        let sample_statistics = Some(sample_statistics);
        // Wrap the sample log-likelihood in an option.
        let sample_log_likelihood = Some(sample_log_likelihood);

        // Construct the CPD.
        CatCPD::with_optionals(
            states,
            conditioning_states,
            parameters,
            sample_statistics,
            sample_log_likelihood,
        )
    }
}

// Implement the CatCPD estimator for the MLE struct.
macro_for!($type in [CatTable, CatWtdTable] {

    impl CPDEstimator<CatCPD> for MLE<'_, $type> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Get states.
            let states = self.dataset.states();
            // Compute sufficient statistics.
            let sample_statistics = SSE::new(self.dataset).fit(x, z);
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, CatTable>::fit(states, x, z, sample_statistics)
        }
    }

    impl ParCPDEstimator<CatCPD> for MLE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Get states.
            let states = self.dataset.states();
            // Compute sufficient statistics in parallel.
            let sample_statistics = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, CatTable>::fit(states, x, z, sample_statistics)
        }
    }

});
