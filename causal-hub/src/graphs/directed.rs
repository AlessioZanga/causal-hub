use std::collections::VecDeque;

use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::Graph;
use crate::{
    set,
    types::{Labels, Set},
    utils::collect_labels,
};

/// A struct representing a directed graph using an adjacency matrix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectedGraph {
    labels: Labels,
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
        // Assert the vertex is within bounds.
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");

        // Iterate over all vertices and filter the ones that are parents.
        self.adjacency_matrix
            .column(x)
            .indexed_iter()
            .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            .collect()
    }

    /// Returns the ancestors of a vertex.
    ///
    /// # Arguments
    ///
    /// * `x` - The vertex for which to find the ancestors.
    ///
    /// # Panics
    ///
    /// * If the vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// A set of indices representing the ancestors of the vertex.
    ///
    pub fn ancestors(&self, x: usize) -> Set<usize> {
        // Assert the vertex is within bounds.
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");

        // Initialize a stack and a visited set.
        let mut stack = VecDeque::new();
        let mut visited = set![];

        // Start with the given vertex.
        stack.push_back(x);

        // While there are vertices to visit ...
        while let Some(y) = stack.pop_back() {
            // For each incoming edge ...
            for z in self.parents(y) {
                // If there is an edge from z to y and z has not been visited ...
                if !visited.contains(&z) {
                    // Mark z as visited.
                    visited.insert(z);
                    // Add z to the stack to visit its ancestors.
                    stack.push_back(z);
                }
            }
        }

        // Return the visited set.
        visited
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
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");

        // Iterate over all vertices and filter the ones that are children.
        self.adjacency_matrix
            .row(x)
            .indexed_iter()
            .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            .collect()
    }

    /// Returns the descendants of a vertex.
    ///
    /// # Arguments
    ///
    /// * `x` - The vertex for which to find the descendants.
    ///
    /// # Panics
    ///
    /// * If the vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// A set of indices representing the descendants of the vertex.
    ///
    pub fn descendants(&self, x: usize) -> Set<usize> {
        // Assert the vertex is within bounds.
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");

        // Initialize a stack and a visited set.
        let mut stack = VecDeque::new();
        let mut visited = set![];

        // Start with the given vertex.
        stack.push_back(x);

        // While there are vertices to visit ...
        while let Some(y) = stack.pop_back() {
            // For each outgoing edge ...
            for z in self.children(y) {
                // If z has not been visited ...
                if !visited.contains(&z) {
                    // Mark z as visited.
                    visited.insert(z);
                    // Add z to the stack to visit its descendants.
                    stack.push_back(z);
                }
            }
        }

        // Return the visited set.
        visited
    }
}

impl Graph for DiGraph {
    fn empty<I, V>(labels: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        // Initialize labels counter.
        let mut n = 0;
        // Collect the labels.
        let mut labels: Labels = labels
            .into_iter()
            .inspect(|_| n += 1)
            .map(|x| x.as_ref().to_owned())
            .collect();

        // Assert no duplicate labels.
        assert_eq!(labels.len(), n, "Labels must be unique.");

        // Sort the labels.
        labels.sort();

        // Initialize the adjacency matrix with `false` values.
        let adjacency_matrix: Array2<_> = Array::from_elem((n, n), false);

        // Debug assert to check the sorting of the labels.
        debug_assert!(labels.iter().is_sorted(), "Vertices labels must be sorted.");

        Self {
            labels,
            adjacency_matrix,
        }
    }

    fn complete<I, V>(labels: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        // Initialize labels counter.
        let mut n = 0;
        // Collect the labels.
        let mut labels: Labels = labels
            .into_iter()
            .inspect(|_| n += 1)
            .map(|x| x.as_ref().to_owned())
            .collect();

        // Assert no duplicate labels.
        assert_eq!(labels.len(), n, "Labels must be unique.");

        // Sort the labels.
        labels.sort();

        // Initialize the adjacency matrix with `true` values.
        let mut adjacency_matrix: Array2<_> = Array::from_elem((n, n), true);
        // Set the diagonal to `false` to avoid self-loops.
        adjacency_matrix.diag_mut().fill(false);

        // Debug assert to check the sorting of the labels.
        debug_assert!(labels.iter().is_sorted(), "Vertices labels must be sorted.");

        Self {
            labels,
            adjacency_matrix,
        }
    }

    fn labels(&self) -> &Labels {
        &self.labels
    }

    fn label_to_index<V>(&self, x: &V) -> usize
    where
        V: AsRef<str>,
    {
        // Get the label as a string reference.
        let x = x.as_ref();
        // Get the index of the label, if it exists.
        self.labels
            .get_index_of(x)
            .unwrap_or_else(|| panic!("Vertex `{x}` label does not exist"))
    }

    fn index_to_label(&self, x: usize) -> &str {
        // Get the label at the index, if it exists.
        self.labels
            .get_index(x)
            .unwrap_or_else(|| panic!("Vertex `{x}` is out of bounds"))
    }

    fn vertices(&self) -> Set<usize> {
        (0..self.labels.len()).collect()
    }

    fn has_vertex(&self, x: usize) -> bool {
        // Check if the vertex is within bounds.
        x < self.labels.len()
    }

    fn edges(&self) -> Set<(usize, usize)> {
        // Iterate over the adjacency matrix and collect the edges.
        self.adjacency_matrix
            .indexed_iter()
            .filter_map(|((x, y), &has_edge)| if has_edge { Some((x, y)) } else { None })
            .collect()
    }

    fn has_edge(&self, x: usize, y: usize) -> bool {
        // Check if the vertices are within bounds.
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");
        assert!(y < self.labels.len(), "Vertex `{y}` is out of bounds");

        self.adjacency_matrix[[x, y]]
    }

    fn add_edge(&mut self, x: usize, y: usize) -> bool {
        // Check if the vertices are within bounds.
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");
        assert!(y < self.labels.len(), "Vertex `{y}` is out of bounds");

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
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");
        assert!(y < self.labels.len(), "Vertex `{y}` is out of bounds");

        // Check if the edge exists.
        if !self.adjacency_matrix[[x, y]] {
            return false;
        }

        // Delete the edge.
        self.adjacency_matrix[[x, y]] = false;

        true
    }

    fn from_adjacency_matrix<I, V>(labels: I, adjacency_matrix: Array2<bool>) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        // Collect the labels.
        let labels = collect_labels(labels);

        // FIXME: Workaround: assert labels are sorted.
        assert!(labels.iter().is_sorted(), "Labels must be sorted.");

        // Assert labels and adjacency matrix dimensions match.
        assert_eq!(
            labels.len(),
            adjacency_matrix.nrows(),
            "Number of labels must match the number of rows in the adjacency matrix."
        );
        // Assert adjacency matrix must be square.
        assert_eq!(
            adjacency_matrix.nrows(),
            adjacency_matrix.ncols(),
            "Adjacency matrix must be square."
        );

        // Create a new graph instance.
        Self {
            labels,
            adjacency_matrix,
        }
    }

    fn to_adjacency_matrix(&self) -> Array2<bool> {
        // Return the adjacency matrix.
        self.adjacency_matrix.clone()
    }
}
