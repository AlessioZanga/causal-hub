mod table;
mod trajectory;

use crate::{datasets::MissingMethod, models::Labelled, types::Labels};

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Debug)]
pub struct MLE<'a, D> {
    dataset: &'a D,
    missing_method: Option<MissingMethod>,
}

impl<'a, D> MLE<'a, D> {
    /// Creates a new maximum likelihood estimator.
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to the dataset to fit the estimator to.
    ///
    /// # Returns
    ///
    /// A new `MaximumLikelihoodEstimator` instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self {
            dataset,
            missing_method: None,
        }
    }

    /// Sets the missing data handling method.
    ///
    /// # Arguments
    ///
    /// * `missing_method` - The missing data handling method to set.
    ///
    /// # Returns
    ///
    /// A new maximum likelihood estimator with the specified missing data handling method.
    ///
    #[inline]
    pub fn with_missing_method(mut self, missing_method: MissingMethod) -> Self {
        self.missing_method = Some(missing_method);
        self
    }
}

impl<D> Labelled for MLE<'_, D>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}
