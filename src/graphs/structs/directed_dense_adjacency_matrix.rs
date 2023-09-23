use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use indexmap::IndexSet;
use itertools::Itertools;
use ndarray::{prelude::*, OwnedRepr};
use serde::{Deserialize, Serialize};

use crate::graphs::{Directed, DirectedGraph, Graph};

/// Define the `DirectedDenseAdjacencyMatrix` struct using a dense adjacency matrix from the `ndarray` crate.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectedDenseAdjacencyMatrix {
    /// The adjacency matrix.
    adjacency_matrix: Array2<bool>,
    /// The vertices labels.
    labels: IndexSet<String>,
    /// The graph size.
    size: usize,
}

// Implement the `Display` trait for the `DirectedDenseAdjacencyMatrix` struct.
impl Display for DirectedDenseAdjacencyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "DirectedGraph {{ ")?;

        // Write vertex set.
        write!(
            f,
            "V = {{{}}}, ",
            self.vertices()
                .map(|x| format!("\"{}\"", self.vertex_to_label(x)))
                .join(", ")
        )?;

        // Write edge set.
        write!(
            f,
            "E = {{{}}}",
            self.edges()
                .map(|(x, y)| format!(
                    "(\"{}\", \"{}\")",
                    self.vertex_to_label(x),
                    self.vertex_to_label(y)
                ))
                .join(", ")
        )?;

        // Write ending character.
        write!(f, " }}")
    }
}

// Implement the `Hash` trait for the `DirectedDenseAdjacencyMatrix` struct.
impl Hash for DirectedDenseAdjacencyMatrix {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Compute the hash of the adjacency matrix.
        self.adjacency_matrix.hash(state);
        // Compute the hash of the vertices labels.
        self.labels.iter().for_each(|x| x.hash(state));
    }
}

/// Define the `EdgesIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
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

