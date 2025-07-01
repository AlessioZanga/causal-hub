mod categorical;
pub use categorical::*;

mod trajectory;
pub use trajectory::*;

use crate::types::Labels;

/// A trait for dataset.
pub trait Dataset {
    /// The type of the values.
    type Values;

    /// The labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> &Labels;

    /// The values of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the values.
    ///
    fn values(&self) -> &Self::Values;

    /// The sample size.
    ///
    /// # Returns
    ///
    /// The number of samples in the dataset.
    ///
    fn sample_size(&self) -> usize;
}
