use ndarray::prelude::*;

use super::{
    CPDEstimator, CSSEstimator, ParCPDEstimator, ParallelConditionalSufficientStatisticsEstimator,
    SSE,
};
use crate::{
    datasets::{CategoricalDataset, CategoricalTrj, CategoricalTrjs, Dataset},
    distributions::{CategoricalCIM, CategoricalCPD},
    types::{FxIndexMap, FxIndexSet},
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

impl CPDEstimator<CategoricalCPD> for MLE<'_, CategoricalDataset> {
    fn fit(&self, x: usize, z: &[usize]) -> CategoricalCPD {
        // Get states and cardinality.
        let states = self.dataset.states();

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, n_z, n) = sse.fit(x, z);

        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&x| x > 0),
            "Failed to get non-zero counts for variable '{}'.",
            self.dataset.labels()[x]
        );

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        let n_z = n_z.mapv(|x| x as f64);

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

impl MLE<'_, CategoricalTrj> {
    // Fit a CIM given sufficient statistics.
    fn fit_cim(
        x: usize,
        z: &[usize],
        n_xz: Array3<usize>,
        t_xz: Array2<f64>,
        n: usize,
        labels: &FxIndexSet<String>,
        states: &FxIndexMap<String, FxIndexSet<String>>,
    ) -> CategoricalCIM {
        // Assert the conditional times counts are not zero.
        assert!(
            t_xz.iter().all(|&x| x > 0.),
            "Failed to get non-zero conditional times for variable '{}'.",
            labels[x]
        );

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);

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

impl CPDEstimator<CategoricalCIM> for MLE<'_, CategoricalTrj> {
    fn fit(&self, x: usize, z: &[usize]) -> CategoricalCIM {
        // Get labels and states.
        let (labels, states) = (self.dataset.labels(), self.dataset.states());

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, t_xz, n) = sse.fit(x, z);

        // Fit the CIM given the sufficient statistics.
        MLE::<'_, CategoricalTrj>::fit_cim(x, z, n_xz, t_xz, n, labels, states)
    }
}

impl CPDEstimator<CategoricalCIM> for MLE<'_, CategoricalTrjs> {
    fn fit(&self, x: usize, z: &[usize]) -> CategoricalCIM {
        // Get labels and states.
        let (labels, states) = (self.dataset.labels(), self.dataset.states());

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics.
        let (n_xz, t_xz, n) = sse.fit(x, z);

        // Fit the CIM given the sufficient statistics.
        MLE::<'_, CategoricalTrj>::fit_cim(x, z, n_xz, t_xz, n, labels, states)
    }
}

impl ParCPDEstimator<CategoricalCIM> for MLE<'_, CategoricalTrjs> {
    fn par_fit(&self, x: usize, z: &[usize]) -> CategoricalCIM {
        // Get labels and states.
        let (labels, states) = (self.dataset.labels(), self.dataset.states());

        // Initialize the sufficient statistics estimator.
        let sse = SSE::new(self.dataset);
        // Compute sufficient statistics in parallel.
        let (n_xz, t_xz, n) = sse.par_fit(x, z);

        // Fit the CIM given the sufficient statistics.
        MLE::<'_, CategoricalTrj>::fit_cim(x, z, n_xz, t_xz, n, labels, states)
    }
}
