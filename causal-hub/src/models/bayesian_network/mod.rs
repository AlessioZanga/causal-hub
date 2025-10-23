mod categorical;
pub use categorical::*;

mod gaussian;
pub use gaussian::*;

use crate::{models::graphs::DiGraph, types::Map};

/// A trait for Bayesian networks.
pub trait BN {
    /// The type of the CPD.
    type CPD;
    /// The type of the evidence.
    type Evidence;
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

    /// Returns the name of the model, if any.
    ///
    /// # Returns
    ///
    /// The name of the model, if it exists.
    ///
    fn name(&self) -> Option<&str>;

    /// Returns the description of the model, if any.
    ///
    /// # Returns
    ///
    /// The description of the model, if it exists.
    ///
    fn description(&self) -> Option<&str>;

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

    /// Creates a new Bayesian network with optional fields.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    /// * `description` - The description of the model.
    /// * `graph` - The underlying graph.
    /// * `cpds` - The conditional probability distributions.
    ///
    /// # Panics
    ///
    /// * Panics if `name` is an empty string.
    /// * Panics if `description` is an empty string.
    ///
    /// # Returns
    ///
    /// A new Bayesian network instance.
    ///
    fn with_optionals<I>(
        name: Option<String>,
        description: Option<String>,
        graph: DiGraph,
        cpds: I,
    ) -> Self
    where
        I: IntoIterator<Item = Self::CPD>;
}
