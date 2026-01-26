use ndarray::{azip, prelude::*};
use rayon::prelude::*;

use crate::{
    datasets::{Dataset, GaussIncTable, GaussTable, GaussWtdTable, IncDataset, MissingMethod},
    estimators::{CSSEstimator, ParCSSEstimator, SSE},
    models::{GaussCPDS, Labelled},
    types::{AXIS_CHUNK_LENGTH, Error, Result, Set},
};

impl SSE<'_, GaussTable> {
    fn fit(d: ArrayView2<f64>, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPDS> {
        // Initialize the sufficient statistics.
        let mut s = {
            let n = 0.;
            let mu_x = Array::zeros(x.len());
            let mu_z = Array::zeros(z.len());
            let m_xx = Array::zeros((x.len(), x.len()));
            let m_xz = Array::zeros((x.len(), z.len()));
            let m_zz = Array::zeros((z.len(), z.len()));
            GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)?
        };

        // Initialize the chunk buffers.
        d.axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .try_for_each(|d| -> Result<_> {
                // Select the columns of the variables.
                let mut d_x = Array::zeros((d.nrows(), x.len()));
                x.iter().enumerate().for_each(|(i, &j)| {
                    d_x.column_mut(i).assign(&d.column(j));
                });
                // Compute the mean.
                let mu_x = d_x
                    .mean_axis(Axis(0))
                    .ok_or(Error::MissingSufficientStatistics)?;

                // Select the columns of the conditioning variables.
                let mut d_z = Array::zeros((d.nrows(), z.len()));
                z.iter().enumerate().for_each(|(i, &j)| {
                    d_z.column_mut(i).assign(&d.column(j));
                });
                // Compute the mean.
                let mu_z = d_z
                    .mean_axis(Axis(0))
                    .ok_or(Error::MissingSufficientStatistics)?;

                // Compute the second moment statistics.
                let m_xx = d_x.t().dot(&d_x);
                let m_xz = d_x.t().dot(&d_z);
                let m_zz = d_z.t().dot(&d_z);

                // Get the sample size.
                let n = d.nrows() as f64;

                // Accumulate the sufficient statistics.
                s += GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)?;

                Ok(())
            })?;

        // Return the sufficient statistics.
        Ok(s)
    }
}

impl CSSEstimator<GaussCPDS> for SSE<'_, GaussTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPDS> {
        // Assert variables and conditioning variables must be disjoint.
        if !x.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint(
                format!("{:?}", x),
                format!("{:?}", z),
            ));
        }
        // Get the values.
        let d = self.dataset.values();
        // Return the sufficient statistics.
        Self::fit(d.view(), x, z)
    }
}

impl ParCSSEstimator<GaussCPDS> for SSE<'_, GaussTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPDS> {
        // Assert variables and conditioning variables must be disjoint.
        if !x.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint(
                format!("{:?}", x),
                format!("{:?}", z),
            ));
        }

        // Initialize the sufficient statistics.
        let s_xz = {
            let n = 0.;
            let mu_x = Array::zeros(x.len());
            let mu_z = Array::zeros(z.len());
            let m_xx = Array::zeros((x.len(), x.len()));
            let m_xz = Array::zeros((x.len(), z.len()));
            let m_zz = Array::zeros((z.len(), z.len()));
            GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)?
        };

        // Get the values.
        let d = self.dataset.values();

        // Get the values.
        d.axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .into_par_iter()
            // Compute the sufficient statistics for each chunk.
            .map(|d| Self::fit(d, x, z))
            // Aggregate the sufficient statistics.
            .try_fold(|| s_xz.clone(), |a, b| Ok(a + b?))
            .try_reduce(|| s_xz.clone(), |a, b| Ok(a + b))
    }
}

