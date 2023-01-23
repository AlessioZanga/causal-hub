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
    /// use causal_hub::prelude::*;
    ///
    /// // Build a new directed graph.
    /// let g = DiGraph::new(
    ///     ["A", "B", "C", "D", "E", "F"],
    ///     [
    ///         ("A", "C"),
    ///         ("B", "C"),
    ///         ("C", "D"),
    ///         ("C", "E"),
    ///     ]
    /// );
    ///
    /// // Compute generic subgraph.
    /// let h = g.subgraph(
    ///     [0, 1, 2, 3],
    ///     [
    ///         (0, 2),
    ///         (1, 2)
    ///     ]
    /// );
    ///
    /// // Assert is subgraph.
    /// assert!(h.is_subgraph(&g));
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
    /// use causal_hub::prelude::*;
    ///
    /// // Build a new directed graph.
    /// let g = DiGraph::new(
    ///     ["A", "B", "C", "D", "E", "F"],
    ///     [
    ///         ("A", "C"),
    ///         ("B", "C"),
    ///         ("C", "D"),
    ///         ("C", "E"),
    ///     ]
    /// );
    ///
    /// // Compute subgraph by vertices.
    /// let h = g.subgraph_by_vertices([0, 1, 2, 3]);
    ///
    /// // Assert is subgraph.
    /// assert!(h.is_subgraph(&g));
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
    /// use causal_hub::prelude::*;
    ///
    /// // Build a new directed graph.
    /// let g = DiGraph::new(
    ///     ["A", "B", "C", "D", "E", "F"],
    ///     [
    ///         ("A", "C"),
    ///         ("B", "C"),
    ///         ("C", "D"),
    ///         ("C", "E"),
    ///     ]
    /// );
    ///
    /// // Compute subgraph by edges.
    /// let h = g.subgraph_by_edges([(0, 2), (1, 2)]);
    ///
    /// // Assert is subgraph.
    /// assert!(h.is_subgraph(&g));
    /// ```
    ///
    fn subgraph_by_edges<J>(&self, edges: J) -> Self
    where
        J: IntoIterator<Item = (usize, usize)>;
}
