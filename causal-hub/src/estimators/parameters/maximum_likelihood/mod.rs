mod table;
mod trajectory;

use crate::{
    datasets::MissingMethod,
    models::Labelled,
    types::{Labels, Map, Set},
};

/// A struct representing a maximum likelihood estimator.
#[derive(Clone, Debug)]
pub struct MLE<'a, D> {
    dataset: &'a D,
    missing_method: Option<MissingMethod>,
    missing_mechanism: Option<Map<usize, Set<usize>>>,
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
            missing_mechanism: None,
        }
    }

    /// Sets the missing data handling method.
    ///
    /// # Arguments
    ///
    /// * `missing_method` - The missing data handling method to set.
    /// * `missing_mechanism` - An optional missing data mechanism to set.
    ///
    /// # Returns
    ///
    /// A new sufficient statistics estimator instance with the specified missing data handling method.
    ///
    #[inline]
    pub fn with_missing_method(
        mut self,
        missing_method: MissingMethod,
        missing_mechanism: Option<Map<usize, Set<usize>>>,
    ) -> Self {
        self.missing_method = Some(missing_method);
        self.missing_mechanism = missing_mechanism;
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
