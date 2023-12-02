use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    iter::FusedIterator,
    ops::Index,
};

use indexmap::IndexSet;
use itertools::Itertools;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    graphs::{Graph, Undirected},
    prelude::FxIndexSet,
    E, L, V,
};

/// Define the `UndirectedDenseAdjacencyMatrix` struct using a dense adjacency matrix from the `ndarray` crate.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UndirectedDenseAdjacencyMatrix {
    /// The adjacency matrix.
    adjacency_matrix: Array2<bool>,
    /// The vertices labels.
    labels: IndexSet<String>,
    /// The graph size.
    size: usize,
}

/// Alias for the `UndirectedDenseAdjacencyMatrix` struct.
pub type UGraph = UndirectedDenseAdjacencyMatrix;

// Implement the `Display` trait for the `UGraph` struct.
impl Display for UGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "UndirectedGraph {{ ")?;

        // Write vertex set.
        write!(
            f,
            "V = {{{}}}, ",
            V!(self).map(|x| format!("\"{}\"", &self[x])).join(", ")
        )?;

        // Write edge set.
        write!(
            f,
            "E = {{{}}}",
            E!(self)
                .map(|(x, y)| format!("(\"{}\", \"{}\")", &self[x], &self[y]))
                .join(", ")
        )?;

        // Write ending character.
        write!(f, " }}")
    }
}

// Implement the `PartialOrd` trait for the `UGraph` struct.
impl PartialOrd for UGraph {
    /// Compare two graphs.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// Return `None` if the graphs are not comparable.
    ///
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // If the vertices are not comparable, return `None`.
        let vertices = iter_set::cmp(L!(self), L!(other))?;
        // If the edges are not comparable, return `None`.
        let edges = iter_set::cmp(E!(self), E!(other))?;

        // If the vertices are equal, return the edges.
        if vertices.is_eq() {
            return Some(edges);
        }
        // If the edges are equal, return the vertices.
        if edges.is_eq() {
            return Some(vertices);
        }
        // If the vertices and the edges are the same, return arbitrarily.
        if vertices.eq(&edges) {
            return Some(vertices);
        }

        // Otherwise, return `None`.
        None
    }
}

// Implement the `Index` trait for the `UGraph` struct.
impl Index<usize> for UGraph {
    type Output = str;

    #[inline]
    fn index(&self, x: usize) -> &Self::Output {
        // Get the vertex label.
        self.vertex_to_label(x)
    }
}

// Implement the `Hash` trait for the `UGraph` struct.
impl Hash for UGraph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Compute the hash of the adjacency matrix.
        self.adjacency_matrix.hash(state);
        // Compute the hash of the vertices labels.
        self.labels.iter().for_each(|x| x.hash(state));
    }
}

/// Define the `EdgesIterator` iterator for the `UGraph` struct.
#[allow(clippy::type_complexity)]
pub struct EdgesIterator<'a> {
    // The edges indices iterator.
    iter: std::iter::FilterMap<
        ndarray::iter::IndexedIter<'a, bool, ndarray::Dim<[usize; 2]>>,
        fn(((usize, usize), &'a bool)) -> Option<(usize, usize)>,
    >,
    // The size of the iterator.
    size: usize,
}

// Implement the `EdgesIterator` iterator for the `UGraph` struct.
impl<'a> EdgesIterator<'a> {
    /// Create a new `EdgesIterator` iterator.
    fn new(graph: &'a UGraph) -> Self {
        // Create the new `EdgesIterator` iterator.
        Self {
            iter: graph
                .adjacency_matrix
                .indexed_iter()
                .filter_map(|((x, y), &flag)|
                    // Check if the edge exists.
                    if flag && x <= y {
                        // Return the edge indices.
                        Some((x, y))
                    } else {
                        // Return `None`.
                        None
                    }
                ),
            size: graph.size,
        }
    }
}

// Implement the `Iterator` trait for the `EdgesIterator` iterator.
impl<'a> Iterator for EdgesIterator<'a> {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next edge indices.
        let next = self.iter.next();

        // Debug assert the iterator size is zero if and only if the next edge indices is `None`.
        debug_assert_eq!(
            self.size == 0,
            next.is_none(),
            "The iterator size is not zero."
        );
        // Debug assert the iterator size is non zero if and only if the next edge indices is `Some(_)`.
        debug_assert_eq!(self.size != 0, next.is_some(), "The iterator size is zero.");

        // Decrement the iterator size.
        self.size = self.size.saturating_sub(1);

        next
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Get the iterator size hint.
        (self.size, Some(self.size))
    }

    #[inline]
    fn count(self) -> usize {
        // Get the iterator count.
        self.size
    }
}

// Implement the `ExactSizeIterator` trait for the `EdgesIterator` iterator.
impl<'a> ExactSizeIterator for EdgesIterator<'a> {}

// Implement the `FusedIterator` trait for the `EdgesIterator` iterator.
impl<'a> FusedIterator for EdgesIterator<'a> {}

/// Define the `AdjacentsIterator` iterator for the `UGraph` struct.
#[allow(clippy::type_complexity)]
pub struct AdjacentsIterator<'a> {
    // The adjacents indices iterator.
    iter: std::iter::FilterMap<
        std::iter::Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &'a bool)) -> Option<usize>,
    >,
}

// Implement the `AdjacentsIterator` iterator for the `UGraph` struct.
impl<'a> AdjacentsIterator<'a> {
    /// Create a new `AdjacentsIterator` iterator.
    fn new(graph: &'a UGraph, x: usize) -> Self {
        // Assert the vertex is in bounds.
        assert!(
            graph.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Create the new `AdjacentsIterator` iterator.
        Self {
            iter: graph
                .adjacency_matrix
                .row(x)
                .into_iter()
                .enumerate()
                .filter_map(|(y, &flag)|
                    // Check if the vertex is adjacent.
                    if flag {
                        // Return the vertex index.
                        Some(y)
                    } else {
                        // Return `None`.
                        None
                    }
                ),
        }
    }
}

// Implement the `Iterator` trait for the `AdjacentsIterator` iterator.
impl<'a> Iterator for AdjacentsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next vertex index.
        self.iter.next()
    }
}

// Implement the `FusedIterator` trait for the `AdjacentsIterator` iterator.
impl<'a> FusedIterator for AdjacentsIterator<'a> {}

