mod categorical;
pub use categorical::*;

use crate::{graphs::DiGraph, types::FxIndexMap};

/// A trait for continuous time Bayesian networks (CTBNs).
pub trait ContinuousTimeBayesianNetwork {
    /// The type of the labels.
    type Labels;
    /// The type of the CIM.
    type CIM;
    /// The type of the initial distribution.
    type InitialDistribution;
    /// The type of the observed event.
    type Event;
    /// The type of the observed trajectory.
    type Trajectory;
    /// The type of a collection of trajectories.
    type Trajectories;

    /// Constructs a new CTBN.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cims` - The conditional intensity matrices.
    ///
    /// # Notes
    ///
    /// The distribution of the initial state (i.e. initial distribution) is uniform.
    /// See `with_initial_distribution` to specify the initial distribution.
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

    /// Returns the initial distribution.
    ///
    /// # Returns
    ///
    /// A reference to the initial distribution.
    ///
    fn initial_distribution(&self) -> &Self::InitialDistribution;

    /// Construct a new categorical CTBN with the given graph, CIMs and initial distribution.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cims` - The conditional intensity matrices.
    /// * `initial_distribution` - The initial distribution as a categorical BN.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * the initial distribution labels do not match the CIMs labels.
    /// * the initial distribution states do not match the CIMs states.
    /// * see `new` for more details.
    ///
    /// # Returns
    ///
    /// A new categorical CTBN.
    ///
    fn with_initial_distribution<I>(
        initial_distribution: Self::InitialDistribution,
        graph: DiGraph,
        cims: I,
    ) -> Self
    where
        I: IntoIterator<Item = Self::CIM>;
}

/// A type alias for a continuous time Bayesian network.
pub use ContinuousTimeBayesianNetwork as CTBN;
