use ndarray::prelude::*;

use super::CPDEstimator;
use crate::{
    data::{CategoricalData, Data},
    distribution::CategoricalCPD,
    types::FxIndexSet,
    utils::RMI,
};

/// A struct representing a Bayesian estimator.
#[derive(Clone, Debug)]
pub struct BayesianEstimator<Pi> {
    prior: Pi,
}

/// A type alias for a bayesian estimator.
pub type BE<Pi> = BayesianEstimator<Pi>;

impl<Pi> BayesianEstimator<Pi> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `prior` - The prior distribution.
    ///
    /// # Returns
    ///
    /// A new `BayesianEstimator` instance.
    ///
    #[inline]
    pub const fn new(prior: Pi) -> Self {
        Self { prior }
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
impl CPDEstimator<CategoricalData, CategoricalCPD> for BE<f64> {
    fn fit(&self, data: &CategoricalData, x: usize, z: &[usize]) -> CategoricalCPD {
        // Concat the variables to fit.
        let x_z: FxIndexSet<_> = [x].iter().chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert_eq!(x_z.len(), 1 + z.len(), "Variables to fit must be unique.");

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
        // Compute the sample size.
        let n = n_z.sum();

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        let n_z = n_z.mapv(|x| x as f64);

        // Align the marginal counts axes.
        let n_z = n_z.insert_axis(Axis(1));

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = *self.prior();
        // Compute the parameters by normalizing the counts with the prior.
        let parameters = (&n_xz + alpha) / (n_z + alpha * cards[x] as f64);

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
