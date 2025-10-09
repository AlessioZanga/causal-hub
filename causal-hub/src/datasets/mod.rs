mod table;
pub use table::*;

mod trajectory;
pub use trajectory::*;

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
