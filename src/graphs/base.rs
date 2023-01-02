use std::{
    fmt::{Debug, Display},
    iter::FusedIterator,
};

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

    /// Vertices iterator type.
    type VerticesIter<'a>: Iterator<Item = usize> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Edges iterator type.
    type EdgesIter<'a>: Iterator<Item = (usize, usize)> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Adjacents vertices iterator type.
    type AdjacentsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// New constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The new constructor of $\mathcal{G}$
    /// returns a graph $G$ based on $V$ and $E$.
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

    /// Gets the vertex index.
    ///
    /// Returns the vertex index given its label.
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
    /// // Get vertex index.
    /// let x = g.index("A");
    ///
    /// // Check vertex index.
    /// assert_eq!(x, 0);
    /// ```
    ///
    fn index(&self, x: &str) -> usize;

    /// Gets the vertex label.
    ///
    /// Returns the vertex label given its index.
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
    /// // Build a 3rd order graph.
    /// let g = Graph::empty(["A", "B", "C"]);
    ///
    /// // Get vertex label.
    /// let x = g.vertex(0);
    ///
    /// // Check vertex label.
    /// assert_eq!(x, "A");
    /// ```
    ///
    fn vertex(&self, x: usize) -> &str;

    /// Vertex iterator.
    ///
    /// Iterates over the vertex set $V$ ordered by index value.
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
    /// assert!(g.vertices().eq(0..g.order()));
    ///
    /// // Use the associated macro 'V!'.
    /// assert!(g.vertices().eq(V!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in V!(g) {
    ///     assert!(g.has_vertex(x));
    /// }
    /// ```
    ///
    fn vertices(&self) -> Self::VerticesIter<'_>;

    /// Order of the graph.
    ///
    /// Return the graph order (aka. $|V|$).
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
    fn order(&self) -> usize {
        V!(self).len()
    }

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
    /// let (x, y, z) = (g.index("A"), g.index("B"), g.order() + 1);
    ///
    /// // Check vertices.
    /// assert!(g.has_vertex(x));
    /// assert!(g.has_vertex(y));
    /// assert!(!g.has_vertex(z));
    /// ```
    ///
    fn has_vertex(&self, x: usize) -> bool {
        V!(self).any(|y| y == x)
    }

    /// Adds vertex to the graph.
    ///
    /// Insert a new vertex index into the graph.
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
    /// assert!(g.has_vertex(x));
    /// ```
    ///
    fn add_vertex<V>(&mut self, x: V) -> usize
    where
        V: Into<String>;

    /// Deletes vertex from the graph.
    ///
    /// Remove given vertex index from the graph.
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
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Add a new vertex.
    /// let x = g.add_vertex("A");
    /// assert!(g.has_vertex(x));
    ///
    /// // Delete the newly added vertex.
    /// assert!(g.del_vertex(x));
    /// assert!(!g.has_vertex(x));
    ///
    /// // Deleting a non-existing vertex return false.
    /// assert!(!g.del_vertex(x));
    /// ```
    ///
    fn del_vertex(&mut self, x: usize) -> bool;

    /// Edge iterator.
    ///
    /// Iterates over the edge set $E$ order by index values.
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
    /// assert!(g.edges().eq([(0, 1), (2, 3)]));
    ///
    /// // Use the associated macro 'E!'.
    /// assert!(g.edges().eq(E!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for (x, y) in E!(g) {
    ///     assert!(g.has_edge(x, y));
    /// }
    /// ```
    ///
    fn edges(&self) -> Self::EdgesIter<'_>;

    /// Size of the graph.
    ///
    /// Return the graph size (aka. $|E|$).
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
    /// let mut g = Graph::from(e);
    /// assert_eq!(g.size(), 5);
    /// ```
    ///
    fn size(&self) -> usize {
        E!(self).len()
    }

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
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
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a graph.
    /// let g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("A"), g.index("B"));
    ///
    /// // Check edge.
    /// assert!(g.has_edge(x, y));
    /// ```
    ///
    fn has_edge(&self, x: usize, y: usize) -> bool {
        E!(self).any(|z| z == (x, y))
    }

    /// Adds edge to the graph.
    ///
    /// Add new edge index into the graph.
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
    /// // Define vertex set.
    /// let v = ["A", "B"];
    ///
    /// // Build a 2nd order graph.
    /// let mut g = Graph::empty(v);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("A"), g.index("B"));
    ///
    /// // Add a new edge from vertex.
    /// assert!(g.add_edge(x, y));
    /// assert!(g.has_edge(x, y));
    ///
    /// // Adding an existing edge return false.
    /// assert!(!g.add_edge(x, y));
    /// ```
    ///
    fn add_edge(&mut self, x: usize, y: usize) -> bool;

    /// Deletes edge from the graph.
    ///
    /// Remove given edge index from the graph.
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
    /// let e = EdgeList::from([("A", "B"), ("D", "C")]);
    ///
    /// // Build a graph.
    /// let mut g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("A"), g.index("B"));
    ///
    /// // Delete an edge.
    /// assert!(g.del_edge(x, y));
    /// assert!(!g.has_edge(x, y));
    ///
    /// // Deleting a non-existing edge return false.
    /// assert!(!g.del_edge(x, y));
    /// ```
    ///
    fn del_edge(&mut self, x: usize, y: usize) -> bool;

    /// Adjacent iterator.
    ///
    /// Iterates over the vertex set $Adj(G, X)$ of a given vertex $X$.
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
    /// // Choose vertex.
    /// let x = g.index("A");
    ///
    /// // Use the adjacent iterator.
    /// assert!(g.adjacents(x).eq([0, 1, 2]));
    ///
    /// // Use the associated macro 'Adj!'.
    /// assert!(g.adjacents(x).eq(Adj!(g, x)));
    ///
    /// // Iterate over the adjacent set.
    /// for y in Adj!(g, x) {
    ///     assert!(g.has_edge(x, y));
    /// }
    /// ```
    ///
    fn adjacents(&self, x: usize) -> Self::AdjacentsIter<'_>;

    /// Checks adjacent vertices in the graph.
    ///
    /// Checks whether the graph has two adjacent vertices or not.
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
    /// let mut g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("A"), g.index("B"));
    ///
    /// // Check edge.
    /// assert!(g.is_adjacent(x, y));
    /// assert!(Adj!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        Adj!(self, x).any(|z| z == y)
    }
}