// Implement the `EdgesIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
impl<'a> EdgesIterator<'a> {
    /// Create a new `EdgesIterator` iterator.
    fn new(graph: &'a DirectedDenseAdjacencyMatrix) -> Self {
        // Create the new `EdgesIterator` iterator.
        Self {
            iter: graph
                .adjacency_matrix
                .indexed_iter()
                .filter_map(|((x, y), &flag)|
                    // Check if the edge exists.
                    if flag {
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

/// Define the `AdjacentsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
#[allow(dead_code, clippy::type_complexity)]
pub struct AdjacentsIterator<'a> {
    // The graph.
    graph: &'a DirectedDenseAdjacencyMatrix,
    // The adjacents indices iterator.
    iter: std::iter::FilterMap<
        std::iter::Enumerate<
            <ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter,
        >,
        fn((usize, bool)) -> Option<usize>,
    >,
}

// Implement the `AdjacentsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
impl<'a> AdjacentsIterator<'a> {
    /// Create a new `AdjacentsIterator` iterator.
    fn new(graph: &'a DirectedDenseAdjacencyMatrix, x: usize) -> Self {
        // Assert the vertex is in bounds.
        assert!(
            graph.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Create the new `AdjacentsIterator` iterator.
        Self {
            graph,
            iter: {
                    // Get the row of the vertex.
                    let row = graph
                        .adjacency_matrix
                        .row(x);
                    // Get the column of the vertex.
                    let col = graph
                        .adjacency_matrix
                        .column(x);
                    // Compute the bitwise-or.
                    &row | &col
                }
                .into_iter()
                .enumerate()
                .filter_map(|(x, flag)|
                    // Check if the vertex is adjacent.
                    if flag {
                        // Return the vertex index.
                        Some(x)
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

// Implement the `Graph` trait for the `DirectedDenseAdjacencyMatrix` struct.
impl Graph for DirectedDenseAdjacencyMatrix {
    // Direction associated type.
    type Direction = Directed;
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
            });

        // Compute the graph size given the adjacency matrix.
        let size = adjacency_matrix.mapv(|x| x as usize).sum();

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
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
            adjacency_matrix.mapv(|x| x as usize).sum(),
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
        let size = order * (order - 1);

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
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
        debug_assert_eq!(size, order * (order - 1), "The graph size is not correct.");

        // Create the new complete graph.
        Self {
            adjacency_matrix,
            labels,
            size,
        }
    }

    // Get the vertices labels iterator.
    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
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
    fn vertices(&self) -> Self::VerticesIter<'_> {
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

        // Update the graph size.
        self.size = self.adjacency_matrix.mapv(|x| x as usize).sum();

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
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
            self.adjacency_matrix.mapv(|x| x as usize).sum(),
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

        // Update the graph size.
        self.size = self.adjacency_matrix.mapv(|x| x as usize).sum();

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
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
            self.adjacency_matrix.mapv(|x| x as usize).sum(),
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
    fn edges(&self) -> Self::EdgesIter<'_> {
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
        // Update the graph size.
        self.size += 1;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
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
            self.adjacency_matrix.mapv(|x| x as usize).sum(),
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
        // Update the graph size.
        self.size -= 1;

        // Debug assert the adjacency matrix is square.
        debug_assert!(
            self.adjacency_matrix.is_square(),
            "The adjacency matrix is not square."
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
            self.adjacency_matrix.mapv(|x| x as usize).sum(),
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
            + self.adjacency_matrix.column(x).mapv(|x| x as usize).sum()
            - self.adjacency_matrix[[x, x]] as usize
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
        (self.adjacency_matrix.mapv(|x| x as usize).sum_axis(Axis(0))
            + self.adjacency_matrix.mapv(|x| x as usize).sum_axis(Axis(1))
            - self.adjacency_matrix.diag().mapv(|x| x as usize))
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
    fn adjacents(&self, x: usize) -> Self::AdjacentsIter<'_> {
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
        self.adjacency_matrix[[x, y]] || self.adjacency_matrix[[y, x]]
    }
}

/// Define the `AncestorsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
#[allow(dead_code, clippy::type_complexity)]
pub struct AncestorsIterator<'a> {
    // The graph.
    graph: &'a DirectedDenseAdjacencyMatrix,
    // The ancestors indices iterator.
    iter: std::iter::FilterMap<
        std::iter::Enumerate<
            <ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter,
        >,
        fn((usize, bool)) -> Option<usize>,
    >,
}

// Implement the `AncestorsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
impl<'a> AncestorsIterator<'a> {
    /// Create a new `AncestorsIterator` iterator.
    pub fn new(graph: &'a DirectedDenseAdjacencyMatrix, x: usize) -> Self {
        // Assert the vertex is in bounds.
        assert!(
            graph.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Create the new `AncestorsIterator` iterator.
        Self {
            graph,
            iter: {
                // Get underlying adjacency matrix.
                let adjacency_matrix = &graph.adjacency_matrix;
                // Initialize previous solution.
                let mut prev = Array1::from_elem((adjacency_matrix.ncols(),), false);
                // Get current ancestors set, i.e. parents set.
                let mut curr = adjacency_matrix.column(x).to_owned();

                // Check stopping criterion.
                while curr != prev {
                    // Update previous.
                    prev.assign(&curr);
                    // Select current parents.
                    let next = adjacency_matrix & &curr;
                    // Collapse new parents.
                    let next = next.fold_axis(Axis(1), false, |acc, f| acc | f);
                    // Accumulate new parents.
                    curr = curr | next;
                }

                curr.into_iter().enumerate().filter_map(|(x, flag)|
                    // Check if the vertex is ancestor.
                    if flag {
                        // Return the vertex index.
                        Some(x)
                    } else {
                        // Return `None`.
                        None
                    }
                )
            },
        }
    }
}

// Implement the `Iterator` trait for the `AncestorsIterator` iterator.
impl<'a> Iterator for AncestorsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next vertex index.
        self.iter.next()
    }
}

/// Define the `ParentsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
#[allow(dead_code, clippy::type_complexity)]
pub struct ParentsIterator<'a> {
    // The graph.
    graph: &'a DirectedDenseAdjacencyMatrix,
    // The parents indices iterator.
    iter: std::iter::FilterMap<
        std::iter::Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

// Implement the `ParentsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
impl<'a> ParentsIterator<'a> {
    #[inline]
    pub fn new(graph: &'a DirectedDenseAdjacencyMatrix, x: usize) -> Self {
        // Assert the vertex is in bounds.
        assert!(
            graph.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Create the new `ParentsIterator` iterator.
        Self {
            graph,
            iter: graph
                .adjacency_matrix
                .column(x)
                .into_iter()
                .enumerate()
                .filter_map(|(x, &flag)|
                    // Check if the vertex is parent.
                    if flag {
                        // Return the vertex index.
                        Some(x)
                    } else {
                        // Return `None`.
                        None
                    }
                ),
        }
    }
}

// Implement the `Iterator` trait for the `ParentsIterator` iterator.
impl<'a> Iterator for ParentsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Define the `ChildrenIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
#[allow(dead_code, clippy::type_complexity)]
pub struct ChildrenIterator<'a> {
    // The graph.
    graph: &'a DirectedDenseAdjacencyMatrix,
    // The children indices iterator.
    iter: std::iter::FilterMap<
        std::iter::Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

// Implement the `ChildrenIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
impl<'a> ChildrenIterator<'a> {
    #[inline]
    pub fn new(graph: &'a DirectedDenseAdjacencyMatrix, x: usize) -> Self {
        // Assert the vertex is in bounds.
        assert!(
            graph.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Create the new `ChildrenIterator` iterator.
        Self {
            graph,
            iter: graph
                .adjacency_matrix
                .row(x)
                .into_iter()
                .enumerate()
                .filter_map(|(x, &flag)|
                    // Check if the vertex is child.
                    if flag {
                        // Return the vertex index.
                        Some(x)
                    } else {
                        // Return `None`.
                        None
                    }
                ),
        }
    }
}

// Implement the `Iterator` trait for the `ChildrenIterator` iterator.
impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Define the `DescendantsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
#[allow(dead_code, clippy::type_complexity)]
pub struct DescendantsIterator<'a> {
    // The graph.
    graph: &'a DirectedDenseAdjacencyMatrix,
    // The descendants indices iterator.
    iter: std::iter::FilterMap<
        std::iter::Enumerate<
            <ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter,
        >,
        fn((usize, bool)) -> Option<usize>,
    >,
}

// Implement the `DescendantsIterator` iterator for the `DirectedDenseAdjacencyMatrix` struct.
impl<'a> DescendantsIterator<'a> {
    pub fn new(graph: &'a DirectedDenseAdjacencyMatrix, x: usize) -> Self {
        // Assert the vertex is in bounds.
        assert!(
            graph.has_vertex(x),
            "The vertex index `{}` is out of bounds.",
            x
        );

        // Create the new `DescendantsIterator` iterator.
        Self {
            graph,
            iter: {
                // Get underlying adjacency matrix.
                let adjacency_matrix = &graph.adjacency_matrix;
                // Initialize previous solution.
                let mut prev = Array1::from_elem((adjacency_matrix.ncols(),), false);
                // Get current descendant set, i.e. children set.
                let mut curr = adjacency_matrix.row(x).to_owned();

                // Check stopping criterion.
                while curr != prev {
                    // Update previous.
                    prev.assign(&curr);
                    // Select current children.
                    let next = &adjacency_matrix.t() & &curr;
                    // Collapse new children.
                    let next = next.fold_axis(Axis(1), false, |acc, f| acc | f);
                    // Accumulate new children.
                    curr = curr | next;
                }

                curr.into_iter().enumerate().filter_map(|(x, flag)|
                    // Check if the vertex is descendant.
                    if flag {
                        // Return the vertex index.
                        Some(x)
                    } else {
                        // Return `None`.
                        None
                    }
                )
            },
        }
    }
}

// Implement the `Iterator` trait for the `DescendantsIterator` iterator.
impl<'a> Iterator for DescendantsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Implement the `DirectedGraph` trait for the `DirectedDenseAdjacencyMatrix` struct.
impl DirectedGraph for DirectedDenseAdjacencyMatrix {
    // Directed edges indices iterator associated type.
    type DirectedEdgesIter<'a> = EdgesIterator<'a>;
    // Ancestors indices iterator associated type.
    type AncestorsIter<'a> = AncestorsIterator<'a>;
    // Parents indices iterator associated type.
    type ParentsIter<'a> = ParentsIterator<'a>;
    // Children indices iterator associated type.
    type ChildrenIter<'a> = ChildrenIterator<'a>;
    // Descendants indices iterator associated type.
    type DescendantsIter<'a> = DescendantsIterator<'a>;

    // Get the undirected graph size.
    #[inline]
    fn directed_size(&self) -> usize {
        // Delegate to the `size` method.
        self.size()
    }

    // Get the graph directed edges indices iterator.
    #[inline]
    fn directed_edges(&self) -> Self::DirectedEdgesIter<'_> {
        // Delegate to the `edges` method.
        self.edges()
    }

    // Check if the directed edge exists.
    #[inline]
    fn has_directed_edge(&self, x: usize, y: usize) -> bool {
        // Delegate to the `has_edge` method.
        self.has_edge(x, y)
    }

    // Add a directed edge.
    #[inline]
    fn add_directed_edge(&mut self, x: usize, y: usize) -> bool {
        // Delegate to the `add_edge` method.
        self.add_edge(x, y)
    }

    // Delete a directed edge.
    #[inline]
    fn del_directed_edge(&mut self, x: usize, y: usize) -> bool {
        // Delegate to the `del_edge` method.
        self.del_edge(x, y)
    }

    // Get the vertex in-degree.
    #[inline]
    fn in_degree(&self, x: usize) -> usize {
        // Sum the column of the adjacency matrix.
        self.adjacency_matrix.column(x).mapv(|x| x as usize).sum()
    }

    // Get the vertices in-degrees.
    #[inline]
    fn in_degrees(&self) -> Vec<usize> {
        // Sum the columns of the adjacency matrix.
        self.adjacency_matrix
            .mapv(|x| x as usize)
            .sum_axis(Axis(0))
            .to_vec()
    }

    // Get the vertex out-degree.
    #[inline]
    fn out_degree(&self, x: usize) -> usize {
        // Sum the row of the adjacency matrix.
        self.adjacency_matrix.row(x).mapv(|x| x as usize).sum()
    }

    // Get the vertices out-degrees.
    #[inline]
    fn out_degrees(&self) -> Vec<usize> {
        // Sum the rows of the adjacency matrix.
        self.adjacency_matrix
            .mapv(|x| x as usize)
            .sum_axis(Axis(1))
            .to_vec()
    }

    // Get the vertex ancestors indices iterator.
    #[inline]
    fn ancestors(&self, x: usize) -> Self::AncestorsIter<'_> {
        // Return the vertex ancestors indices iterator.
        AncestorsIterator::new(self, x)
    }

    // Check if the vertex is an ancestor of a vertex.
    #[inline]
    fn is_ancestor(&self, x: usize, y: usize) -> bool {
        // Check if the vertex is an ancestor of a vertex.
        self.ancestors(y).any(|z| z == x)
    }

    // Get the vertex parents indices iterator.
    #[inline]
    fn parents(&self, x: usize) -> Self::ParentsIter<'_> {
        // Return the vertex parents indices iterator.
        ParentsIterator::new(self, x)
    }

    // Check if the vertex is a parent of a vertex.
    #[inline]
    fn is_parent(&self, x: usize, y: usize) -> bool {
        // Check if the vertex is a parent of a vertex.
        self.adjacency_matrix[[y, x]]
    }

    // Get the vertex children indices iterator.
    #[inline]
    fn children(&self, x: usize) -> Self::ChildrenIter<'_> {
        // Return the vertex children indices iterator.
        ChildrenIterator::new(self, x)
    }

    // Check if the vertex is a child of a vertex.
    #[inline]
    fn is_child(&self, x: usize, y: usize) -> bool {
        // Check if the vertex is a child of a vertex.
        self.adjacency_matrix[[x, y]]
    }

    // Get the vertex descendants indices iterator.
    #[inline]
    fn descendants(&self, x: usize) -> Self::DescendantsIter<'_> {
        // Return the vertex descendants indices iterator.
        DescendantsIterator::new(self, x)
    }

    // Check if the vertex is a descendant of a vertex.
    #[inline]
    fn is_descendant(&self, x: usize, y: usize) -> bool {
        // Check if the vertex is a descendant of a vertex.
        self.descendants(y).any(|z| z == x)
    }
}

/// Alias for the `DirectedDenseAdjacencyMatrix` struct.
pub type DGraph = DirectedDenseAdjacencyMatrix;

// Test the `DirectedDenseAdjacencyMatrix` struct.
#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use itertools::Itertools;

    use super::DGraph;
    use crate::{
        graphs::{DirectedGraph, Graph},
        Adj, E, L, V,
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
        let graph = DGraph::new(vertices.clone(), edges.clone());
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
        let graph = DGraph::new(vertices.clone(), edges.clone());
        // Print the graph.
        assert_eq!(
            format!("{:?}", graph),
            concat!(
                "DirectedDenseAdjacencyMatrix { ",
                "adjacency_matrix: ",
                "[[false, true, true, true, false, false, false, false],\n ",
                "[false, false, true, true, false, false, false, false],\n ",
                "[false, false, false, true, false, false, false, false],\n ",
                "[false, false, false, false, true, true, false, false],\n ",
                "[false, false, false, false, false, true, true, true],\n ",
                "[false, false, false, false, false, false, true, true],\n ",
                "[false, false, false, false, false, false, false, true],\n ",
                "[false, false, false, false, false, false, false, false]], ",
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
        let graph = DGraph::default();

        // Check the graph order.
        assert_eq!(graph.order(), 0);
        // Check the graph size.
        assert_eq!(graph.size(), 0);
        // Check the vertices labels.
        assert_eq!(graph.labels().collect_vec(), Vec::<&str>::new());
        // Check the vertices indices.
        assert_eq!(graph.vertices().collect_vec(), Vec::<usize>::new());
        // Check the edges indices.
        assert_eq!(graph.edges().collect_vec(), Vec::<(usize, usize)>::new());
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
        let graph = DGraph::new(vertices.clone(), edges.clone());
        // Print the graph.
        assert_eq!(
            format!("{}", graph),
            concat!(
                "DirectedGraph { ",
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
        let graph = DGraph::new(vertices.clone(), edges.clone());
        // Assert the graph is equal to the cloned graph.
        assert_eq!(graph, graph.clone());

        // Create a new graph.
        let mut graph_i = DGraph::new(vertices.clone(), edges.clone());

        // Delete a vertex.
        graph_i.del_vertex(0);
        // Assert the graph is not equal to the modified graph.
        assert_ne!(graph, graph_i);

        // Create a new graph.
        let mut graph_i = DGraph::new(vertices.clone(), edges.clone());

        // Delete an edge.
        graph_i.del_edge(0, 1);
        // Assert the graph is not equal to the modified graph.
        assert_ne!(graph, graph_i);
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
        let graph = DGraph::new(vertices.clone(), edges.clone());
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
        let mut graph_i = DGraph::new(vertices.clone(), edges.clone());

        // Delete a vertex.
        graph_i.del_vertex(0);
        // Initialize the hasher.
        let mut hasher_i = DefaultHasher::new();
        // Hash the modified graph.
        graph_i.hash(&mut hasher_i);
        // Assert the hashes are not equal.
        assert_ne!(hasher.finish(), hasher_i.finish());

        // Create a new graph.
        let mut graph_i = DGraph::new(vertices.clone(), edges.clone());

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
        let graph = DGraph::new(vertices.clone(), edges.clone());
        // Serialize and deserialize the graph.
        let graph =
            serde_json::from_str::<DGraph>(&serde_json::to_string(&graph).unwrap()).unwrap();

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

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
                (6, 7),
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
        let graph = DGraph::null();

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
        let graph = DGraph::empty(vertices.clone());

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
        let graph = DGraph::complete(vertices.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
        // Check the graph size.
        assert_eq!(graph.size(), vertices.len() * (vertices.len() - 1));
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
                (1, 0),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (2, 0),
                (2, 1),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (2, 7),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 4),
                (3, 5),
                (3, 6),
                (3, 7),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 5),
                (4, 6),
                (4, 7),
                (5, 0),
                (5, 1),
                (5, 2),
                (5, 3),
                (5, 4),
                (5, 6),
                (5, 7),
                (6, 0),
                (6, 1),
                (6, 2),
                (6, 3),
                (6, 4),
                (6, 5),
                (6, 7),
                (7, 0),
                (7, 1),
                (7, 2),
                (7, 3),
                (7, 4),
                (7, 5),
                (7, 6)
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
    fn labels() {
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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the vertices labels.
        assert_eq!(L!(graph).collect_vec(), vertices);
    }

    // Test the `vertex_to_label` method, should panic.
    #[test]
    #[should_panic]
    fn vertex_to_label_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

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
        let graph = DGraph::null();

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

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

    // Test the `order` method.
    #[test]
    fn order() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len());
    }

    // Test the `vertices` method.
    #[test]
    fn vertices() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the vertices indices.
        assert_eq!(V!(graph).collect_vec(), (0..vertices.len()).collect_vec());
    }

    // Test the `has_vertex` method.
    #[test]
    fn has_vertex() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

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
        let mut graph = DGraph::empty(vertices.clone());

        // Add a vertex.
        graph.add_vertex("E");

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() + 1);
        // Check the graph size.
        assert_eq!(graph.size(), 0);
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
    }

    // Test the `del_vertex` method.
    #[test]
    fn del_vertex() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let mut graph = DGraph::empty(vertices.clone());

        // Delete a vertex.
        graph.del_vertex(0);

        // Check the graph order.
        assert_eq!(graph.order(), vertices.len() - 1);
        // Check the graph size.
        assert_eq!(graph.size(), 0);
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
    }

    // Test the `size` method.
    #[test]
    fn size() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the graph size.
        assert_eq!(graph.size(), edges.len());
    }

    // Test the `edges` method.
    #[test]
    fn edges() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the edges indices.
        assert_eq!(E!(graph).collect_vec(), Vec::<(usize, usize)>::new());

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the edges indices.
        assert_eq!(
            E!(graph).collect_vec(),
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]
        );
    }

    // Test the `has_edge` method, should panic.
    #[test]
    #[should_panic]
    fn has_edge_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

        // Check if the edge exists.
        graph.has_edge(0, 1);
    }

    // Test the `has_edge` method.
    #[test]
    fn has_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check if the edge exists.
        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 3));

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(0, 3));
        assert!(graph.has_edge(1, 2));
        assert!(graph.has_edge(1, 3));
        assert!(graph.has_edge(2, 3));
    }

    // Test the `add_edge` method, should panic.
    #[test]
    #[should_panic]
    fn add_edge_should_panic() {
        // Create a new null graph.
        let mut graph = DGraph::null();

        // Add an edge.
        graph.add_edge(0, 1);
    }

    // Test the `add_edge` method.
    #[test]
    fn add_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G", "H"];

        // Create a new graph.
        let mut graph = DGraph::empty(vertices.clone());

        // Check the graph size.
        assert_eq!(graph.size(), 0);

        // Add a new edge.
        graph.add_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.size(), 1);
        // Check if the edge exists.
        assert!(graph.has_edge(0, 1));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 0));
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
        assert!(graph.has_edge(0, 2));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 0));
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
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(3, 3));
        // Check if the edge does not exist.
        assert!(!graph.has_edge(0, 0));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 0));
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
        let mut graph = DGraph::null();

        // Delete an edge.
        graph.del_edge(0, 1);
    }

    // Test the `del_edge` method.
    #[test]
    fn del_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let mut graph = DGraph::empty(vertices.clone());

        // Delete an edge.
        graph.del_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.size(), 0);
        // Check if the edge exists.
        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 0));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(2, 3));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(3, 1));
        assert!(!graph.has_edge(3, 2));

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
        let mut graph = DGraph::new(vertices.clone(), edges.clone());

        // Delete an edge.
        graph.del_edge(0, 1);

        // Check the graph size.
        assert_eq!(graph.size(), edges.len() - 1);
        // Check if the edge exists.
        assert!(!graph.has_edge(0, 1));
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(0, 3));
        assert!(!graph.has_edge(1, 0));
        assert!(graph.has_edge(1, 2));
        assert!(graph.has_edge(1, 3));
        assert!(!graph.has_edge(2, 0));
        assert!(!graph.has_edge(2, 1));
        assert!(graph.has_edge(2, 3));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(3, 1));
        assert!(!graph.has_edge(3, 2));
    }

    // Test the `degree` method, should panic.
    #[test]
    #[should_panic]
    fn degree_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

        // Check the vertex degree.
        graph.degree(0);
    }

    // Test the `degree` method.
    #[test]
    fn degree() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the vertex degree.
        assert_eq!(graph.degree(0), 3);
        assert_eq!(graph.degree(1), 3);
        assert_eq!(graph.degree(2), 3);
        assert_eq!(graph.degree(3), 3);
    }

    // Test the `degrees` method.
    #[test]
    fn degrees() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), vec![0, 0, 0, 0]);

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the vertices degrees.
        assert_eq!(graph.degrees(), vec![3, 3, 3, 3]);
    }

    // Test the `adjacents` method, should panic.
    #[test]
    #[should_panic]
    fn adjacents_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

        // Check the adjacents indices.
        Adj!(graph, 0).collect_vec();
    }

    // Test the `adjacents` method.
    #[test]
    fn adjacents() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the adjacents indices.
        assert_eq!(Adj!(graph, 0).collect_vec(), vec![1, 2, 3]);
        assert_eq!(Adj!(graph, 1).collect_vec(), vec![0, 2, 3]);
        assert_eq!(Adj!(graph, 2).collect_vec(), vec![0, 1, 3]);
        assert_eq!(Adj!(graph, 3).collect_vec(), vec![0, 1, 2]);
    }

    // Test the `is_adjacent` method, should panic.
    #[test]
    #[should_panic]
    fn is_adjacent_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

        // Check if the vertices are adjacent.
        graph.is_adjacent(0, 1);
    }

    // Test the `is_adjacent` method.
    #[test]
    fn is_adjacent() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check if the vertices are adjacent.
        assert!(!graph.is_adjacent(0, 1));
        assert!(!graph.is_adjacent(0, 2));
        assert!(!graph.is_adjacent(0, 3));
        assert!(!graph.is_adjacent(0, 4));
        assert!(!graph.is_adjacent(1, 0));
        assert!(!graph.is_adjacent(1, 2));
        assert!(!graph.is_adjacent(1, 3));
        assert!(!graph.is_adjacent(1, 4));
        assert!(!graph.is_adjacent(2, 0));
        assert!(!graph.is_adjacent(2, 1));
        assert!(!graph.is_adjacent(2, 3));
        assert!(!graph.is_adjacent(2, 4));
        assert!(!graph.is_adjacent(3, 0));
        assert!(!graph.is_adjacent(3, 1));
        assert!(!graph.is_adjacent(3, 2));
        assert!(!graph.is_adjacent(3, 4));
        assert!(!graph.is_adjacent(4, 0));
        assert!(!graph.is_adjacent(4, 1));
        assert!(!graph.is_adjacent(4, 2));
        assert!(!graph.is_adjacent(4, 3));

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check if the vertices are adjacent.
        assert!(graph.is_adjacent(0, 1));
        assert!(graph.is_adjacent(0, 2));
        assert!(graph.is_adjacent(0, 3));
        assert!(graph.is_adjacent(0, 4));
        assert!(graph.is_adjacent(1, 0));
        assert!(graph.is_adjacent(1, 2));
        assert!(graph.is_adjacent(1, 3));
        assert!(graph.is_adjacent(1, 4));
        assert!(graph.is_adjacent(2, 0));
        assert!(graph.is_adjacent(2, 1));
        assert!(graph.is_adjacent(2, 3));
        assert!(graph.is_adjacent(2, 4));
        assert!(graph.is_adjacent(3, 0));
        assert!(graph.is_adjacent(3, 1));
        assert!(graph.is_adjacent(3, 2));
        assert!(graph.is_adjacent(3, 4));
        assert!(graph.is_adjacent(4, 0));
        assert!(graph.is_adjacent(4, 1));
        assert!(graph.is_adjacent(4, 2));
        assert!(graph.is_adjacent(4, 3));
    }

    // Test the `directed_size` method.
    #[test]
    fn directed_size() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the graph directed size.
        assert_eq!(graph.directed_size(), 0);

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the graph directed size.
        assert_eq!(graph.directed_size(), edges.len());
    }

    // Test the `directed_edges` method.
    #[test]
    fn directed_edges() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the edges indices.
        assert_eq!(
            graph.directed_edges().collect_vec(),
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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the edges indices.
        assert_eq!(
            graph.directed_edges().collect_vec(),
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

    // Test the `has_directed_edge` method, should panic.
    #[test]
    #[should_panic]
    fn has_directed_edge_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

        // Check if the edge exists.
        graph.has_directed_edge(0, 1);
    }

    // Test the `has_directed_edge` method.
    #[test]
    fn has_directed_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check if the edge exists.
        assert!(!graph.has_directed_edge(0, 1));
        assert!(!graph.has_directed_edge(0, 2));
        assert!(!graph.has_directed_edge(0, 3));
        assert!(!graph.has_directed_edge(0, 4));
        assert!(!graph.has_directed_edge(0, 5));
        assert!(!graph.has_directed_edge(1, 0));
        assert!(!graph.has_directed_edge(1, 2));
        assert!(!graph.has_directed_edge(1, 3));
        assert!(!graph.has_directed_edge(1, 4));
        assert!(!graph.has_directed_edge(1, 5));
        assert!(!graph.has_directed_edge(2, 0));
        assert!(!graph.has_directed_edge(2, 1));
        assert!(!graph.has_directed_edge(2, 3));
        assert!(!graph.has_directed_edge(2, 4));
        assert!(!graph.has_directed_edge(2, 5));
        assert!(!graph.has_directed_edge(3, 0));
        assert!(!graph.has_directed_edge(3, 1));
        assert!(!graph.has_directed_edge(3, 2));
        assert!(!graph.has_directed_edge(3, 4));
        assert!(!graph.has_directed_edge(3, 5));
        assert!(!graph.has_directed_edge(4, 0));
        assert!(!graph.has_directed_edge(4, 1));
        assert!(!graph.has_directed_edge(4, 2));
        assert!(!graph.has_directed_edge(4, 3));
        assert!(!graph.has_directed_edge(4, 5));
        assert!(!graph.has_directed_edge(5, 0));
        assert!(!graph.has_directed_edge(5, 1));
        assert!(!graph.has_directed_edge(5, 2));
        assert!(!graph.has_directed_edge(5, 3));
        assert!(!graph.has_directed_edge(5, 4));

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("A", "E"),
            ("A", "F"),
            ("B", "C"),
            ("B", "D"),
            ("B", "E"),
            ("B", "F"),
            ("C", "D"),
            ("C", "E"),
            ("C", "F"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
        ];

        // Create a new graph.
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check if the edge exists.
        assert!(graph.has_directed_edge(0, 1));
        assert!(graph.has_directed_edge(0, 2));
        assert!(graph.has_directed_edge(0, 3));
        assert!(graph.has_directed_edge(0, 4));
        assert!(graph.has_directed_edge(0, 5));
        assert!(graph.has_directed_edge(1, 2));
        assert!(graph.has_directed_edge(1, 3));
        assert!(graph.has_directed_edge(1, 4));
        assert!(graph.has_directed_edge(1, 5));
        assert!(graph.has_directed_edge(2, 3));
        assert!(graph.has_directed_edge(2, 4));
        assert!(graph.has_directed_edge(2, 5));
        assert!(graph.has_directed_edge(3, 4));
        assert!(graph.has_directed_edge(3, 5));
        assert!(graph.has_directed_edge(4, 5));
        // Check if the edge does not exist.
        assert!(!graph.has_directed_edge(1, 0));
        assert!(!graph.has_directed_edge(2, 0));
        assert!(!graph.has_directed_edge(3, 0));
        assert!(!graph.has_directed_edge(4, 0));
        assert!(!graph.has_directed_edge(5, 0));
        assert!(!graph.has_directed_edge(2, 1));
        assert!(!graph.has_directed_edge(3, 1));
        assert!(!graph.has_directed_edge(4, 1));
        assert!(!graph.has_directed_edge(5, 1));
        assert!(!graph.has_directed_edge(3, 2));
        assert!(!graph.has_directed_edge(4, 2));
        assert!(!graph.has_directed_edge(5, 2));
        assert!(!graph.has_directed_edge(4, 3));
        assert!(!graph.has_directed_edge(5, 3));
        assert!(!graph.has_directed_edge(5, 4));
    }

    // Test the `add_directed_edge` method, should panic.
    #[test]
    #[should_panic]
    fn add_directed_edge_should_panic() {
        // Create a new null graph.
        let mut graph = DGraph::null();

        // Add a directed edge.
        graph.add_directed_edge(0, 1);
    }

    // Test the `add_directed_edge` method.
    #[test]
    fn add_directed_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G"];

        // Create a new graph.
        let mut graph = DGraph::empty(vertices.clone());

        // Check the graph directed size.
        assert_eq!(graph.directed_size(), 0);

        // Add a new directed edge.
        graph.add_directed_edge(0, 1);

        // Check the graph directed size.
        assert_eq!(graph.directed_size(), 1);
        // Check if the edge exists.
        assert!(graph.has_directed_edge(0, 1));
        // Check if the edge does not exist.
        assert!(!graph.has_directed_edge(0, 0));
        assert!(!graph.has_directed_edge(0, 2));
        assert!(!graph.has_directed_edge(0, 3));
        assert!(!graph.has_directed_edge(0, 4));
        assert!(!graph.has_directed_edge(0, 5));
        assert!(!graph.has_directed_edge(0, 6));
        assert!(!graph.has_directed_edge(1, 0));
        assert!(!graph.has_directed_edge(1, 2));
        assert!(!graph.has_directed_edge(1, 3));
        assert!(!graph.has_directed_edge(1, 4));
        assert!(!graph.has_directed_edge(1, 5));
        assert!(!graph.has_directed_edge(1, 6));
        assert!(!graph.has_directed_edge(2, 0));
        assert!(!graph.has_directed_edge(2, 1));
        assert!(!graph.has_directed_edge(2, 3));
        assert!(!graph.has_directed_edge(2, 4));
        assert!(!graph.has_directed_edge(2, 5));
        assert!(!graph.has_directed_edge(2, 6));
        assert!(!graph.has_directed_edge(3, 0));
        assert!(!graph.has_directed_edge(3, 1));
        assert!(!graph.has_directed_edge(3, 2));
        assert!(!graph.has_directed_edge(3, 4));
        assert!(!graph.has_directed_edge(3, 5));
        assert!(!graph.has_directed_edge(3, 6));
        assert!(!graph.has_directed_edge(4, 0));
        assert!(!graph.has_directed_edge(4, 1));
        assert!(!graph.has_directed_edge(4, 2));
        assert!(!graph.has_directed_edge(4, 3));
        assert!(!graph.has_directed_edge(4, 5));
        assert!(!graph.has_directed_edge(4, 6));
        assert!(!graph.has_directed_edge(5, 0));
        assert!(!graph.has_directed_edge(5, 1));
        assert!(!graph.has_directed_edge(5, 2));
        assert!(!graph.has_directed_edge(5, 3));
        assert!(!graph.has_directed_edge(5, 4));
        assert!(!graph.has_directed_edge(5, 6));
        assert!(!graph.has_directed_edge(6, 0));
        assert!(!graph.has_directed_edge(6, 1));
        assert!(!graph.has_directed_edge(6, 2));
        assert!(!graph.has_directed_edge(6, 3));
        assert!(!graph.has_directed_edge(6, 4));
        assert!(!graph.has_directed_edge(6, 5));
    }

    // Test the `del_directed_edge` method, should panic.
    #[test]
    #[should_panic]
    fn del_directed_edge_should_panic() {
        // Create a new null graph.
        let mut graph = DGraph::null();

        // Delete a directed edge.
        graph.del_directed_edge(0, 1);
    }

    // Test the `del_directed_edge` method.
    #[test]
    fn del_directed_edge() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D", "E", "F", "G"];

        // Create a new graph.
        let mut graph = DGraph::empty(vertices.clone());

        // Delete a directed edge.
        graph.del_directed_edge(0, 1);

        // Check the graph directed size.
        assert_eq!(graph.directed_size(), 0);
        // Check if the edge exists.
        assert!(!graph.has_directed_edge(0, 1));
        // Check if the edge does not exist.
        assert!(!graph.has_directed_edge(0, 0));
        assert!(!graph.has_directed_edge(0, 2));
        assert!(!graph.has_directed_edge(0, 3));
        assert!(!graph.has_directed_edge(0, 4));
        assert!(!graph.has_directed_edge(0, 5));
        assert!(!graph.has_directed_edge(0, 6));
        assert!(!graph.has_directed_edge(1, 0));
        assert!(!graph.has_directed_edge(1, 2));
        assert!(!graph.has_directed_edge(1, 3));
        assert!(!graph.has_directed_edge(1, 4));
        assert!(!graph.has_directed_edge(1, 5));
        assert!(!graph.has_directed_edge(1, 6));
        assert!(!graph.has_directed_edge(2, 0));
        assert!(!graph.has_directed_edge(2, 1));
        assert!(!graph.has_directed_edge(2, 3));
        assert!(!graph.has_directed_edge(2, 4));
        assert!(!graph.has_directed_edge(2, 5));
        assert!(!graph.has_directed_edge(2, 6));
        assert!(!graph.has_directed_edge(3, 0));
        assert!(!graph.has_directed_edge(3, 1));
        assert!(!graph.has_directed_edge(3, 2));
        assert!(!graph.has_directed_edge(3, 4));
        assert!(!graph.has_directed_edge(3, 5));
        assert!(!graph.has_directed_edge(3, 6));
        assert!(!graph.has_directed_edge(4, 0));
        assert!(!graph.has_directed_edge(4, 1));
        assert!(!graph.has_directed_edge(4, 2));
        assert!(!graph.has_directed_edge(4, 3));
        assert!(!graph.has_directed_edge(4, 5));
        assert!(!graph.has_directed_edge(4, 6));
        assert!(!graph.has_directed_edge(5, 0));
        assert!(!graph.has_directed_edge(5, 1));
        assert!(!graph.has_directed_edge(5, 2));
        assert!(!graph.has_directed_edge(5, 3));
        assert!(!graph.has_directed_edge(5, 4));
        assert!(!graph.has_directed_edge(5, 6));
        assert!(!graph.has_directed_edge(6, 0));
        assert!(!graph.has_directed_edge(6, 1));
        assert!(!graph.has_directed_edge(6, 2));
        assert!(!graph.has_directed_edge(6, 3));
        assert!(!graph.has_directed_edge(6, 4));
        assert!(!graph.has_directed_edge(6, 5));

        // Initialize the edges labels.
        let edges = vec![
            ("A", "B"),
            ("A", "C"),
            ("A", "D"),
            ("A", "E"),
            ("A", "F"),
            ("B", "C"),
            ("B", "D"),
            ("B", "E"),
            ("B", "F"),
            ("C", "D"),
            ("C", "E"),
            ("C", "F"),
            ("D", "E"),
            ("D", "F"),
            ("E", "F"),
        ];

        // Create a new graph.
        let mut graph = DGraph::new(vertices.clone(), edges.clone());

        // Delete a directed edge.
        graph.del_directed_edge(0, 1);

        // Check the graph directed size.
        assert_eq!(graph.directed_size(), edges.len() - 1);
        // Check if the edge exists.
        assert!(graph.has_directed_edge(0, 2));
        assert!(graph.has_directed_edge(0, 3));
        assert!(graph.has_directed_edge(0, 4));
        assert!(graph.has_directed_edge(0, 5));
        assert!(graph.has_directed_edge(1, 2));
        assert!(graph.has_directed_edge(1, 3));
        assert!(graph.has_directed_edge(1, 4));
        assert!(graph.has_directed_edge(1, 5));
        assert!(graph.has_directed_edge(2, 3));
        assert!(graph.has_directed_edge(2, 4));
        assert!(graph.has_directed_edge(2, 5));
        assert!(graph.has_directed_edge(3, 4));
        assert!(graph.has_directed_edge(3, 5));
        assert!(graph.has_directed_edge(4, 5));
        // Check if the edge does not exist.
        assert!(!graph.has_directed_edge(0, 0));
        assert!(!graph.has_directed_edge(0, 1));
        assert!(!graph.has_directed_edge(0, 6));
        assert!(!graph.has_directed_edge(1, 0));
        assert!(!graph.has_directed_edge(1, 6));
        assert!(!graph.has_directed_edge(2, 0));
        assert!(!graph.has_directed_edge(2, 1));
        assert!(!graph.has_directed_edge(2, 6));
        assert!(!graph.has_directed_edge(3, 0));
        assert!(!graph.has_directed_edge(3, 1));
        assert!(!graph.has_directed_edge(3, 2));
        assert!(!graph.has_directed_edge(3, 6));
        assert!(!graph.has_directed_edge(4, 0));
        assert!(!graph.has_directed_edge(4, 1));
        assert!(!graph.has_directed_edge(4, 2));
        assert!(!graph.has_directed_edge(4, 3));
        assert!(!graph.has_directed_edge(4, 6));
        assert!(!graph.has_directed_edge(5, 0));
        assert!(!graph.has_directed_edge(5, 1));
        assert!(!graph.has_directed_edge(5, 2));
        assert!(!graph.has_directed_edge(5, 3));
        assert!(!graph.has_directed_edge(5, 4));
        assert!(!graph.has_directed_edge(5, 6));
        assert!(!graph.has_directed_edge(6, 0));
        assert!(!graph.has_directed_edge(6, 1));
        assert!(!graph.has_directed_edge(6, 2));
        assert!(!graph.has_directed_edge(6, 3));
        assert!(!graph.has_directed_edge(6, 4));
        assert!(!graph.has_directed_edge(6, 5));
    }

    // Test the `in_degree` method, should panic.
    #[test]
    #[should_panic]
    fn in_degree_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

        // Check the vertex in-degree.
        graph.in_degree(0);
    }

    // Test the `in_degree` method.
    #[test]
    fn in_degree() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the vertex in-degree.
        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 0);
        assert_eq!(graph.in_degree(2), 0);
        assert_eq!(graph.in_degree(3), 0);

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the vertex in-degree.
        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 2);
        assert_eq!(graph.in_degree(3), 3);
    }

    // Test the `in_degrees` method.
    #[test]
    fn in_degrees() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the vertices in-degrees.
        assert_eq!(graph.in_degrees(), vec![0, 0, 0, 0]);

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the vertices in-degrees.
        assert_eq!(graph.in_degrees(), vec![0, 1, 2, 3]);
    }

    // Test the `out_degree` method, should panic.
    #[test]
    #[should_panic]
    fn out_degree_should_panic() {
        // Create a new null graph.
        let graph = DGraph::null();

        // Check the vertex out-degree.
        graph.out_degree(0);
    }

    // Test the `out_degree` method.
    #[test]
    fn out_degree() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the vertex out-degree.
        assert_eq!(graph.out_degree(0), 0);
        assert_eq!(graph.out_degree(1), 0);
        assert_eq!(graph.out_degree(2), 0);
        assert_eq!(graph.out_degree(3), 0);

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the vertex out-degree.
        assert_eq!(graph.out_degree(0), 3);
        assert_eq!(graph.out_degree(1), 2);
        assert_eq!(graph.out_degree(2), 1);
        assert_eq!(graph.out_degree(3), 0);
    }

    // Test the `out_degrees` method.
    #[test]
    fn out_degrees() {
        // Initialize the vertices labels.
        let vertices = vec!["A", "B", "C", "D"];

        // Create a new graph.
        let graph = DGraph::empty(vertices.clone());

        // Check the vertices out-degrees.
        assert_eq!(graph.out_degrees(), vec![0, 0, 0, 0]);

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
        let graph = DGraph::new(vertices.clone(), edges.clone());

        // Check the vertices out-degrees.
        assert_eq!(graph.out_degrees(), vec![3, 2, 1, 0]);
    }
}
