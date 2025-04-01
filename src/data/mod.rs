mod categorical;
pub use categorical::*;

/// A trait for data.
pub trait Data {
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
}
