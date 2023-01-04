use super::BaseGraph;

/// Neighbors iterator.
///
/// Return the vertex iterator representing $Ne(\mathcal{G}, X)$.
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
    type NeighborsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Neighbors iterator.
    ///
    /// Iterates over the vertex set $Ne(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
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
    /// let g = Graph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.index("A");
    ///
    /// // Use the neighbors iterator.
    /// assert!(g.neighbors(x).eq([0, 1, 2]));
    ///
    /// // Use the associated macro 'Ne!'.
    /// assert!(g.neighbors(x).eq(Ne!(g, x)));
    /// ```
    ///
    fn neighbors(&self, x: usize) -> Self::NeighborsIter<'_>;

    /// Checks neighbor vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is neighbor of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex indexes does not exist in the graph.
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
    /// let g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("A"), g.index("B"));
    ///
    /// // Check edge.
    /// assert!(g.is_neighbor(x, y));
    /// assert!(Ne!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_neighbor(&self, x: usize, y: usize) -> bool {
        self.neighbors(x).any(|z| z == y)
    }

    /// Degree of a vertex.
    ///
    /// Computes the degree of a given vertex, i.e. $|Ne(\mathcal{G}, X)|$.
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
