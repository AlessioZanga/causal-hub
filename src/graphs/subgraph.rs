use super::{BaseGraph, PartialOrdGraph};

/// Subgraph trait.
pub trait SubGraph: BaseGraph + PartialOrdGraph {
    /// Constructs the generic subgraph.
    ///
    /// Constructs a generic subgraph given a subset of vertices and edges.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME: Add doc examples.
    /// ```
    ///
    fn subgraph<I, J>(&self, vertices: I, edges: J) -> Self
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = (usize, usize)>;

    /// Constructs the vertex-induced subgraph.
    ///
    /// Constructs a subgraph given a subset of vertices.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME: Add doc examples.
    /// ```
    ///
    fn subgraph_by_vertices<I>(&self, vertices: I) -> Self
    where
        I: IntoIterator<Item = usize>;

    /// Constructs the edge-induced subgraph.
    ///
    /// Constructs a subgraph given a subset of edges.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME: Add doc examples.
    /// ```
    ///
    fn subgraph_by_edges<J>(&self, edges: J) -> Self
    where
        J: IntoIterator<Item = (usize, usize)>;
}
