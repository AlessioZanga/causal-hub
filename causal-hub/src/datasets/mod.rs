mod missing;
pub use missing::*;

mod table;
pub use table::*;

mod trajectory;
use std::borrow::Cow;

pub use trajectory::*;

use crate::{
    models::HasLabels,
    types::{Result, Set},
};

/// A trait for dataset.
pub trait Dataset: HasLabels {
    /// The type of the values.
    type Values;
    /// The type of the support metadata (variable ranges / state sets).
    type Support: Clone;
    /// The type of the evidence,
    type Evidence;
    /// The type of the evidence iterator.
    type EvidenceIter<'a>: Iterator<Item = Result<Self::Evidence>>
    where
        Self: 'a;

    /// Returns the support metadata of the dataset.
    ///
    /// # Returns
    ///
    /// A reference to the support metadata.
    ///
    fn support(&self) -> Cow<'_, Self::Support>;

    /// The values of the dataset.
    ///
    /// # Returns
    ///
    /// A reference to the values.
    ///
    fn values(&self) -> &Self::Values;

    /// An iterator over the evidence in the dataset.
    ///
    /// # Returns
    ///
    /// An iterator over the evidence in the dataset.
    ///
    fn evidence_iter(&self) -> Self::EvidenceIter<'_>;

    /// The sample size.
    ///
    /// # Notes
    ///
    /// If the dataset is weighted, this should return the sum of the weights.
    ///
    /// # Returns
    ///
    /// The number of samples in the dataset.
    ///
    fn sample_size(&self) -> f64;

    /// Restrict the dataset to the specified variables.
    ///
    /// # Arguments
    ///
    /// * `x` - Set of variables to select.
    ///
    /// # Errors
    ///
    /// * If the set of variables is empty.
    /// * If any variable in the set is out of bounds.
    ///
    /// # Returns
    ///
    /// A dataset restricted to the specified variables.
    ///
    fn select(&self, x: &Set<usize>) -> Result<Self>
    where
        Self: Sized;
}
