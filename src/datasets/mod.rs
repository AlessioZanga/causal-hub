mod categorical;
pub use categorical::*;

mod trajectory;
pub use trajectory::*;

/// A trait for dataset.
pub trait Dataset {
    /// The type of the labels.
    type Labels;
    /// The type of the values.
    type Values;

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> &Self::Labels;

    /// Returns the values.
    ///
    /// # Returns
    ///
    /// A reference to the values.
    ///
    fn values(&self) -> &Self::Values;

    /// Returns the sample size.
    ///
    /// # Returns
    ///
    /// The number of samples in the dataset.
    ///
    fn sample_size(&self) -> usize;
}
