mod categorical;
pub use categorical::*;

use crate::{graph::DiGraph, types::FxIndexMap};

/// A trait for Bayesian networks.
pub trait BayesianNetwork {
    /// The type of the labels.
    type Labels;
    /// The type of the CPD.
    type CPD;

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
    /// A reference to the cpds.
    ///
    fn cpds(&self) -> &FxIndexMap<String, Self::CPD>;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;

    /// Constructor of the Bayesian network given the graph and the parameters.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cpds` - The map of labels-distributions.
    ///
    /// # Returns
    ///
    /// The Bayesian network.
    ///
    fn with_graph_cpds(graph: DiGraph, cpds: FxIndexMap<String, Self::CPD>) -> Self;
}
