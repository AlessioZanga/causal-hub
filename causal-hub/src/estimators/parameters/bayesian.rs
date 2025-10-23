use dry::macro_for;
use ndarray::prelude::*;
use statrs::function::gamma::ln_gamma;

use crate::{
    datasets::{CatTable, CatTrj, CatTrjs, CatWtdTable, CatWtdTrj, CatWtdTrjs},
    estimators::{
        CIMEstimator, CPDEstimator, CSSEstimator, ParCIMEstimator, ParCPDEstimator,
        ParCSSEstimator, SSE,
    },
    models::{CatCIM, CatCIMS, CatCPD, CatCPDS, Labelled},
    types::{Labels, Set, States},
};

/// A struct representing a Bayesian estimator.
#[derive(Clone, Copy, Debug)]
pub struct BE<'a, D, T> {
    dataset: &'a D,
    prior: T,
}

impl<'a, D> BE<'a, D, ()> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to the dataset to fit the estimator to.
    ///
    /// # Returns
    ///
    /// A new Bayesian estimator.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self { dataset, prior: () }
    }
}

impl<'a, D, T> BE<'a, D, T> {
    /// Sets the prior distribution.
    ///
    /// # Arguments
    ///
    /// * `prior` - The prior distribution to set.
    ///
    /// # Returns
    ///
    /// A new Bayesian estimator with the specified prior.
    ///
    #[inline]
    pub fn with_prior<U>(self, prior: U) -> BE<'a, D, U> {
        BE {
            dataset: self.dataset,
            prior,
        }
    }
}

impl<D, T> Labelled for BE<'_, D, T>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

impl BE<'_, CatTable, usize> {
    // Fit a CPD given sufficient statistics.
    fn fit(
        states: &States,
        shape: &Array1<usize>,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: CatCPDS,
        prior: usize,
    ) -> CatCPD {
        // Get the conditional counts.
        let n_xz = sample_statistics.sample_conditional_counts();
        // Marginalize the counts.
        let n_z = n_xz.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = prior;
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");

        // Cast alpha to floating point.
        let alpha = alpha as f64;

        // Add the prior to the counts.
        let n_xz = n_xz + alpha;
        let n_z = n_z + alpha * x.iter().map(|&i| shape[i]).product::<usize>() as f64;
        // Compute the parameters by normalizing the counts with the prior.
        let parameters = &n_xz / &n_z;

        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some((&n_xz * parameters.ln()).sum());

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

// Implement the CPD estimator for the BE struct.
macro_for!($type in [CatTable, CatWtdTable] {

    impl CPDEstimator<CatCPD> for BE<'_, $type, ()> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Default to uniform prior.
            self.clone().with_prior(1).fit(x, z)
        }
    }

    impl CPDEstimator<CatCPD> for BE<'_, $type, usize> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Get (states, shape, prior).
            let (states, shape, prior) = (self.dataset.states(), self.dataset.shape(), self.prior);
            // Compute sufficient statistics.
            let sample_statistics = SSE::new(self.dataset).fit(x, z);
            // Fit the CPD given the sufficient statistics.
            BE::<'_, CatTable, _>::fit(states, shape, x, z, sample_statistics, prior)
        }
    }

    impl ParCPDEstimator<CatCPD> for BE<'_, $type, ()> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Default to uniform prior.
            self.clone().with_prior(1).fit(x, z)
        }
    }

    impl ParCPDEstimator<CatCPD> for BE<'_, $type, usize> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
            // Get (states, shape, prior).
            let (states, shape, prior) = (self.dataset.states(), self.dataset.shape(), self.prior);
            // Compute sufficient statistics in parallel.
            let sample_statistics = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CPD given the sufficient statistics.
            BE::<'_, CatTable, _>::fit(states, shape, x, z, sample_statistics, prior)
        }
    }

});

impl BE<'_, CatTrj, (usize, f64)> {
    // Fit a CIM given sufficient statistics.
    fn fit(
        states: &States,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: CatCIMS,
        prior: (usize, f64),
    ) -> CatCIM {
        // Get the prior, as the alpha of Dirichlet and tau of Gamma.
        let (alpha, tau) = prior;
        // Assert alpha is positive.
        assert!(alpha > 0, "Alpha must be positive.");
        // Assert tau is positive.
        assert!(tau > 0.0, "Tau must be positive.");

        // Get the conditional counts and times.
        let n_xz = sample_statistics.sample_conditional_counts();
        let t_xz = sample_statistics.sample_conditional_times();

        // Insert axis to align the dimensions.
        let t_xz = &t_xz.clone().insert_axis(Axis(2));

        // Get the shape of the conditioning variables.
        let s_z = n_xz.shape()[0] as f64;
        // Scale the prior by the shape.
        let alpha = alpha as f64 / s_z;
        let tau = tau / s_z;

        // Add the prior to the counts and times.
        let n_xz = n_xz + alpha;
        let t_xz = t_xz + tau;
        // Estimate the parameters by normalizing the counts.
        let mut parameters = &n_xz / &t_xz;
        // Fix the diagonal.
        parameters.outer_iter_mut().for_each(|mut q| {
            // Fill the diagonal with zeros.
            q.diag_mut().fill(0.);
            // Compute the negative sum of the rows.
            let q_neg_sum = -q.sum_axis(Axis(1));
            // Assign the negative sum to the diagonal.
            q.diag_mut().assign(&q_neg_sum);
        });

        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some({
            // Sum counts.
            let n_z = n_xz.sum_axis(Axis(2));
            let t_z = t_xz.sum_axis(Axis(2));
            // Compute the sample log-likelihood.
            let ll_q_xz = {
                // Compute the sample log-likelihood.
                (&n_z + 1.).mapv(ln_gamma).sum() + (alpha + 1.) * f64::ln(tau) //.
                - (ln_gamma(alpha + 1.) + ((&n_z + 1.) * &t_z.ln()).sum())
            };
            // Compute the sample log-likelihood.
            let ll_p_xz = {
                // Compute the sample log-likelihood.
                (ln_gamma(alpha) - n_z.mapv(ln_gamma).sum())     //.
                + (ln_gamma(alpha) - n_xz.mapv(ln_gamma).sum())
            };
            // Return the total log-likelihood.
            ll_q_xz + ll_p_xz
        });

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

// Implement the CIM estimator for the BE struct.
macro_for!($type in [CatTrj, CatWtdTrj, CatTrjs, CatWtdTrjs] {

    impl CIMEstimator<CatCIM> for BE<'_, $type, ()> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Default to uniform prior.
            self.clone().with_prior((1, 1.)).fit(x, z)
        }
    }

    impl CIMEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), self.prior);
            // Compute sufficient statistics.
            let sample_statistics = SSE::new(self.dataset).fit(x, z);
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit(states, x, z, sample_statistics, prior)
        }
    }

});

// Implement the parallel CIM estimator for the BE struct.
macro_for!($type in [CatTrjs, CatWtdTrjs] {

    impl ParCIMEstimator<CatCIM> for BE<'_, $type, ()> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Default to uniform prior.
            self.clone().with_prior((1, 1.)).fit(x, z)
        }
    }

    impl ParCIMEstimator<CatCIM> for BE<'_, $type, (usize, f64)> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
            // Get (states, prior).
            let (states, prior) = (self.dataset.states(), self.prior);
            // Compute sufficient statistics in parallel.
            let sample_statistics = SSE::new(self.dataset).par_fit(x, z);
            // Fit the CIM given the sufficient statistics.
            BE::<'_, CatTrj, _>::fit(states, x, z, sample_statistics, prior)
        }
    }

});
