mod approximate_inference;
pub use approximate_inference::*;

mod causal_inference;
pub use causal_inference::*;

mod backdoor_criterion;
pub use backdoor_criterion::*;

mod graphical_separation;
pub use graphical_separation::*;

mod topological_order;
pub use topological_order::*;

use crate::types::Set;

/// A trait for inference with Bayesian Networks.
pub trait BNInference<T> {
    /// Predict the values of `x` conditioned on `z` using `n` samples, without evidence.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Panics
    ///
    /// * Panics if `x` is empty.
    /// * Panics if `x` and `z` are not disjoint.
    /// * Panics if `x` or `z` are not in the model.
    ///
    /// # Returns
    ///
    /// The predicted values of `x` conditioned on `z`.
    ///
    fn predict(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}

/// A trait for parallel inference with Bayesian Networks.
pub trait ParBNInference<T> {
    /// Predict the values of `x` conditioned on `z` using `n` samples, without evidence, in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Panics
    ///
    /// * Panics if `x` is empty.
    /// * Panics if `x` and `z` are not disjoint.
    /// * Panics if `x` or `z` are not in the model.
    ///
    /// # Returns
    ///
    /// The predicted values of `x` conditioned on `z`.
    ///
    fn par_predict(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}
