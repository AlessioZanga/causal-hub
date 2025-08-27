use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::Graph;
use crate::{
    types::{Labels, Set},
    utils::collect_labels,
};

/// A struct representing an undirected graph using an adjacency matrix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnGraph {
    labels: Labels,
    adjacency_matrix: Array2<bool>,
}

impl UnGraph {
    /// Returns the neighbors of a vertex.
    ///
    /// # Arguments
    ///
    /// * `x` - The vertex for which to find the neighbors.
    ///
    /// # Panics
    ///
    /// * If the vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// A set of indices representing the neighbors of the vertex.
    ///
    pub fn neighbors(&self, x: usize) -> Set<usize> {
        // Check if the vertex is within bounds.
        assert!(x < self.labels.len(), "Vertex `{x}` is out of bounds");

        // Iterate over all vertices and filter the ones that are neighbors.
        self.adjacency_matrix
            .row(x)
            .into_iter()
            .enumerate()
            .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            .collect()
    }
}

impl Graph for UnGraph {
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
            .filter_map(|((x, y), &has_edge)| {
                // Since the graph is undirected, we only need to check one direction.
                if has_edge && x <= y {
                    Some((x, y))
                } else {
                    None
                }
            })
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
        self.adjacency_matrix[[y, x]] = true;

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
        self.adjacency_matrix[[y, x]] = false;

        true
    }

    fn from_adjacency_matrix<I, V>(labels: I, adjacency_matrix: Array2<bool>) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        // Collect the labels.
        let labels = collect_labels(labels);

        // Assert labels are sorted.
        // TODO: Refactor code and remove this assumption.
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
        // Assert the adjacency matrix is symmetric.
        assert_eq!(
            adjacency_matrix,
            adjacency_matrix.t(),
            "Adjacency matrix must be symmetric."
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
