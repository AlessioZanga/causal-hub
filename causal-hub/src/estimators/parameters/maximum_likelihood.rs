use dry::macro_for;
use ndarray::prelude::*;
use ndarray_linalg::Determinant;

use crate::{
    datasets::{
        CatTable, CatTrj, CatTrjs, CatWtdTable, CatWtdTrj, CatWtdTrjs, GaussTable, GaussWtdTable,
    },
    estimators::{
        CIMEstimator, CPDEstimator, CSSEstimator, ParCIMEstimator, ParCPDEstimator,
        ParCSSEstimator, SSE,
    },
    models::{CatCIM, CatCIMS, CatCPD, CatCPDS, GaussCPD, GaussCPDP, GaussCPDS, Labelled},
    types::{LN_2_PI, Labels, Set, States},
    utils::PseudoInverse,
};

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Copy, Debug)]
pub struct MLE<'a, D> {
    dataset: &'a D,
}

impl<'a, D> MLE<'a, D> {
    /// Creates a new maximum likelihood estimator.
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to the dataset to fit the estimator to.
    ///
    /// # Returns
    ///
    /// A new `MaximumLikelihoodEstimator` instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self { dataset }
    }
}

impl<D> Labelled for MLE<'_, D>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

impl MLE<'_, CatTable> {
    fn fit(states: &States, x: &Set<usize>, z: &Set<usize>, sample_statistics: CatCPDS) -> CatCPD {
        // Get the conditional counts.
        let n_xz = sample_statistics.sample_conditional_counts();
        // Marginalize the counts.
        let n_z = &n_xz.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&x| x > 0.),
            "Failed to get non-zero counts.",
        );

        // Compute the parameters by normalizing the counts.
        let parameters = n_xz / n_z;

        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = (n_xz * (&parameters + eps).ln()).sum();

        // Subset the conditioning labels, states and shape.
        let conditioning_states = z
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();
        // Get the labels of the conditioned variables.
        let states = x
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();

        // Wrap the sample statistics in an option.
        let sample_statistics = Some(sample_statistics);
        // Wrap the sample log-likelihood in an option.
        let sample_log_likelihood = Some(sample_log_likelihood);

        // Construct the CPD.
        CatCPD::with_optionals(
            states,
            conditioning_states,
            parameters,
            sample_statistics,
            sample_log_likelihood,
        )
    }
}

// Implement the CatCPD estimator for the MLE struct.
macro_for!($type in [CatTable, CatWtdTable] {

    impl CPDEstimator<CatCPD> for MLE<'_, $type> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Get states.
            let states = self.dataset.states();
            // Compute sufficient statistics.
            let sample_statistics = SSE::new(self.dataset).fit(x, z);
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, CatTable>::fit(states, x, z, sample_statistics)
        }
    }

    impl ParCPDEstimator<CatCPD> for MLE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Get states.
            let states = self.dataset.states();
            // Compute sufficient statistics in parallel.
            let sample_statistics = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, CatTable>::fit(states, x, z, sample_statistics)
        }
    }

});

impl MLE<'_, GaussTable> {
    fn fit(
        labels: &Labels,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: GaussCPDS,
    ) -> GaussCPD {
        // Get the sample covariance matrices and size.
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
            let s_zz_pinv = s_zz.pinv();
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
        let (_, ln_det) = s.sln_det().expect("Failed to compute determinant of S.");
        let sample_log_likelihood = -0.5 * n * (p * LN_2_PI + ln_det + p);

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

// Implement the GaussCPD estimator for the MLE struct.
macro_for!($type in [GaussTable, GaussWtdTable] {

    impl CPDEstimator<GaussCPD> for MLE<'_, $type> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPD {
            // Get labels.
            let labels = self.dataset.labels();
            // Compute sufficient statistics.
            let sample_statistics = SSE::new(self.dataset).fit(x, z);
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, GaussTable>::fit(labels, x, z, sample_statistics)
        }
    }

    impl ParCPDEstimator<GaussCPD> for MLE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> GaussCPD {
            // Get labels.
            let labels = self.dataset.labels();
            // Compute sufficient statistics in parallel.
            let sample_statistics = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CPD given the sufficient statistics.
            MLE::<'_, GaussTable>::fit(labels, x, z, sample_statistics)
        }
    }

});

