use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{Dataset, GaussTable, GaussWtdTable},
    estimation::{CSSEstimator, ParCSSEstimator, SSE},
    models::GaussCPDS,
    types::{AXIS_CHUNK_LENGTH, Set},
};

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
        self.dataset
            .values()
            .axis_chunks_iter(Axis(0), AXIS_CHUNK_LENGTH)
            .into_par_iter()
            .map(|d| {
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
            })
            // Aggregate the sufficient statistics.
            .fold(|| s_xz.clone(), |a, b| a + b)
            .reduce(|| s_xz.clone(), |a, b| a + b)
    }
}

impl CSSEstimator<GaussCPDS> for SSE<'_, GaussWtdTable> {
    fn fit(&self, _x: &Set<usize>, _z: &Set<usize>) -> GaussCPDS {
        todo!() // FIXME:
    }
}

impl ParCSSEstimator<GaussCPDS> for SSE<'_, GaussWtdTable> {
    fn par_fit(&self, _x: &Set<usize>, _z: &Set<usize>) -> GaussCPDS {
        todo!() // FIXME:
    }
}
