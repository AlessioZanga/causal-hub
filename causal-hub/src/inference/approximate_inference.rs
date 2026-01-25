use std::cell::RefCell;

use dry::macro_for;
use rand::{Rng, SeedableRng};

use crate::{
    estimators::{BE, CPDEstimator, ParCPDEstimator},
    inference::Modelled,
    models::{BN, CatBN, GaussBN, Labelled},
    samplers::{BNSampler, ForwardSampler, ImportanceSampler, ParBNSampler},
    types::{Error, Result, Set},
};

/// An approximate inference engine.
#[derive(Debug)]
pub struct ApproximateInference<'a, R, M, E, F> {
    rng: RefCell<&'a mut R>,
    model: &'a M,
    evidence: Option<&'a E>,
    estimator: Option<F>,
    sample_size: Option<usize>,
}

impl<'a, R, M> ApproximateInference<'a, R, M, (), ()> {
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
        // Wrap the RNG in a RefCell.
        let rng = RefCell::new(rng);

        Self {
            rng,
            model,
            evidence: None,
            estimator: None,
            sample_size: None,
        }
    }
}

impl<'a, R, M, E, F> ApproximateInference<'a, R, M, E, F> {
    /// Add an estimator to the approximate inference instance.
    ///
    /// # Arguments
    ///
    /// * `estimator` - A function that takes a reference to samples, the sets of variables `x` and `z`, and returns a CPD estimator.
    ///
    /// # Returns
    ///
    /// Return a new approximate inference instance with the estimator.
    ///
    pub fn with_estimator<T, A, B>(self, estimator: T) -> ApproximateInference<'a, R, M, E, T>
    where
        T: Fn(&A, &Set<usize>, &Set<usize>) -> B,
    {
        ApproximateInference {
            rng: self.rng,
            model: self.model,
            evidence: self.evidence,
            estimator: Some(estimator),
            sample_size: self.sample_size,
        }
    }

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
    pub fn with_evidence<T>(self, evidence: &'a T) -> ApproximateInference<'a, R, M, T, F> {
        ApproximateInference {
            rng: self.rng,
            model: self.model,
            evidence: Some(evidence),
            estimator: self.estimator,
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
    pub fn with_sample_size(mut self, n: usize) -> Result<Self> {
        // Assert the sample size is positive.
        if n == 0 {
            return Err(Error::InvalidParameter(
                "n".into(),
                "Sample size must be positive.".into(),
            ));
        }
        // Set the sample size.
        self.sample_size = Some(n);
        Ok(self)
    }
}

impl<R, M, E, F> Modelled<M> for ApproximateInference<'_, R, M, E, F> {
    #[inline]
    fn model(&self) -> &M {
        self.model
    }
}

/// A trait for inference with Bayesian Networks.
pub trait BNInference<T>
where
    T: BN,
{
    /// Estimate the values of `x` conditioned on `z` using `n` samples.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Returns
    ///
    /// The estimated values of `x` conditioned on `z`.
    ///
    fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<T::CPD>;
}

impl<'a, R, E, F> ApproximateInference<'a, R, CatBN, E, F> {
    #[inline]
    fn sample_size(&self, x: &Set<usize>, z: &Set<usize>) -> usize {
        // Get the sample size or compute it if not provided.
        self.sample_size.unwrap_or_else(|| {
            // Get the shape of the variables X and Z.
            let (x_shape, z_shape): (usize, usize) = (
                x.iter().map(|&i| self.model.shape()[i]).product(),
                z.iter().map(|&i| self.model.shape()[i]).product(),
            );
            // Return the sample size as PAC-like bounds:
            //  (|Z| * (|X| - 1)) * ln(1 / delta) / epsilon^2, or approximately
            //  (|Z| * (|X| - 1)) * 1200 for delta = 0.05 and epsilon = 0.05.
            z_shape * (x_shape - 1) * 1200
        })
    }
}

impl<'a, R, E, F> ApproximateInference<'a, R, GaussBN, E, F> {
    #[inline]
    fn sample_size(&self, x: &Set<usize>, z: &Set<usize>) -> usize {
        // Get the sample size or compute it if not provided.
        self.sample_size.unwrap_or_else(|| {
            // Get the shape of the variables X and Z.
            let (x_shape, z_shape) = (x.len(), z.len());
            // Return the sample size as PAC-like bounds:
            //  (|X| * |Z| + (|X| * (|X| + 1)) / 2) * ln(1 / delta) / epsilon^2, or approximately
            //  (|X| * |Z| + (|X| * (|X| + 1)) / 2) * 1200, for delta = 0.05 and epsilon = 0.05.
            //  |X| * (|Z| + (|X| + 1) / 2) * 1200, for delta = 0.05 and epsilon = 0.05.
            x_shape * (z_shape + x_shape.div_ceil(2)) * 1200
        })
    }
}

macro_for!($type in [CatBN, GaussBN] {

    impl<R> BNInference<$type> for ApproximateInference<'_, R, $type, (), ()>
    where
        R: Rng,
    {
        fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Assert X and Z are in the model.
            for &i in x.union(z) {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Initialize the sampler.
            let sampler = ForwardSampler::new(&mut rng, self.model)?;
            // Generate n samples from the model.
            // TODO: Avoid generating the full dataset,
            //       e.g., by only sampling the variables in X U Z, and
            //       by using batching to reduce memory usage.
            let dataset = sampler.sample_n(n)?;
            // Fit the CPD.
            BE::new(&dataset).fit(x, z)
        }
    }

    impl<R, F> BNInference<$type> for ApproximateInference<'_, R, $type, (), F>
    where
        R: Rng,
        F: Fn(&<$type as BN>::Samples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Assert X and Z are in the model.
            for &i in x.union(z) {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Initialize the sampler.
            let sampler = ForwardSampler::new(&mut rng, self.model)?;
            // Generate n samples from the model.
            // TODO: Avoid generating the full dataset,
            //       e.g., by only sampling the variables in X U Z, and
            //       by using batching to reduce memory usage.
            let dataset = sampler.sample_n(n)?;
            // Fit the CPD.
            match &self.estimator {
                // Use the provided estimator.
                Some(f) => f(&dataset, x, z),
                // Otherwise, use the Bayesian estimator.
                None => BE::new(&dataset).fit(x, z),
            }
        }
    }

    impl<R> BNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, ()>
    where
        R: Rng,
    {
        fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Assert X and Z are in the model.
            for &i in x.union(z) {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Check if evidence is actually provided.
            match self.evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::new(&mut rng, self.model, evidence)?;
                    // Generate n samples from the model.
                    // TODO: Avoid generating the full dataset,
                    //       e.g., by only sampling the variables in X U Z, and
                    //       by using batching to reduce memory usage.
                    let dataset = sampler.sample_n(n)?;
                    // Fit the CPD.
                    BE::new(&dataset).fit(x, z)
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, self.model)
                    .with_sample_size(n)?
                    .estimate(x, z),
            }
        }
    }

    impl<R, F> BNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, F>
    where
        R: Rng,
        F: Fn(&<$type as BN>::WeightedSamples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Assert X and Z are in the model.
            for &i in x.union(z) {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Check if evidence is actually provided.
            match self.evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::new(&mut rng, self.model, evidence)?;
                    // Generate n samples from the model.
                    // TODO: Avoid generating the full dataset,
                    //       e.g., by only sampling the variables in X U Z, and
                    //       by using batching to reduce memory usage.
                    let dataset = sampler.sample_n(n)?;
                    // Fit the CPD.
                    match &self.estimator {
                        // Use the provided estimator.
                        Some(f) => f(&dataset, x, z),
                        // Otherwise, use the Bayesian estimator.
                        None => BE::new(&dataset).fit(x, z),
                    }
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, self.model)
                    .with_sample_size(n)?
                    .estimate(x, z),
            }
        }
    }

});

