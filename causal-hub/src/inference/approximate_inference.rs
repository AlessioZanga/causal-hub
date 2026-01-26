use std::cell::RefCell;

use dry::macro_for;
use rand::{Rng, SeedableRng};

use crate::{
    estimators::{BE, CPDEstimator, ParCPDEstimator},
    inference::Modelled,
    models::{BN, CatBN, GaussBN, Labelled},
    samplers::{BNSampler, ForwardSampler, ImportanceSampler, ParBNSampler},
    set,
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
        // Check the sample size is positive.
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
    /// Estimate the values of `x` conditioned on `z`.
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
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the ancestors of the X U Z set.
            let x_z = x | z;
            let an_x_z = self.model.graph().ancestors(&x_z)?;
            let an_x_z = &an_x_z | &x_z;
            // Restrict the model to the ancestors.
            let an_x_z_model = self.model.select(&an_x_z)?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_model.indices_from(z, self.model.labels())?;
            // Initialize the sampler.
            let sampler = ForwardSampler::new(&mut rng, &an_x_z_model)?;
            // Generate n samples from the model.
            let dataset = sampler.sample_n(n)?;
            // Fit the CPD.
            BE::new(&dataset).fit(&an_x, &an_z)
        }
    }

    impl<R, F> BNInference<$type> for ApproximateInference<'_, R, $type, (), F>
    where
        R: Rng,
        F: Fn(&<$type as BN>::Samples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the ancestors of the X U Z set.
            let x_z = x | z;
            let an_x_z = self.model.graph().ancestors(&x_z)?;
            let an_x_z = &an_x_z | &x_z;
            // Restrict the model to the ancestors.
            let an_x_z_model = self.model.select(&an_x_z)?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_model.indices_from(z, self.model.labels())?;
            // Initialize the sampler.
            let sampler = ForwardSampler::new(&mut rng, &an_x_z_model)?;
            // Generate n samples from the model.
            let dataset = sampler.sample_n(n)?;
            // Fit the CPD.
            match &self.estimator {
                // Use the provided estimator.
                Some(f) => f(&dataset, &an_x, &an_z),
                // Otherwise, use the Bayesian estimator.
                None => BE::new(&dataset).fit(&an_x, &an_z),
            }
        }
    }

    impl<R> BNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, ()>
    where
        R: Rng,
    {
        fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the evidence variables.
            let e = self.evidence.map_or_else(
                || set![],
                |e| e.evidences()
                    .iter()
                    .flatten()
                    .map(|e| e.event())
                    .collect()
            );
            // Get the ancestors of the X U Z U E set.
            let x_z_e = &(x | z) | &e;
            let an_x_z_e = self.model.graph().ancestors(&x_z_e)?;
            let an_x_z_e = &an_x_z_e | &x_z_e;
            // Restrict the model to the ancestors.
            let an_x_z_e_model = self.model.select(&an_x_z_e)?;
            // Restrict the evidence to the restricted model.
            let evidence = self.evidence.map(|e| e.select(&an_x_z_e)).transpose()?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_e_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_e_model.indices_from(z, self.model.labels())?;
            // Check if evidence is actually provided.
            match evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::new(&mut rng, &an_x_z_e_model, &evidence)?;
                    // Generate n samples from the model.
                    let dataset = sampler.sample_n(n)?;
                    // Fit the CPD.
                    BE::new(&dataset).fit(&an_x, &an_z)
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, &an_x_z_e_model)
                    .with_sample_size(n)?
                    .estimate(&an_x, &an_z),
            }
        }
    }

    impl<R, F> BNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, F>
    where
        R: Rng,
        F: Fn(&<$type as BN>::WeightedSamples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the evidence variables.
            let e = self.evidence.map_or_else(
                || set![],
                |e| e.evidences()
                    .iter()
                    .flatten()
                    .map(|e| e.event())
                    .collect()
            );
            // Get the ancestors of the X U Z U E set.
            let x_z_e = &(x | z) | &e;
            let an_x_z_e = self.model.graph().ancestors(&x_z_e)?;
            let an_x_z_e = &an_x_z_e | &x_z_e;
            // Restrict the model to the ancestors.
            let an_x_z_e_model = self.model.select(&an_x_z_e)?;
            // Restrict the evidence to the restricted model.
            let evidence = self.evidence.map(|e| e.select(&an_x_z_e)).transpose()?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_e_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_e_model.indices_from(z, self.model.labels())?;
            // Check if evidence is actually provided.
            match evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::new(&mut rng, &an_x_z_e_model, &evidence)?;
                    // Generate n samples from the model.
                    let dataset = sampler.sample_n(n)?;
                    // Fit the CPD.
                    match &self.estimator {
                        // Use the provided estimator.
                        Some(f) => f(&dataset, &an_x, &an_z),
                        // Otherwise, use the Bayesian estimator.
                        None => BE::new(&dataset).fit(&an_x, &an_z),
                    }
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, &an_x_z_e_model)
                    .with_sample_size(n)?
                    .estimate(&an_x, &an_z),
            }
        }
    }

});

