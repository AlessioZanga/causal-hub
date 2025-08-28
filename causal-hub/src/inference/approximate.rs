use crate::types::Set;

/// A trait for Bayesian network approximate inference.
pub trait BNApproximateInference {
    /// The output type of the inference.
    type Output;

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
    fn predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> Self::Output;
}

/// A trait for parallel Bayesian network approximate inference.
pub trait ParBNApproximateInference {
    /// The output type of the inference.
    type Output;

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
    fn par_predict(&mut self, x: &Set<usize>, z: &Set<usize>, n: usize) -> Self::Output;
}
