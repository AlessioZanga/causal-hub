use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{Dataset, GaussTable, GaussWtdTable},
    estimators::{CSSEstimator, ParCSSEstimator, SSE},
    models::GaussCPDS,
    types::{AXIS_CHUNK_LENGTH, Set},
};

impl SSE<'_, GaussTable> {
    fn fit(d: ArrayView2<f64>, x: &Set<usize>, z: &Set<usize>) -> GaussCPDS {
        // Select the columns of the variables.
        let mut d_x = Array::zeros((d.nrows(), x.len()));
        for (i, &j) in x.iter().enumerate() {
            d_x.column_mut(i).assign(&d.column(j));
        }
        // Compute the mean.
        let mu_x = d_x.mean_axis(Axis(0)).unwrap();

        // Select the columns of the conditioning variables.
        let mut d_z = Array::zeros((d.nrows(), z.len()));
        for (i, &j) in z.iter().enumerate() {
            d_z.column_mut(i).assign(&d.column(j));
        }
        // Compute the mean.
        let mu_z = d_z.mean_axis(Axis(0)).unwrap();

        // Compute the second moment statistics.
        let m_xx = d_x.t().dot(&d_x);
        let m_xz = d_x.t().dot(&d_z);
        let m_zz = d_z.t().dot(&d_z);

        // Get the sample size.
        let n = d.nrows() as f64;

        // Return the sufficient statistics.
        GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)
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
        // Return the sufficient statistics.
        Self::fit(d.view(), x, z)
    }
}

impl ParCSSEstimator<GaussCPDS> for SSE<'_, GaussTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPDS {
        // Assert variables and conditioning variables must be disjoint.
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );

        // Initialize the sufficient statistics.
        let s_xz = {
            let n = 0.;
            let mu_x = Array::zeros(x.len());
            let mu_z = Array::zeros(z.len());
            let m_xx = Array::zeros((x.len(), x.len()));
            let m_xz = Array::zeros((x.len(), z.len()));
            let m_zz = Array::zeros((z.len(), z.len()));
            GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)
        };

        // Get the values.
        let d = self.dataset.values();

        // Get the values.
        d.axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .into_par_iter()
            // Compute the sufficient statistics for each chunk.
            .map(|d| Self::fit(d, x, z))
            // Aggregate the sufficient statistics.
            .fold(|| s_xz.clone(), |a, b| a + b)
            .reduce(|| s_xz.clone(), |a, b| a + b)
    }
}

impl SSE<'_, GaussWtdTable> {
    fn fit(
        d: ArrayView2<f64>,
        norm_w: ArrayView2<f64>,
        sum_w: f64,
        x: &Set<usize>,
        z: &Set<usize>,
    ) -> GaussCPDS {
        // Select the columns of the variables.
        let mut d_x = Array::zeros((d.nrows(), x.len()));
        for (i, &j) in x.iter().enumerate() {
            d_x.column_mut(i).assign(&d.column(j));
        }
        // Compute the weighted mean.
        let mu_x = (&norm_w * &d_x).mean_axis(Axis(0)).unwrap();

        // Select the columns of the conditioning variables.
        let mut d_z = Array::zeros((d.nrows(), z.len()));
        for (i, &j) in z.iter().enumerate() {
            d_z.column_mut(i).assign(&d.column(j));
        }
        // Compute the weighted mean.
        let mu_z = (&norm_w * &d_z).mean_axis(Axis(0)).unwrap();

        // Compute the root weights for centering.
        let sqrt_w = norm_w.mapv(f64::sqrt);
        let d_sqrt_w_x = &sqrt_w * &d_x;
        let d_sqrt_w_z = &sqrt_w * &d_z;

        // Compute the weighted second moment statistics.
        let m_xx = d_sqrt_w_x.t().dot(&d_sqrt_w_x);
        let m_xz = d_sqrt_w_x.t().dot(&d_sqrt_w_z);
        let m_zz = d_sqrt_w_z.t().dot(&d_sqrt_w_z);

        // Get the sample (mass) size.
        let n = sum_w;

        // Return the sufficient statistics.
        GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)
    }
}

impl CSSEstimator<GaussCPDS> for SSE<'_, GaussWtdTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPDS {
        // Assert variables and conditioning variables must be disjoint.
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );

        // Get the values.
        let d = self.dataset.values().values();
        // Get the weights.
        let w = self.dataset.weights();
        // Sum the weights to normalize.
        let sum_w = w.sum();
        // Normalize the weights.
        let w = w / sum_w;
        // Align the axis for broadcasting.
        let w = w.insert_axis(Axis(1));

        // Return the sufficient statistics.
        Self::fit(d.view(), w.view(), sum_w, x, z)
    }
}

impl ParCSSEstimator<GaussCPDS> for SSE<'_, GaussWtdTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPDS {
        // Assert variables and conditioning variables must be disjoint.
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );

        // Initialize the sufficient statistics.
        let s_xz = {
            let n = 0.;
            let mu_x = Array::zeros(x.len());
            let mu_z = Array::zeros(z.len());
            let m_xx = Array::zeros((x.len(), x.len()));
            let m_xz = Array::zeros((x.len(), z.len()));
            let m_zz = Array::zeros((z.len(), z.len()));
            GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)
        };

        // Get the values.
        let values = self.dataset.values().values();
        // Get the weights.
        let weights = self.dataset.weights();

        // Sum the weights to normalize.
        let sum_w: f64 = weights.par_iter().sum();
        // Normalize the weights.
        let weights = {
            // Clone the weights.
            let mut weights = weights.clone();
            // Normalize the weights in parallel.
            weights
                .axis_chunks_iter_mut(Axis(0), AXIS_CHUNK_LENGTH)
                .into_par_iter()
                .for_each(|mut w| w /= sum_w);
            // Return the normalized weights.
            weights
        };
        // Align the axis for broadcasting.
        let weights = weights.insert_axis(Axis(1));

        // Get the values.
        values
            .axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .into_par_iter()
            .zip(weights.axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH))
            // Compute the sufficient statistics for each chunk.
            .map(|(d, w)| Self::fit(d, w, sum_w, x, z))
            // Aggregate the sufficient statistics.
            .fold(|| s_xz.clone(), |a, b| a + b)
            .reduce(|| s_xz.clone(), |a, b| a + b)
    }
}
