/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Debug, Default)]
pub struct MaximumLikelihoodEstimator;

/// A type alias for a maximum likelihood estimator.
pub type MLE = MaximumLikelihoodEstimator;

impl MaximumLikelihoodEstimator {
    /// Creates a new maximum likelihood estimator.
    ///
    /// # Returns
    ///
    /// A new `MaximumLikelihoodEstimator` instance.
    ///
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}
