/// A struct representing a Bayesian estimator.
#[derive(Clone, Debug)]
pub struct BayesianEstimator<'a, D, Pi> {
    data: &'a D,
    prior_distribution: Pi,
}

/// A type alias for a bayesian estimator.
pub type BE<'a, D, Pi> = BayesianEstimator<'a, D, Pi>;

impl<'a, D, Pi> BayesianEstimator<'a, D, Pi> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to fit the estimator to.
    /// * `prior_distribution` - The prior distribution parameter.
    ///
    /// # Returns
    ///
    /// A new `BayesianEstimator` instance.
    ///
    #[inline]
    pub const fn new(data: &'a D, prior_distribution: Pi) -> Self {
        Self {
            data,
            prior_distribution,
        }
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

    /// Returns the prior distribution.
    ///
    /// # Returns
    ///
    /// A reference to the prior.
    ///
    #[inline]
    pub const fn prior_distribution(&self) -> &Pi {
        &self.prior_distribution
    }
}
