use crate::{set, types::Set};

/// A causal inference engine.
pub struct CausalInference;

/// A trait for causal inference with Bayesian Networks.
pub trait BNCausalInference<T> {
    /// Estimate the average causal effect of `X` on `Y` as E(Y | do(X)).
    ///
    /// # Arguments
    ///
    /// * `x` - The treatment variables.
    /// * `y` - The outcome variables.
    ///
    /// # Returns
    ///
    /// The estimated average causal effect of `X` on `Y`.
    ///
    fn average_causal_effect(&mut self, x: &Set<usize>, y: &Set<usize>) -> Option<T> {
        self.conditional_average_causal_effect(x, y, &set![])
    }

    /// Estimate the conditional average causal effect of `X` on `Y` given `Z` as E(Y | do(X), Z).
    ///
    /// # Arguments
    ///
    /// * `x` - The treatment variables.
    /// * `y` - The outcome variables.
    /// * `z` - The conditioning variables.
    ///
    /// # Returns
    ///
    /// The estimated conditional average causal effect of `X` on `Y` given `Z`.
    ///
    fn conditional_average_causal_effect(
        &mut self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
    ) -> Option<T>;
}
