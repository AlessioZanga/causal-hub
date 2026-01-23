use dry::macro_for;
use ndarray::prelude::*;
use ndarray_linalg::Determinant;

use crate::{
    datasets::{GaussIncTable, GaussTable, GaussWtdTable},
    estimators::{CPDEstimator, CSSEstimator, MLE, ParCPDEstimator, ParCSSEstimator, SSE},
    models::{GaussCPD, GaussCPDP, GaussCPDS, Labelled},
    types::{Error, LN_2_PI, Labels, Result, Set},
    utils::PseudoInverse,
};

impl MLE<'_, GaussTable> {
    fn fit(
        labels: &Labels,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: GaussCPDS,
    ) -> Result<GaussCPD> {
        // Get the sample scatter matrices and size.
        let (mu_x, mu_z, s_xx, s_xz, s_zz, n) = (
            sample_statistics.sample_response_mean(),
            sample_statistics.sample_design_mean(),
            sample_statistics.sample_response_covariance(),
            sample_statistics.sample_cross_covariance(),
            sample_statistics.sample_design_covariance(),
            sample_statistics.sample_size(),
        );

        // Compute the parameters in closed form.
        let (a, b, s) = if z.is_empty() {
            // Compute the parameters as the empirical mean and covariance.
            let a = Array2::zeros((x.len(), 0));
            let b = mu_x.clone();
            let s = s_xx / n;
            // Return the parameters.
            (a, b, s)
        } else {
            // Compute the pseudo-inverse of S_zz.
            let s_zz_pinv = s_zz.pinv()?;
            // Compute the coefficient matrix.
            let a = s_xz.dot(&s_zz_pinv);
            // Compute the intercept vector.
            let b = mu_x - &a.dot(mu_z);
            // Compute the covariance matrix.
            let s = (s_xx - &a.dot(&s_xz.t())) / n;
            // Return the parameters.
            (a, b, s)
        };

        // Compute the sample log-likelihood.
        let p = x.len() as f64;
        let (_, ln_det) = s
            .sln_det()
            .map_err(|e| Error::Linalg(format!("Failed to compute determinant of S: {e}")))?;
        let sample_log_likelihood = -0.5 * n * (p * LN_2_PI + ln_det + p);

        // Construct the CPD parameters.
        let parameters = GaussCPDP::new(a, b, s)?;

        // Subset the conditioning labels, states and shape.
        let conditioning_labels = z.iter().map(|&i| labels[i].clone()).collect();
        // Get the labels of the conditioned variables.
        let labels = x.iter().map(|&i| labels[i].clone()).collect();

        // Wrap the sample statistics in an option.
        let sample_statistics = Some(sample_statistics);
        // Wrap the sample log-likelihood in an option.
        let sample_log_likelihood = Some(sample_log_likelihood);

        // Construct the CPD.
        GaussCPD::with_optionals(
            labels,
            conditioning_labels,
            parameters,
            sample_statistics,
            sample_log_likelihood,
        )
    }
}

// Implement the GaussCPD estimator for the MLE struct.
macro_for!($type in [GaussTable, GaussIncTable, GaussWtdTable] {

    impl CPDEstimator<GaussCPD> for MLE<'_, $type> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPD> {
            // Get labels.
            let labels = self.dataset.labels();
            // Set sufficient statistics estimator.
            let sample_statistics = SSE::new(self.dataset);
            // Set missing handling method, if any.
            let sample_statistics = sample_statistics.with_missing_method(
                self.missing_method,
                self.missing_mechanism.clone()
            );
            // Compute sufficient statistics.
            let sample_statistics = sample_statistics.fit(x, z)?;
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, GaussTable>::fit(labels, x, z, sample_statistics)
        }
    }

    impl ParCPDEstimator<GaussCPD> for MLE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<GaussCPD> {
            // Get labels.
            let labels = self.dataset.labels();
            // Set sufficient statistics estimator.
            let sample_statistics = SSE::new(self.dataset);
            // Set missing handling method, if any.
            let sample_statistics = sample_statistics.with_missing_method(
                self.missing_method,
                self.missing_mechanism.clone()
            );
            // Compute sufficient statistics in parallel.
            let sample_statistics = sample_statistics.par_fit(x, z)?;
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, GaussTable>::fit(labels, x, z, sample_statistics)
        }
    }

});
