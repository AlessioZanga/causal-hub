use dry::macro_for;
use ndarray::prelude::*;

use crate::{
    datasets::{CatIncTable, CatTable, CatWtdTable},
    estimators::{BE, CPDEstimator, CSSEstimator, ParCPDEstimator, ParCSSEstimator, SSE},
    models::{CatCPD, CatCPDS},
    types::{Error, Result, Set, States},
};

impl BE<'_, CatTable, usize> {
    // Fit a CPD given sufficient statistics.
    fn fit(
        states: &States,
        shape: &Array1<usize>,
        x: &Set<usize>,
        z: &Set<usize>,
        sample_statistics: CatCPDS,
        prior: usize,
    ) -> Result<CatCPD> {
        // Get the conditional counts.
        let n_xz = sample_statistics.sample_conditional_counts();
        // Marginalize the counts.
        let n_z = n_xz.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = prior;
        // Assert alpha is positive.
        if alpha == 0 {
            return Err(Error::IllegalArgument("Alpha must be positive.".into()));
        }

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
                let (k, v) = states
                    .get_index(i)
                    .ok_or_else(|| Error::Dataset(format!("Index {} out of bounds", i)))?;
                Ok((k.clone(), v.clone()))
            })
            .collect::<Result<_>>()?;
        // Get the labels of the conditioned variables.
        let states = x
            .iter()
            .map(|&i| {
                let (k, v) = states
                    .get_index(i)
                    .ok_or_else(|| Error::Dataset(format!("Index {} out of bounds", i)))?;
                Ok((k.clone(), v.clone()))
            })
            .collect::<Result<_>>()?;

        // Wrap the sample statistics in an option.
        let sample_statistics = Some(sample_statistics);

        // Construct the CPD.
        Ok(CatCPD::with_optionals(
            states,
            conditioning_states,
            parameters,
            sample_statistics,
            sample_log_likelihood,
        )?)
    }
}

// Implement the CPD estimator for the BE struct.
macro_for!($type in [CatTable, CatIncTable, CatWtdTable] {

    impl CPDEstimator<CatCPD> for BE<'_, $type, ()> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCPD> {
            // Default to uniform prior.
            self.clone().with_prior(1).fit(x, z)
        }
    }

    impl CPDEstimator<CatCPD> for BE<'_, $type, usize> {
        #[inline]
        fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCPD> {
            // Get (states, shape, prior).
            let (states, shape, prior) = (self.dataset.states(), self.dataset.shape(), self.prior);
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
            BE::<'_, CatTable, _>::fit(states, shape, x, z, sample_statistics, prior)
        }
    }

    impl ParCPDEstimator<CatCPD> for BE<'_, $type, ()> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCPD> {
            // Default to uniform prior.
            self.clone().with_prior(1).par_fit(x, z)
        }
    }

    impl ParCPDEstimator<CatCPD> for BE<'_, $type, usize> {
        #[inline]
        fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCPD> {
            // Get (states, shape, prior).
            let (states, shape, prior) = (self.dataset.states(), self.dataset.shape(), self.prior);
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
            BE::<'_, CatTable, _>::fit(states, shape, x, z, sample_statistics, prior)
        }
    }

});
