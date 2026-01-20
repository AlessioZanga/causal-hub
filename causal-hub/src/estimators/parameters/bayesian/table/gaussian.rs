use dry::macro_for;
use ndarray::prelude::*;
use ndarray_linalg::Determinant;

use crate::{
    datasets::{GaussIncTable, GaussTable, GaussWtdTable},
    estimators::{BE, CPDEstimator, CSSEstimator, ParCPDEstimator, ParCSSEstimator, SSE},
    models::{GaussCPD, GaussCPDP, GaussCPDS, Labelled},
    types::{LN_2_PI, Labels, Set},
    utils::PseudoInverse,
};

impl BE<'_, GaussTable, f64> {
    fn fit(
        labels: &Labels,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: GaussCPDS,
        prior: f64,
    ) -> GaussCPD {
        // Assert likelihood of prior.
        assert!(prior >= 0.0, "Prior must be non-negative.");

        // Get the sample covariance matrices and size.
        let (mu_x, mu_z, s_xx, s_xz, s_zz, n) = (
            sample_statistics.sample_response_mean(),
            sample_statistics.sample_design_mean(),
            sample_statistics.sample_response_covariance(),
            sample_statistics.sample_cross_covariance(),
            sample_statistics.sample_design_covariance(),
            sample_statistics.sample_size(),
        );

        // Apply prior (Pseudo-counts style).
        // Assume prior ~ N(0, I) with weight `prior` (nu).
        let nu = prior;
        let n_post = n + nu;

        // Update means.
        let mu_x_post = mu_x * (n / n_post);
        let mu_z_post = mu_z * (n / n_post);

        // Update Scatter Matrices (Centered Covariances).
        // Helper for rank-1 update: (n * nu / n_post) * mu * mu^T.
        let f = n * nu / n_post;

        // S_xx.
        let mut s_xx_post = s_xx.clone();
        // Add nu * I to diagonal.
        for i in 0..s_xx_post.nrows() {
            s_xx_post[[i, i]] += nu;
        }
        // Add mean adjustment.
        s_xx_post = s_xx_post
            + f * &mu_x
                .view()
                .insert_axis(Axis(1))
                .dot(&mu_x.view().insert_axis(Axis(0)));

        // S_zz.
        let mut s_zz_post = s_zz.clone();
        for i in 0..s_zz_post.nrows() {
            s_zz_post[[i, i]] += nu;
        }
        s_zz_post = s_zz_post
            + f * &mu_z
                .view()
                .insert_axis(Axis(1))
                .dot(&mu_z.view().insert_axis(Axis(0)));

        // S_xz.
        let s_xz_post = s_xz
            + f * &mu_x
                .view()
                .insert_axis(Axis(1))
                .dot(&mu_z.view().insert_axis(Axis(0)));

        // Compute the parameters in closed form.
        let (a, b, s) = if z.is_empty() {
            // Compute the parameters as the empirical mean and covariance.
            let a = Array2::zeros((x.len(), 0));
            let b = mu_x_post.clone();
            let s = s_xx_post / n_post;
            // Return the parameters.
            (a, b, s)
        } else {
            // Compute the pseudo-inverse of S_zz.
            let s_zz_pinv = s_zz_post.pinv();
            // Compute the coefficient matrix.
            let a = s_xz_post.dot(&s_zz_pinv);
            // Compute the intercept vector.
            let b = mu_x_post - &a.dot(&mu_z_post);
            // Compute the covariance matrix.
            let s = (s_xx_post - &a.dot(&s_xz_post.t())) / n_post;
            // Return the parameters.
            (a, b, s)
        };

        // Compute the sample log-likelihood.
        let p = x.len() as f64;
        let (_, ln_det) = s.sln_det().expect("Failed to compute determinant of S.");
        // This is the likelihood of the posterior "samples".
        let sample_log_likelihood = -0.5 * n_post * (p * LN_2_PI + ln_det + p);

        // Construct the CPD parameters.
        let parameters = GaussCPDP::new(a, b, s);

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

// Implement the GaussCPD estimator for the BE struct.
macro_for!($type in [GaussTable, GaussIncTable, GaussWtdTable] {

    impl CPDEstimator<GaussCPD> for BE<'_, $type, f64> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPD {
            // Get labels.
            let labels = self.dataset.labels();
            // Get prior.
            let prior = self.prior;
            // Set sufficient statistics estimator.
            let sample_statistics = SSE::new(self.dataset);
            // Set missing handling method, if any.
            let sample_statistics = sample_statistics.with_missing_method(
                self.missing_method,
                self.missing_mechanism.clone()
            );
            // Compute sufficient statistics.
            let sample_statistics = sample_statistics.fit(x, z);
            // Fit the CPD given the sufficient statistics.
            BE::<'_, GaussTable, f64>::fit(labels, x, z, sample_statistics, prior)
        }
    }

    impl ParCPDEstimator<GaussCPD> for BE<'_, $type, f64> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPD {
            // Get labels.
            let labels = self.dataset.labels();
            // Get prior.
            let prior = self.prior;
            // Set sufficient statistics estimator.
            let sample_statistics = SSE::new(self.dataset);
            // Set missing handling method, if any.
            let sample_statistics = sample_statistics.with_missing_method(
                self.missing_method,
                self.missing_mechanism.clone()
            );
            // Compute sufficient statistics.
            let sample_statistics = sample_statistics.par_fit(x, z);
            // Fit the CPD given the sufficient statistics.
            BE::<'_, GaussTable, f64>::fit(labels, x, z, sample_statistics, prior)
        }
    }

    impl CPDEstimator<GaussCPD> for BE<'_, $type, ()> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPD {
            // Default to BDeu prior? No, BGe equivalent standard normal.
            self.clone().with_prior(1.0).fit(x, z)
        }
    }

    impl ParCPDEstimator<GaussCPD> for BE<'_, $type, ()> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPD {
            // Default to BDeu prior? No, BGe equivalent standard normal.
            self.clone().with_prior(1.0).par_fit(x, z)
        }
    }
});
