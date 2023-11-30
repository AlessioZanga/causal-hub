use super::{Undirected, UndirectedGraph};
use crate::{graphs::Graph, E, L};

/// Define the `An` ancestor macro.
#[macro_export]
macro_rules! An {
    ($g:expr, $x:expr) => {
        $g.ancestors_iter($x)
    };
}

/// Define the `Pa` parent macro.
#[macro_export]
macro_rules! Pa {
    ($g:expr, $x:expr) => {
        $g.parents_iter($x)
    };
}

/// Define the `Ch` child macro.
#[macro_export]
macro_rules! Ch {
    ($g:expr, $x:expr) => {
        $g.children_iter($x)
    };
}

/// Define the `De` descendant macro.
#[macro_export]
macro_rules! De {
    ($g:expr, $x:expr) => {
        $g.descendants_iter($x)
    };
}

/// Define the `Directed` direction type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Directed {}

/// Define the `DirectedGraph` trait.
///
/// If the `Direction` associated type is `Directed`, then
/// the methods of this trait are delegated to the `Graph` trait.
///
pub trait DirectedGraph: Graph {
    /// Directed edges indices iterator associated type.
    type DirectedEdgesIter<'a>: Iterator<Item = (usize, usize)>
    where
        Self: 'a;

    /// Ancestors indices iterator associated type.
    type AncestorsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Parents indices iterator associated type.
    type ParentsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Children indices iterator associated type.
    type ChildrenIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Descendants indices iterator associated type.
    type DescendantsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Associated undirected graph type.
    type UndirectedGraph: UndirectedGraph<Direction = Undirected>;

    /// Get the directed size.
    ///
    /// The directed size is the number of directed edges.
    ///
    /// # Returns
    /// The directed size.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn directed_size(&self) -> usize;

    /// Get the directed edges indices iterator.
    ///
    /// # Returns
    /// The directed edges indices iterator.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The directed edges indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    fn directed_edges_iter(&self) -> Self::DirectedEdgesIter<'_>;

    /// Check if the directed edge exists.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the directed edge exists, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn has_directed_edge(&self, x: usize, y: usize) -> bool;

    /// Add a directed edge.
    ///
    /// The directed edge is added only if it does not exist.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the directed edge was added, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn add_directed_edge(&mut self, x: usize, y: usize) -> bool;

    /// Delete a directed edge.
    ///
    /// The directed edge is deleted only if it exists.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the directed edge was deleted, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn del_directed_edge(&mut self, x: usize, y: usize) -> bool;

    /// Get the vertex in-degree.
    ///
    /// The in-degree of a vertex is the number of directed edges pointing to it.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex in-degree.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn in_degree(&self, x: usize) -> usize;

    /// Get the vertices in-degrees.
    ///
    /// Also known as the in-degree vector.
    ///
    /// # Returns
    /// The vertices in-degrees.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn in_degrees(&self) -> Vec<usize>;

    /// Get the vertex out-degree.
    ///
    /// The out-degree of a vertex is the number of directed edges pointing from it.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex out-degree.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn out_degree(&self, x: usize) -> usize;

    /// Get the vertices out-degrees.
    ///
    /// Also known as the out-degree vector.
    ///
    /// # Returns
    /// The vertices out-degrees.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn out_degrees(&self) -> Vec<usize>;

    /// Get the vertex ancestors indices iterator.
    ///
    /// The vertex ancestors indices are the vertices with a directed path to the vertex.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex ancestors indices iterator.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The vertex ancestors indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    fn ancestors_iter(&self, x: usize) -> Self::AncestorsIter<'_>;

    /// Check if the vertex is an ancestor of a vertex.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the first vertex is an ancestor of the second vertex, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// For repeated calls, it is more efficient to store the ancestors of the first vertex in a set.
    ///
    fn is_ancestor(&self, x: usize, y: usize) -> bool;

    /// Get the vertex parents indices iterator.
    ///
    /// The vertex parents indices are the vertices with a directed edge to the vertex.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex parents indices iterator.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The vertex parents indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    fn parents_iter(&self, x: usize) -> Self::ParentsIter<'_>;

    /// Check if the vertex is a parent of a vertex.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the first vertex is a parent of the second vertex, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn is_parent(&self, x: usize, y: usize) -> bool;

    /// Get the vertex children indices iterator.
    ///
    /// The vertex children indices are the vertices with a directed edge from the vertex.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex children indices iterator.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The vertex children indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    fn children_iter(&self, x: usize) -> Self::ChildrenIter<'_>;

    /// Check if the vertex is a child of a vertex.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the first vertex is a child of the second vertex, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn is_child(&self, x: usize, y: usize) -> bool;

    /// Get the vertex descendants indices iterator.
    ///
    /// The vertex descendants indices are the vertices with a directed path from the vertex.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex descendants indices iterator.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The vertex descendants indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    fn descendants_iter(&self, x: usize) -> Self::DescendantsIter<'_>;

    /// Check if the vertex is a descendant of a vertex.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the first vertex is a descendant of the second vertex, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// For repeated calls, it is more efficient to store the descendants of the first vertex in a set.
    ///
    fn is_descendant(&self, x: usize, y: usize) -> bool;

    /// Get the associated undirected graph.
    ///
    /// # Returns
    /// The associated undirected graph where each directed edge is mapped to an undirected edge.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The undirected graph is constructed by adding each directed edge as an undirected edge.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let digraph = DGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Get the associated undirected graph.
    /// let graph = digraph.to_undirected();
    ///
    /// // Check graphs have the same vertices.
    /// assert!(L!(digraph).eq(L!(graph)));
    ///
    /// // Check the undirected edges.
    /// for (x, y) in E!(digraph) {
    ///     assert!(graph.has_edge(x, y));
    ///     assert!(graph.has_edge(y, x));
    /// }
    /// ```
    ///
    fn to_undirected(&self) -> Self::UndirectedGraph {
        // Initialize an empty undirected graph.
        let mut graph = Self::UndirectedGraph::empty(L!(self));

        // Add the edges.
        for (x, y) in E!(self) {
            graph.add_undirected_edge(x, y);
        }

        // Debug assert each directed edge is also an undirected edge.
        debug_assert!(
            E!(self).all(|(x, y)| graph.has_edge(x, y) && graph.has_edge(y, x)),
            "Each directed edge must be an undirected edge"
        );

        graph
    }
}
