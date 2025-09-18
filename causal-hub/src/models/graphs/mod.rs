mod directed;
pub use directed::*;

mod undirected;
use ndarray::prelude::*;
pub use undirected::*;

use crate::types::{Labels, Set};

/// A trait for graphs.
pub trait Graph {
    /// Creates an empty graph with the given labels.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the vertices in the graph.
    ///
    /// # Notes
    ///
    /// * Labels will be sorted in alphabetical order.
    ///
    /// # Panics
    ///
    /// * If the labels are not unique.
    ///
    /// # Returns
    ///
    /// A new graph instance.
    ///
    fn empty<I, V>(labels: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>;

    /// Creates a complete graph with the given labels.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the vertices in the graph.
    ///
    /// # Notes
    ///
    /// * Labels will be sorted in alphabetical order.
    /// * No self-loops are created.
    ///
    /// # Panics
    ///
    /// * If the labels are not unique.
    ///
    /// # Returns
    ///
    /// A new graph instance.
    ///
    fn complete<I, V>(labels: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>;

    /// Returns the iterator of vertices in the graph.
    ///
    /// # Returns
    ///
    /// A set representing the vertices in the graph.
    ///
    fn vertices(&self) -> Set<usize>;

    /// Checks if a vertex exists in the graph.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the vertex.
    ///
    /// # Returns
    ///
    /// `true` if the vertex exists, `false` otherwise.
    ///
    fn has_vertex(&self, x: usize) -> bool;

    /// Returns the iterator of edges in the graph.
    ///
    /// # Returns
    ///
    /// A set of tuples representing the edges in the graph.
    ///
    fn edges(&self) -> Set<(usize, usize)>;

    /// Checks if there is an edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Panics
    ///
    /// * If any of the vertices are out of bounds.
    ///
    /// # Returns
    ///
    /// `true` if there is an edge between `x` and `y`, `false` otherwise.
    ///
    fn has_edge(&self, x: usize, y: usize) -> bool;

    /// Adds an edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Panics
    ///
    /// * If any of the vertices are out of bounds.
    ///
    /// # Returns
    ///
    /// `true` if the edge was added, `false` if it already existed.
    ///
    fn add_edge(&mut self, x: usize, y: usize) -> bool;

    /// Deletes the edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Panics
    ///
    /// * If any of the vertices are out of bounds.
    ///
    /// # Returns
    ///
    /// `true` if the edge was deleted, `false` if it did not exist.
    ///
    fn del_edge(&mut self, x: usize, y: usize) -> bool;

    /// Creates a graph from an adjacency matrix and labels.
    ///
    /// # Arguments
    ///
    /// * `labels` - An iterator over the labels of the vertices.
    /// * `adjacency_matrix` - A reference to a 2D array representing the adjacency matrix.
    ///
    /// # Returns
    ///
    /// A new graph instance.
    ///
    fn from_adjacency_matrix(labels: Labels, adjacency_matrix: Array2<bool>) -> Self;

    /// Converts the graph to an adjacency matrix.
    ///
    /// # Returns
    ///
    /// A 2D array representing the adjacency matrix of the graph.
    ///
    fn to_adjacency_matrix(&self) -> Array2<bool>;
}
