mod categorical;
pub use categorical::*;

use crate::{models::graphs::DiGraph, types::Map};

/// A trait for continuous time Bayesian networks (CTBNs).
pub trait CTBN {
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

    /// Returns the initial distribution.
    ///
    /// # Returns
    ///
    /// A reference to the initial distribution.
    ///
    fn initial_distribution(&self) -> &Self::InitialDistribution;

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
    fn cims(&self) -> &Map<String, Self::CIM>;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;
}
