use dry::macro_for;
use ndarray::prelude::*;

use crate::{
    datasets::{CatTable, CatTrj, CatTrjs, CatWtdTable, CatWtdTrj, CatWtdTrjs, Dataset},
    estimation::{CPDEstimator, CSSEstimator, ParCPDEstimator, ParCSSEstimator, SSE},
    models::{CatCIM, CatCPD},
    types::{Labels, Set, States},
};

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Copy, Debug)]
pub struct MLE<'a, D> {
    dataset: &'a D,
}

impl<'a, D> MLE<'a, D> {
    /// Creates a new maximum likelihood estimator.
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to the dataset to fit the estimator to.
    ///
    /// # Returns
    ///
    /// A new `MaximumLikelihoodEstimator` instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self { dataset }
    }
}

// Implement the CPD estimator for the MLE struct.
macro_for!($type in [CatTable, CatWtdTable] {

    impl CPDEstimator<CatCPD> for MLE<'_, $type> {
        #[inline]
        fn labels(&self) -> &Labels {
            self.dataset.labels()
        }

        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Get states and cardinality.
            let states = self.dataset.states();

            // Initialize the sufficient statistics estimator.
            let sse = SSE::new(self.dataset);
            // Compute sufficient statistics.
            let n_xz = sse.fit(x, z);

            // Marginalize the counts.
            let n_z = n_xz.sum_axis(Axis(1)).insert_axis(Axis(1));

            // Assert the marginal counts are not zero.
            assert!(
                n_z.iter().all(|&x| x > 0.),
                "Failed to get non-zero counts.",
            );

            // Compute the sample size.
            let n = n_z.sum();

            // Compute the parameters by normalizing the counts.
            let parameters = &n_xz / &n_z;

            // Set epsilon to avoid ln(0).
            let eps = f64::MIN_POSITIVE;
            // Compute the sample log-likelihood, avoiding ln(0).
            let sample_log_likelihood = Some((&n_xz * (&parameters + eps).ln()).sum());

            // Set the sample conditional counts.
            let sample_conditional_counts = Some(n_xz.clone());
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

});

impl MLE<'_, CatTrj> {
    // Fit a CIM given sufficient statistics.
    fn fit_cim(
        states: &States,
        x: &Set<usize>,
        z: &Set<usize>,
        n_xz: Array3<f64>,
        t_xz: Array3<f64>,
    ) -> CatCIM {
        // Assert the conditional times counts are not zero.
        assert!(
            t_xz.iter().all(|&x| x > 0.),
            "Failed to get non-zero conditional times."
        );

        // Compute the sample size.
        let n = n_xz.sum();

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

        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = Some({
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
                (&n_xz * (p_xz + eps).ln()).sum()
            };
            // Return the total log-likelihood.
            ll_q_xz + ll_p_xz
        });

        // Set the sample conditional counts.
        let sample_conditional_counts = Some(n_xz.clone());
        // Set the sample conditional times.
        let sample_conditional_times = Some(t_xz.clone());
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

// Implement the CIM estimator for the MLE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CPDEstimator<CatCIM> for MLE<'_, $type> {
        #[inline]
        fn labels(&self) -> &Labels {
            self.dataset.labels()
        }

        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get states.
            let states = self.dataset.states();

            // Initialize the sufficient statistics estimator.
            let sse = SSE::new(self.dataset);
            // Compute sufficient statistics.
            let (n_xz, t_xz) = sse.fit(x, z);

            // Fit the CIM given the sufficient statistics.
            MLE::<'_, CatTrj>::fit_cim(states, x, z, n_xz, t_xz)
        }
    }

});

// Implement the parallel version of the CIM estimator for the MLE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCPDEstimator<CatCIM> for MLE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get states.
            let states = self.dataset.states();

            // Initialize the sufficient statistics estimator.
            let sse = SSE::new(self.dataset);
            // Compute sufficient statistics in parallel.
            let (n_xz, t_xz) = sse.par_fit(x, z);

            // Fit the CIM given the sufficient statistics.
            MLE::<'_, CatTrj>::fit_cim(states, x, z, n_xz, t_xz)
        }
    }

});
