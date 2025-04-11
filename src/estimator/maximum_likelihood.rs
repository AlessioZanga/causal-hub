use itertools::Itertools;
use ndarray::prelude::*;

use super::CPDEstimator;
use crate::{
    data::{CategoricalData, Data},
    distribution::CategoricalCPD,
    utils::RMI,
};

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Debug, Default)]
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

impl CPDEstimator<CategoricalData, CategoricalCPD> for MLE {
    fn fit(&self, data: &CategoricalData, x: usize, z: &[usize]) -> CategoricalCPD {
        // Concat the variables to fit.
        let x_z: Vec<_> = [x].iter().chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert!(
            x_z.iter().unique().count() == x_z.len(),
            "Variables to fit must be unique."
        );

        // Get the reference to the labels, states and cardinality.
        let (labels, states, cards) = (data.labels(), data.states(), data.cardinality());

        // Assert the variables to fit are in the data.
        assert!(
            x_z.iter().all(|&i| i < labels.len()),
            "Variables to fit must be in the data."
        );

        // Initialize ravel multi index.
        let rmi = RMI::new(z.iter().map(|&i| cards[i]));
        // Initialize the joint counts.
        let mut n_xz: Array2<usize> = Array::zeros((rmi.cardinality().product(), cards[x]));

        // Count the occurrences of the states.
        data.values().rows().into_iter().for_each(|row| {
            // Get the value of X as index.
            let idx_x = row[x] as usize;
            // Get the value of Z as index using the strides.
            let idx_z = rmi.ravel(z.iter().map(|&i| row[i] as usize));
            // Increment the joint counts.
            n_xz[[idx_z, idx_x]] += 1;
        });

        // Marginalize the counts.
        let n_z = n_xz.sum_axis(Axis(1));
        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&i| i > 0),
            "Failed to get non-zero counts for variable '{}'.",
            labels[x]
        );
        // Compute the sample size.
        let n = n_z.sum();

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
