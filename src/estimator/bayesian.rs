use crate::distribution::Distribution;

/// A struct representing a Bayesian estimator.
#[derive(Clone, Debug)]
pub struct BayesianEstimator<'a, P>
where
    P: Distribution,
{
    // Required fields.
    data: &'a P::Data,
    alpha: f64,
}

impl<'a, P> BayesianEstimator<'a, P>
where
    P: Distribution,
{
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to fit the estimator to.
    /// * `alpha` - The prior parameter.
    ///
    /// # Returns
    ///
    /// A new `BayesianEstimator` instance.
    ///
    #[inline]
    pub const fn new(data: &'a P::Data, alpha: f64) -> Self {
        Self { data, alpha }
    }

    /// Returns a reference to the data.
    ///
    /// # Returns
    ///
    /// A reference to the data.
    ///
    #[inline]
    pub const fn data(&self) -> &'a P::Data {
        self.data
    }

    /// Returns the prior parameter.
    ///
    /// # Returns
    ///
    /// The prior parameter.
    ///
    #[inline]
    pub const fn alpha(&self) -> f64 {
        self.alpha
    }
}

/// A type alias for a bayesian estimator.
pub type BE<'a, P> = BayesianEstimator<'a, P>;
