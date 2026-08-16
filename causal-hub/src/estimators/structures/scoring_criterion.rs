use std::marker::PhantomData;

use dry::macro_for;

use crate::{
    estimators::CPDEstimator,
    models::{CIM, CPD, CatCIM, CatCPD, GaussCPD, Labelled},
    types::{Error, Labels, Result, Set},
};

/// A trait for scoring criteria used in score-based structure learning.
pub trait ScoringCriterion {
    /// Computes the score for a given variable and its conditioning set.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to score.
    /// * `z` - The conditioning set.
    ///
    /// # Returns
    ///
    /// The computed score.
    ///
    fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64>;
}

macro_rules! impl_scoring_struct {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        pub struct $name<'a, E, P> {
            _p_marker: PhantomData<P>,
            estimator: &'a E,
        }

        impl<'a, E, P> $name<'a, E, P> {
            #[doc = concat!("Creates a new `", stringify!($name), "` instance.")]
            ///
            /// # Arguments
            ///
            /// * `estimator` - A reference to the estimator.
            ///
            /// # Returns
            ///
            #[doc = concat!("A new `", stringify!($name), "` instance.")]
            #[inline]
            pub const fn new(estimator: &'a E) -> Self {
                Self {
                    _p_marker: PhantomData,
                    estimator,
                }
            }
        }

        impl<'a, E, P> Labelled for $name<'a, E, P>
        where
            E: Labelled,
        {
            #[inline]
            fn labels(&self) -> &Labels {
                self.estimator.labels()
            }
        }
    };
}

impl_scoring_struct!(LL, "The Log Likelihood (`LL`).");
impl_scoring_struct!(AIC, "The Akaike Information Criterion (`AIC`).");
impl_scoring_struct!(AICC, "The Akaike Information Criterion Corrected (`AICc`).");
impl_scoring_struct!(BIC, "The Bayesian Information Criterion (`BIC`).");
impl_scoring_struct!(
    BICC,
    "The Bayesian Information Criterion Corrected (`BICc`)."
);
impl_scoring_struct!(HQC, "The Hannan-Quinn Criterion (`HQC`).");

macro_for!($type in [CatCPD, GaussCPD, CatCIM] {

    impl<E> ScoringCriterion for LL<'_, E, $type>
    where
        E: CPDEstimator<$type>,
    {
        #[inline]
        fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64> {
            // Compute the intensity matrices for the sets.
            let p_xz = self.estimator.fit(x, z)?;
            // Get the log-likelihood.
            let log_likelihood = p_xz
                .fitted_log_likelihood()
                .ok_or_else(|| Error::MissingLogLikelihood())?;

            // Compute the score.
            Ok(log_likelihood)
        }
    }

    impl<E> ScoringCriterion for AIC<'_, E, $type>
    where
        E: CPDEstimator<$type>,
    {
        #[inline]
        fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64> {
            // Compute the intensity matrices for the sets.
            let p_xz = self.estimator.fit(x, z)?;
            // Get the log-likelihood.
            let log_likelihood = p_xz
                .fitted_log_likelihood()
                .ok_or_else(|| Error::MissingLogLikelihood())?;
            // Get the number of parameters.
            let k = p_xz.parameters_size() as f64;

            // Compute the score.
            Ok(log_likelihood - k)
        }
    }

    impl<E> ScoringCriterion for AICC<'_, E, $type>
    where
        E: CPDEstimator<$type>,
    {
        #[inline]
        fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64> {
            // Compute the intensity matrices for the sets.
            let p_xz = self.estimator.fit(x, z)?;
            // Get the sample size.
            let n = p_xz
                .fitted_statistics()
                .ok_or_else(|| Error::MissingSufficientStatistics())?
                .fitted_size();
            // Get the log-likelihood.
            let log_likelihood = p_xz
                .fitted_log_likelihood()
                .ok_or_else(|| Error::MissingLogLikelihood())?;
            // Get the number of parameters.
            let k = p_xz.parameters_size() as f64;

            // Apply sample size correction.
            let c = n / (f64::max(n - k - 2., 1.));

            // Compute the score.
            Ok(log_likelihood - k * c)
        }
    }

    impl<E> ScoringCriterion for BIC<'_, E, $type>
    where
        E: CPDEstimator<$type>,
    {
        #[inline]
        fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64> {
            // Compute the intensity matrices for the sets.
            let p_xz = self.estimator.fit(x, z)?;
            // Get the sample size.
            let n = p_xz
                .fitted_statistics()
                .ok_or_else(|| Error::MissingSufficientStatistics())?
                .fitted_size();
            // Get the log-likelihood.
            let log_likelihood = p_xz
                .fitted_log_likelihood()
                .ok_or_else(|| Error::MissingLogLikelihood())?;
            // Get the number of parameters.
            let k = p_xz.parameters_size() as f64;

            // Compute the score.
            Ok(log_likelihood - 0.5 * k * f64::ln(n))
        }
    }

    impl<E> ScoringCriterion for BICC<'_, E, $type>
    where
        E: CPDEstimator<$type>,
    {
        #[inline]
        fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64> {
            // Compute the intensity matrices for the sets.
            let p_xz = self.estimator.fit(x, z)?;
            // Get the sample size.
            let n = p_xz
                .fitted_statistics()
                .ok_or_else(|| Error::MissingSufficientStatistics())?
                .fitted_size();
            // Get the log-likelihood.
            let log_likelihood = p_xz
                .fitted_log_likelihood()
                .ok_or_else(|| Error::MissingLogLikelihood())?;
            // Get the number of parameters.
            let k = p_xz.parameters_size() as f64;

            // Apply sample size correction.
            let c = n / (f64::max(n - k - 2., 1.));

            // Compute the score.
            Ok(log_likelihood - 0.5 * k * c * f64::ln(n))
        }
    }

    impl<E> ScoringCriterion for HQC<'_, E, $type>
    where
        E: CPDEstimator<$type>,
    {
        #[inline]
        fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64> {
            // Compute the intensity matrices for the sets.
            let p_xz = self.estimator.fit(x, z)?;
            // Get the sample size.
            let n = p_xz
                .fitted_statistics()
                .ok_or_else(|| Error::MissingSufficientStatistics())?
                .fitted_size();
            // Get the log-likelihood.
            let log_likelihood = p_xz
                .fitted_log_likelihood()
                .ok_or_else(|| Error::MissingLogLikelihood())?;
            // Get the number of parameters.
            let k = p_xz.parameters_size() as f64;

            // Compute the score.
            Ok(log_likelihood - 0.5 * k * f64::ln(f64::ln(n)))
        }
    }

});
