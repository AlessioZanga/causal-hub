use dry::macro_for;
use ndarray::prelude::*;

use super::{
    CPDEstimator, CSSEstimator, ParCPDEstimator, ParallelConditionalSufficientStatisticsEstimator,
    SSE,
};
use crate::{
    datasets::{CatData, CatTrj, CatTrjs, CatWtdTrj, CatWtdTrjs, Dataset},
    distributions::{CPD, CatCIM, CatCPD},
    types::{Labels, States},
};

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Copy, Debug)]
pub struct MaximumLikelihoodEstimator<'a, D> {
    dataset: &'a D,
}

/// A type alias for a maximum likelihood estimator.
pub type MLE<'a, D> = MaximumLikelihoodEstimator<'a, D>;

impl<'a, D> MaximumLikelihoodEstimator<'a, D> {
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

impl CPDEstimator<CatCPD> for MLE<'_, CatData> {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }

    fn fit_transform(&self, x: usize, z: &[usize]) -> (<CatCPD as CPD>::SS, CatCPD) {
        // Get states and cardinality.
        let states = self.dataset.states();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, n_z, n) = sse.fit(x, z);

        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&x| x > 0.),
            "Failed to get non-zero counts for variable '{}'.",
            self.dataset.labels()[x]
        );

        // Align the dimensions of the counts.
        let n_z = n_z.insert_axis(Axis(1));
        // Compute the parameters by normalizing the counts.
        let parameters = &n_xz / &n_z;

        // Set the sample size.
        let sample_size = Some(n);

        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = Some((&n_xz * (&parameters + eps).ln()).sum());

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

impl MLE<'_, CatTrj> {
    // Fit a CIM given sufficient statistics.
    fn fit_transform_cim(
        x: usize,
        z: &[usize],
        n_xz: Array3<f64>,
        t_xz: Array2<f64>,
        n: f64,
        labels: &Labels,
        states: &States,
    ) -> ((Array3<f64>, Array2<f64>, f64), CatCIM) {
        // Assert the conditional times counts are not zero.
        assert!(
            t_xz.iter().all(|&x| x > 0.),
            "Failed to get non-zero conditional times for variable '{}'.",
            labels[x]
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

// Implement the CIM estimator for the MLE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CPDEstimator<CatCIM> for MLE<'_, $type> {
        #[inline]
        fn labels(&self) -> &Labels {
            self.dataset.labels()
        }

        fn fit_transform(&self, x: usize, z: &[usize]) -> (<CatCIM as CPD>::SS, CatCIM) {
            // Get labels and states.
            let (labels, states) = (self.dataset.labels(), self.dataset.states());

            // Initialize the sufficient statistics estimator.
            let sse = SSE::new(self.dataset);
            // Compute sufficient statistics.
            let (n_xz, t_xz, n) = sse.fit(x, z);

            // Fit the CIM given the sufficient statistics.
            MLE::<'_, CatTrj>::fit_transform_cim(x, z, n_xz, t_xz, n, labels, states)
        }
    }

});

// Implement the parallel version of the CIM estimator for the MLE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCPDEstimator<CatCIM> for MLE<'_, $type> {
        fn par_fit_transform(&self, x: usize, z: &[usize]) -> (<CatCIM as CPD>::SS, CatCIM) {
            // Get labels and states.
            let (labels, states) = (self.dataset.labels(), self.dataset.states());

            // Initialize the sufficient statistics estimator.
            let sse = SSE::new(self.dataset);
            // Compute sufficient statistics in parallel.
            let (n_xz, t_xz, n) = sse.par_fit(x, z);

            // Fit the CIM given the sufficient statistics.
            MLE::<'_, CatTrj>::fit_transform_cim(x, z, n_xz, t_xz, n, labels, states)
        }
    }

});
