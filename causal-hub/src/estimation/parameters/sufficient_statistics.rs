use dry::macro_for;
use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{CatTable, CatTrj, CatTrjs, CatWtdTable, CatWtdTrj, CatWtdTrjs, Dataset},
    estimation::{CSSEstimator, ParCSSEstimator},
    models::{CPD, CatCIM, CatCPD},
    types::{Labels, Set},
    utils::MI,
};

/// A struct representing a sufficient statistics estimator.
#[derive(Clone, Copy, Debug)]
pub struct SSE<'a, D> {
    dataset: &'a D,
}

impl<'a, D> SSE<'a, D> {
    /// Constructs a new sufficient statistics estimator.
    ///
    /// # Returns
    ///
    /// A new `SSE` instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self { dataset }
    }
}

impl CSSEstimator<<CatCPD as CPD>::SS> for SSE<'_, CatTable> {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }

    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> <CatCPD as CPD>::SS {
        // Assert variables and conditioning variables must be disjoint..
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );

        // Get the shape.
        let shape = self.dataset.shape();

        // Initialize the multi index.
        let m_idx_x = MI::new(x.iter().map(|&i| shape[i]));
        let m_idx_z = MI::new(z.iter().map(|&i| shape[i]));
        // Get the shape of the conditioned and conditioning variables.
        let s_x = m_idx_x.shape().product();
        let s_z = m_idx_z.shape().product();

        // Initialize the joint counts.
        let mut n_xz: Array2<usize> = Array::zeros((s_z, s_x));

        // Count the occurrences of the states.
        self.dataset.values().rows().into_iter().for_each(|row| {
            // Get the value of X and Z as index.
            let idx_x = m_idx_x.ravel(x.iter().map(|&i| row[i] as usize));
            let idx_z = m_idx_z.ravel(z.iter().map(|&i| row[i] as usize));
            // Increment the joint counts.
            n_xz[[idx_z, idx_x]] += 1;
        });

        // Cast the counts to floating point.
        n_xz.mapv(|x| x as f64)
    }
}

impl CSSEstimator<<CatCPD as CPD>::SS> for SSE<'_, CatWtdTable> {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }

    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> <CatCPD as CPD>::SS {
        // Assert variables and conditioning variables must be disjoint..
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );

        // Get the shape.
        let shape = self.dataset.shape();

        // Initialize the multi index.
        let m_idx_x = MI::new(x.iter().map(|&i| shape[i]));
        let m_idx_z = MI::new(z.iter().map(|&i| shape[i]));
        // Get the shape of the conditioned and conditioning variables.
        let s_x = m_idx_x.shape().product();
        let s_z = m_idx_z.shape().product();

        // Initialize the joint counts.
        let mut n_xz: Array2<f64> = Array::zeros((s_z, s_x));

        // Get the unweighted values and weights.
        let values = self.dataset.values().values();
        let weights = self.dataset.weights();

        // Count the occurrences of the states.
        values
            .rows()
            .into_iter()
            .zip(weights)
            .for_each(|(row, &weight)| {
                // Get the value of X and Z as index.
                let idx_x = m_idx_x.ravel(x.iter().map(|&i| row[i] as usize));
                let idx_z = m_idx_z.ravel(z.iter().map(|&i| row[i] as usize));
                // Increment the joint counts.
                n_xz[[idx_z, idx_x]] += weight;
            });

        n_xz
    }
}

