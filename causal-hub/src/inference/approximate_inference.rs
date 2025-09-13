use rand::{Rng, SeedableRng};

use crate::{
    datasets::CatEv,
    estimation::{CPDEstimator, MLE},
    models::{CatBN, CatCPD},
    samplers::{BNSampler, ForwardSampler, ImportanceSampler, ParBNSampler},
    types::Set,
};

/// An approximate inference engine.
#[derive(Debug)]
pub struct ApproximateInference<'a, R, M, E> {
    rng: &'a mut R,
    model: &'a M,
    evidence: Option<&'a E>,
}

impl<'a, R, M> ApproximateInference<'a, R, M, ()> {
    /// Construct a new approximate inference instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator.
    /// * `model` - A reference to the model to sample from.
    ///
    /// # Returns
    ///
    /// Return a new approximate inference instance.
    ///
    #[inline]
    pub const fn new(rng: &'a mut R, model: &'a M) -> Self {
        Self {
            rng,
            model,
            evidence: None,
        }
    }
}

impl<'a, R, M, E> ApproximateInference<'a, R, M, E> {
    /// Add evidence to the approximate inference instance.
    ///
    /// # Arguments
    ///
    /// * `evidence` - A reference to the evidence.
    ///
    /// # Returns
    ///
    /// Return a new approximate inference instance with evidence.
    ///
    #[inline]
    pub const fn with_evidence<T>(self, evidence: &'a T) -> ApproximateInference<'a, R, M, T> {
        ApproximateInference {
            rng: self.rng,
            model: self.model,
            evidence: Some(evidence),
        }
    }
}

/// A trait for approximate inference with Bayesian Networks.
pub trait BNApproxInference<T> {
    /// Predict the values of `x` conditioned on `z` using `n` samples, without evidence.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    /// * `n` - The number of samples to use for the prediction.
    ///
    /// # Returns
    ///
    /// The predicted values of `x` conditioned on `z`.
    ///
    fn predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> T;
}

impl<R: Rng> BNApproxInference<CatCPD> for ApproximateInference<'_, R, CatBN, ()> {
    fn predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> CatCPD {
        // Initialize the sampler.
        let mut sampler = ForwardSampler::new(self.rng, self.model);
        // Generate n samples from the model.
        // TODO: Avoid generating the full dataset,
        //       e.g., by only sampling the variables in X U Z, and
        //       by using batching to reduce memory usage.
        let dataset = sampler.sample_n(n);
        // Initialize the estimator.
        let estimator = MLE::new(&dataset);
        // Fit the CPD.
        estimator.fit(x, z)
    }
}

impl<R: Rng> BNApproxInference<CatCPD> for ApproximateInference<'_, R, CatBN, CatEv> {
    fn predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> CatCPD {
        // Check if evidence is actually provided.
        match self.evidence {
            // Get the evidence.
            Some(evidence) => {
                // Initialize the sampler.
                let mut sampler = ImportanceSampler::new(self.rng, self.model, evidence);
                // Generate n samples from the model.
                // TODO: Avoid generating the full dataset,
                //       e.g., by only sampling the variables in X U Z, and
                //       by using batching to reduce memory usage.
                let dataset = sampler.sample_n(n);
                // Initialize the estimator.
                let estimator = MLE::new(&dataset);
                // Fit the CPD.
                estimator.fit(x, z)
            }
            // Delegate to empty evidence case.
            None => ApproximateInference::new(self.rng, self.model).predict(x, z, n),
        }
    }
}

/// A trait for parallel Bayesian network approximate inference.
pub trait ParBNApproxInference<T> {
    /// Predict the values of `x` conditioned on `z` using `n` samples, without evidence, in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    /// * `n` - The number of samples to use for the prediction.
    ///
    /// # Returns
    ///
    /// The predicted values of `x` conditioned on `z`.
    ///
    fn par_predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> T;
}

impl<R: Rng + SeedableRng> ParBNApproxInference<CatCPD> for ApproximateInference<'_, R, CatBN, ()> {
    fn par_predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> CatCPD {
        // Initialize the sampler.
        let mut sampler = ForwardSampler::new(self.rng, self.model);
        // Generate n samples from the model.
        // TODO: Avoid generating the full dataset,
        //       e.g., by only sampling the variables in X U Z, and
        //       by using batching to reduce memory usage.
        let dataset = sampler.par_sample_n(n);
        // Initialize the estimator.
        let estimator = MLE::new(&dataset);
        // Fit the CPD.
        estimator.fit(x, z)
    }
}

impl<R: Rng + SeedableRng> ParBNApproxInference<CatCPD>
    for ApproximateInference<'_, R, CatBN, CatEv>
{
    fn par_predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> CatCPD {
        // Check if evidence is actually provided.
        match self.evidence {
            // Get the evidence.
            Some(evidence) => {
                // Initialize the sampler.
                let mut sampler = ImportanceSampler::new(self.rng, self.model, evidence);
                // Generate n samples from the model.
                // TODO: Avoid generating the full dataset,
                //       e.g., by only sampling the variables in X U Z, and
                //       by using batching to reduce memory usage.
                let dataset = sampler.par_sample_n(n);
                // Initialize the estimator.
                let estimator = MLE::new(&dataset);
                // Fit the CPD.
                estimator.fit(x, z)
            }
            // Delegate to empty evidence case.
            None => ApproximateInference::new(self.rng, self.model).predict(x, z, n),
        }
    }
}