impl SSE<'_, GaussWtdTable> {
    fn fit(
        d: ArrayView2<f64>,
        norm_w: ArrayView2<f64>,
        sum_w: f64,
        x: &Set<usize>,
        z: &Set<usize>,
    ) -> Result<GaussCPDS> {
        // Initialize the sufficient statistics.
        let mut s = {
            let n = 0.;
            let mu_x = Array::zeros(x.len());
            let mu_z = Array::zeros(z.len());
            let m_xx = Array::zeros((x.len(), x.len()));
            let m_xz = Array::zeros((x.len(), z.len()));
            let m_zz = Array::zeros((z.len(), z.len()));
            GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)?
        };

        // Initialize the chunk buffers.
        d.axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .zip(norm_w.axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH))
            .try_for_each(|(d, w)| -> Result<_> {
                // Compute the root weights for centering.
                let sqrt_w = w.mapv(f64::sqrt);

                // Select the columns of the variables.
                let mut d_x = Array::zeros((d.nrows(), x.len()));
                x.iter().enumerate().for_each(|(i, &j)| {
                    azip!((c_i in &mut d_x.column_mut(i), c_j in d.column(j), w in &sqrt_w.column(0)) *c_i = c_j * w);
                });
                // Compute the weighted mean.
                let mu_x = (&d_x * &sqrt_w).sum_axis(Axis(0));

                // Select the columns of the conditioning variables.
                let mut d_z = Array::zeros((d.nrows(), z.len()));
                z.iter().enumerate().for_each(|(i, &j)| {
                    azip!((c_i in &mut d_z.column_mut(i), c_j in d.column(j), w in &sqrt_w.column(0)) *c_i = c_j * w);
                });
                // Compute the weighted mean.
                let mu_z = (&d_z * &sqrt_w).sum_axis(Axis(0));

                // Compute the weighted second moment statistics.
                let m_xx = d_x.t().dot(&d_x) * sum_w;
                let m_xz = d_x.t().dot(&d_z) * sum_w;
                let m_zz = d_z.t().dot(&d_z) * sum_w;

                // Get the sample (mass) size.
                let w_sum = w.sum();
                let n = w_sum * sum_w;

                // Accumulate the sufficient statistics.
                if w_sum > 0. {
                    s += GaussCPDS::new(mu_x / w_sum, mu_z / w_sum, m_xx, m_xz, m_zz, n)?;
                }
                Ok(())
            })?;

        // Return the sufficient statistics.
        Ok(s)
    }
}

impl CSSEstimator<GaussCPDS> for SSE<'_, GaussWtdTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPDS> {
        // Assert variables and conditioning variables must be disjoint.
        if !x.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint(
                format!("{:?}", x),
                format!("{:?}", z),
            ));
        }

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
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPDS> {
        // Assert variables and conditioning variables must be disjoint.
        if !x.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint(
                format!("{:?}", x),
                format!("{:?}", z),
            ));
        }

        // Initialize the sufficient statistics.
        let s_xz = {
            let n = 0.;
            let mu_x = Array::zeros(x.len());
            let mu_z = Array::zeros(z.len());
            let m_xx = Array::zeros((x.len(), x.len()));
            let m_xz = Array::zeros((x.len(), z.len()));
            let m_zz = Array::zeros((z.len(), z.len()));
            GaussCPDS::new(mu_x, mu_z, m_xx, m_xz, m_zz, n)?
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
            .try_fold(|| s_xz.clone(), |a, b| Ok(a + b?))
            .try_reduce(|| s_xz.clone(), |a, b| Ok(a + b))
    }
}

impl CSSEstimator<GaussCPDS> for SSE<'_, GaussIncTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPDS> {
        // Get the union of X and Z.
        let x_z = Some(&(x | z));
        // Get the missing method or default to PW.
        let m = self.missing_method.as_ref().unwrap_or(&MissingMethod::PW);
        // Get the missing mechanism or default to None.
        let r = self.missing_mechanism.as_ref();

        // Apply the missing handling method.
        let d = self.dataset.apply_missing_method(m, x_z, r)?;

        // Get the labels of the original dataset.
        let labels = self.dataset.labels();
        // Map the indices from the original dataset to the new one.
        let x = d.indices_from(x, labels)?;
        let z = d.indices_from(z, labels)?;

        // Estimate based on the resulting dataset.
        d.map_either(
            |d| SSE::new(&d).fit(&x, &z), // Complete case.
            |d| SSE::new(&d).fit(&x, &z), // Weighted case.
        )
        .into_inner()
    }
}

impl ParCSSEstimator<GaussCPDS> for SSE<'_, GaussIncTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPDS> {
        // Get the union of X and Z.
        let x_z = Some(&(x | z));
        // Get the missing method or default to PW.
        let m = self.missing_method.as_ref().unwrap_or(&MissingMethod::PW);
        // Get the missing mechanism or default to None.
        let r = self.missing_mechanism.as_ref();

        // Apply the missing handling method.
        let d = self.dataset.apply_missing_method(m, x_z, r)?;

        // Get the labels of the original dataset.
        let labels = self.dataset.labels();
        // Map the indices from the original dataset to the new one.
        let x = d.indices_from(x, labels)?;
        let z = d.indices_from(z, labels)?;

        // Estimate based on the resulting dataset.
        d.map_either(
            |d| SSE::new(&d).par_fit(&x, &z), // Complete case.
            |d| SSE::new(&d).par_fit(&x, &z), // Weighted case.
        )
        .into_inner()
    }
}