impl CSSEstimator<<CatCIM as CPD>::SS> for SSE<'_, CatTrj> {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }

    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> <CatCIM as CPD>::SS {
        // Assert variables and conditioning variables must be disjoint..
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );

        // Get the shape.
        let shape = self.dataset.shape();

        // Initialize the multi index.
        let m_idx_x = MI::new(x.iter().map(|&i| shape[i]));
        let m_idx_z = MI::new(z.iter().map(|&i| shape[i]));
        // Get the shape of the conditioned and conditioning variables.
        let s_x = m_idx_x.shape().product();
        let s_z = m_idx_z.shape().product();

        // Initialize the joint counts.
        let mut n_xz: Array3<usize> = Array::zeros((s_z, s_x, s_x));
        // Initialize the time spent in that state.
        let mut t_xz: Array2<f64> = Array::zeros((s_z, s_x));

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
                let idx_x_i = m_idx_x.ravel(x.iter().map(|&i| e_i[i] as usize));
                let idx_x_j = m_idx_x.ravel(x.iter().map(|&i| e_j[i] as usize));
                // Get the value of Z as index using the strides.
                let idx_z = m_idx_z.ravel(z.iter().map(|&i| e_i[i] as usize));
                // Increment the count when conditioned variable transitions.
                n_xz[[idx_z, idx_x_i, idx_x_j]] += (idx_x_i != idx_x_j) as usize;
                // Increment the time in that state.
                t_xz[[idx_z, idx_x_i]] += t_j - t_i;
            });

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        // Align the dimensions of the counts and times.
        let t_xz = t_xz.insert_axis(Axis(2));

        (n_xz, t_xz)
    }
}

impl CSSEstimator<<CatCIM as CPD>::SS> for SSE<'_, CatWtdTrj> {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }

    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> <CatCIM as CPD>::SS {
        // Get the weight of the trajectory.
        let w = self.dataset.weight();
        // Compute the unweighted sufficient statistics.
        let (n_xz, t_xz) = SSE::new(self.dataset.trajectory()).fit(x, z);
        // Apply the weight to the sufficient statistics.
        (n_xz * w, t_xz * w)
    }
}

// Implement the CSSEstimator and ParCSSEstimator traits for both CatTrjs and CatWtdTrjs.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl CSSEstimator<<CatCIM as CPD>::SS> for SSE<'_, $type> {
        #[inline]
        fn labels(&self) -> &Labels {
            self.dataset.labels()
        }

        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> <CatCIM as CPD>::SS {
            // Get the shape.
            let shape = self.dataset.shape();

            // Get the shape of the conditioned and conditioning variables.
            let s_x = x.iter().map(|&i| shape[i]).product();
            let s_z = z.iter().map(|&i| shape[i]).product();

            // Initialize the joint counts.
            let n_xz: Array3<f64> = Array::zeros((s_z, s_x, s_x));
            // Initialize the time spent in that state.
            let t_xz: Array3<f64> = Array::zeros((s_z, s_x, 1));

            // Iterate over the trajectories.
            self.dataset
                .into_iter()
                // Sum the sufficient statistics of each trajectory.
                .fold((n_xz, t_xz), |(n_xz_a, t_xz_a), trj_b| {
                    // Compute the sufficient statistics of the trajectory.
                    let (n_xz_b, t_xz_b) = SSE::new(trj_b).fit(x, z);
                    // Sum the sufficient statistics.
                    (n_xz_a + n_xz_b, t_xz_a + t_xz_b)
                })
        }
    }

    impl ParCSSEstimator<<CatCIM as CPD>::SS> for SSE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> <CatCIM as CPD>::SS {
            // Get the shape.
            let shape = self.dataset.shape();

            // Get the shape of the conditioned and conditioning variables.
            let s_x = x.iter().map(|&i| shape[i]).product();
            let s_z = z.iter().map(|&i| shape[i]).product();

            // Initialize the joint counts.
            let n_xz: Array3<f64> = Array::zeros((s_z, s_x, s_x));
            // Initialize the time spent in that state.
            let t_xz: Array3<f64> = Array::zeros((s_z, s_x, 1));

            // Iterate over the trajectories in parallel.
            self.dataset
                .par_iter()
                // Sum the sufficient statistics of each trajectory.
                .fold(
                    || (n_xz.clone(), t_xz.clone()),
                    |(n_xz_a, t_xz_a), trj_b| {
                        // Compute the sufficient statistics of the trajectory.
                        let (n_xz_b, t_xz_b) = SSE::new(trj_b).fit(x, z);
                        // Sum the sufficient statistics.
                        (n_xz_a + n_xz_b, t_xz_a + t_xz_b)
                    },
                )
                .reduce(
                    || (n_xz.clone(), t_xz.clone()),
                    |(n_xz_a, t_xz_a), (n_xz_b, t_xz_b)| {
                        // Sum the sufficient statistics.
                        (n_xz_a + n_xz_b, t_xz_a + t_xz_b)
                    },
                )
        }
    }

});
