use crate::{
    graphs::{directions, IntoUndirectedGraph},
    prelude::UndirectedGraph,
};

/// Moral graph trait.
pub trait MoralGraph: IntoUndirectedGraph {
    /// Associated moral graph type.
    type MoralGraph: UndirectedGraph<Direction = directions::Undirected>;

    /// Build a moral graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use itertools::Itertools;
    ///
    /// use causal_hub::prelude::*;
    /// use causal_hub::models::MoralGraph;
    ///
    /// // Build a new directed graph.
    /// let g = DiGraph::new(
    ///     ["A", "B", "C", "D", "E"],
    ///     [("A", "C"), ("B", "C")]
    /// );
    ///
    /// // Build the associated moral graph.
    /// let h = g.moral();
    ///
    /// // Assert previous parents are connected.
    /// for x in V!(g) {
    ///     for (y, z) in Pa!(g, x).tuple_windows() {
    ///         assert!(h.has_edge_by_index(y, z));
    ///     }
    /// }
    /// ```
    ///
    fn moral(&self) -> Self::MoralGraph;
}