// Implement the `Graph` trait for the `UGraph` struct.
impl Graph for UGraph {
    // Direction associated type.
    type Direction = Undirected;
    // Vertex labels iterator associated type.
    type LabelsIter<'a> =
        std::iter::Map<indexmap::set::Iter<'a, String>, fn(&'a String) -> &'a str>;
    // Vertex indices iterator associated type.
    type VerticesIter<'a> = std::ops::Range<usize>;
    // Edge indices iterator associated type.
    type EdgesIter<'a> = EdgesIterator<'a>;
    // Adjacents indices iterator associated type.
    type AdjacentsIter<'a> = AdjacentsIterator<'a>;

    /// Create a new graph.
    ///
    /// # Complexity
    /// - Time: `O(|V| + |E|)`,
    /// - Space: `O(|V| + |E|)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    fn new<V, I, J>(vertices: I, edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
    {
        // Collect and deduplicate the vertices labels.
        let mut labels: IndexSet<_> = vertices
            .into_iter()
            // Convert the vertices labels to strings.
            .map(|x| x.into())
            // Collect the vertices labels.
            .collect();
        // Collect the edges labels.
        let edges: IndexSet<_> = edges
            .into_iter()
            // Convert the edges labels to strings.
            .map(|(x, y)| (x.into(), y.into()))
            // Collect the edges labels.
            .collect();
        // Add the edges labels to the vertices labels.
        edges.iter().for_each(|(x, y)| {
            labels.insert(x.clone());
            labels.insert(y.clone());
        });
        // Sort the vertices labels.
        labels.sort();

        // Get the new graph order.
        let order = labels.len();
        // Initialize the adjacency matrix.
        let mut adjacency_matrix = Array2::from_elem((order, order), false);
        // Add the edges to the adjacency matrix.
        edges
            .into_iter()
            // Map the edges labels to the edges indices.
            .map(|(x, y)| {
                (
                    labels.get_index_of(&x).unwrap(),
                    labels.get_index_of(&y).unwrap(),
                )
            })
            // Add the edges to the adjacency matrix.
            .for_each(|(x, y)| {
                adjacency_matrix[[x, y]] = true;
                adjacency_matrix[[y, x]] = true;
            });

        // Compute the graph size given the adjacency matrix.
        let size = {
            // Sum the adjacency matrix elements.
            let total_sum = adjacency_matrix.mapv(|x| x as usize).sum();
            // Sum the adjacency matrix diagonal elements.
            let diagonal_sum = adjacency_matrix.mapv(|x| x as usize).diag().sum();
            // Compute the graph size.
            (total_sum - diagonal_sum) / 2 + diagonal_sum
        };

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            adjacency_matrix,
            adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            adjacency_matrix.nrows(),
            labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(
            size,
            {
                // Sum the adjacency matrix elements.
                let total_sum = adjacency_matrix.mapv(|x| x as usize).sum();
                // Sum the adjacency matrix diagonal elements.
                let diagonal_sum = adjacency_matrix.mapv(|x| x as usize).diag().sum();
                // Compute the graph size.
                (total_sum - diagonal_sum) / 2 + diagonal_sum
            },
            "The graph size is not correct."
        );

        // Create the new graph.
        Self {
            adjacency_matrix,
            labels,
            size,
        }
    }

    // Create a new null graph.
    fn null() -> Self {
        // Initialize the vertices labels.
        let labels = IndexSet::new();
        // Initialize the adjacency matrix.
        let adjacency_matrix = Array2::from_elem((0, 0), false);
        // Initialize the graph size.
        let size = 0;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            adjacency_matrix,
            adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            adjacency_matrix.nrows(),
            labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(size, 0, "The graph size is not correct.");

        // Create the new null graph.
        Self {
            adjacency_matrix,
            labels,
            size,
        }
    }

    /// Create a new empty graph.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    fn empty<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Collect and deduplicate the vertices labels.
        let mut labels: IndexSet<_> = vertices
            .into_iter()
            // Convert the vertices labels to strings.
            .map(|x| x.into())
            // Collect the vertices labels.
            .collect();
        // Sort the vertices labels.
        labels.sort();

        // Get the new graph order.
        let order = labels.len();
        // Initialize the adjacency matrix.
        let adjacency_matrix = Array2::from_elem((order, order), false);
        // Initialize the graph size.
        let size = 0;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            adjacency_matrix,
            adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            adjacency_matrix.nrows(),
            labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(size, 0, "The graph size is not correct.");

        // Create the new empty graph.
        Self {
            adjacency_matrix,
            labels,
            size,
        }
    }

    // Create a new complete graph.
    fn complete<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Collect and deduplicate the vertices labels.
        let mut labels: IndexSet<_> = vertices
            .into_iter()
            // Convert the vertices labels to strings.
            .map(|x| x.into())
            // Collect the vertices labels.
            .collect();
        // Sort the vertices labels.
        labels.sort();

        // Get the new graph order.
        let order = labels.len();
        // Initialize the adjacency matrix.
        let mut adjacency_matrix = Array2::from_elem((order, order), true);
        // Set the adjacency matrix diagonal to false.
        adjacency_matrix.diag_mut().fill(false);
        // Initialize the graph size.
        let size = order * (order - 1) / 2;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            adjacency_matrix,
            adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the adjacency matrix diagonal is false.
        debug_assert!(
            adjacency_matrix.diag().iter().all(|x| !x),
            "The adjacency matrix diagonal is not false."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            adjacency_matrix.nrows(),
            labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(
            size,
            order * (order - 1) / 2,
            "The graph size is not correct."
        );

        // Create the new complete graph.
        Self {
            adjacency_matrix,
            labels,
            size,
        }
    }

    // Get the vertices labels iterator.
    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        // Get the vertices labels iterator.
        self.labels.iter().map(|x| x.as_str())
    }

    // Get the vertex label.
    #[inline]
    fn vertex_to_label(&self, x: usize) -> &str {
        // Get the vertex label.
        self.labels
            .get_index(x)
            .unwrap_or_else(|| panic!("The vertex index `{}` is out of bounds.", x))
    }

    // Get the vertex index.
    #[inline]
    fn label_to_vertex(&self, x: &str) -> usize {
        // Get the vertex index.
        self.labels
            .get_index_of(x)
            .unwrap_or_else(|| panic!("The vertex label `{}` does not exist.", x))
    }

    // Get the graph order.
    #[inline]
    fn order(&self) -> usize {
        // Get the graph order.
        self.labels.len()
    }

    // Get the vertices indices iterator.
    #[inline]
    fn vertices_iter(&self) -> Self::VerticesIter<'_> {
        // Get the vertices indices iterator.
        0..self.order()
    }

    // Check if the vertex exists.
    #[inline]
    fn has_vertex(&self, x: usize) -> bool {
        // Check if the vertex exists.
        x < self.order()
    }

    /// Add a vertex.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    fn add_vertex<V>(&mut self, x: V) -> (usize, bool)
    where
        V: Into<String>,
    {
        // Get the vertex label.
        let x = x.into();
        // Check if the vertex label already exists.
        if let Some(x) = self.labels.get_index_of(&x) {
            // Return the vertex index and `false`.
            return (x, false);
        }

        // Insert the vertex label.
        self.labels.insert(x.clone());
        // Sort the vertices labels.
        self.labels.sort();
        // Get the vertex index.
        let x = self.labels.get_index_of(&x).unwrap();

        // Get the new graph order.
        let order = self.labels.len();
        // Initialize the new adjacency matrix.
        let mut adjacency_matrix = Array2::from_elem((order, order), false);
        // Add the old adjacency matrix to the new adjacency matrix.
        adjacency_matrix
            .slice_mut(s![..x, ..x])
            .assign(&self.adjacency_matrix.slice(s![..x, ..x]));
        adjacency_matrix
            .slice_mut(s![..x, x + 1..])
            .assign(&self.adjacency_matrix.slice(s![..x, x..]));
        adjacency_matrix
            .slice_mut(s![x + 1.., ..x])
            .assign(&self.adjacency_matrix.slice(s![x.., ..x]));
        adjacency_matrix
            .slice_mut(s![x + 1.., x + 1..])
            .assign(&self.adjacency_matrix.slice(s![x.., x..]));
        // Update the adjacency matrix.
        self.adjacency_matrix = adjacency_matrix;

        // Compute the new graph size.
        let size = {
            // Sum the adjacency matrix elements.
            let total_sum = self.adjacency_matrix.mapv(|x| x as usize).sum();
            // Sum the adjacency matrix diagonal elements.
            let diagonal_sum = self.adjacency_matrix.mapv(|x| x as usize).diag().sum();
            // Compute the graph size.
            (total_sum - diagonal_sum) / 2 + diagonal_sum
        };
        // Update the graph size.
        self.size = size;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            self.adjacency_matrix,
            self.adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            self.labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            self.adjacency_matrix.nrows(),
            self.labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(
            self.size,
            {
                // Sum the adjacency matrix elements.
                let total_sum = self.adjacency_matrix.mapv(|x| x as usize).sum();
                // Sum the adjacency matrix diagonal elements.
                let diagonal_sum = self.adjacency_matrix.mapv(|x| x as usize).diag().sum();
                // Compute the graph size.
                (total_sum - diagonal_sum) / 2 + diagonal_sum
            },
            "The graph size is not correct."
        );

        // Return the vertex index and `true`.
        (x, true)
    }

    /// Delete a vertex.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    fn del_vertex(&mut self, x: usize) -> bool {
        // Check if the vertex exists.
        if !self.has_vertex(x) {
            // Return `false`.
            return false;
        }

        // Delete the vertex label.
        self.labels.shift_remove_index(x);
        // Get the new graph order.
        let order = self.labels.len();
        // Initialize the new adjacency matrix.
        let mut adjacency_matrix = Array2::from_elem((order, order), false);
        // Add the old adjacency matrix to the new adjacency matrix.
        adjacency_matrix
            .slice_mut(s![..x, ..x])
            .assign(&self.adjacency_matrix.slice(s![..x, ..x]));
        adjacency_matrix
            .slice_mut(s![..x, x..])
            .assign(&self.adjacency_matrix.slice(s![..x, x + 1..]));
        adjacency_matrix
            .slice_mut(s![x.., ..x])
            .assign(&self.adjacency_matrix.slice(s![x + 1.., ..x]));
        adjacency_matrix
            .slice_mut(s![x.., x..])
            .assign(&self.adjacency_matrix.slice(s![x + 1.., x + 1..]));
        // Update the adjacency matrix.
        self.adjacency_matrix = adjacency_matrix;

        // Compute the new graph size.
        let size = {
            // Sum the adjacency matrix elements.
            let total_sum = self.adjacency_matrix.mapv(|x| x as usize).sum();
            // Sum the adjacency matrix diagonal elements.
            let diagonal_sum = self.adjacency_matrix.mapv(|x| x as usize).diag().sum();
            // Compute the graph size.
            (total_sum - diagonal_sum) / 2 + diagonal_sum
        };
        // Update the graph size.
        self.size = size;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            self.adjacency_matrix,
            self.adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            self.labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            self.adjacency_matrix.nrows(),
            self.labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(
            self.size,
            {
                // Sum the adjacency matrix elements.
                let total_sum = self.adjacency_matrix.mapv(|x| x as usize).sum();
                // Sum the adjacency matrix diagonal elements.
                let diagonal_sum = self.adjacency_matrix.mapv(|x| x as usize).diag().sum();
                // Compute the graph size.
                (total_sum - diagonal_sum) / 2 + diagonal_sum
            },
            "The graph size is not correct."
        );

        // Return `true`.
        true
    }

    // Get the graph size.
    #[inline]
    fn size(&self) -> usize {
        // Get the graph size.
        self.size
    }

    /// Get the edges indices iterator.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn edges_iter(&self) -> Self::EdgesIter<'_> {
        // Get the edges indices iterator.
        Self::EdgesIter::new(self)
    }

    /// Check if the edge exists.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn has_edge(&self, x: usize, y: usize) -> bool {
        // Assert the vertex indices are in bounds.
        assert!(
            self.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );
        assert!(
            self.has_vertex(y),
            "The vertex index `{}` is out of bounds.",
            y
        );

        // Check if the edge exists.
        self.adjacency_matrix[[x, y]]
    }

    /// Add an edge.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn add_edge(&mut self, x: usize, y: usize) -> bool {
        // Assert the vertex indices are in bounds.
        assert!(
            self.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );
        assert!(
            self.has_vertex(y),
            "The vertex index `{}` is out of bounds.",
            y
        );

        // Check if the edge already exists.
        if self.has_edge(x, y) {
            // Return `false`.
            return false;
        }

        // Update the adjacency matrix.
        self.adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[y, x]] = true;
        // Update the graph size.
        self.size += 1;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            self.adjacency_matrix,
            self.adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            self.labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            self.adjacency_matrix.nrows(),
            self.labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(
            self.size,
            {
                // Sum the adjacency matrix elements.
                let total_sum = self.adjacency_matrix.mapv(|x| x as usize).sum();
                // Sum the adjacency matrix diagonal elements.
                let diagonal_sum = self.adjacency_matrix.mapv(|x| x as usize).diag().sum();
                // Compute the graph size.
                (total_sum - diagonal_sum) / 2 + diagonal_sum
            },
            "The graph size is not correct."
        );

        // Return `true`.
        true
    }

    /// Delete an edge.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn del_edge(&mut self, x: usize, y: usize) -> bool {
        // Assert the vertex indices are in bounds.
        assert!(
            self.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );
        assert!(
            self.has_vertex(y),
            "The vertex index `{}` is out of bounds.",
            y
        );

        // Check if the edge does not exist.
        if !self.has_edge(x, y) {
            // Return `false`.
            return false;
        }

        // Update the adjacency matrix.
        self.adjacency_matrix[[x, y]] = false;
        self.adjacency_matrix[[y, x]] = false;
        // Update the graph size.
        self.size -= 1;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
        );
        // Debug assert the adjacency matrix is symmetric.
        debug_assert_eq!(
            self.adjacency_matrix,
            self.adjacency_matrix.t(),
            "The adjacency matrix is not symmetric."
        );
        // Debug assert the graph labels are unique and lexically sorted.
        debug_assert!(
            self.labels.iter().tuple_windows().all(|(x, y)| x < y),
            "The graph labels are not sorted."
        );
        // Debug assert the graph order is correct.
        debug_assert_eq!(
            self.adjacency_matrix.nrows(),
            self.labels.len(),
            "The graph order is not correct."
        );
        // Debug assert the graph size is correct.
        debug_assert_eq!(
            self.size,
            {
                // Sum the adjacency matrix elements.
                let total_sum = self.adjacency_matrix.mapv(|x| x as usize).sum();
                // Sum the adjacency matrix diagonal elements.
                let diagonal_sum = self.adjacency_matrix.mapv(|x| x as usize).diag().sum();
                // Compute the graph size.
                (total_sum - diagonal_sum) / 2 + diagonal_sum
            },
            "The graph size is not correct."
        );

        // Return `true`.
        true
    }

    /// Get the vertex degree.
    ///
    /// # Complexity
    /// - Time: `O(|V|)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn degree(&self, x: usize) -> usize {
        // Assert the vertex index is in bounds.
        assert!(
            self.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Get the vertex degree.
        self.adjacency_matrix.row(x).mapv(|x| x as usize).sum()
    }

    /// Get the vertices degrees.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn degrees(&self) -> Vec<usize> {
        // Get the vertices degrees.
        self.adjacency_matrix
            .mapv(|x| x as usize)
            .sum_axis(Axis(1))
            .to_vec()
    }

    /// Get the vertex adjacents indices iterator.
    ///
    /// # Complexity
    /// - Time: `O(|V|)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn adjacents_iter(&self, x: usize) -> Self::AdjacentsIter<'_> {
        // Assert the vertex index is in bounds.
        assert!(
            self.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Get the vertex adjacents indices iterator.
        Self::AdjacentsIter::new(self, x)
    }

    /// Check if two vertices are adjacent.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        // Assert the vertex indices are in bounds.
        assert!(
            self.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );
        assert!(
            self.has_vertex(y),
            "The vertex index `{}` is out of bounds.",
            y
        );

        // Check if the vertices are adjacent.
        self.adjacency_matrix[[x, y]]
    }

    // Get the subgraph induced by the vertices indices iterator.
    fn subgraph<I, J>(&self, vertices: I, edges: J) -> Self
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = (usize, usize)>,
    {
        // Get the vertices labels.
        let vertices = vertices.into_iter().map(|x| &self[x]);
        // Ge the edges labels.
        let edges = edges
            .into_iter()
            .inspect(|&(x, y)| {
                assert!(
                    self.has_edge(x, y),
                    "The edge ({x}, {y}) does not exist in the supergraph."
                )
            })
            .map(|(x, y)| (&self[x], &self[y]));

        // Create the new graph.
        Self::new(vertices, edges)
    }

    // Get the subgraph induced by the vertices indices iterator.
    fn subgraph_by_vertices<I>(&self, vertices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Collect the vertices.
        let vertices: FxIndexSet<_> = vertices.into_iter().collect();
        // Get the edges labels.
        let edges = E!(self)
            .filter(|(x, y)| vertices.contains(x) && vertices.contains(y))
            .map(|(x, y)| (&self[x], &self[y]))
            .collect_vec();
        // Get the vertices labels.
        let vertices = vertices.into_iter().map(|x| &self[x]);

        // Create the new graph.
        Self::new(vertices, edges)
    }

    // Get the subgraph induced by the edges indices iterator.
    fn subgraph_by_edges<J>(&self, edges: J) -> Self
    where
        J: IntoIterator<Item = (usize, usize)>,
    {
        // Get the edges labels.
        let edges = edges
            .into_iter()
            .inspect(|&(x, y)| {
                assert!(
                    self.has_edge(x, y),
                    "The edge ({x}, {y}) does not exist in the supergraph."
                )
            })
            .map(|(x, y)| (&self[x], &self[y]));

        // Create the new graph.
        Self::new([], edges)
    }

    // Check if the graph is a subgraph of a given graph.
    #[inline]
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    // Check if the graph is a supergraph of a given graph.
    #[inline]
    fn is_supergraph(&self, other: &Self) -> bool {
        self >= other
    }
}

