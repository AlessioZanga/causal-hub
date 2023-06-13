use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::FusedIterator,
};

use serde::{Deserialize, Serialize};

/// Labels iterator.
///
/// Return the labels iterator representing $L(\mathcal{G})$.
///
#[macro_export]
macro_rules! L {
    ($g:expr) => {
        $g.get_vertices()
    };
}

/// Vertex iterator.
///
/// Return the vertex iterator representing $V(\mathcal{G})$.
///
#[macro_export]
macro_rules! V {
    ($g:expr) => {
        $g.get_vertices_index()
    };
}

/// Edge iterator.
///
/// Return the edges iterator representing $E(\mathcal{G})$.
///
#[macro_export]
macro_rules! E {
    ($g:expr) => {
        $g.get_edges_index()
    };
}

/// Adjacency iterator.
///
/// Return the vertex iterator representing $Adj(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Adj {
    ($g:expr, $x:expr) => {
        $g.get_adjacents_index($x)
    };
}

/// Base graph trait.
pub trait BaseGraph:
    Clone + Debug + Default + Display + Eq + Hash + Send + Sync + Serialize + for<'a> Deserialize<'a>
{
    /// Data type.
    type Data;

    /// Directional type.
    type Direction;

    /// Labels iterator type.
    type VerticesIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Vertices iterator type.
    type VerticesIndexIter<'a>: Clone + Iterator<Item = usize> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Edges iterator type.
    type EdgesIndexIter<'a>: Iterator<Item = (usize, usize)> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Adjacents vertices iterator type.
    type AdjacentsIndexIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// New constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The new constructor of $\mathcal{G}$
    /// returns a graph $\mathcal{G}$ based on $\mathbf{V}$ and $\mathbf{E}$.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a new graph.
    /// let g = Graph::new(
    ///     ["0", "1", "2"],
    ///     [("0", "1"), ("1", "2")]
    /// );
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also not empty.
    /// assert_eq!(g.size(), 2);
    /// ```
    ///
    fn new<V, I, J>(vertices: I, edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>;

    /// Null constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The null constructor of $\mathcal{G}$
    /// returns a null graph $\mathcal{G}$ (i.e. both $\mathbf{V}$ and $\mathbf{E}$ are empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a null graph.
    /// let g = Graph::null();
    ///
    /// // The vertex set is empty.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn null() -> Self;

    /// Empty constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The empty constructor of $\mathcal{G}$
    /// returns an empty graph $\mathcal{G}$ (i.e. $\mathbf{E}$ is empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build an empty graph.
    /// let g = Graph::empty(["A", "B", "C"]);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn empty<V, I>(labels: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>;

    /// Complete constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The complete constructor of $\mathcal{G}$
    /// returns an complete graph $\mathcal{G}$ (i.e. $\mathbf{E}$ is $V \times V$).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a complete graph.
    /// let g = DiGraph::complete(["A", "B", "C"]);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also not empty.
    /// assert_eq!(g.size(), 6);
    /// ```
    ///
    fn complete<V, I>(labels: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>;

    /// Clears the graph.
    ///
    /// Clears the graph, removing both vertex and edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "D")]);
    ///
    /// // Build a new graph.
    /// let mut g = Graph::from(e);
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
    /// ```
    ///
    fn clear(&mut self);

    /// Order of the graph.
    ///
    /// Return the graph order (aka. $|\mathbf{V}|$).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a 5th order graph.
    /// let g = Graph::empty(["A", "B", "C", "D", "E"]);
    ///
    /// // Check order.
    /// assert_eq!(g.order(), 5);
    /// ```
    ///
    fn order(&self) -> usize;

    /// Labels iterator.
    ///
    /// Iterates over the vertex labels set.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::empty(["A", "B", "C"]);
    ///
    /// // Use the vertex set iterator.
    /// assert!(L!(g).eq(["A", "B", "C"]));
    ///
    /// // Iterate over the vertex set.
    /// for x in L!(g) {
    ///     assert!(g.has_vertex_by_index(g.get_vertex_index(x)));
    /// }
    /// ```
    ///
    fn get_vertices(&self) -> Self::VerticesIter<'_>;

    /// Gets the vertex label.
    ///
    /// Returns the vertex label given its identifier.
    ///
    /// # Panics
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::empty(["A", "B", "C"]);
    ///
    /// // Get vertex label.
    /// let x = g.get_vertex_by_index(0);
    ///
    /// // Check vertex label.
    /// assert_eq!(x, "A");
    /// ```
    ///
    fn get_vertex_by_index(&self, x: usize) -> &str;

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
    /// assert!(g.has_vertex_by_index(x));
    /// ```
    ///
    fn add_vertex<V>(&mut self, x: V) -> usize
    where
        V: Into<String>;

    /// Vertex iterator.
    ///
    /// Iterates over the vertex set $\mathbf{V}$ ordered by identifier value.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::empty(["A", "B", "C"]);
    ///
    /// // Use the vertex set iterator.
    /// assert!(g.get_vertices_index().eq(0..g.order()));
    ///
    /// // Use the associated macro 'V!'.
    /// assert!(g.get_vertices_index().eq(V!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in V!(g) {
    ///     assert!(g.has_vertex_by_index(x));
    /// }
    /// ```
    ///
    fn get_vertices_index(&self) -> Self::VerticesIndexIter<'_>;

    /// Gets the vertex identifier.
    ///
    /// Returns the vertex identifier given its label.
    ///
    /// # Panics
    ///
    /// The vertex label does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::empty(["A", "B", "C"]);
    ///
    /// // Get vertex identifier.
    /// let x = g.get_vertex_index("A");
    ///
    /// // Check vertex identifier.
    /// assert_eq!(x, 0);
    /// ```
    ///
    fn get_vertex_index(&self, x: &str) -> usize;

    /// Checks vertex in the graph.
    ///
    /// Checks whether the graph has a given vertex or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define vertex set.
    /// let v = ["A", "B"];
    ///
    /// // Build a 2nd order graph.
    /// let g = Graph::empty(v);
    ///
    /// // Choose vertices.
    /// let (x, y, z) = (g.get_vertex_index("A"), g.get_vertex_index("B"), g.order() + 1);
    ///
    /// // Check vertices.
    /// assert!(g.has_vertex_by_index(x));
    /// assert!(g.has_vertex_by_index(y));
    /// assert!(!g.has_vertex_by_index(z));
    /// ```
    ///
    fn has_vertex_by_index(&self, x: usize) -> bool;

    /// Deletes vertex from the graph.
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Panics
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
    /// assert!(g.has_vertex_by_index(x));
    ///
    /// // Delete the newly added vertex.
    /// assert!(g.del_vertex_by_index(x));
    /// assert!(!g.has_vertex_by_index(x));
    ///
    /// // Deleting a non-existing vertex return false.
    /// assert!(!g.del_vertex_by_index(x));
    /// ```
    ///
    fn del_vertex_by_index(&mut self, x: usize) -> bool;

    /// Size of the graph.
    ///
    /// Return the graph size (aka. $|\mathbf{E}|$).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([
    ///     ("A", "B"), ("C", "A"), ("D", "C"), ("B", "C"), ("A", "A")
    /// ]);
    ///
    /// // Build a new graph.
    /// let g = Graph::from(e);
    /// assert_eq!(g.size(), 5);
    /// ```
    ///
    fn size(&self) -> usize;

    /// Edge iterator.
    ///
    /// Iterates over the edge set $\mathbf{E}$ order by identifier values.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a 4th order graph.
    /// let g = Graph::from(e);
    ///
    /// // Use the vertex set iterator.
    /// assert!(g.get_edges_index().eq([(0, 1), (2, 3)]));
    ///
    /// // Use the associated macro 'E!'.
    /// assert!(g.get_edges_index().eq(E!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for (x, y) in E!(g) {
    ///     assert!(g.has_edge_by_index(x, y));
    /// }
    /// ```
    ///
    fn get_edges_index(&self) -> Self::EdgesIndexIter<'_>;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
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
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a graph.
    /// let g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.get_vertex_index("A"), g.get_vertex_index("B"));
    ///
    /// // Check edge.
    /// assert!(g.has_edge_by_index(x, y));
    /// ```
    ///
    fn has_edge_by_index(&self, x: usize, y: usize) -> bool;

    /// Adds edge to the graph.
    ///
    /// Add new edge identifier into the graph.
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
    /// // Define vertex set.
    /// let v = ["A", "B"];
    ///
    /// // Build a 2nd order graph.
    /// let mut g = Graph::empty(v);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.get_vertex_index("A"), g.get_vertex_index("B"));
    ///
    /// // Add a new edge from vertex.
    /// assert!(g.add_edge_by_index(x, y));
    /// assert!(g.has_edge_by_index(x, y));
    ///
    /// // Adding an existing edge return false.
    /// assert!(!g.add_edge_by_index(x, y));
    /// ```
    ///
    fn add_edge_by_index(&mut self, x: usize, y: usize) -> bool;

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
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
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a graph.
    /// let mut g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.get_vertex_index("A"), g.get_vertex_index("B"));
    ///
    /// // Delete an edge.
    /// assert!(g.del_edge_by_index(x, y));
    /// assert!(!g.has_edge_by_index(x, y));
    ///
    /// // Deleting a non-existing edge return false.
    /// assert!(!g.del_edge_by_index(x, y));
    /// ```
    ///
    fn del_edge_by_index(&mut self, x: usize, y: usize) -> bool;

    /// Adjacent iterator.
    ///
    /// Iterates over the vertex set $Adj(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// The vertex identifier does not exist in the graph.
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
    /// let x = g.get_vertex_index("A");
    ///
    /// // Use the adjacent iterator.
    /// assert!(g.get_adjacents_index(x).eq([0, 1, 2]));
    ///
    /// // Use the associated macro 'Adj!'.
    /// assert!(g.get_adjacents_index(x).eq(Adj!(g, x)));
    ///
    /// // Iterate over the adjacent set.
    /// for y in Adj!(g, x) {
    ///     assert!(g.has_edge_by_index(x, y));
    /// }
    /// ```
    ///
    fn get_adjacents_index(&self, x: usize) -> Self::AdjacentsIndexIter<'_>;

    /// Checks adjacent vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is adjacent to another vertex $X$ or not.
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
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.get_vertex_index("A"), g.get_vertex_index("B"));
    ///
    /// // Check edge.
    /// assert!(g.is_adjacent_by_index(x, y));
    /// assert!(Adj!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_adjacent_by_index(&self, x: usize, y: usize) -> bool;
}
