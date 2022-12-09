use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::FusedIterator,
};

/// Directions pseudo-enumerator for generics algorithms.
pub mod directions {
    /// Undirected pseudo-enumerator for generics algorithms.
    pub struct Undirected;
    /// Directed pseudo-enumerator for generics algorithms.
    pub struct Directed;
}

/// Base graph trait.
pub trait BaseGraph: Clone + Debug + Display {
    /// Data type.
    type Data;

    /// Directional type.
    type Direction;

    /// Vertex type.
    type Vertex: Clone + Debug + Eq + Ord + Hash;

    /// Vertices iterator type.
    type VerticesIter<'a>: ExactSizeIterator<Item = &'a Self::Vertex> + FusedIterator
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Edge type.
    // TODO: Replace with "associated type defaults" once stabilized.
    type Edge<'a>: From<(&'a Self::Vertex, &'a Self::Vertex)>
        + Into<(&'a Self::Vertex, &'a Self::Vertex)>
        + Eq
        + Ord
        + Hash
    where
        Self: 'a;

    /// Edges iterator type.
    type EdgesIter<'a>: ExactSizeIterator<Item = Self::Edge<'a>> + FusedIterator
    where
        Self: 'a;

    /// Adjacents vertices iterator type.
    type AdjacentsIter<'a>: Iterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// New constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The new constructor of $\mathcal{G}$
    /// returns a graph $G$ based on $V$ and $E$.
    ///
    /// # FIXME: Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a new graph.
    /// let g = Graph::new((0..3), [(0, 1), (1, 2)]);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also not empty.
    /// assert_eq!(g.size(), 2);
    /// ```
    ///
    fn new<I, J>(vertices: I, edges: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex)>;

    /// Clears the graph.
    ///
    /// Clears the graph, removing both vertex and edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "D")]);
    ///
    /// // Build a new graph.
    /// let mut g = Graph::try_from(e)?;
    ///
    /// // The graph *is not* null.
    /// assert_ne!(g.order(), 0);
    /// assert_ne!(g.size(), 0);
    ///
    /// // Clear the graph.
    /// g.clear();
    ///
    /// // The graph *is* null.
    /// assert_eq!(g.order(), 0);
    /// assert_eq!(g.size(), 0);
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn clear(&mut self);

    /// Order of the graph.
    ///
    /// Return the graph order (aka. $|V|$).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Build a 5th order graph.
    /// let g = Graph::empty(["A", "B", "C", "D", "E"])?;
    /// assert_eq!(g.order(), 5);
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn order(&self) -> usize;

    /// Vertex iterator.
    ///
    /// Iterates over the vertex set $V$ ordered by identifier value.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Build a 3rd order graph.
    /// let g = Graph::empty(["A", "B", "C"])?;
    ///
    /// // Use the vertex set iterator.
    /// assert!(g.vertices().eq(&["A", "B", "C"]));
    ///
    /// // Use the associated macro 'V!'.
    /// assert!(g.vertices().eq(V!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in V!(g) {
    ///     assert!(g.has_vertex(x));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn vertices(&self) -> Self::VerticesIter<'_>;

    /// Checks vertex in the graph.
    ///
    /// Checks whether the graph has a given vertex or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Define vertex set.
    /// let v = ["A", "B"];
    ///
    /// // Build a 2nd order graph.
    /// let g = Graph::empty(v)?;
    ///
    /// // Choose vertices.
    /// let (x, y, z) = ("A".into(), "B".into(), "C".into());
    ///
    /// // Check vertices.
    /// assert!(g.has_vertex(&x));
    /// assert!(g.has_vertex(&y));
    /// assert!(!g.has_vertex(&z));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn has_vertex(&self, x: &Self::Vertex) -> bool;

    /// Adds vertex to the graph.
    ///
    /// Insert a new vertex identifier into the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Add a new vertex.
    /// let x = g.add_vertex("A");
    /// assert!(g.has_vertex(&x));
    /// ```
    ///
    fn add_vertex<V>(&mut self, x: V) -> Self::Vertex
    where
        V: Into<Self::Vertex>;

    /// Deletes vertex from the graph.
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # FIXME: Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Add a new vertex.
    /// let x = g.add_vertex("A");
    /// assert!(g.has_vertex(&x));
    ///
    /// // Delete the newly added vertex.
    /// assert!(g.del_vertex(&x));
    /// assert!(!g.has_vertex(&x));
    ///
    /// // Deleting a non-existing vertex return false.
    /// assert!(!g.del_vertex(&x));
    /// ```
    ///
    fn del_vertex(&mut self, x: &Self::Vertex) -> bool;

