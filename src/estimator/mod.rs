mod bayesian;
mod maximum_likelihood;
pub use bayesian::*;
pub use maximum_likelihood::*;

/// A trait for estimators.
pub trait Estimator {
    /// The type of the data.
    type Data;
    /// The type of the output distribution.
    type Distribution;

    /// Fits the estimator to the data and returns a distribution.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The fitted distribution.
    ///
    fn fit(&self, x: usize, z: &[usize]) -> Self::Distribution;
}
