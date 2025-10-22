use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{CatTable, CatWtdTable, Dataset},
    estimators::{CSSEstimator, ParCSSEstimator, SSE},
    models::CatCPDS,
    types::{AXIS_CHUNK_LENGTH, Set},
    utils::MI,
};

impl CSSEstimator<CatCPDS> for SSE<'_, CatTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPDS {
        // Assert variables and conditioning variables must be disjoint.
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

impl ParCSSEstimator<CatCPDS> for SSE<'_, CatTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPDS {
        // Assert variables and conditioning variables must be disjoint.
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
        let n_xz: Array2<usize> = Array::zeros((s_z, s_x));

        // Count the occurrences of the states.
        let n_xz = self
            .dataset
            .values()
            .axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .into_par_iter()
            .map(|values| {
                // Clone the zeros joint counts.
                let mut n_xz = n_xz.clone();
                // Count the occurrences of the states.
                values.rows().into_iter().for_each(|row| {
                    // Get the value of X and Z as index.
                    let idx_x = m_idx_x.ravel(x.iter().map(|&i| row[i] as usize));
                    let idx_z = m_idx_z.ravel(z.iter().map(|&i| row[i] as usize));
                    // Increment the joint counts.
                    n_xz[[idx_z, idx_x]] += 1;
                });
                // Return the local joint counts.
                n_xz
            })
            // Aggregate the local joint counts.
            .fold(|| n_xz.clone(), |a, b| a + b)
            .reduce(|| n_xz.clone(), |a, b| a + b);

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
        // Assert variables and conditioning variables must be disjoint.
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

impl ParCSSEstimator<CatCPDS> for SSE<'_, CatWtdTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPDS {
        // Assert variables and conditioning variables must be disjoint.
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
        let n_xz: Array2<f64> = Array::zeros((s_z, s_x));

        // Get the unweighted values and weights.
        let values = self.dataset.values().values();
        let weights = self.dataset.weights();

        // Count the occurrences of the states.
        let n_xz = values
            .axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .into_par_iter()
            .zip(weights.axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH))
            .map(|(values, weights)| {
                // Clone the zeros joint counts.
                let mut n_xz = n_xz.clone();
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
                // Return the local joint counts.
                n_xz
            })
            // Aggregate the local joint counts.
            .fold(|| n_xz.clone(), |a, b| a + b)
            .reduce(|| n_xz.clone(), |a, b| a + b);

        // Compute the sample size.
        let n = n_xz.sum();

        // Return the sufficient statistics.
        CatCPDS::new(n_xz, n)
    }
}
