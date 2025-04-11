use ndarray::prelude::*;

use super::CSSEstimator;
use crate::{
    data::{CategoricalData, Data},
    types::FxIndexSet,
    utils::RMI,
};

/// A struct representing a sufficient statistics estimator.
#[derive(Clone, Copy, Debug, Default)]
pub struct SufficientStatisticsEstimator;

impl SufficientStatisticsEstimator {
    /// Constructs a new sufficient statistics estimator.
    ///
    /// # Returns
    ///
    /// A new `SufficientStatisticsEstimator` instance.
    ///
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

/// A type alias for a sufficient statistics estimator.
pub type SSE = SufficientStatisticsEstimator;

type _T = (Array2<f64>, Array1<f64>, usize);

impl CSSEstimator<CategoricalData, _T> for SSE {
    fn fit(&self, data: &CategoricalData, x: usize, z: &[usize]) -> _T {
        // Concat the variables to fit.
        let x_z: FxIndexSet<_> = [x].iter().chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert_eq!(x_z.len(), 1 + z.len(), "Variables to fit must be unique.");

        // Get the reference to the labels, states and cardinality.
        let (labels, cards) = (data.labels(), data.cardinality());

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

        (n_xz, n_z, n)
    }
}
