mod categorical;
pub use categorical::*;

mod intensity_matrix;
pub use intensity_matrix::*;

/// A trait for conditional probability distributions.
pub trait ConditionalProbabilityDistribution {
    /// The type of the label.
    type Label;
    /// The type of the conditioning labels.
    type ConditioningLabels;
    /// The type of the parameters.
    type Parameters;

    /// Returns the label of the conditioned variable.
    ///
    /// # Returns
    ///
    /// A reference to the label.
    ///
    fn label(&self) -> &Self::Label;

    /// Returns the labels of the conditioned variables.
    ///
    /// # Returns
    ///
    /// A reference to the conditioning labels.
    ///
    fn conditioning_labels(&self) -> &Self::ConditioningLabels;

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

/// A type alias for the conditional probability distribution.
pub use ConditionalProbabilityDistribution as CPD;
