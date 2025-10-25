mod table;
pub use table::*;

mod trajectory;
pub use trajectory::*;

use crate::types::Set;

/// A trait for dataset.
pub trait Dataset {
    /// The type of the values.
    type Values;

    /// The values of the dataset.
    ///
    /// # Returns
    ///
    /// A reference to the values.
    ///
    fn values(&self) -> &Self::Values;

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
}

/// A trait for incomplete datasets.
pub trait IncDataset: Dataset + Sized {
    /// The type of the missing data indicator.
    type Missing;
    /// The value of the missing data indicator.
    const MISSING: Self::Missing;

    /// Get the missing information.
    ///
    /// # Returns
    ///
    /// A reference to the missing information.
    ///
    fn missing(&self) -> &MissingTable;

    /// Perform list-wise deletion to handle missing data.
    ///
    /// # Returns
    ///
    /// A new dataset with list-wise deletion applied.
    ///
    fn lw_deletion(&self) -> Self;

    /// Perform pair-wise deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for pair-wise deletion.
    ///
    /// # Returns
    ///
    /// A new dataset with pair-wise deletion applied.
    ///
    fn pw_deletion(&self, x: &Set<usize>) -> Self;

    /// Perform inverse probability weighting deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for inverse probability weighting deletion.
    ///
    /// # Returns
    ///
    /// A new dataset with inverse probability weighting deletion applied.
    ///
    fn ipw_deletion(&self, x: &Set<usize>) -> Self;

    /// Perform augmented inverse probability weighting deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for augmented inverse probability weighting deletion.
    ///
    /// # Returns
    ///
    /// A new dataset with augmented inverse probability weighting deletion applied.
    ///
    fn aipw_deletion(&self, x: &Set<usize>) -> Self;
}
