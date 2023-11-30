use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::FusedIterator,
    ops::Index,
};

use serde::{Deserialize, Serialize};

/// Define the `L` labels macro.
#[macro_export]
macro_rules! L {
    ($g:expr) => {
        $g.labels()
    };
}

/// Define the `V` vertices macro.
#[macro_export]
macro_rules! V {
    ($g:expr) => {
        $g.vertices()
    };
}

/// Define the `E` edges macro.
#[macro_export]
macro_rules! E {
    ($g:expr) => {
        $g.edges()
    };
}

/// Define the `Adj` adjacents macro.
#[macro_export]
macro_rules! Adj {
    ($g:expr, $x:expr) => {
        $g.adjacents($x)
    };
}

/// Define the `Graph` trait.
///
/// # Methods
///
/// | Method            | Description                                   |
/// |-------------------|-----------------------------------------------|
/// | `new`             | $\mathcal{G} = (\mathbf{V}, \mathbf{E})$      |
/// | `null`            | $\mathcal{G} = (\varnothing, \varnothing)$    |
/// | `empty`           | $\mathcal{G} = (\mathbf{V}, \varnothing)$     |
/// | `complete`        | $\mathcal{G} = (\mathbf{V}, \mathbf{E})$      |
///
/// | Method            | Description                                   |
/// |-------------------|-----------------------------------------------|
/// | `order`           | $\|\mathbf{V}\|$                              |
/// | `vertices`        | $\mathbf{V}$                                  |
/// | `has_vertex`      | $X \in \mathbf{V}$                            |
/// | `add_vertex`      | $\mathbf{V} \cup \lbrace X \rbrace$           |
/// | `del_vertex`      | $\mathbf{V} \setminus \lbrace X \rbrace$      |
///
/// | Method            | Description                                   |
/// |-------------------|-----------------------------------------------|
/// | `size`            | $\|\mathbf{E}\|$                              |
/// | `edges`           | $\mathbf{E}$                                  |
/// | `has_edge`        | $(X, Y) \in \mathbf{E}$                       |
/// | `add_edge`        | $\mathbf{E} \cup \lbrace (X, Y) \rbrace$      |
/// | `del_edge`        | $\mathbf{E} \setminus \lbrace (X, Y) \rbrace$ |
/// | `degree`          | $\|Adj(X)\|$                                  |
/// | `adjacents`       | $Adj(X)$                                      |
/// | `is_adjacent`     | $Y \in Adj(X)$                                |
///
/// | Method            | Description                                   |
/// |-------------------|-----------------------------------------------|
/// | `subgraph`        | $\mathcal{G'} = (\mathbf{V'}, \mathbf{E'})$   |
/// | `is_subgraph`     | $\mathcal{G'} \subseteq \mathcal{G}$          |
/// | `is_supergraph`   | $\mathcal{G} \subseteq \mathcal{G'}$          |
///
pub trait Graph:
    Clone
    + Debug
    + Default
    + Display
    + Eq
    + PartialOrd
    + Index<usize, Output = str>
    + Hash
    + Serialize
    + for<'a> Deserialize<'a>
{
    /// Direction associated type.
    type Direction;
    /// Vertex labels iterator associated type.
    type LabelsIter<'a>: ExactSizeIterator<Item = &'a str> + FusedIterator
    where
        Self: 'a;
    /// Vertex indices iterator associated type.
    type VerticesIter<'a>: ExactSizeIterator<Item = usize> + FusedIterator
    where
        Self: 'a;
    /// Edge indices iterator associated type.
    type EdgesIter<'a>: ExactSizeIterator<Item = (usize, usize)> + FusedIterator
    where
        Self: 'a;
    /// Adjacents indices iterator associated type.
    type AdjacentsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Create a new graph.
    ///
    /// # Description
    /// $\mathcal{G} = (\mathbf{V}, \mathbf{E})$
    ///
    /// # Arguments
    /// * `vertices` - The vertices labels,
    /// * `edges` - The edges labels.
    ///
    /// # Returns
    /// The new graph.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// The vertices and edges are represented by labels.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph order is correct.
    /// assert_eq!(graph.order(), 4);
    /// // Assert the graph size is correct.
    /// assert_eq!(graph.size(), 4);
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "C", "D"]);
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3]);
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 3), (1, 2), (2, 3)]);
    /// ```
    ///
    fn new<V, I, J>(vertices: I, edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>;

    /// Create a new null graph.
    ///
    /// The null graph is a graph with no vertices and no edges.
    ///
    /// # Description
    /// $\mathcal{G} = (\varnothing, \varnothing)$
    ///
    /// # Returns
    /// The new null graph.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new null graph.
    /// let graph = UGraph::null();
    ///
    /// // Assert the graph order is correct.
    /// assert_eq!(graph.order(), 0);
    /// // Assert the graph size is correct.
    /// assert_eq!(graph.size(), 0);
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), Vec::<&str>::new());
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), Vec::<usize>::new());
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), Vec::<(usize, usize)>::new());
    /// ```
    ///
    fn null() -> Self;

    /// Create a new empty graph.
    ///
    /// The empty graph is a graph with no edges.
    ///
    /// # Description
    /// $\mathcal{G} = (\mathbf{V}, \varnothing)$
    ///
    /// # Arguments
    /// * `vertices` - The vertices labels.
    ///
    /// # Returns
    /// The new empty graph.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new empty graph.
    /// let graph = UGraph::empty(vec!["A", "B", "C", "D"]);
    ///
    /// // Assert the graph order is correct.
    /// assert_eq!(graph.order(), 4);
    /// // Assert the graph size is correct.
    /// assert_eq!(graph.size(), 0);
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "C", "D"]);
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3]);
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![]);
    /// ```
    ///
    fn empty<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>;

    /// Create a new complete graph.
    ///
    /// The complete graph is a graph where each vertex is connected to all other vertices, except itself.
    ///
    /// # Description
    /// $\mathcal{G} = (\mathbf{V}, \mathbf{E}) \quad \text{s.t.} \quad \mathbf{E} = \lbrace (X, Y) \mid X, Y \in \mathbf{V} \medspace \wedge \medspace X \neq Y \rbrace$
    ///
    /// # Arguments
    /// * `vertices` - The vertices labels.
    ///
    /// # Returns
    /// The new complete graph.
    ///
    /// # Complexity
    /// - Time: `O(n^2)`,
    /// - Space: `O(n^2)`.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new complete graph.
    /// let graph = UGraph::complete(vec!["A", "B", "C", "D"]);
    ///
    /// // Assert the graph order is correct.
    /// assert_eq!(graph.order(), 4);
    /// // Assert the graph size is correct.
    /// assert_eq!(graph.size(), 6);
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "C", "D"]);
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3]);
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    /// ```
    ///
    fn complete<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>;

    /// Get the vertices labels iterator.
    ///
    /// # Returns
    /// The vertices labels iterator.
    ///
    /// # Complexity
    /// - Time: `O(|V|)`,
    /// - Space: `O(|V|)`.
    ///
    /// # Notes
    /// The vertices labels are:
    /// - Unique,
    /// - Sorted lexically in ascending order,
    /// - In the same order as the vertices indices iterator,
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "C", "D"]);
    /// ```
    ///
    fn labels(&self) -> Self::LabelsIter<'_>;

    /// Get the vertex label.
    ///
    /// The vertex label is the vertex name.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex label.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.vertex_to_label(0), "A");
    /// assert_eq!(graph.vertex_to_label(1), "B");
    /// assert_eq!(graph.vertex_to_label(2), "C");
    /// assert_eq!(graph.vertex_to_label(3), "D");
    /// ```
    ///
    fn vertex_to_label(&self, x: usize) -> &str;

    /// Get the vertex index.
    ///
    /// The vertex index is the vertex identifier.
    ///
    /// # Arguments
    /// * `x` - The vertex label.
    ///
    /// # Returns
    /// The vertex index.
    ///
    /// # Panics
    /// If the vertex label does not exist.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.label_to_vertex("A"), 0);
    /// assert_eq!(graph.label_to_vertex("B"), 1);
    /// assert_eq!(graph.label_to_vertex("C"), 2);
    /// assert_eq!(graph.label_to_vertex("D"), 3);
    /// ```
    ///
    fn label_to_vertex(&self, x: &str) -> usize;

    /// Get the graph order.
    ///
    /// The order of a graph is the number of vertices.
    ///
    /// # Description
    /// $|\mathbf{V}|$
    ///
    /// # Returns
    /// The graph order.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph order is correct.
    /// assert_eq!(graph.order(), 4);
    /// ```
    ///
    fn order(&self) -> usize;

    /// Get the vertices indices iterator.
    ///
    /// # Description
    /// $\mathbf{V}$
    ///
    /// # Returns
    /// The vertices indices iterator.
    ///
    /// # Complexity
    /// - Time: `O(|V|)`,
    /// - Space: `O(|V|)`.
    ///
    /// # Notes
    /// The vertices indices are:
    /// - Unique,
    /// - Sorted in ascending order,
    /// - In the same order as the vertices labels iterator,
    /// - In the `[0, |V|)` range.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3]);
    /// ```
    ///
    fn vertices(&self) -> Self::VerticesIter<'_>;

    /// Check if the vertex exists.
    ///
    /// # Description
    /// $X \in \mathbf{V}$
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// `true` if the vertex exists, otherwise `false`.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.has_vertex(0), true);
    /// assert_eq!(graph.has_vertex(1), true);
    /// assert_eq!(graph.has_vertex(2), true);
    /// assert_eq!(graph.has_vertex(3), true);
    /// assert_eq!(graph.has_vertex(4), false);
    /// ```
    ///
    fn has_vertex(&self, x: usize) -> bool;

    /// Add a vertex.
    ///
    /// The vertex is added only if the vertex label does not exist.
    ///
    /// # Description
    /// $\mathbf{V} \cup \lbrace X \rbrace$
    ///
    /// # Arguments
    /// * `x` - The vertex label.
    ///
    /// # Returns
    /// The vertex index and a boolean indicating if the vertex was added.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// If the vertex label is added, then the vertices indices are reindexed
    /// so that the vertices indices are ordered according to the vertices labels.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let mut graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "E"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "E"), ("E", "A")]
    /// );
    ///
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "C", "E"]);
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3]);
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 3), (1, 2), (2, 3)]);
    ///
    /// // Add a new vertex.
    /// let (x, added) = graph.add_vertex("D");
    ///
    /// // Assert the vertex was added.
    /// assert_eq!(added, true);
    ///
    /// // Assert the vertex index is correct.
    /// assert_eq!(x, 3);
    ///
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "C", "D", "E"]);
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3, 4]);
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 4), (1, 2), (2, 4)]);
    /// ```
    ///
    fn add_vertex<V>(&mut self, x: V) -> (usize, bool)
    where
        V: Into<String>;

    /// Delete a vertex.
    ///
    /// The vertex is deleted only if the vertex index exists.
    ///
    /// # Description
    /// $\mathbf{V} \setminus \lbrace X \rbrace$
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// `true` if the vertex was deleted, otherwise `false`.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// If the vertex is deleted, then the vertices indices are reindexed
    /// so that the vertices indices are ordered according to the vertices labels.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let mut graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D", "E"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "E"), ("E", "A")]
    /// );
    ///
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "C", "D", "E"]);
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3, 4]);
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 4), (1, 2), (2, 3), (3, 4)]);
    ///
    /// // Delete a vertex.
    /// let deleted = graph.del_vertex(2);
    ///
    /// // Assert the vertex was deleted.
    /// assert_eq!(deleted, true);
    ///
    /// // Assert the graph vertices labels are correct.
    /// assert_eq!(graph.labels().collect_vec(), vec!["A", "B", "D", "E"]);
    /// // Assert the graph vertices indices are correct.
    /// assert_eq!(graph.vertices().collect_vec(), vec![0, 1, 2, 3]);
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 3), (2, 3)]);
    /// ```
    ///
    fn del_vertex(&mut self, x: usize) -> bool;

    /// Get the graph size.
    ///
    /// The size of a graph is the number of edges.
    ///
    /// # Description
    /// $|\mathbf{E}|$
    ///
    /// # Returns
    /// The graph size.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph size is correct.
    /// assert_eq!(graph.size(), 4);
    /// ```
    ///
    fn size(&self) -> usize;

    /// Get the edges indices iterator.
    ///
    /// # Description
    /// $\mathbf{E}$
    ///
    /// # Returns
    /// The edges indices iterator.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// The edges indices are:
    /// - Unique,
    /// - Sorted in ascending order,
    /// - The first vertex index is less or equal to the second vertex index.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 3), (1, 2), (2, 3)]);
    /// ```
    ///
    fn edges(&self) -> Self::EdgesIter<'_>;

    /// Check if the edge exists.
    ///
    /// # Description
    /// $(X, Y) \in \mathbf{E}$
    ///
    /// # Arguments
    /// * `x` - The first vertex index,
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the edge exists, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// If the graph is undirected, then the order of the vertices does not matter.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.has_edge(0, 1), true);
    /// assert_eq!(graph.has_edge(0, 2), false);
    /// assert_eq!(graph.has_edge(0, 3), true);
    /// assert_eq!(graph.has_edge(1, 2), true);
    /// assert_eq!(graph.has_edge(1, 3), false);
    /// assert_eq!(graph.has_edge(2, 3), true);
    /// ```
    ///
    fn has_edge(&self, x: usize, y: usize) -> bool;

    /// Add an edge.
    ///
    /// The edge is added only if the edge does not exist.
    ///
    /// # Description
    /// $\mathbf{E} \cup \lbrace (X, Y) \rbrace$
    ///
    /// # Arguments
    /// * `x` - The first vertex index,
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the edge was added, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// If the graph is undirected, then the order of the vertices does not matter.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let mut graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 3), (1, 2), (2, 3)]);
    ///
    /// // Add a new edge.
    /// let added = graph.add_edge(0, 2);
    ///
    /// // Assert the edge was added.
    /// assert_eq!(added, true);
    ///
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)]);
    /// ```
    ///
    fn add_edge(&mut self, x: usize, y: usize) -> bool;

    /// Delete an edge.
    ///
    /// The edge is deleted only if the edge exists.
    ///
    /// # Description
    /// $\mathbf{E} \setminus \lbrace (X, Y) \rbrace$
    ///
    /// # Arguments
    /// * `x` - The first vertex index,
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the edge was deleted, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// If the graph is undirected, then the order of the vertices does not matter.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let mut graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (0, 3), (1, 2), (2, 3)]);
    ///
    /// // Delete an edge.
    /// let deleted = graph.del_edge(0, 3);
    ///
    /// // Assert the edge was deleted.
    /// assert_eq!(deleted, true);
    ///
    /// // Assert the graph edges indices are correct.
    /// assert_eq!(graph.edges().collect_vec(), vec![(0, 1), (1, 2), (2, 3)]);
    /// ```
    ///
    fn del_edge(&mut self, x: usize, y: usize) -> bool;

    /// Get the vertex degree.
    ///
    /// The degree of a vertex is the number of edges incident to the vertex.
    ///
    /// # Description
    /// $|Adj(X)|$
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex degree.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices degrees are correct.
    /// assert_eq!(graph.degree(0), 2);
    /// assert_eq!(graph.degree(1), 2);
    /// assert_eq!(graph.degree(2), 2);
    /// assert_eq!(graph.degree(3), 2);
    /// ```
    ///
    fn degree(&self, x: usize) -> usize;

    /// Get the vertices degrees.
    ///
    /// Also called the degree vector of the graph.
    ///
    /// # Description
    /// $|Adj(X)| \quad \forall X \in \mathbf{V}$
    ///
    /// # Returns
    /// The vertices degrees.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices degrees are correct.
    /// assert_eq!(graph.degrees(), vec![2, 2, 2, 2]);
    /// ```
    ///
    fn degrees(&self) -> Vec<usize>;

    /// Get the vertex adjacents indices iterator.
    ///
    /// The vertex adjacents are the vertices with an edge incident to the vertex.
    ///
    /// # Description
    /// $Adj(X)$
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex adjacents indices iterator.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// The vertex adjacents indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices adjacents indices are correct.
    /// assert_eq!(graph.adjacents(0).collect_vec(), vec![1, 3]);
    /// assert_eq!(graph.adjacents(1).collect_vec(), vec![0, 2]);
    /// assert_eq!(graph.adjacents(2).collect_vec(), vec![1, 3]);
    /// assert_eq!(graph.adjacents(3).collect_vec(), vec![0, 2]);
    /// ```
    ///
    fn adjacents(&self, x: usize) -> Self::AdjacentsIter<'_>;

    /// Check if two vertices are adjacent.
    ///
    /// # Description
    /// $Y \in Adj(X)$
    ///
    /// # Arguments
    /// * `x` - The first vertex index,
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the vertices are adjacent, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// If the graph is undirected, then the order of the vertices does not matter.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Assert the graph vertices adjacents indices are correct.
    /// assert_eq!(graph.is_adjacent(0, 1), true);
    /// assert_eq!(graph.is_adjacent(0, 2), false);
    /// assert_eq!(graph.is_adjacent(0, 3), true);
    /// assert_eq!(graph.is_adjacent(1, 2), true);
    /// assert_eq!(graph.is_adjacent(1, 3), false);
    /// assert_eq!(graph.is_adjacent(2, 3), true);
    /// ```
    ///
    fn is_adjacent(&self, x: usize, y: usize) -> bool;

    /// Get the subgraph induced by the given vertices and edges.
    ///
    /// # Description
    /// $\mathcal{G'} = (\mathbf{V'}, \mathbf{E'}) \; \text{with} \; \mathbf{V'} \subseteq \mathbf{V} \wedge \mathbf{E'} \subseteq \mathbf{E}$
    ///
    /// # Arguments
    /// * `vertices` - The vertices indices,
    /// * `edges` - The edges indices.
    ///
    /// # Returns
    /// The subgraph induced by the given vertices and edges.
    ///
    /// # Panics
    /// If the vertex or edge indices are out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// The vertices indices will be reindexed in the `[0, |V'|)` range.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Get the subgraph induced by the given vertices and edges.
    /// let subgraph = graph.subgraph(vec![0, 1, 2], vec![(0, 1)]);
    ///
    /// // Assert the subgraph vertices labels are correct.
    /// assert_eq!(subgraph.labels().collect_vec(), vec!["A", "B", "C"]);
    /// // Assert the subgraph vertices indices are correct.
    /// assert_eq!(subgraph.vertices().collect_vec(), vec![0, 1, 2]);
    /// // Assert the subgraph edges indices are correct.
    /// assert_eq!(subgraph.edges().collect_vec(), vec![(0, 1)]);
    /// ```
    ///
    fn subgraph<I, J>(&self, vertices: I, edges: J) -> Self
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = (usize, usize)>;

    /// Get the subgraph induced by the given vertices.
    ///
    /// # Description
    /// $\mathcal{G'} = (\mathbf{V'}, \mathbf{E'}) \; \text{with} \; \mathbf{V'} \subseteq \mathbf{V} \wedge \mathbf{E'} \subseteq \mathbf{E}$
    ///
    /// # Arguments
    /// * `vertices` - The vertices indices.
    ///
    /// # Returns
    /// The subgraph induced by the given vertices.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// The vertices indices will be reindexed in the `[0, |V'|)` range.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///    // The vertices labels.
    ///   vec!["A", "B", "C", "D"],
    ///   // The edges labels.
    ///  vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Get the subgraph induced by the given vertices.
    /// let subgraph = graph.subgraph_by_vertices(vec![0, 1, 2]);
    ///
    /// // Assert the subgraph vertices labels are correct.
    /// assert_eq!(subgraph.labels().collect_vec(), vec!["A", "B", "C"]);
    /// // Assert the subgraph vertices indices are correct.
    /// assert_eq!(subgraph.vertices().collect_vec(), vec![0, 1, 2]);
    /// // Assert the subgraph edges indices are correct.
    /// assert_eq!(subgraph.edges().collect_vec(), vec![(0, 1), (1, 2)]);
    /// ```
    ///
    fn subgraph_by_vertices<I>(&self, vertices: I) -> Self
    where
        I: IntoIterator<Item = usize>;

    /// Get the subgraph induced by the given edges.
    ///
    /// # Description
    /// $\mathcal{G'} = (\mathbf{V'}, \mathbf{E'}) \; \text{with} \; \mathbf{V'} \subseteq \mathbf{V} \wedge \mathbf{E'} \subseteq \mathbf{E}$
    ///
    /// # Arguments
    /// * `edges` - The edges indices.
    ///
    /// # Returns
    /// The subgraph induced by the given edges.
    ///
    /// # Panics
    /// If the edge indices are out of bounds.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Notes
    /// The vertices indices will be reindexed in the `[0, |V'|)` range.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    /// use itertools::Itertools;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Get the subgraph induced by the given edges.
    /// let subgraph = graph.subgraph_by_edges(vec![(0, 1), (1, 2)]);
    ///
    /// // Assert the subgraph vertices labels are correct.
    /// assert_eq!(subgraph.labels().collect_vec(), vec!["A", "B", "C"]);
    /// // Assert the subgraph vertices indices are correct.
    /// assert_eq!(subgraph.vertices().collect_vec(), vec![0, 1, 2]);
    /// // Assert the subgraph edges indices are correct.
    /// assert_eq!(subgraph.edges().collect_vec(), vec![(0, 1), (1, 2)]);
    /// ```
    ///
    fn subgraph_by_edges<J>(&self, edges: J) -> Self
    where
        J: IntoIterator<Item = (usize, usize)>;

    /// Check if the graph is a subgraph of a given graph.
    ///
    /// # Description
    /// $\mathcal{G} \subseteq \mathcal{G'}$
    ///
    /// # Arguments
    /// * `other` - The other graph.
    ///
    /// # Returns
    /// `true` if the graph is a subgraph of the given graph, otherwise `false`.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let graph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Get the subgraph induced by the given vertices.
    /// let subgraph = graph.subgraph_by_vertices(vec![0, 1, 2]);
    ///
    /// // Assert the graph is a subgraph of the initial graph.
    /// assert_eq!(subgraph.is_subgraph(&graph), true);
    /// ```
    ///
    fn is_subgraph(&self, other: &Self) -> bool;

    /// Check if the graph is a supergraph of a given graph.
    ///
    /// # Description
    /// $\mathcal{G} \supseteq \mathcal{G'}$
    ///
    /// # Arguments
    /// * `other` - The other graph.
    ///
    /// # Returns
    /// `true` if the graph is a supergraph of the given graph, otherwise `false`.
    ///
    /// # Complexity
    /// Check the actual implementation.
    ///
    /// # Examples
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Create a new graph.
    /// let supergraph = UGraph::new(
    ///     // The vertices labels.
    ///     vec!["A", "B", "C", "D"],
    ///     // The edges labels.
    ///     vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")]
    /// );
    ///
    /// // Get the subgraph induced by the given vertices.
    /// let graph = supergraph.subgraph_by_vertices(vec![0, 1, 2]);
    ///
    /// // Assert the graph is a supergraph of the initial graph.
    /// assert_eq!(supergraph.is_supergraph(&graph), true);
    /// ```
    ///
    fn is_supergraph(&self, other: &Self) -> bool;
}