// Test the `UGraph` struct.
#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use itertools::Itertools;

    use super::UGraph;
    use crate::{
        graphs::{Graph, UndirectedGraph},
        Adj, Ne, E, L, V,
    };

    // Test the `clone` method.
    #[test]
    fn clone() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());
        // Assert the graph is equal to the cloned graph.
        assert_eq!(graph, graph.clone());
    }

    // Test the `debug` method.
    #[test]
    fn debug() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());
        // Print the graph.
        assert_eq!(
            format!("{:?}", graph),
            concat!(
                "UndirectedDenseAdjacencyMatrix { ",
                "adjacency_matrix: ",
                "[[false, true, true, true, false, false, false, false],\n ",
                "[true, false, true, true, false, false, false, false],\n ",
                "[true, true, false, true, false, false, false, false],\n ",
                "[true, true, true, false, true, true, false, false],\n ",
                "[false, false, false, true, false, true, true, true],\n ",
                "[false, false, false, true, true, false, true, true],\n ",
                "[false, false, false, false, true, true, false, true],\n ",
                "[false, false, false, false, true, true, true, false]], ",
                "shape=[8, 8], strides=[8, 1], layout=Cc (0x5), const ndim=2, ",
                "labels: {\"A\", \"B\", \"C\", \"D\", \"E\", \"F\", \"G\", \"H\"}, ",
                "size: 14 ",
                "}"
            )
        );
    }

    // Test the `default` method.
    #[test]
    fn default() {
        // Create a new default graph.
        let graph = UGraph::default();

        // Check the graph order.
        assert_eq!(graph.order(), 0);
        // Check the graph size.
        assert_eq!(graph.size(), 0);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), Vec::<&str>::new());
        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), Vec::<usize>::new());
        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), Vec::<(usize, usize)>::new());
    }

    // Test the `display` method.
    #[test]
    fn display() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());
        // Print the graph.
        assert_eq!(
            format!("{}", graph),
            concat!(
                "UndirectedGraph { ",
                "V = {\"A\", \"B\", \"C\", \"D\", \"E\", \"F\", \"G\", \"H\"}, ",
                "E = {(\"A\", \"B\"), (\"A\", \"C\"), (\"A\", \"D\"), (\"B\", \"C\"), (\"B\", \"D\"), (\"C\", \"D\"), (\"D\", \"E\"), (\"D\", \"F\"), (\"E\", \"F\"), (\"E\", \"G\"), (\"E\", \"H\"), (\"F\", \"G\"), (\"F\", \"H\"), (\"G\", \"H\")} ",
                "}"
            )
        );
    }

    // Test the `eq` method.
    #[test]
    fn eq() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());
        // Assert the graph is equal to the cloned graph.
        assert_eq!(graph, graph.clone());

        // Create a new graph.
        let mut graph_i = UGraph::new(vertices.clone(), edges.clone());

        // Delete a vertex.
        graph_i.del_vertex(0);
        // Assert the graph is not equal to the modified graph.
        assert_ne!(graph, graph_i);

        // Create a new graph.
        let mut graph_i = UGraph::new(vertices.clone(), edges.clone());

        // Delete an edge.
        graph_i.del_edge(0, 1);
        // Assert the graph is not equal to the modified graph.
        assert_ne!(graph, graph_i);
    }

    // Test the `partial_cmp` method.
    #[test]
    fn partial_cmp() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C"];
        // Initialize the edges labels.
        let edges = vec![("A", "B"), ("B", "C")];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());
        // Assert the graph is equal to the cloned graph.
        assert_eq!(graph.partial_cmp(&graph.clone()), Some(Ordering::Equal));

        // Create a new graph.
        let mut subgraph = UGraph::new(vertices.clone(), edges.clone());

        // Delete a vertex.
        subgraph.del_vertex(0);
        // Assert the graph is not equal to the modified graph.
        assert_eq!(graph.partial_cmp(&subgraph), Some(Ordering::Greater));
        assert!(graph.is_supergraph(&subgraph));
        assert_eq!(subgraph.partial_cmp(&graph), Some(Ordering::Less));
        assert!(subgraph.is_subgraph(&graph));

        // Create a new graph.
        let mut subgraph = UGraph::new(vertices.clone(), edges.clone());

        // Delete an edge.
        subgraph.del_edge(0, 1);
        // Assert the graph is not equal to the modified graph.
        assert_eq!(graph.partial_cmp(&subgraph), Some(Ordering::Greater));
        assert!(graph.is_supergraph(&subgraph));
        assert_eq!(subgraph.partial_cmp(&graph), Some(Ordering::Less));
        assert!(subgraph.is_subgraph(&graph));

        // Create a new graph.
        let mut supergraph = UGraph::new(vertices.clone(), edges.clone());

        // Add a vertex.
        supergraph.add_vertex("D");
        // Assert the graph is not equal to the modified graph.
        assert_eq!(graph.partial_cmp(&supergraph), Some(Ordering::Less));
        assert!(graph.is_subgraph(&supergraph));
        assert_eq!(supergraph.partial_cmp(&graph), Some(Ordering::Greater));
        assert!(supergraph.is_supergraph(&graph));

        // Create a new graph.
        let mut supergraph = UGraph::new(vertices.clone(), edges.clone());

        // Add an edge.
        supergraph.add_edge(0, 2);
        // Assert the graph is not equal to the modified graph.
        assert_eq!(graph.partial_cmp(&supergraph), Some(Ordering::Less));
        assert!(graph.is_subgraph(&supergraph));
        assert_eq!(supergraph.partial_cmp(&graph), Some(Ordering::Greater));
        assert!(supergraph.is_supergraph(&graph));

        // Create two non-comparable graphs.
        let mut graph_i = UGraph::new(vertices.clone(), edges.clone());
        let mut graph_j = UGraph::new(vertices.clone(), edges.clone());

        // Add a vertex.
        graph_i.add_vertex("D");
        // Add a vertex.
        graph_j.add_vertex("E");

        // Assert the graphs are not comparable.
        assert_eq!(graph_i.partial_cmp(&graph_j), None);
        assert!(!graph_i.is_subgraph(&graph_j));
        assert!(!graph_j.is_subgraph(&graph_i));
        assert_eq!(graph_j.partial_cmp(&graph_i), None);
        assert!(!graph_i.is_supergraph(&graph_j));
        assert!(!graph_j.is_supergraph(&graph_i));

        // Create two non-comparable graphs.
        let mut graph_i = UGraph::new(vertices.clone(), edges.clone());
        let mut graph_j = UGraph::new(vertices.clone(), edges.clone());

        // Delete an edge.
        graph_i.del_edge(0, 1);
        // Delete an edge.
        graph_j.del_edge(1, 2);

        // Assert the graphs are not comparable.
        assert_eq!(graph_i.partial_cmp(&graph_j), None);
        assert!(!graph_i.is_subgraph(&graph_j));
        assert!(!graph_j.is_subgraph(&graph_i));
        assert_eq!(graph_j.partial_cmp(&graph_i), None);
        assert!(!graph_i.is_supergraph(&graph_j));
        assert!(!graph_j.is_supergraph(&graph_i));
    }

    // Test the `hash` method.
    #[test]
    fn hash() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());
        // Initialize the hasher.
        let mut hasher = DefaultHasher::new();
        // Hash the graph.
        graph.hash(&mut hasher);
        // Initialize the hasher for the cloned graph.
        let mut hasher_cloned = DefaultHasher::new();
        // Hash the cloned graph.
        graph.clone().hash(&mut hasher_cloned);
        // Assert the hashes are equal.
        assert_eq!(hasher.finish(), hasher_cloned.finish());

        // Create a new graph.
        let mut graph_i = UGraph::new(vertices.clone(), edges.clone());

        // Delete a vertex.
        graph_i.del_vertex(0);
        // Initialize the hasher.
        let mut hasher_i = DefaultHasher::new();
        // Hash the modified graph.
        graph_i.hash(&mut hasher_i);
        // Assert the hashes are not equal.
        assert_ne!(hasher.finish(), hasher_i.finish());

        // Create a new graph.
        let mut graph_i = UGraph::new(vertices.clone(), edges.clone());

        // Delete an edge.
        graph_i.del_edge(0, 1);

        // Initialize the hasher.
        let mut hasher_i = DefaultHasher::new();
        // Hash the modified graph.
        graph_i.hash(&mut hasher_i);
        // Assert the hashes are not equal.
        assert_ne!(hasher.finish(), hasher_i.finish());
    }

    // Test the `serialize` and `deserialize` method.
    #[test]
    fn serialize_deserialize() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());
        // Serialize and deserialize the graph.
        let graph =
            serde_json::from_str::<UGraph>(&serde_json::to_string(&graph).unwrap()).unwrap();

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), edges.len());
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), vertices);
        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), (0..vertices.len()).collect_vec());
        // Check the edges indices.
        assert_eq!(
            E!(graph).collect_vec(),
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 3),
                (2, 3),
                (3, 4),
                (3, 5),
                (4, 5),
                (4, 6),
                (4, 7),
                (5, 6),
                (5, 7),
                (6, 7)
            ]
        );
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2, 3]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2, 3]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1, 3]);
        assert_eq!(Adj!(graph, 3).collect_vec(), vec![0, 1, 2, 4, 5]);
        assert_eq!(Adj!(graph, 4).collect_vec(), vec![3, 5, 6, 7]);
        assert_eq!(Adj!(graph, 5).collect_vec(), vec![3, 4, 6, 7]);
        assert_eq!(Adj!(graph, 6).collect_vec(), vec![4, 5, 7]);
        assert_eq!(Adj!(graph, 7).collect_vec(), vec![4, 5, 6]);
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        assert_eq!(graph.vertex_to_label(3), "D");
        assert_eq!(graph.vertex_to_label(4), "E");
        assert_eq!(graph.vertex_to_label(5), "F");
        assert_eq!(graph.vertex_to_label(6), "G");
        assert_eq!(graph.vertex_to_label(7), "H");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);
        assert_eq!(graph.label_to_vertex("D"), 3);
        assert_eq!(graph.label_to_vertex("E"), 4);
        assert_eq!(graph.label_to_vertex("F"), 5);
        assert_eq!(graph.label_to_vertex("G"), 6);
        assert_eq!(graph.label_to_vertex("H"), 7);
    }

    // Test the `new` method.
    #[test]
    fn new() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), edges.len());
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), vertices);
        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), (0..vertices.len()).collect_vec());
        // Check the edges indices.
        assert_eq!(
            E!(graph).collect_vec(),
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 3),
                (2, 3),
                (3, 4),
                (3, 5),
                (4, 5),
                (4, 6),
                (4, 7),
                (5, 6),
                (5, 7),
                (6, 7)
            ]
        );
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2, 3]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2, 3]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1, 3]);
        assert_eq!(Adj!(graph, 3).collect_vec(), vec![0, 1, 2, 4, 5]);
        assert_eq!(Adj!(graph, 4).collect_vec(), vec![3, 5, 6, 7]);
        assert_eq!(Adj!(graph, 5).collect_vec(), vec![3, 4, 6, 7]);
        assert_eq!(Adj!(graph, 6).collect_vec(), vec![4, 5, 7]);
        assert_eq!(Adj!(graph, 7).collect_vec(), vec![4, 5, 6]);
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        assert_eq!(graph.vertex_to_label(3), "D");
        assert_eq!(graph.vertex_to_label(4), "E");
        assert_eq!(graph.vertex_to_label(5), "F");
        assert_eq!(graph.vertex_to_label(6), "G");
        assert_eq!(graph.vertex_to_label(7), "H");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);
        assert_eq!(graph.label_to_vertex("D"), 3);
        assert_eq!(graph.label_to_vertex("E"), 4);
        assert_eq!(graph.label_to_vertex("F"), 5);
        assert_eq!(graph.label_to_vertex("G"), 6);
        assert_eq!(graph.label_to_vertex("H"), 7);
    }

    // Test the `null` method.
    #[test]
    fn null() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Check the graph order.
        assert_eq!(graph.order(), 0);
        // Check the graph size.
        assert_eq!(graph.size(), 0);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), Vec::<&str>::new());
        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), Vec::<usize>::new());
        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), Vec::<(usize, usize)>::new());
    }

    // Test the `empty` method.
    #[test]
    fn empty() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];

        // Create a new empty graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), 0);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), vertices);
        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), (0..vertices.len()).collect_vec());
        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), Vec::<(usize, usize)>::new());
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 1).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 2).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 3).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 4).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 5).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 6).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 7).collect_vec(), Vec::<usize>::new());
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        assert_eq!(graph.vertex_to_label(3), "D");
        assert_eq!(graph.vertex_to_label(4), "E");
        assert_eq!(graph.vertex_to_label(5), "F");
        assert_eq!(graph.vertex_to_label(6), "G");
        assert_eq!(graph.vertex_to_label(7), "H");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);
        assert_eq!(graph.label_to_vertex("D"), 3);
        assert_eq!(graph.label_to_vertex("E"), 4);
        assert_eq!(graph.label_to_vertex("F"), 5);
        assert_eq!(graph.label_to_vertex("G"), 6);
        assert_eq!(graph.label_to_vertex("H"), 7);
    }

    // Test the `complete` method.
    #[test]
    fn complete() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];

        // Create a new complete graph.
        let graph = UGraph::complete(vertices.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), vertices.len() * (vertices.len() - 1) / 2);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), vertices);
        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), (0..vertices.len()).collect_vec());
        // Check the edges indices.
        assert_eq!(
            E!(graph).collect_vec(),
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (0, 6),
                (0, 7),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (2, 7),
                (3, 4),
                (3, 5),
                (3, 6),
                (3, 7),
                (4, 5),
                (4, 6),
                (4, 7),
                (5, 6),
                (5, 7),
                (6, 7)
            ]
        );
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2, 3, 4, 5, 6, 7]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1, 3, 4, 5, 6, 7]);
        assert_eq!(Adj!(graph, 3).collect_vec(), vec![0, 1, 2, 4, 5, 6, 7]);
        assert_eq!(Adj!(graph, 4).collect_vec(), vec![0, 1, 2, 3, 5, 6, 7]);
        assert_eq!(Adj!(graph, 5).collect_vec(), vec![0, 1, 2, 3, 4, 6, 7]);
        assert_eq!(Adj!(graph, 6).collect_vec(), vec![0, 1, 2, 3, 4, 5, 7]);
        assert_eq!(Adj!(graph, 7).collect_vec(), vec![0, 1, 2, 3, 4, 5, 6]);
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        assert_eq!(graph.vertex_to_label(3), "D");
        assert_eq!(graph.vertex_to_label(4), "E");
        assert_eq!(graph.vertex_to_label(5), "F");
        assert_eq!(graph.vertex_to_label(6), "G");
        assert_eq!(graph.vertex_to_label(7), "H");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);
        assert_eq!(graph.label_to_vertex("D"), 3);
        assert_eq!(graph.label_to_vertex("E"), 4);
        assert_eq!(graph.label_to_vertex("F"), 5);
        assert_eq!(graph.label_to_vertex("G"), 6);
        assert_eq!(graph.label_to_vertex("H"), 7);
    }

    // Test the `labels` method.
    #[test]
    fn labels_iter() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), vertices);
    }

    // Test the `vertex_to_label` method, should panic.
    #[test]
    #[should_panic]
    fn vertex_to_label_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Check the vertex to label.
        graph.vertex_to_label(0);
    }

    // Test the `vertex_to_label` method.
    #[test]
    fn vertex_to_label() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        assert_eq!(graph.vertex_to_label(3), "D");
        assert_eq!(graph.vertex_to_label(4), "E");
        assert_eq!(graph.vertex_to_label(5), "F");
        assert_eq!(graph.vertex_to_label(6), "G");
        assert_eq!(graph.vertex_to_label(7), "H");
    }

    // Test the `label_to_vertex` method, should panic.
    #[test]
    #[should_panic]
    fn label_to_vertex_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Check the label to vertex.
        graph.label_to_vertex("A");
    }

    // Test the `label_to_vertex` method.
    #[test]
    fn label_to_vertex() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
            ("E", "G"),
            ("E", "H"),
            ("F", "G"),
            ("F", "H"),
            ("G", "H"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);
        assert_eq!(graph.label_to_vertex("D"), 3);
        assert_eq!(graph.label_to_vertex("E"), 4);
        assert_eq!(graph.label_to_vertex("F"), 5);
        assert_eq!(graph.label_to_vertex("G"), 6);
        assert_eq!(graph.label_to_vertex("H"), 7);
    }

    // Test the `vertices` method.
    #[test]
    fn vertices_iter() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), (0..vertices.len()).collect_vec());
    }

    // Test the `has_vertex` method.
    #[test]
    fn has_vertex() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check if the vertices exist.
        assert!(graph.has_vertex(0));
        assert!(graph.has_vertex(1));
        assert!(graph.has_vertex(2));
        assert!(graph.has_vertex(3));
        // Check if the vertices do not exist.
        assert!(!graph.has_vertex(4));
        assert!(!graph.has_vertex(5));
        assert!(!graph.has_vertex(6));
        assert!(!graph.has_vertex(7));
    }

    // Test the `add_vertex` method.
    #[test]
    fn add_vertex() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let mut graph = UGraph::empty(vertices.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());

        // Add a new vertex.
        graph.add_vertex("E");

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() + 1);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), ["A", "B", "C", "D", "E"]);
        // Check the vertices indices.
        assert_eq!(
            V!(graph).collect_vec(),
            (0..vertices.len() + 1).collect_vec()
        );
        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), Vec::<(usize, usize)>::new());
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 1).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 2).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 3).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 4).collect_vec(), Vec::<usize>::new());
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        assert_eq!(graph.vertex_to_label(3), "D");
        assert_eq!(graph.vertex_to_label(4), "E");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);
        assert_eq!(graph.label_to_vertex("D"), 3);
        assert_eq!(graph.label_to_vertex("E"), 4);

        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "E"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "E"),
            ("B", "C"),
            ("B", "E"),
            ("C", "E"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), edges.len());

        // Add a new vertex.
        graph.add_vertex("D");

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() + 1);
        // Check the graph size.
        assert_eq!(graph.size(), edges.len());
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), ["A", "B", "C", "D", "E"]);
        // Check the vertices indices.
        assert_eq!(
            V!(graph).collect_vec(),
            (0..vertices.len() + 1).collect_vec()
        );
        // Check the edges indices.
        assert_eq!(
            E!(graph).collect_vec(),
            vec![(0, 1), (0, 2), (0, 4), (1, 2), (1, 4), (2, 4)]
        );

        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "E", "H"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "E"),
            ("B", "C"),
            ("B", "E"),
            ("C", "H"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), edges.len());

        // Add a new vertex.
        graph.add_vertex("D");

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() + 1);
        // Check the graph size.
        assert_eq!(graph.size(), edges.len());
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), ["A", "B", "C", "D", "E", "H"]);
        // Check the vertices indices.
        assert_eq!(
            V!(graph).collect_vec(),
            (0..vertices.len() + 1).collect_vec()
        );
        // Check the edges indices.
        assert_eq!(
            E!(graph).collect_vec(),
            vec![(0, 1), (0, 2), (0, 4), (1, 2), (1, 4), (2, 5)]
        );
    }

    // Test the `del_vertex` method.
    #[test]
    fn del_vertex() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let mut graph = UGraph::empty(vertices.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());

        // Delete a vertex.
        graph.del_vertex(0);

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() - 1);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), ["B", "C", "D"]);
        // Check the vertices indices.
        assert_eq!(
            V!(graph).collect_vec(),
            (0..vertices.len() - 1).collect_vec()
        );
        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), Vec::<(usize, usize)>::new());
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 1).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 2).collect_vec(), Vec::<usize>::new());
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "B");
        assert_eq!(graph.vertex_to_label(1), "C");
        assert_eq!(graph.vertex_to_label(2), "D");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("B"), 0);
        assert_eq!(graph.label_to_vertex("C"), 1);
        assert_eq!(graph.label_to_vertex("D"), 2);

        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "E"];
        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "E"),
            ("B", "C"),
            ("B", "E"),
            ("C", "E"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), edges.len());

        // Delete a vertex.
        graph.del_vertex(3);

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() - 1);
        // Check the graph size.
        assert_eq!(graph.size(), edges.len() - 3);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), ["A", "B", "C"]);
        // Check the vertices indices.
        assert_eq!(
            V!(graph).collect_vec(),
            (0..vertices.len() - 1).collect_vec()
        );
        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), vec![(0, 1), (0, 2), (1, 2)]);
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1]);
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);

        // Delete non existing vertex.
        graph.del_vertex(3);

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() - 1);
        // Check the graph size.
        assert_eq!(graph.size(), edges.len() - 3);
        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), ["A", "B", "C"]);
        // Check the vertices indices.
        assert_eq!(
            V!(graph).collect_vec(),
            (0..vertices.len() - 1).collect_vec()
        );
        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), vec![(0, 1), (0, 2), (1, 2)]);
        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1]);
        // Check the vertex to label.
        assert_eq!(graph.vertex_to_label(0), "A");
        assert_eq!(graph.vertex_to_label(1), "B");
        assert_eq!(graph.vertex_to_label(2), "C");
        // Check the label to vertex.
        assert_eq!(graph.label_to_vertex("A"), 0);
        assert_eq!(graph.label_to_vertex("B"), 1);
        assert_eq!(graph.label_to_vertex("C"), 2);
    }

    // Test the `size` method.
    #[test]
    fn size() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the graph size.
        assert_eq!(graph.size(), 0);

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the graph size.
        assert_eq!(graph.size(), edges.len());
    }

    // Test the `has_edge` method, should panic.
    #[test]
    #[should_panic]
    fn has_edge_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Check if the edge exists.
        graph.has_edge(0, 1);
    }

    // Test the `has_edge` method.
    #[test]
    fn has_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check if the edge exists.
        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 3));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(1, 1));
        assert!(!graph.has_edge(2, 2));
        assert!(!graph.has_edge(3, 3));

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(0, 3));
        assert!(graph.has_edge(1, 2));
        assert!(graph.has_edge(1, 3));
        assert!(graph.has_edge(2, 3));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(1, 1));
        assert!(!graph.has_edge(2, 2));
        assert!(!graph.has_edge(3, 3));

        // Delete a vertex.
        graph.del_vertex(3);

        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(1, 2));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(1, 1));
        assert!(!graph.has_edge(2, 2));

        // Delete a vertex.
        graph.del_vertex(1);

        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(1, 1));
    }

    // Test the `add_edge` method, should panic.
    #[test]
    #[should_panic]
    fn add_edge_should_panic() {
        // Create a new null graph.
        let mut graph = UGraph::null();

        // Add a new edge.
        graph.add_edge(0, 1);
    }

    // Test the `add_edge` method.
    #[test]
    fn add_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let mut graph = UGraph::empty(vertices.clone());

        // Check the graph size.
        assert_eq!(graph.size(), 0);

        // Add a new edge.
        graph.add_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.size(), 1);
        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 0));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(2, 3));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(3, 1));
        assert!(!graph.has_edge(3, 2));

        // Add a new edge.
        graph.add_edge(0, 2);

        // Check the graph size.
        assert_eq!(graph.size(), 2);
        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(2, 0));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(2, 3));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(3, 1));
        assert!(!graph.has_edge(3, 2));

        // Add a new edge.
        graph.add_edge(3, 3);

        // Check the graph size.
        assert_eq!(graph.size(), 3);
        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(2, 0));
        assert!(graph.has_edge(3, 3));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(2, 3));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(3, 1));
        assert!(!graph.has_edge(3, 2));
    }

    // Test the `del_edge` method, should panic.
    #[test]
    #[should_panic]
    fn del_edge_should_panic() {
        // Create a new null graph.
        let mut graph = UGraph::null();

        // Delete an edge.
        graph.del_edge(0, 1);
    }

    // Test the `del_edge` method.
    #[test]
    fn del_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let mut graph = UGraph::empty(vertices.clone());

        // Check the graph size.
        assert_eq!(graph.size(), 0);

        // Add a new edge.
        graph.add_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.size(), 1);
        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 0));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(2, 3));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(3, 1));
        assert!(!graph.has_edge(3, 2));

        // Delete an edge.
        graph.del_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.size(), 0);
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(1, 1));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 0));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(2, 2));
        assert!(!graph.has_edge(2, 3));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(3, 1));
        assert!(!graph.has_edge(3, 2));
    }

    // Test the `adjacents` method, should panic.
    #[test]
    #[should_panic]
    fn adjacents_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Check the adjacents indices.
        Adj!(graph, 0);
    }

    // Test the `degree` method, should panic.
    #[test]
    #[should_panic]
    fn degree_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Check the vertex degree.
        graph.degree(0);
    }

    // Test the `degree` method.
    #[test]
    fn degree() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the vertex degree.
        assert_eq!(graph.degree(0), 0);
        assert_eq!(graph.degree(1), 0);
        assert_eq!(graph.degree(2), 0);
        assert_eq!(graph.degree(3), 0);

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the vertex degree.
        assert_eq!(graph.degree(0), 3);
        assert_eq!(graph.degree(1), 3);
        assert_eq!(graph.degree(2), 3);
        assert_eq!(graph.degree(3), 3);

        // Delete a vertex.
        graph.del_vertex(3);

        // Check the vertex degree.
        assert_eq!(graph.degree(0), 2);
        assert_eq!(graph.degree(1), 2);
        assert_eq!(graph.degree(2), 2);

        // Delete a vertex.
        graph.del_vertex(1);

        // Check the vertex degree.
        assert_eq!(graph.degree(0), 1);
        assert_eq!(graph.degree(1), 1);

        // Delete a vertex.
        graph.del_vertex(0);

        // Check the vertex degree.
        assert_eq!(graph.degree(0), 0);
    }

    // Test the `degrees` method.
    #[test]
    fn degrees() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), vec![0, 0, 0, 0]);

        // Initialize the edges labels.
        let edges = vec![("A", "B"), ("A", "C"), ("B", "C"), ("C", "D")];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), vec![2, 2, 3, 1]);

        // Delete a vertex.
        graph.del_vertex(2);

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), vec![1, 1, 0]);

        // Delete a vertex.
        graph.del_vertex(1);

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), vec![0, 0]);

        // Delete a vertex.
        graph.del_vertex(0);

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), vec![0]);

        // Delete a vertex.
        graph.del_vertex(0);

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), Vec::<usize>::new());
    }

    // Test the `adjacents` method.
    #[test]
    fn adjacents_iter() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 1).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 2).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 3).collect_vec(), Vec::<usize>::new());

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2, 3]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2, 3]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1, 3]);
        assert_eq!(Adj!(graph, 3).collect_vec(), vec![0, 1, 2]);

        // Add a vertex.
        graph.add_vertex("E");

        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2, 3]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2, 3]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1, 3]);
        assert_eq!(Adj!(graph, 3).collect_vec(), vec![0, 1, 2]);
        assert_eq!(Adj!(graph, 4).collect_vec(), Vec::<usize>::new());

        // Delete a vertex.
        graph.del_vertex(3);

        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1]);
        assert_eq!(Adj!(graph, 3).collect_vec(), Vec::<usize>::new());

        // Delete a vertex.
        graph.del_vertex(1);

        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0]);
        assert_eq!(Adj!(graph, 2).collect_vec(), Vec::<usize>::new());

        // Delete a vertex.
        graph.del_vertex(0);

        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), Vec::<usize>::new());
        assert_eq!(Adj!(graph, 1).collect_vec(), Vec::<usize>::new());
    }

    // Test the `is_adjacent` method, should panic.
    #[test]
    #[should_panic]
    fn is_adjacent_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Check if the vertices are adjacent.
        graph.is_adjacent(0, 1);
    }

    // Test the `is_adjacent` method.
    #[test]
    fn is_adjacent() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check if the vertices are adjacent.
        assert!(!graph.is_adjacent(0, 1));
        assert!(!graph.is_adjacent(0, 2));
        assert!(!graph.is_adjacent(0, 3));
        assert!(!graph.is_adjacent(1, 2));
        assert!(!graph.is_adjacent(1, 3));
        assert!(!graph.is_adjacent(2, 3));
        // Check if the vertices are not adjacent.
        assert!(!graph.is_adjacent(0, 0));
        assert!(!graph.is_adjacent(1, 1));
        assert!(!graph.is_adjacent(2, 2));
        assert!(!graph.is_adjacent(3, 3));

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check if the vertices are adjacent.
        assert!(graph.is_adjacent(0, 1));
        assert!(graph.is_adjacent(0, 2));
        assert!(graph.is_adjacent(0, 3));
        assert!(graph.is_adjacent(1, 2));
        assert!(graph.is_adjacent(1, 3));
        assert!(graph.is_adjacent(2, 3));
        // Check if the vertices are not adjacent.
        assert!(!graph.is_adjacent(0, 0));
        assert!(!graph.is_adjacent(1, 1));
        assert!(!graph.is_adjacent(2, 2));
        assert!(!graph.is_adjacent(3, 3));

        // Delete a vertex.
        graph.del_vertex(3);

        // Check if the vertices are adjacent.
        assert!(graph.is_adjacent(0, 1));
        assert!(graph.is_adjacent(0, 2));
        assert!(graph.is_adjacent(1, 2));
        // Check if the vertices are not adjacent.
        assert!(!graph.is_adjacent(0, 0));
    }

    // Test the `subgraph` method, should panic.
    #[test]
    #[should_panic]
    fn subgraph_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Get the subgraph.
        graph.subgraph([0, 1, 2], [(0, 1), (1, 2)]);
    }

    // Test the `subgraph` method.
    #[test]
    fn subgraph() {
        // Create a new graph.
        let graph = UGraph::new(
            vec!["A", "B", "C", "D"],
            vec![
                ("A", "B"),
                ("A", "C"),
                ("A", "D"),
                ("B", "C"),
                ("B", "D"),
                ("C", "D"),
            ],
        );

        // Get the subgraph.
        let subgraph = graph.subgraph([0, 1, 3], [(0, 1), (1, 3)]);

        // Check the subgraph order.
        assert_eq!(subgraph.order(), 3);
        // Check the subgraph size.
        assert_eq!(subgraph.size(), 2);
        // Check the vertices labels.
        assert_eq!(L!(subgraph).collect_vec(), ["A", "B", "D"]);
        // Check the vertices indices.
        assert_eq!(V!(subgraph).collect_vec(), [0, 1, 2]);
        // Check the edges indices.
        assert_eq!(E!(subgraph).collect_vec(), vec![(0, 1), (1, 2)]);
    }

    // Test the `subgraph_by_vertices` method, should panic.
    #[test]
    #[should_panic]
    fn subgraph_by_vertices_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Get the subgraph.
        graph.subgraph_by_vertices([0, 1, 2]);
    }

    // Test the `subgraph_by_vertices` method.
    #[test]
    fn subgraph_by_vertices() {
        // Create a new graph.
        let graph = UGraph::new(
            vec!["A", "B", "C", "D"],
            vec![
                ("A", "B"),
                ("A", "C"),
                ("A", "D"),
                ("B", "C"),
                ("B", "D"),
                ("C", "D"),
            ],
        );

        // Get the subgraph.
        let subgraph = graph.subgraph_by_vertices([0, 1, 3]);

        // Check the subgraph order.
        assert_eq!(subgraph.order(), 3);
        // Check the subgraph size.
        assert_eq!(subgraph.size(), 3);
        // Check the vertices labels.
        assert_eq!(L!(subgraph).collect_vec(), ["A", "B", "D"]);
        // Check the vertices indices.
        assert_eq!(V!(subgraph).collect_vec(), [0, 1, 2]);
        // Check the edges indices.
        assert_eq!(E!(subgraph).collect_vec(), vec![(0, 1), (0, 2), (1, 2)]);
    }

    // Test the `subgraph_by_edges` method, should panic.
    #[test]
    #[should_panic]
    fn subgraph_by_edges_should_panic() {
        // Create a new null graph.
        let graph = UGraph::null();

        // Get the subgraph.
        graph.subgraph_by_edges([(0, 1), (1, 2)]);
    }

    // Test the `subgraph_by_edges` method.
    #[test]
    fn subgraph_by_edges() {
        // Create a new graph.
        let graph = UGraph::new(
            vec!["A", "B", "C", "D"],
            vec![
                ("A", "B"),
                ("A", "C"),
                ("A", "D"),
                ("B", "C"),
                ("B", "D"),
                ("C", "D"),
            ],
        );

        // Get the subgraph.
        let subgraph = graph.subgraph_by_edges([(0, 1), (1, 3)]);

        // Check the subgraph order.
        assert_eq!(subgraph.order(), 3);
        // Check the subgraph size.
        assert_eq!(subgraph.size(), 2);
        // Check the vertices labels.
        assert_eq!(L!(subgraph).collect_vec(), ["A", "B", "D"]);
        // Check the vertices indices.
        assert_eq!(V!(subgraph).collect_vec(), [0, 1, 2]);
        // Check the edges indices.
        assert_eq!(E!(subgraph).collect_vec(), vec![(0, 1), (1, 2)]);
    }

    // Test the `is_subgraph` method.
    #[test]
    fn is_subgraph() {
        // Create a new graph.
        let graph = UGraph::new(
            vec!["A", "B", "C", "D"],
            vec![
                ("A", "B"),
                ("A", "C"),
                ("A", "D"),
                ("B", "C"),
                ("B", "D"),
                ("C", "D"),
            ],
        );

        // Get the subgraph.
        let subgraph = graph.subgraph_by_vertices([0, 1, 3]);

        // Check if the subgraph is a subgraph.
        assert!(subgraph.is_subgraph(&graph));
        // Check if the graph is not a subgraph.
        assert!(!graph.is_subgraph(&subgraph));
    }

    // Test the `is_supergraph` method.
    #[test]
    fn is_supergraph() {
        // Create a new graph.
        let graph = UGraph::new(
            vec!["A", "B", "C", "D"],
            vec![
                ("A", "B"),
                ("A", "C"),
                ("A", "D"),
                ("B", "C"),
                ("B", "D"),
                ("C", "D"),
            ],
        );

        // Get the subgraph.
        let subgraph = graph.subgraph_by_vertices([0, 1, 3]);

        // Check if the graph is a supergraph.
        assert!(graph.is_supergraph(&subgraph));
        // Check if the subgraph is not a supergraph.
        assert!(!subgraph.is_supergraph(&graph));
    }

    // Test the `undirected_size` method.
    #[test]
    fn undirected_size() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the undirected graph size.
        assert_eq!(graph.undirected_size(), 0);

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("B", "C"),
            ("B", "D"),
            ("C", "D"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the undirected graph size.
        assert_eq!(graph.undirected_size(), edges.len());
    }

    // Test the `undirected_edges` method.
    #[test]
    fn undirected_edges() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the undirected edges indices.
        assert_eq!(
            graph.undirected_edges_iter().collect_vec(),
            Vec::<(usize, usize)>::new()
        );

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("A", "E"),
            ("B", "C"),
            ("B", "D"),
            ("B", "E"),
            ("C", "D"),
            ("C", "E"),
            ("D", "E"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the undirected edges indices.
        assert_eq!(
            graph.undirected_edges_iter().collect_vec(),
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 3),
                (2, 4),
                (3, 4)
            ]
        );
    }

    // Test the `has_undirected_edge` method, should panic.
    #[test]
    #[should_panic]
    fn has_undirected_edge_should_panic() {
        // Create a new null undirected graph.
        let graph = UGraph::null();

        // Check if the edge exists.
        graph.has_undirected_edge(0, 1);
    }

    // Test the `has_undirected_edge` method.
    #[test]
    fn has_undirected_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check if the edge exists.
        assert!(!graph.has_undirected_edge(0, 1));
        assert!(!graph.has_undirected_edge(0, 2));
        assert!(!graph.has_undirected_edge(0, 3));
        assert!(!graph.has_undirected_edge(0, 4));
        assert!(!graph.has_undirected_edge(1, 2));
        assert!(!graph.has_undirected_edge(1, 3));
        assert!(!graph.has_undirected_edge(1, 4));
        assert!(!graph.has_undirected_edge(2, 3));
        assert!(!graph.has_undirected_edge(2, 4));
        assert!(!graph.has_undirected_edge(3, 4));
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(1, 1));
        assert!(!graph.has_undirected_edge(2, 2));
        assert!(!graph.has_undirected_edge(3, 3));
        assert!(!graph.has_undirected_edge(4, 4));

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("A", "E"),
            ("B", "C"),
            ("B", "D"),
            ("B", "E"),
            ("C", "D"),
            ("C", "E"),
            ("D", "E"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check if the edge exists.
        assert!(graph.has_undirected_edge(0, 1));
        assert!(graph.has_undirected_edge(0, 2));
        assert!(graph.has_undirected_edge(0, 3));
        assert!(graph.has_undirected_edge(0, 4));
        assert!(graph.has_undirected_edge(1, 2));
        assert!(graph.has_undirected_edge(1, 3));
        assert!(graph.has_undirected_edge(1, 4));
        assert!(graph.has_undirected_edge(2, 3));
        assert!(graph.has_undirected_edge(2, 4));
        assert!(graph.has_undirected_edge(3, 4));
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(1, 1));
        assert!(!graph.has_undirected_edge(2, 2));
        assert!(!graph.has_undirected_edge(3, 3));
        assert!(!graph.has_undirected_edge(4, 4));

        // Delete a vertex.
        graph.del_vertex(4);

        // Check if the edge exists.
        assert!(graph.has_undirected_edge(0, 1));
        assert!(graph.has_undirected_edge(0, 2));
        assert!(graph.has_undirected_edge(0, 3));
        assert!(graph.has_undirected_edge(1, 2));
        assert!(graph.has_undirected_edge(1, 3));
        assert!(graph.has_undirected_edge(2, 3));
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(1, 1));
        assert!(!graph.has_undirected_edge(2, 2));
        assert!(!graph.has_undirected_edge(3, 3));
    }

    // Test the `add_undirected_edge` method, should panic.
    #[test]
    #[should_panic]
    fn add_undirected_edge_should_panic() {
        // Create a new null undirected graph.
        let mut graph = UGraph::null();

        // Add a new edge.
        graph.add_undirected_edge(0, 1);
    }

    // Test the `add_undirected_edge` method.
    #[test]
    fn add_undirected_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let mut graph = UGraph::empty(vertices.clone());

        // Check the graph size.
        assert_eq!(graph.undirected_size(), 0);

        // Add a new edge.
        graph.add_undirected_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.undirected_size(), 1);
        // Check if the edge exists.
        assert!(graph.has_undirected_edge(0, 1));
        assert!(graph.has_undirected_edge(1, 0));
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(0, 2));
        assert!(!graph.has_undirected_edge(0, 3));
        assert!(!graph.has_undirected_edge(0, 4));
        assert!(!graph.has_undirected_edge(1, 2));
        assert!(!graph.has_undirected_edge(1, 3));
        assert!(!graph.has_undirected_edge(1, 4));
        assert!(!graph.has_undirected_edge(2, 0));
        assert!(!graph.has_undirected_edge(2, 1));
        assert!(!graph.has_undirected_edge(2, 3));
        assert!(!graph.has_undirected_edge(2, 4));
        assert!(!graph.has_undirected_edge(3, 0));
        assert!(!graph.has_undirected_edge(3, 1));
        assert!(!graph.has_undirected_edge(3, 2));
        assert!(!graph.has_undirected_edge(3, 4));
        assert!(!graph.has_undirected_edge(4, 0));
        assert!(!graph.has_undirected_edge(4, 1));
        assert!(!graph.has_undirected_edge(4, 2));
        assert!(!graph.has_undirected_edge(4, 3));

        // Add a new edge.
        graph.add_undirected_edge(0, 2);

        // Check the graph size.
        assert_eq!(graph.undirected_size(), 2);
        // Check if the edge exists.
        assert!(graph.has_undirected_edge(0, 1));
        assert!(graph.has_undirected_edge(1, 0));
        assert!(graph.has_undirected_edge(0, 2));
        assert!(graph.has_undirected_edge(2, 0));
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(0, 3));
        assert!(!graph.has_undirected_edge(0, 4));
        assert!(!graph.has_undirected_edge(1, 2));
        assert!(!graph.has_undirected_edge(1, 3));
        assert!(!graph.has_undirected_edge(1, 4));
        assert!(!graph.has_undirected_edge(2, 1));
        assert!(!graph.has_undirected_edge(2, 3));
        assert!(!graph.has_undirected_edge(2, 4));
        assert!(!graph.has_undirected_edge(3, 0));
        assert!(!graph.has_undirected_edge(3, 1));
        assert!(!graph.has_undirected_edge(3, 2));
        assert!(!graph.has_undirected_edge(3, 4));
        assert!(!graph.has_undirected_edge(4, 0));
        assert!(!graph.has_undirected_edge(4, 1));
        assert!(!graph.has_undirected_edge(4, 2));
        assert!(!graph.has_undirected_edge(4, 3));

        // Add a new edge.
        graph.add_undirected_edge(4, 4);

        // Check the graph size.
        assert_eq!(graph.undirected_size(), 3);
        // Check if the edge exists.
        assert!(graph.has_undirected_edge(0, 1));
        assert!(graph.has_undirected_edge(1, 0));
        assert!(graph.has_undirected_edge(0, 2));
        assert!(graph.has_undirected_edge(2, 0));
        assert!(graph.has_undirected_edge(4, 4));
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(0, 3));
        assert!(!graph.has_undirected_edge(1, 2));
        assert!(!graph.has_undirected_edge(1, 3));
        assert!(!graph.has_undirected_edge(1, 4));
        assert!(!graph.has_undirected_edge(2, 1));
        assert!(!graph.has_undirected_edge(2, 3));
        assert!(!graph.has_undirected_edge(2, 4));
        assert!(!graph.has_undirected_edge(3, 0));
        assert!(!graph.has_undirected_edge(3, 1));
        assert!(!graph.has_undirected_edge(3, 2));
        assert!(!graph.has_undirected_edge(3, 4));
        assert!(!graph.has_undirected_edge(4, 0));
        assert!(!graph.has_undirected_edge(4, 1));
        assert!(!graph.has_undirected_edge(4, 2));
        assert!(!graph.has_undirected_edge(4, 3));
    }

    // Test the `del_undirected_edge` method, should panic.
    #[test]
    #[should_panic]
    fn del_undirected_edge_should_panic() {
        // Create a new null undirected graph.
        let mut graph = UGraph::null();

        // Delete an edge.
        graph.del_undirected_edge(0, 1);
    }

    // Test the `del_undirected_edge` method.
    #[test]
    fn del_undirected_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let mut graph = UGraph::empty(vertices.clone());

        // Check the graph size.
        assert_eq!(graph.undirected_size(), 0);

        // Add a new edge.
        graph.add_undirected_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.undirected_size(), 1);
        // Check if the edge exists.
        assert!(graph.has_undirected_edge(0, 1));
        assert!(graph.has_undirected_edge(1, 0));
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(0, 2));
        assert!(!graph.has_undirected_edge(0, 3));
        assert!(!graph.has_undirected_edge(0, 4));
        assert!(!graph.has_undirected_edge(1, 2));
        assert!(!graph.has_undirected_edge(1, 3));
        assert!(!graph.has_undirected_edge(1, 4));
        assert!(!graph.has_undirected_edge(2, 0));
        assert!(!graph.has_undirected_edge(2, 1));
        assert!(!graph.has_undirected_edge(2, 3));
        assert!(!graph.has_undirected_edge(2, 4));
        assert!(!graph.has_undirected_edge(3, 0));
        assert!(!graph.has_undirected_edge(3, 1));
        assert!(!graph.has_undirected_edge(3, 2));
        assert!(!graph.has_undirected_edge(3, 4));
        assert!(!graph.has_undirected_edge(4, 0));
        assert!(!graph.has_undirected_edge(4, 1));
        assert!(!graph.has_undirected_edge(4, 2));
        assert!(!graph.has_undirected_edge(4, 3));

        // Delete an edge.
        graph.del_undirected_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.undirected_size(), 0);
        // Check if the edge does not exist.
        assert!(!graph.has_undirected_edge(0, 0));
        assert!(!graph.has_undirected_edge(0, 1));
        assert!(!graph.has_undirected_edge(0, 2));
        assert!(!graph.has_undirected_edge(0, 3));
        assert!(!graph.has_undirected_edge(0, 4));
        assert!(!graph.has_undirected_edge(1, 0));
        assert!(!graph.has_undirected_edge(1, 1));
        assert!(!graph.has_undirected_edge(1, 2));
        assert!(!graph.has_undirected_edge(1, 3));
        assert!(!graph.has_undirected_edge(1, 4));
        assert!(!graph.has_undirected_edge(2, 0));
        assert!(!graph.has_undirected_edge(2, 1));
        assert!(!graph.has_undirected_edge(2, 2));
        assert!(!graph.has_undirected_edge(2, 3));
        assert!(!graph.has_undirected_edge(2, 4));
        assert!(!graph.has_undirected_edge(3, 0));
        assert!(!graph.has_undirected_edge(3, 1));
        assert!(!graph.has_undirected_edge(3, 2));
        assert!(!graph.has_undirected_edge(3, 3));
        assert!(!graph.has_undirected_edge(3, 4));
        assert!(!graph.has_undirected_edge(4, 0));
        assert!(!graph.has_undirected_edge(4, 1));
        assert!(!graph.has_undirected_edge(4, 2));
        assert!(!graph.has_undirected_edge(4, 3));
        assert!(!graph.has_undirected_edge(4, 4));
    }

    // Test the `neighbors` method, should panic.
    #[test]
    #[should_panic]
    fn neighbors_should_panic() {
        // Create a new null undirected graph.
        let graph = UGraph::null();

        // Check the neighbors indices.
        Ne!(graph, 0);
    }

    // Test the `neighbors` method.
    #[test]
    fn neighbors() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check the neighbors indices.
        assert_eq!(Ne!(graph, 0).collect_vec(), Vec::<usize>::new());
        assert_eq!(Ne!(graph, 1).collect_vec(), Vec::<usize>::new());
        assert_eq!(Ne!(graph, 2).collect_vec(), Vec::<usize>::new());
        assert_eq!(Ne!(graph, 3).collect_vec(), Vec::<usize>::new());
        assert_eq!(Ne!(graph, 4).collect_vec(), Vec::<usize>::new());

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("A", "E"),
            ("B", "C"),
            ("B", "D"),
            ("B", "E"),
            ("C", "D"),
            ("C", "E"),
            ("D", "E"),
        ];

        // Create a new graph.
        let mut graph = UGraph::new(vertices.clone(), edges.clone());

        // Check the neighbors indices.
        assert_eq!(Ne!(graph, 0).collect_vec(), vec![1, 2, 3, 4]);
        assert_eq!(Ne!(graph, 1).collect_vec(), vec![0, 2, 3, 4]);
        assert_eq!(Ne!(graph, 2).collect_vec(), vec![0, 1, 3, 4]);
        assert_eq!(Ne!(graph, 3).collect_vec(), vec![0, 1, 2, 4]);
        assert_eq!(Ne!(graph, 4).collect_vec(), vec![0, 1, 2, 3]);

        // Delete a vertex.
        graph.del_vertex(4);

        // Check the neighbors indices.
        assert_eq!(Ne!(graph, 0).collect_vec(), vec![1, 2, 3]);
        assert_eq!(Ne!(graph, 1).collect_vec(), vec![0, 2, 3]);
        assert_eq!(Ne!(graph, 2).collect_vec(), vec![0, 1, 3]);
        assert_eq!(Ne!(graph, 3).collect_vec(), vec![0, 1, 2]);

        // Delete a vertex.
        graph.del_vertex(3);

        // Check the neighbors indices.
        assert_eq!(Ne!(graph, 0).collect_vec(), vec![1, 2]);
        assert_eq!(Ne!(graph, 1).collect_vec(), vec![0, 2]);
        assert_eq!(Ne!(graph, 2).collect_vec(), vec![0, 1]);

        // Delete a vertex.
        graph.del_vertex(2);

        // Check the neighbors indices.
        assert_eq!(Ne!(graph, 0).collect_vec(), vec![1]);
        assert_eq!(Ne!(graph, 1).collect_vec(), vec![0]);

        // Delete a vertex.
        graph.del_vertex(0);

        // Check the neighbors indices.
        assert_eq!(Ne!(graph, 0).collect_vec(), Vec::<usize>::new());
    }

    // Test the `is_neighbor` method, should panic.
    #[test]
    #[should_panic]
    fn is_neighbor_should_panic() {
        // Create a new null undirected graph.
        let graph = UGraph::null();

        // Check if the vertices are neighbors.
        graph.is_neighbor(0, 1);
    }

    // Test the `is_neighbor` method.
    #[test]
    fn is_neighbor() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let graph = UGraph::empty(vertices.clone());

        // Check if the vertices are neighbors.
        assert!(!graph.is_neighbor(0, 1));
        assert!(!graph.is_neighbor(0, 2));
        assert!(!graph.is_neighbor(0, 3));
        assert!(!graph.is_neighbor(0, 4));
        assert!(!graph.is_neighbor(1, 2));
        assert!(!graph.is_neighbor(1, 3));
        assert!(!graph.is_neighbor(1, 4));
        assert!(!graph.is_neighbor(2, 3));
        assert!(!graph.is_neighbor(2, 4));
        assert!(!graph.is_neighbor(3, 4));
        // Check if the vertices are not neighbors.
        assert!(!graph.is_neighbor(0, 0));
        assert!(!graph.is_neighbor(1, 1));
        assert!(!graph.is_neighbor(2, 2));
        assert!(!graph.is_neighbor(3, 3));
        assert!(!graph.is_neighbor(4, 4));

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("A", "E"),
            ("B", "C"),
            ("B", "D"),
            ("B", "E"),
            ("C", "D"),
            ("C", "E"),
            ("D", "E"),
        ];

        // Create a new graph.
        let graph = UGraph::new(vertices.clone(), edges.clone());

        // Check if the vertices are neighbors.
        assert!(graph.is_neighbor(0, 1));
        assert!(graph.is_neighbor(0, 2));
        assert!(graph.is_neighbor(0, 3));
        assert!(graph.is_neighbor(0, 4));
        assert!(graph.is_neighbor(1, 2));
        assert!(graph.is_neighbor(1, 3));
        assert!(graph.is_neighbor(1, 4));
        assert!(graph.is_neighbor(2, 3));
        assert!(graph.is_neighbor(2, 4));
        assert!(graph.is_neighbor(3, 4));
        // Check if the vertices are not neighbors.
        assert!(!graph.is_neighbor(0, 0));
        assert!(!graph.is_neighbor(1, 1));
        assert!(!graph.is_neighbor(2, 2));
        assert!(!graph.is_neighbor(3, 3));
        assert!(!graph.is_neighbor(4, 4));
    }
}
