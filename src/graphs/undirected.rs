use std::iter::FusedIterator;

use super::BaseGraph;

/// Neighbors iterator.
///
/// Return the vertex iterator representing $Ne(G, X)$.
///
#[macro_export]
macro_rules! Ne {
    ($g:expr, $x:expr) => {
        $g.neighbors($x)
    };
}

/// Undirected graph trait.
pub trait UndirectedGraph: BaseGraph {
    /// Neighbors iterator type.
    type NeighborsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Iterator over the neighbors set.
    // FIXME: Add docs.
    fn neighbors(&self, x: usize) -> Self::NeighborsIter<'_>;

    /// Checks if a vertex is neighbor of another vertex.
    // FIXME: Add docs.
    fn is_neighbor(&self, x: usize, y: usize) -> bool {
        self.neighbors(x).any(|z| z == y)
    }

    /// Degree of a vertex.
    ///
    /// Computes the degree of a given vertex.
    ///
    /// # Panics
    ///
    /// The vertex index does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let mut g = Graph::from(e);
    ///
    /// // Choose a vertex.
    /// let x = g.index("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.degree(x), 3);
    /// assert_eq!(g.degree(x), Ne!(g, x).count());
    /// ```
    ///
    fn degree(&self, x: usize) -> usize {
        Ne!(self, x).count()
    }
}
