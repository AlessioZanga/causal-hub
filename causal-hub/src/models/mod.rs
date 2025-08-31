mod bayesian_network;
pub use bayesian_network::*;

mod continuous_time_bayesian_network;
pub use continuous_time_bayesian_network::*;

mod graphs;
pub use graphs::*;

use crate::types::Labels;

/// A trait for conditional probability distributions.
pub trait CPD {
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