    /// Size of the graph.
    ///
    /// Return the graph size (aka. $|E|$).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Define edge set.
    /// let e = EdgeList::from([
    ///     ("A", "B"), ("C", "A"), ("D", "C"), ("B", "C"), ("A", "A")
    /// ]);
    ///
    /// // Build a new graph.
    /// let mut g = Graph::try_from(e)?;
    /// assert_eq!(g.size(), 5);
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn size(&self) -> usize;

    /// Edge iterator.
    ///
    /// Iterates over the edge set $E$ order by identifier values.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a 4th order graph.
    /// let g = Graph::try_from(e)?;
    ///
    /// // Use the vertex set iterator.
    /// let e = [(&"A".into(), &"B".into()), (&"C".into(), &"D".into())];
    /// assert!(g.edges().eq(e));
    ///
    /// // Use the associated macro 'E!'.
    /// assert!(g.edges().eq(E!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for (x, y) in E!(g) {
    ///     assert!(g.has_edge(&x, &y));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn edges(&self) -> Self::EdgesIter<'_>;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # FIXME: Panics
    ///
    /// Panics if at least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a graph.
    /// let g = Graph::try_from(e)?;
    ///
    /// // Choose an edge.
    /// let (x, y) = ("A".into(), "B".into());
    ///
    /// // Check edge.
    /// assert!(g.has_edge(&x, &y));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Adds edge to the graph.
    ///
    /// Add new edge identifier into the graph.
    ///
    /// # FIXME: Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Define vertex set.
    /// let v = ["A", "B"];
    ///
    /// // Build a 2nd order graph.
    /// let mut g = Graph::empty(v)?;
    ///
    /// // Choose an edge.
    /// let (x, y) = ("A".into(), "B".into());
    ///
    /// // Add a new edge from vertex.
    /// assert!(g.add_edge(&x, &y));
    /// assert!(g.has_edge(&x, &y));
    ///
    /// // Adding an existing edge return false.
    /// assert!(!g.add_edge(&x, &y));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # FIXME: Panics
    ///
    /// Panics if at least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a graph.
    /// let mut g = Graph::try_from(e)?;
    ///
    /// // Choose an edge.
    /// let (x, y) = ("A".into(), "B".into());
    ///
    /// // Delete an edge.
    /// assert!(g.del_edge(&x, &y));
    /// assert!(!g.has_edge(&x, &y));
    ///
    /// // Deleting a non-existing edge return false.
    /// assert!(!g.del_edge(&x, &y));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Adjacent iterator.
    ///
    /// Iterates over the vertex set $Adj(G, X)$ of a given vertex $X$.
    ///
    /// # FIXME: Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # FIXME: Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a graph from edges.
    /// let g = Graph::from_edges([(0, 1), (2, 0), (0, 0)]);
    ///
    /// // Use the adjacent iterator.
    /// assert!(g.adjacents_iter(&0).eq(&[0, 1, 2]));
    ///
    /// // Use the associated macro 'Adj!'.
    /// assert!(g.adjacents_iter(&0).eq(Adj!(g, &0)));
    ///
    /// // Iterate over the adjacent set.
    /// for &x in Adj!(g, &0) {
    ///     assert!(g.has_edge(&0, &x));
    /// }
    /// ```
    ///
    fn adjacents<'a>(&'a self, x: &'a Self::Vertex) -> Self::AdjacentsIter<'a>;

    /// Checks if a vertex is adjacent to another vertex.
    // FIXME: Add docs.
    fn is_adjacent(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;
}

/// Vertex iterator.
///
/// Return the vertex iterator representing $V(G)$.
///
#[macro_export]
macro_rules! V {
    ($g:expr) => {
        $g.vertices()
    };
}

/// Edge iterator.
///
/// Return the edges iterator representing $E(G)$.
///
#[macro_export]
macro_rules! E {
    ($g:expr) => {
        $g.edges()
    };
}

/// Adjacency iterator.
///
/// Return the vertex iterator representing $Adj(G, X)$.
///
#[macro_export]
macro_rules! Adj {
    ($g:expr, $x:expr) => {
        $g.adjacents($x)
    };
}
