mod categorical;
pub use categorical::*;

use crate::{
    graphs::DiGraph,
    types::{Labels, Map},
};

/// A trait for Bayesian networks.
pub trait BayesianNetwork {
    /// The type of the CPD.
    type CPD;
    /// The type of the sample.
    type Sample;
    /// The type of the samples.
    type Samples;

    /// Constructs a new Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cpds` - The conditional probability distributions.
    ///
    /// # Returns
    ///
    /// A new Bayesian network instance.
    ///
    fn new<I>(graph: DiGraph, cpds: I) -> Self
    where
        I: IntoIterator<Item = Self::CPD>;

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> &Labels;

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
    /// A reference to the cpds.
    ///
    fn cpds(&self) -> &Map<String, Self::CPD>;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;

    /// Returns the topological order of the graph.
    ///
    /// # Returns
    ///
    /// A reference to the topological order.
    ///
    fn topological_order(&self) -> &[usize];
}

/// A type alias for a Bayesian network.
pub use BayesianNetwork as BN;
