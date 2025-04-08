/// A struct representing a Bayesian estimator.
#[derive(Clone, Debug)]
pub struct BayesianEstimator<Pi> {
    prior: Pi,
}

/// A type alias for a bayesian estimator.
pub type BE<Pi> = BayesianEstimator<Pi>;

impl<Pi> BayesianEstimator<Pi> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `prior` - The prior distribution.
    ///
    /// # Returns
    ///
    /// A new `BayesianEstimator` instance.
    ///
    #[inline]
    pub const fn new(prior: Pi) -> Self {
        Self { prior }
    }

    /// Returns the prior distribution.
    ///
    /// # Returns
    ///
    /// A reference to the prior.
    ///
    #[inline]
    pub const fn prior(&self) -> &Pi {
        &self.prior
    }
}