/// A trait for parallel inference with Bayesian Networks.
pub trait ParBNInference<T>
where
    T: BN,
{
    /// Estimate the values of `x` conditioned on `z` using `n` samples, in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Errors
    ///
    /// * `IllegalArgument` if `x` is empty.
    /// * `IllegalArgument` if `x` and `z` are not disjoint.
    /// * `IllegalArgument` if `x` or `z` are not in the model.
    ///
    /// # Returns
    ///
    /// The estimated values of `x` conditioned on `z`.
    ///
    fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<T::CPD>;
}

macro_for!($type in [CatBN, GaussBN] {

    impl<R> ParBNInference<$type> for ApproximateInference<'_, R, $type, (), ()>
    where
        R: Rng + SeedableRng,
    {
        fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::IllegalArgument("Variables X must not be empty.".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be disjoint.".into(),
                ));
            }
            // Assert X and Z are in the model.
            if !x.union(z).all(|&i| i < self.model.labels().len()) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be in the model.".into(),
                ));
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Initialize the sampler.
            let sampler = ForwardSampler::<R, _>::new(&mut rng, self.model)?;
            // Generate n samples from the model.
            // TODO: Avoid generating the full dataset,
            //       e.g., by only sampling the variables in X U Z, and
            //       by using batching to reduce memory usage.
            let dataset = sampler.par_sample_n(n)?;
            // Fit the CPD.
            BE::new(&dataset).par_fit(x, z)
        }
    }

    impl<R, F> ParBNInference<$type> for ApproximateInference<'_, R, $type, (), F>
    where
        R: Rng + SeedableRng,
        F: Fn(&<$type as BN>::Samples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::IllegalArgument("Variables X must not be empty.".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be disjoint.".into(),
                ));
            }
            // Assert X and Z are in the model.
            if !x.union(z).all(|&i| i < self.model.labels().len()) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be in the model.".into(),
                ));
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Initialize the sampler.
            let sampler = ForwardSampler::<R, _>::new(&mut rng, self.model)?;
            // Generate n samples from the model.
            // TODO: Avoid generating the full dataset,
            //       e.g., by only sampling the variables in X U Z, and
            //       by using batching to reduce memory usage.
            let dataset = sampler.par_sample_n(n)?;
            // Fit the CPD.
            match &self.estimator {
                // Use the provided estimator.
                Some(f) => f(&dataset, x, z),
                // Otherwise, use the Bayesian estimator.
                None => BE::new(&dataset).par_fit(x, z),
            }
        }
    }

    impl<R> ParBNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, ()>
    where
        R: Rng + SeedableRng,
    {
        fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::IllegalArgument("Variables X must not be empty.".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be disjoint.".into(),
                ));
            }
            // Assert X and Z are in the model.
            if !x.union(z).all(|&i| i < self.model.labels().len()) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be in the model.".into(),
                ));
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Check if evidence is actually provided.
            match self.evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::<R, _, _>::new(&mut rng, self.model, evidence)?;
                    // Generate n samples from the model.
                    // TODO: Avoid generating the full dataset,
                    //       e.g., by only sampling the variables in X U Z, and
                    //       by using batching to reduce memory usage.
                    let dataset = sampler.par_sample_n(n)?;
                    // Fit the CPD.
                    BE::new(&dataset).par_fit(x, z)
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, self.model)
                    .with_sample_size(n)?
                    .estimate(x, z),
            }
        }
    }

    impl<R, F> ParBNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, F>
    where
        R: Rng + SeedableRng,
        F: Fn(&<$type as BN>::WeightedSamples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::IllegalArgument("Variables X must not be empty.".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be disjoint.".into(),
                ));
            }
            // Assert X and Z are in the model.
            if !x.union(z).all(|&i| i < self.model.labels().len()) {
                return Err(Error::IllegalArgument(
                    "Variables X and Z must be in the model.".into(),
                ));
            }

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Check if evidence is actually provided.
            match self.evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::<R, _, _>::new(&mut rng, self.model, evidence)?;
                    // Generate n samples from the model.
                    // TODO: Avoid generating the full dataset,
                    //       e.g., by only sampling the variables in X U Z, and
                    //       by using batching to reduce memory usage.
                    let dataset = sampler.par_sample_n(n)?;
                    // Fit the CPD.
                    match &self.estimator {
                        // Use the provided estimator.
                        Some(f) => f(&dataset, x, z),
                        // Otherwise, use the Bayesian estimator.
                        None => BE::new(&dataset).par_fit(x, z),
                    }
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, self.model)
                    .with_sample_size(n)?
                    .estimate(x, z),
            }
        }
    }

});