/// A trait for parallel inference with Bayesian Networks.
pub trait ParBNInference<T>
where
    T: BN,
{
    /// Estimate the values of `x` conditioned on `z`, in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Errors
    ///
    /// * `EmptySet` if `x` is empty.
    /// * `SetsNotDisjoint` if `x` and `z` are not disjoint.
    /// * `VertexOutOfBounds` if `x` or `z` are not in the model.
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
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the ancestors of the X U Z set.
            let x_z = x | z;
            let an_x_z = self.model.graph().ancestors(&x_z)?;
            let an_x_z = &an_x_z | &x_z;
            // Restrict the model to the ancestors.
            let an_x_z_model = self.model.select(&an_x_z)?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_model.indices_from(z, self.model.labels())?;
            // Initialize the sampler.
            let sampler = ForwardSampler::<R, _>::new(&mut rng, &an_x_z_model)?;
            // Generate n samples from the model.
            let dataset = sampler.par_sample_n(n)?;
            // Fit the CPD.
            BE::new(&dataset).par_fit(&an_x, &an_z)
        }
    }

    impl<R, F> ParBNInference<$type> for ApproximateInference<'_, R, $type, (), F>
    where
        R: Rng + SeedableRng,
        F: Fn(&<$type as BN>::Samples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the ancestors of the X U Z set.
            let x_z = x | z;
            let an_x_z = self.model.graph().ancestors(&x_z)?;
            let an_x_z = &an_x_z | &x_z;
            // Restrict the model to the ancestors.
            let an_x_z_model = self.model.select(&an_x_z)?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_model.indices_from(z, self.model.labels())?;
            // Initialize the sampler.
            let sampler = ForwardSampler::<R, _>::new(&mut rng, &an_x_z_model)?;
            // Generate n samples from the model.
            let dataset = sampler.par_sample_n(n)?;
            // Fit the CPD.
            match &self.estimator {
                // Use the provided estimator.
                Some(f) => f(&dataset, &an_x, &an_z),
                // Otherwise, use the Bayesian estimator.
                None => BE::new(&dataset).par_fit(&an_x, &an_z),
            }
        }
    }

    impl<R> ParBNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, ()>
    where
        R: Rng + SeedableRng,
    {
        fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the evidence variables.
            let e = self.evidence.map_or_else(
                || set![],
                |e| e.evidences()
                    .iter()
                    .flatten()
                    .map(|e| e.event())
                    .collect()
            );
            // Get the ancestors of the X U Z U E set.
            let x_z_e = &(x | z) | &e;
            let an_x_z_e = self.model.graph().ancestors(&x_z_e)?;
            let an_x_z_e = &an_x_z_e | &x_z_e;
            // Restrict the model to the ancestors.
            let an_x_z_e_model = self.model.select(&an_x_z_e)?;
            // Restrict the evidence to the restricted model.
            let evidence = self.evidence.map(|e| e.select(&an_x_z_e)).transpose()?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_e_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_e_model.indices_from(z, self.model.labels())?;
            // Check if evidence is actually provided.
            match evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::<R, _, _>::new(&mut rng, &an_x_z_e_model, &evidence)?;
                    // Generate n samples from the model.
                    let dataset = sampler.par_sample_n(n)?;
                    // Fit the CPD.
                    BE::new(&dataset).par_fit(&an_x, &an_z)
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, &an_x_z_e_model)
                    .with_sample_size(n)?
                    .estimate(&an_x, &an_z),
            }
        }
    }

    impl<R, F> ParBNInference<$type> for ApproximateInference<'_, R, $type, <$type as BN>::Evidence, F>
    where
        R: Rng + SeedableRng,
        F: Fn(&<$type as BN>::WeightedSamples, &Set<usize>, &Set<usize>) -> Result<<$type as BN>::CPD>,
    {
        fn par_estimate(&self, x: &Set<usize>, z: &Set<usize>) -> Result<<$type as BN>::CPD> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Check X and Z are in the model.
            x.union(z).try_for_each(|&i| {
                if i >= self.model.labels().len() {
                    return Err(Error::VertexOutOfBounds(i));
                }
                Ok(())
            })?;

            // Get the sample size.
            let n = self.sample_size(x, z);
            // Get the RNG.
            let mut rng = self.rng.borrow_mut();
            // Get the evidence variables.
            let e = self.evidence.map_or_else(
                || set![],
                |e| e.evidences()
                    .iter()
                    .flatten()
                    .map(|e| e.event())
                    .collect()
            );
            // Get the ancestors of the X U Z U E set.
            let x_z_e = &(x | z) | &e;
            let an_x_z_e = self.model.graph().ancestors(&x_z_e)?;
            let an_x_z_e = &an_x_z_e | &x_z_e;
            // Restrict the model to the ancestors.
            let an_x_z_e_model = self.model.select(&an_x_z_e)?;
            // Restrict the evidence to the restricted model.
            let evidence = self.evidence.map(|e| e.select(&an_x_z_e)).transpose()?;
            // Map the indices of X and Z to the restricted model.
            let an_x = an_x_z_e_model.indices_from(x, self.model.labels())?;
            let an_z = an_x_z_e_model.indices_from(z, self.model.labels())?;
            // Check if evidence is actually provided.
            match evidence {
                // Get the evidence.
                Some(evidence) => {
                    // Initialize the sampler.
                    let sampler = ImportanceSampler::<R, _, _>::new(&mut rng, &an_x_z_e_model, &evidence)?;
                    // Generate n samples from the model.
                    let dataset = sampler.par_sample_n(n)?;
                    // Fit the CPD.
                    match &self.estimator {
                        // Use the provided estimator.
                        Some(f) => f(&dataset, &an_x, &an_z),
                        // Otherwise, use the Bayesian estimator.
                        None => BE::new(&dataset).par_fit(&an_x, &an_z),
                    }
                }
                // Delegate to empty evidence case.
                None => ApproximateInference::new(&mut rng, &an_x_z_e_model)
                    .with_sample_size(n)?
                    .estimate(&an_x, &an_z),
            }
        }
    }

});
