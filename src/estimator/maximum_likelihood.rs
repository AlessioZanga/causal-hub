/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Debug)]
pub struct MaximumLikelihoodEstimator<'a, D> {
    data: &'a D,
}

/// A type alias for a maximum likelihood estimator.
pub type MLE<'a, D> = MaximumLikelihoodEstimator<'a, D>;

impl<'a, D> MaximumLikelihoodEstimator<'a, D> {
    /// Creates a new maximum likelihood estimator.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to fit the estimator to.
    ///
    /// # Returns
    ///
    /// A new `MaximumLikelihoodEstimator` instance.
    ///
    #[inline]
    pub const fn new(data: &'a D) -> Self {
        Self { data }
    }

    /// Returns a reference to the data.
    ///
    /// # Returns
    ///
    /// A reference to the data.
    ///
    #[inline]
    pub const fn data(&self) -> &'a D {
        self.data
    }
}
