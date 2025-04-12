use itertools::Itertools;
use ndarray::prelude::*;

use super::CSSEstimator;
use crate::{
    dataset::{CategoricalDataset, CategoricalTrj, Dataset},
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

// (conditional counts, marginal counts, sample size)
type _T0 = (Array2<f64>, Array1<f64>, usize);

impl CSSEstimator<CategoricalDataset, _T0> for SSE {
    fn fit(&self, dataset: &CategoricalDataset, x: usize, z: &[usize]) -> _T0 {
        // Concat the variables to fit.
        let x_z: FxIndexSet<_> = [x].iter().chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert_eq!(x_z.len(), 1 + z.len(), "Variables to fit must be unique.");

        // Get the reference to the labels, states and cardinality.
        let (labels, cards) = (dataset.labels(), dataset.cardinality());

        // Assert the variables to fit are in the dataset.
        assert!(
            x_z.iter().all(|&i| i < labels.len()),
            "Variables to fit must be in the dataset."
        );

        // Initialize ravel multi index.
        let idx = RMI::new(z.iter().map(|&i| cards[i]));
        // Initialize the joint counts.
        let mut n_xz: Array2<usize> = Array::zeros((idx.cardinality().product(), cards[x]));

        // Count the occurrences of the states.
        dataset.values().rows().into_iter().for_each(|row| {
            // Get the value of X as index.
            let idx_x = row[x] as usize;
            // Get the value of Z as index using the strides.
            let idx_z = idx.ravel(z.iter().map(|&i| row[i] as usize));
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

// (conditional counts, conditional time spent, sample size)
type _T1 = (Array3<f64>, Array2<f64>, usize);

impl CSSEstimator<CategoricalTrj, _T1> for SSE {
    fn fit(&self, trj: &CategoricalTrj, x: usize, z: &[usize]) -> _T1 {
        // Get the cardinality of the trajectory.
        let cards = trj.cardinality();
        // Construct the ravel multi index.
        let idx = RMI::new(z.iter().map(|&i| cards[i]));
        // Get the cardinality of the conditioned and conditioning variables.
        let (c_x, c_z) = (cards[x], idx.cardinality().product());

        // Initialize the joint counts.
        let mut n_xz: Array3<usize> = Array::zeros((c_z, c_x, c_x));
        // Initialize the time spent in that state.
        let mut t_xz: Array2<f64> = Array::zeros((c_z, c_x));

        // Iterate over the trajectory events.
        trj.events()
            .rows()
            .into_iter()
            .zip(trj.times())
            .tuple_windows()
            // Compare the current and next event.
            .for_each(|((e_i, t_i), (e_j, t_j))| {
                // Get the value of X as index.
                let (x_i, x_j) = (e_i[x] as usize, e_j[x] as usize);
                // Get the value of Z as index using the strides.
                let z_i = idx.ravel(z.iter().map(|&i| e_i[i] as usize));
                // Increment the count when conditioned variable transitions.
                n_xz[[z_i, x_i, x_j]] += (x_i != x_j) as usize;
                // Increment the time in that state.
                t_xz[[z_i, x_i]] += t_j - t_i;
            });

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        // Get the sample size.
        let n = trj.sample_size();

        (n_xz, t_xz, n)
    }
}
