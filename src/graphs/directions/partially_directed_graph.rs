use super::{Directed, DirectedGraph, UndirectedGraph};

/// Define the `uE` undirected edges macro.
#[macro_export]
macro_rules! uE {
    ($g:expr) => {
        $g.undirected_edges_iter()
    };
}

/// Define the `dE` directed edges macro.
#[macro_export]
macro_rules! dE {
    ($g:expr) => {
        $g.directed_edges_iter()
    };
}

/// Define the `PartiallyDirected` direction type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartiallyDirected {}

/// Define the `PartiallyDirectedGraph` trait.
pub trait PartiallyDirectedGraph: UndirectedGraph + DirectedGraph {
    /// Associated directed graph type.
    type DirectedGraph: DirectedGraph<Direction = Directed>;

    /// Reference to the directed subgraph.
    fn directed_subgraph(&self) -> &Self::DirectedGraph;

    /// Reference to the undirected subgraph.
    fn undirected_subgraph(&self) -> &Self::UndirectedGraph;

    /// Set an already existing edge as directed.
    ///
    /// If the edge does not exist, then no edge is added.
    ///
    /// # Description
    /// $\mathbf{E} \cup \lbrace (X \rightarrow Y) \rbrace \quad \text{iff} \quad (X - Y) \in \mathbf{E}$
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the edge is set as directed, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn set_directed_edge(&mut self, x: usize, y: usize) -> bool;

    /// Set an already existing edge as undirected.
    ///
    /// If the edge does not exist, then no edge is added.
    ///
    /// # Description
    /// $\mathbf{E} \cup \lbrace (X - Y) \rbrace \quad \text{iff} \quad (X \rightarrow Y) \in \mathbf{E} \medspace \vee \medspace (Y \rightarrow X) \in \mathbf{E}$
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the edge is set as undirected, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn set_undirected_edge(&mut self, x: usize, y: usize) -> bool;
}
