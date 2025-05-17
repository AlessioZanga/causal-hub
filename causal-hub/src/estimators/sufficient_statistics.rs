use dry::macro_for;
use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use super::{CSSEstimator, ParCSSEstimator};
use crate::{
    datasets::{CatData, CatTrj, CatTrjs, CatWtdTrj, CatWtdTrjs, Dataset},
    types::FxIndexSet,
    utils::RMI,
};

/// A struct representing a sufficient statistics estimator.
#[derive(Clone, Copy, Debug)]
pub struct SufficientStatisticsEstimator<'a, D> {
    dataset: &'a D,
}

impl<'a, D> SufficientStatisticsEstimator<'a, D> {
    /// Constructs a new sufficient statistics estimator.
    ///
    /// # Returns
    ///
    /// A new `SufficientStatisticsEstimator` instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self { dataset }
    }
}

/// A type alias for a sufficient statistics estimator.
pub type SSE<'a, D> = SufficientStatisticsEstimator<'a, D>;

impl CSSEstimator for SSE<'_, CatData> {
    // (conditional counts, marginal counts, sample size)
    type SufficientStatistics = (Array2<f64>, Array1<f64>, f64);

    fn fit(&self, x: usize, z: &[usize]) -> Self::SufficientStatistics {
        // Concat the variables to fit.
        let x_z: FxIndexSet<_> = std::iter::once(&x).chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert_eq!(x_z.len(), 1 + z.len(), "Variables to fit must be unique.");

        // Get the reference to the labels, states and cardinality.
        let (labels, cards) = (self.dataset.labels(), self.dataset.cardinality());

        // Assert the variables to fit are in the dataset.
        assert!(
            x_z.iter().all(|&i| i < labels.len()),
            "Variables to fit must be in the dataset."
        );

        // Initialize ravel multi index.
        let idx = RMI::new(z.iter().map(|&i| cards[i]));
        // Get the cardinality of the conditioned and conditioning variables.
        let (c_x, c_z) = (cards[x], idx.cardinality().product());

        // Initialize the joint counts.
        let mut n_xz: Array2<usize> = Array::zeros((c_z, c_x));

        // Count the occurrences of the states.
        self.dataset.values().rows().into_iter().for_each(|row| {
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
        let n = n as f64;

        (n_xz, n_z, n)
    }
}

impl CSSEstimator for SSE<'_, CatTrj> {
    // (conditional counts, conditional time spent, sample size)
    type SufficientStatistics = (Array3<f64>, Array2<f64>, f64);

    fn fit(&self, x: usize, z: &[usize]) -> Self::SufficientStatistics {
        // Get the cardinality of the trajectory.
        let cards = self.dataset.cardinality();
        // Construct the ravel multi index.
        let idx = RMI::new(z.iter().map(|&i| cards[i]));
        // Get the cardinality of the conditioned and conditioning variables.
        let (c_x, c_z) = (cards[x], idx.cardinality().product());

        // Initialize the joint counts.
        let mut n_xz: Array3<usize> = Array::zeros((c_z, c_x, c_x));
        // Initialize the time spent in that state.
        let mut t_xz: Array2<f64> = Array::zeros((c_z, c_x));

        // Iterate over the trajectory events.
        self.dataset
            .values()
            .rows()
            .into_iter()
            .zip(self.dataset.times())
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

        // Get the sample size.
        let n = n_xz.sum();

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        let n = n as f64;

        (n_xz, t_xz, n)
    }
}

impl CSSEstimator for SSE<'_, CatWtdTrj> {
    // (conditional counts, conditional time spent, sample size)
    type SufficientStatistics = (Array3<f64>, Array2<f64>, f64);

    fn fit(&self, x: usize, z: &[usize]) -> Self::SufficientStatistics {
        // Get the weight of the trajectory.
        let w = self.dataset.weight();
        // Compute the unweighted sufficient statistics.
        let (n_xz, t_xz, n) = SSE::new(self.dataset.trajectory()).fit(x, z);
        // Apply the weight to the sufficient statistics.
        (n_xz * w, t_xz * w, n * w)
    }
}

// Implement the CSSEstimator and ParCSSEstimator traits for both CatTrjs and CatWtdTrjs.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl CSSEstimator for SSE<'_, $type> {
        // (conditional counts, conditional time spent, sample size)
        type SufficientStatistics = (Array3<f64>, Array2<f64>, f64);

        fn fit(&self, x: usize, z: &[usize]) -> Self::SufficientStatistics {
            // Get the cardinality of the trajectory.
            let cards = self.dataset.cardinality();
            // Get the cardinality of the conditioned and conditioning variables.
            let (c_x, c_z) = (cards[x], z.iter().map(|&i| cards[i]).product());

            // Initialize the joint counts.
            let n_xz: Array3<f64> = Array::zeros((c_z, c_x, c_x));
            // Initialize the time spent in that state.
            let t_xz: Array2<f64> = Array::zeros((c_z, c_x));

            // Iterate over the trajectories.
            self.dataset
                .into_iter()
                // Sum the sufficient statistics of each trajectory.
                .fold((n_xz, t_xz, 0.), |(n_xz_a, t_xz_a, n_a), trj_b| {
                    // Compute the sufficient statistics of the trajectory.
                    let (n_xz_b, t_xz_b, n_b) = SSE::new(trj_b).fit(x, z);
                    // Sum the sufficient statistics.
                    (n_xz_a + n_xz_b, t_xz_a + t_xz_b, n_a + n_b)
                })
        }
    }

    impl ParCSSEstimator for SSE<'_, $type> {
        // (conditional counts, conditional time spent, sample size)
        type SufficientStatistics = (Array3<f64>, Array2<f64>, f64);

        fn par_fit(&self, x: usize, z: &[usize]) -> Self::SufficientStatistics {
            // Get the cardinality of the trajectory.
            let cards = self.dataset.cardinality();
            // Get the cardinality of the conditioned and conditioning variables.
            let (c_x, c_z) = (cards[x], z.iter().map(|&i| cards[i]).product());

            // Initialize the joint counts.
            let n_xz: Array3<f64> = Array::zeros((c_z, c_x, c_x));
            // Initialize the time spent in that state.
            let t_xz: Array2<f64> = Array::zeros((c_z, c_x));

            // Iterate over the trajectories in parallel.
            self.dataset
                .par_iter()
                // Sum the sufficient statistics of each trajectory.
                .fold(
                    || (n_xz.clone(), t_xz.clone(), 0.),
                    |(n_xz_a, t_xz_a, n_a), trj_b| {
                        // Compute the sufficient statistics of the trajectory.
                        let (n_xz_b, t_xz_b, n_b) = SSE::new(trj_b).fit(x, z);
                        // Sum the sufficient statistics.
                        (n_xz_a + n_xz_b, t_xz_a + t_xz_b, n_a + n_b)
                    },
                )
                .reduce(
                    || (n_xz.clone(), t_xz.clone(), 0.),
                    |(n_xz_a, t_xz_a, n_a), (n_xz_b, t_xz_b, n_b)| {
                        // Sum the sufficient statistics.
                        (n_xz_a + n_xz_b, t_xz_a + t_xz_b, n_a + n_b)
                    },
                )
        }
    }

});
