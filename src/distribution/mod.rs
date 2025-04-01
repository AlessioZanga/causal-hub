mod categorical;
pub use categorical::*;

/// A trait for probability distributions.
pub trait Distribution {
    /// The type of the labels.
    type Labels;
    /// The type of the parameters.
    type Parameters;
    /// The type of the data.
    type Data;

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> &Self::Labels;

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    fn parameters(&self) -> &Self::Parameters;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;
}
