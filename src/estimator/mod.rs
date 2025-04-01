mod maximum_likelihood;
pub use maximum_likelihood::*;

pub trait Estimator {
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
    /// A distribution.
    ///
    fn fit(&self, x: usize, z: &[usize]) -> Self::Distribution;
}
