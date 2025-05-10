mod categorical;
pub use categorical::*;

mod categorical_evidence;
pub use categorical_evidence::*;

mod trajectory;
pub use trajectory::*;

mod trajectory_evidence;
pub use trajectory_evidence::*;

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
