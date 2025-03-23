use crate::distribution::Distribution;

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Debug)]
pub struct MaximumLikelihoodEstimator<'a, P>
where
    P: Distribution,
{
    // Required fields.
    data: &'a P::Data,
}

impl<'a, P> MaximumLikelihoodEstimator<'a, P>
where
    P: Distribution,
{
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
    pub const fn new(data: &'a P::Data) -> Self {
        Self { data }
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
}
