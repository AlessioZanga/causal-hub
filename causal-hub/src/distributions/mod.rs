mod categorical;
pub use categorical::*;

mod intensity_matrix;
pub use intensity_matrix::*;

use crate::types::Labels;

/// A trait for conditional probability distributions.
pub trait ConditionalProbabilityDistribution {
    /// The type of the parameters.
    type Parameters;
    /// The type of the sufficient statistics.
    type SS;

    /// Returns the label of the conditioned variable.
    ///
    /// # Returns
    ///
    /// A reference to the label.
    ///
    fn labels(&self) -> &Labels;

    /// Returns the labels of the conditioned variables.
    ///
    /// # Returns
    ///
    /// A reference to the conditioning labels.
    ///
    fn conditioning_labels(&self) -> &Labels;

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
