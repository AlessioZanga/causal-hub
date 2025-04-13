mod categorical;
pub use categorical::*;

use crate::{graphs::DiGraph, types::FxIndexMap};

/// A trait for continuous time Bayesian networks (CTBNs).
pub trait ContinuousTimeBayesianNetwork {
    /// The type of the labels.
    type Labels;
    /// The type of the CIM.
    type CIM;

    /// Constructs a new CTBN.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cims` - The conditional intensity matrices.
    ///
    /// # Returns
    ///
    /// A new CTBN instance.
    ///
    fn new<I>(graph: DiGraph, cims: I) -> Self
    where
        I: IntoIterator<Item = Self::CIM>;

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> &Self::Labels;

    /// Returns the underlying graph.
    ///
    /// # Returns
    ///
    /// A reference to the graph.
    ///
    fn graph(&self) -> &DiGraph;

    /// Returns the a map labels-distributions.
    ///
    /// # Returns
    ///
    /// A reference to the CIMs.
    ///
    fn cims(&self) -> &FxIndexMap<String, Self::CIM>;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;
}
