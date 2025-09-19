use dry::macro_for;
use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{
        CatTable, CatTrj, CatTrjs, CatWtdTable, CatWtdTrj, CatWtdTrjs, Dataset, GaussTable,
    },
    estimation::{CSSEstimator, ParCSSEstimator},
    models::{CatCIMS, CatCPDS, GaussCPDS, Labelled},
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

impl<D> Labelled for SSE<'_, D>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

impl CSSEstimator<CatCPDS> for SSE<'_, CatTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPDS {
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
        let n_xz = n_xz.mapv(|x| x as f64);
        // Compute the sample size.
        let n = n_xz.sum();

        // Return the sufficient statistics.
        CatCPDS::new(n_xz, n)
    }
}

impl CSSEstimator<CatCPDS> for SSE<'_, CatWtdTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPDS {
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

        // Compute the sample size.
        let n = n_xz.sum();

        // Return the sufficient statistics.
        CatCPDS::new(n_xz, n)
    }
}

impl CSSEstimator<GaussCPDS> for SSE<'_, GaussTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPDS {
        // Assert variables and conditioning variables must be disjoint.
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );

        // Get the values.
        let d = self.dataset.values();

        // Select the columns of the variables.
        let mut c_x = Array::zeros((d.nrows(), x.len()));
        for (i, &j) in x.iter().enumerate() {
            c_x.column_mut(i).assign(&d.column(j));
        }
        // Compute the mean.
        let mu_x = c_x.mean_axis(Axis(0)).unwrap();
        // Center the values by subtracting the mean.
        c_x -= &mu_x.clone().insert_axis(Axis(0));

        // Select the columns of the conditioning variables.
        let mut c_z = Array::zeros((d.nrows(), z.len()));
        for (i, &j) in z.iter().enumerate() {
            c_z.column_mut(i).assign(&d.column(j));
        }
        // Compute the mean.
        let mu_z = c_z.mean_axis(Axis(0)).unwrap();
        // Center the values by subtracting the mean.
        c_z -= &mu_z.clone().insert_axis(Axis(0));

        // Compute the sufficient statistics.
        let s_xx = c_x.t().dot(&c_x);
        let s_xz = c_x.t().dot(&c_z);
        let s_zz = c_z.t().dot(&c_z);

        // Get the sample size.
        let n = d.nrows() as f64;

        // Return the sufficient statistics.
        GaussCPDS::new(mu_x, mu_z, s_xx, s_xz, s_zz, n)
    }
}

impl CSSEstimator<CatCIMS> for SSE<'_, CatTrj> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIMS {
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
        // Compute the sample size.
        let n = n_xz.sum();

        CatCIMS::new(n_xz, t_xz, n)
    }
}

impl CSSEstimator<CatCIMS> for SSE<'_, CatWtdTrj> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIMS {
        // Get the weight of the trajectory.
        let w = self.dataset.weight();
        // Compute the unweighted sufficient statistics.
        let s = SSE::new(self.dataset.trajectory()).fit(x, z);
        // Destructure the sufficient statistics.
        let n_xz = s.sample_conditional_counts();
        let t_xz = s.sample_conditional_times();
        let n = s.sample_size();
        // Apply the weight to the sufficient statistics.
        CatCIMS::new(n_xz * w, t_xz * w, n * w)
    }
}

// Implement the CSSEstimator and ParCSSEstimator traits for both CatTrjs and CatWtdTrjs.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl CSSEstimator<CatCIMS> for SSE<'_, $type> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIMS {
            // Get the shape.
            let shape = self.dataset.shape();

            // Get the shape of the conditioned and conditioning variables.
            let s_x = x.iter().map(|&i| shape[i]).product();
            let s_z = z.iter().map(|&i| shape[i]).product();

            // Initialize the sufficient statistics.
            let s = CatCIMS::new(
                // Initialize the joint counts.
                Array3::zeros((s_z, s_x, s_x)),
                // Initialize the time spent in that state.
                Array2::zeros((s_z, s_x)),
                // Initialize the sample size.
                0.,
            );

            // Iterate over the trajectories.
            let s = self.dataset
                .into_iter()
                // Sum the sufficient statistics of each trajectory.
                .fold(s, |s_a, trj_b| s_a + SSE::new(trj_b).fit(x, z));

            // Return the sufficient statistics.
            s
        }
    }

    impl ParCSSEstimator<CatCIMS> for SSE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIMS {
            // Get the shape.
            let shape = self.dataset.shape();

            // Get the shape of the conditioned and conditioning variables.
            let s_x = x.iter().map(|&i| shape[i]).product();
            let s_z = z.iter().map(|&i| shape[i]).product();

            // Initialize the sufficient statistics.
            let s = CatCIMS::new(
                // Initialize the joint counts.
                Array3::zeros((s_z, s_x, s_x)),
                // Initialize the time spent in that state.
                Array2::zeros((s_z, s_x)),
                // Initialize the sample size.
                0.,
            );

            // Iterate over the trajectories in parallel.
            let s = self.dataset
                .par_iter()
                // Sum the sufficient statistics of each trajectory.
                .fold(
                    || s.clone(),
                    |s_a, trj_b| s_a + SSE::new(trj_b).fit(x, z),
                )
                .reduce(
                    || s.clone(),
                    |s_a, s_b| s_a + s_b
                );

            // Return the sufficient statistics.
            s
        }
    }
});
