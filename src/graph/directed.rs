use std::ops::Range;

use ndarray::prelude::*;

use super::Graph;
use crate::types::FxIndexSet;

/// A struct representing a directed graph using an adjacency matrix.
///
#[derive(Clone, Debug)]
pub struct DirectedGraph {
    labels: FxIndexSet<String>,
    adjacency_matrix: Array2<bool>,
}

/// A type alias for a directed graph.
pub type DiGraph = DirectedGraph;

impl DiGraph {
    /// Returns the parents of a vertex.
    ///
    /// # Arguments
    ///
    /// * `x` - The vertex for which to find the parents.
    ///
    /// # Panics
    ///
    /// * If the vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// A vector of indices representing the parents of the vertex.
    ///
    pub fn parents(&self, x: usize) -> Vec<usize> {
        // Check if the vertex is within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);

        // Iterate over all vertices and filter the ones that are parents.
        self.adjacency_matrix
            .column(x)
            .indexed_iter()
            .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            .collect()
    }

    /// Returns the children of a vertex.
    ///
    /// # Arguments
    ///
    /// * `x` - The vertex for which to find the children.
    ///
    /// # Panics
    ///
    /// * If the vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// A vector of indices representing the children of the vertex.
    ///
    pub fn children(&self, x: usize) -> Vec<usize> {
        // Check if the vertex is within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);

        // Iterate over all vertices and filter the ones that are children.
        self.adjacency_matrix
            .row(x)
            .indexed_iter()
            .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            .collect()
    }
}

impl Graph for DiGraph {
    type Labels = FxIndexSet<String>;
    type Vertices = Range<usize>;

    fn empty(labels: Vec<&str>) -> Self {
        // Get the size of the graph from the number of labels.
        let size = labels.len();
        // Convert the array of string slices to a vector of strings.
        let labels: FxIndexSet<_> = labels.iter().map(|s| s.to_string()).collect();
        // Assert no duplicate labels.
        assert_eq!(size, labels.len(), "Labels must be unique.");

        // Initialize the adjacency matrix with `false` values.
        let adjacency_matrix = Array2::from_elem((size, size), false);

        Self {
            labels,
            adjacency_matrix,
        }
    }

    fn labels(&self) -> &Self::Labels {
        &self.labels
    }

    fn vertices(&self) -> Self::Vertices {
        0..self.labels.len()
    }

    fn has_edge(&self, x: usize, y: usize) -> bool {
        // Check if the vertices are within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        self.adjacency_matrix[[x, y]]
    }

    fn add_edge(&mut self, x: usize, y: usize) -> bool {
        // Check if the vertices are within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        // Check if the edge already exists.
        if self.adjacency_matrix[[x, y]] {
            return false;
        }

        // Add the edge.
        self.adjacency_matrix[[x, y]] = true;

        true
    }

    fn del_edge(&mut self, x: usize, y: usize) -> bool {
        // Check if the vertices are within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        // Check if the edge exists.
        if !self.adjacency_matrix[[x, y]] {
            return false;
        }

        // Delete the edge.
        self.adjacency_matrix[[x, y]] = false;

        true
    }
}
