use std::ops::Range;

use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::Graph;
use crate::types::FxIndexSet;

/// A struct representing an undirected graph using an adjacency matrix.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndirectedGraph {
    labels: FxIndexSet<String>,
    adjacency_matrix: Array2<bool>,
}

/// A type alias for an undirected graph.
pub type UnGraph = UndirectedGraph;

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
    /// A vector of indices representing the neighbors of the vertex.
    ///
    pub fn neighbors(&self, x: usize) -> Vec<usize> {
        // Check if the vertex is within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);

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
    type Labels = FxIndexSet<String>;
    type Vertices = Range<usize>;

    fn empty<I, V>(labels: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        // Initialize labels counter.
        let mut n = 0;
        // Collect the labels.
        let mut labels: FxIndexSet<_> = labels
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
        self.adjacency_matrix[[y, x]] = true;

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
        self.adjacency_matrix[[y, x]] = false;

        true
    }
}