impl MLE<'_, CatTrj> {
    // Fit a CIM given sufficient statistics.
    fn fit(states: &States, x: &Set<usize>, z: &Set<usize>, sample_statistics: CatCIMS) -> CatCIM {
        // Get the conditional counts and times.
        let n_xz = sample_statistics.sample_conditional_counts();
        let t_xz = sample_statistics.sample_conditional_times();

        // Assert the conditional times counts are not zero.
        assert!(
            t_xz.iter().all(|&x| x > 0.),
            "Failed to get non-zero conditional times."
        );

        // Insert axis to align the dimensions.
        let t_xz = &t_xz.clone().insert_axis(Axis(2));

        // Estimate the parameters by normalizing the counts.
        let mut parameters = n_xz / t_xz;
        // Fix the diagonal.
        parameters.outer_iter_mut().for_each(|mut q| {
            // Fill the diagonal with zeros.
            q.diag_mut().fill(0.);
            // Compute the negative sum of the rows.
            let q_neg_sum = -q.sum_axis(Axis(1));
            // Assign the negative sum to the diagonal.
            q.diag_mut().assign(&q_neg_sum);
        });

        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = {
            // Compute the sample log-likelihood.
            let ll_q_xz = {
                // Sum counts, aligning the dimensions.
                let n_z = n_xz.sum_axis(Axis(2));
                let t_z = t_xz.sum_axis(Axis(2));
                // Clone the parameters.
                let mut q_z = Array::zeros(n_z.dim());
                // Get the diagonals.
                parameters
                    .outer_iter()
                    .zip(q_z.outer_iter_mut())
                    .for_each(|(p, mut q)| {
                        q.assign(&(-&p.diag()));
                    });
                // Compute the sample log-likelihood.
                (&n_z * (&q_z + eps).ln()).sum() + (-&q_z * &t_z).sum()
            };
            // Compute the sample log-likelihood.
            let ll_p_xz = {
                // Clone the parameters.
                let mut p_xz = parameters.clone();
                // Set diagonal to zero.
                p_xz.outer_iter_mut().for_each(|mut p| {
                    // Fill the diagonal with zeros.
                    p.diag_mut().fill(0.);
                });
                // Normalize the parameters, align the dimensions.
                p_xz /= &p_xz.sum_axis(Axis(2)).insert_axis(Axis(2));
                // Compute the sample log-likelihood.
                (n_xz * (p_xz + eps).ln()).sum()
            };
            // Return the total log-likelihood.
            ll_q_xz + ll_p_xz
        };

        // Subset the conditioning labels, states and shape.
        let conditioning_states = z
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();
        // Get the labels of the conditioned variables.
        let states = x
            .iter()
            .map(|&i| {
                let (k, v) = states.get_index(i).unwrap();
                (k.clone(), v.clone())
            })
            .collect();

        // Wrap the sufficient statistics in an option.
        let sample_statistics = Some(sample_statistics);
        // Wrap the sample log-likelihood in an option.
        let sample_log_likelihood = Some(sample_log_likelihood);

        // Construct the CIM.
        CatCIM::with_optionals(
            states,
            conditioning_states,
            parameters,
            sample_statistics,
            sample_log_likelihood,
        )
    }
}

// Implement the CatCIM estimator for the MLE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CIMEstimator<CatCIM> for MLE<'_, $type> {
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get states.
            let states = self.dataset.states();
            // Compute sufficient statistics.
            let sample_statistics = SSE::new(self.dataset).fit(x, z);
            // Fit the CIM given the sufficient statistics.
            MLE::<'_, CatTrj>::fit(states, x, z, sample_statistics)
        }
    }

});

// Implement the parallel version of the CIM estimator for the MLE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCIMEstimator<CatCIM> for MLE<'_, $type> {
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get states.
            let states = self.dataset.states();
            // Compute sufficient statistics in parallel.
            let sample_statistics = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CIM given the sufficient statistics.
            MLE::<'_, CatTrj>::fit(states, x, z, sample_statistics)
        }
    }

});
