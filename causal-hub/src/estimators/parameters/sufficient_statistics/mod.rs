mod table;
mod trajectory;

use crate::{
    datasets::MissingMethod,
    models::Labelled,
    types::{Labels, Map, Set},
};

/// A struct representing a sufficient statistics estimator.
#[derive(Clone, Debug)]
pub struct SSE<'a, D> {
    dataset: &'a D,
    missing_method: Option<MissingMethod>,
    missing_mechanism: Option<Map<usize, Set<usize>>>,
}

impl<'a, D> SSE<'a, D> {
    /// Constructs a new sufficient statistics estimator.
    ///
    /// # Returns
    ///
    /// A new sufficient statistics estimator instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self {
            dataset,
            missing_method: None,
            missing_mechanism: None,
        }
    }

    /// Sets the missing handling method.
    ///
    /// # Arguments
    ///
    /// * `missing_method` - An optional missing handling method to set.
    /// * `missing_mechanism` - An optional missing mechanism to set.
    ///
    /// # Returns
    ///
    /// A new estimator with the specified missing handling method.
    ///
    #[inline]
    pub fn with_missing_method(
        mut self,
        missing_method: Option<MissingMethod>,
        missing_mechanism: Option<Map<usize, Set<usize>>>,
    ) -> Self {
        self.missing_method = missing_method;
        self.missing_mechanism = missing_mechanism;
        self
    }
}

impl<D> Labelled for SSE<'_, D>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}
