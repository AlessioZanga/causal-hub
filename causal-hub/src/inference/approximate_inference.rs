use rand::{Rng, SeedableRng};

use crate::{
    datasets::CatEv,
    estimation::{CPDEstimator, MLE},
    inference::{BNInference, ParBNInference},
    models::{BN, CatBN, CatCPD},
    samplers::{BNSampler, ForwardSampler, ImportanceSampler, ParBNSampler},
    types::Set,
};

/// An approximate inference engine.
#[derive(Debug)]
pub struct ApproximateInference<'a, R, M, E> {
    rng: &'a mut R,
    model: &'a M,
    evidence: Option<&'a E>,
    sample_size: Option<usize>,
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
            sample_size: None,
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
            sample_size: self.sample_size,
        }
    }

    /// Set the sample size for the approximate inference instance.
    ///
    /// # Arguments
    ///
    /// * `n` - The sample size.
    ///
    /// # Returns
    ///
    /// Return a new approximate inference instance with the specified sample size.
    ///
    #[inline]
    pub const fn with_sample_size(mut self, n: usize) -> Self {
        self.sample_size = Some(n);
        self
    }
}

impl<'a, R, E> ApproximateInference<'a, R, CatBN, E> {
    #[inline]
    fn sample_size(&self, x: &Set<usize>, z: &Set<usize>) -> usize {
        // Get the sample size or compute it if not provided.
        self.sample_size.unwrap_or_else(|| {
            // Get the shape of the variables X.
            let x_shape: usize = x.iter().map(|&i| self.model.shape()[i]).product();
            // Get the shape of the variables Z.
            let z_shape: usize = z.iter().map(|&i| self.model.shape()[i]).product();
            // Return the sample size as PAC-like bounds:
            //  (|Z| * (|X| - 1) * ln(1 / delta) / epsilon^2), or approximately
            //  (|Z| * (|X| - 1) * 1200 for delta = 0.05 and epsilon = 0.05.
            z_shape * (x_shape - 1) * 1200
        })
    }
}

impl<R: Rng> BNInference<CatCPD> for ApproximateInference<'_, R, CatBN, ()> {
    fn predict(&mut self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
        // Assert X is not empty.
        assert!(!x.is_empty(), "Variables X must not be empty.");
        // Assert X and Z are disjoint.
        assert!(x.is_disjoint(z), "Variables X and Z must be disjoint.");
        // Assert X and Z are in the model.
        assert!(
            x.union(z).all(|&i| i < self.model.labels().len()),
            "Variables X and Z must be in the model."
        );

        // Get the sample size.
        let n = self.sample_size(x, z);
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

impl<R: Rng> BNInference<CatCPD> for ApproximateInference<'_, R, CatBN, CatEv> {
    fn predict(&mut self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
        // Assert X is not empty.
        assert!(!x.is_empty(), "Variables X must not be empty.");
        // Assert X and Z are disjoint.
        assert!(x.is_disjoint(z), "Variables X and Z must be disjoint.");
        // Assert X and Z are in the model.
        assert!(
            x.union(z).all(|&i| i < self.model.labels().len()),
            "Variables X and Z must be in the model."
        );

        // Get the sample size.
        let n = self.sample_size(x, z);
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
            None => ApproximateInference::new(self.rng, self.model)
                .with_sample_size(n)
                .predict(x, z),
        }
    }
}

impl<R: Rng + SeedableRng> ParBNInference<CatCPD> for ApproximateInference<'_, R, CatBN, ()> {
    fn par_predict(&mut self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
        // Assert X is not empty.
        assert!(!x.is_empty(), "Variables X must not be empty.");
        // Assert X and Z are disjoint.
        assert!(x.is_disjoint(z), "Variables X and Z must be disjoint.");
        // Assert X and Z are in the model.
        assert!(
            x.union(z).all(|&i| i < self.model.labels().len()),
            "Variables X and Z must be in the model."
        );

        // Get the sample size.
        let n = self.sample_size(x, z);
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

impl<R: Rng + SeedableRng> ParBNInference<CatCPD> for ApproximateInference<'_, R, CatBN, CatEv> {
    fn par_predict(&mut self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
        // Assert X is not empty.
        assert!(!x.is_empty(), "Variables X must not be empty.");
        // Assert X and Z are disjoint.
        assert!(x.is_disjoint(z), "Variables X and Z must be disjoint.");
        // Assert X and Z are in the model.
        assert!(
            x.union(z).all(|&i| i < self.model.labels().len()),
            "Variables X and Z must be in the model."
        );

        // Get the sample size.
        let n = self.sample_size(x, z);
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
            None => ApproximateInference::new(self.rng, self.model)
                .with_sample_size(n)
                .predict(x, z),
        }
    }
}
