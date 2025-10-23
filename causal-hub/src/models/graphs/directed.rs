use std::collections::VecDeque;

use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    impl_json_io,
    models::{Graph, Labelled},
    set,
    types::{Labels, Set},
};

/// A struct representing a directed graph using an adjacency matrix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiGraph {
    labels: Labels,
    adjacency_matrix: Array2<bool>,
}

impl DiGraph {
    /// Returns the parents of a set of vertices.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of vertices for which to find the parents.
    ///
    /// # Panics
    ///
    /// * If any vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// The parents of the vertices.
    ///
    pub fn parents(&self, x: &Set<usize>) -> Set<usize> {
        // Assert the vertices are within bounds.
        x.iter().for_each(|&v| {
            assert!(v < self.labels.len(), "Vertex `{v}` is out of bounds");
        });

        // Iterate over all vertices and filter the ones that are parents.
        let mut parents: Set<_> = x
            .into_iter()
            .flat_map(|&v| {
                self.adjacency_matrix
                    .column(v)
                    .into_iter()
                    .enumerate()
                    .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            })
            .collect();

        // Sort the parents.
        parents.sort();

        // Return the parents.
        parents
    }

    /// Returns the ancestors of a set of vertices.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of vertices for which to find the ancestors.
    ///
    /// # Panics
    ///
    /// * If any vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// The ancestors of the vertices.
    ///
    pub fn ancestors(&self, x: &Set<usize>) -> Set<usize> {
        // Assert the vertices are within bounds.
        x.iter().for_each(|&v| {
            assert!(v < self.labels.len(), "Vertex `{v}` is out of bounds");
        });

        // Initialize a stack and a visited set.
        let mut stack = VecDeque::new();
        let mut visited = set![];

        // Start with the given vertices.
        stack.extend(x);

        // While there are vertices to visit ...
        while let Some(y) = stack.pop_back() {
            // For each incoming edge ...
            for z in self.parents(&set![y]) {
                // If there is an edge from z to y and z has not been visited ...
                if !visited.contains(&z) {
                    // Mark z as visited.
                    visited.insert(z);
                    // Add z to the stack to visit its ancestors.
                    stack.push_back(z);
                }
            }
        }

        // Sort the visited set.
        visited.sort();

        // Return the visited set.
        visited
    }

    /// Returns the children of a set of vertices.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of vertices for which to find the children.
    ///
    /// # Panics
    ///
    /// * If any vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// The children of the vertices.
    ///
    pub fn children(&self, x: &Set<usize>) -> Set<usize> {
        // Check if the vertices are within bounds.
        x.iter().for_each(|&v| {
            assert!(v < self.labels.len(), "Vertex `{v}` is out of bounds");
        });

        // Iterate over all vertices and filter the ones that are children.
        let mut children: Set<_> = x
            .into_iter()
            .flat_map(|&v| {
                self.adjacency_matrix
                    .row(v)
                    .into_iter()
                    .enumerate()
                    .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            })
            .collect();

        // Sort the children.
        children.sort();

        // Return the children.
        children
    }

    /// Returns the descendants of a set of vertices.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of vertices for which to find the descendants.
    ///
    /// # Panics
    ///
    /// * If any vertex is out of bounds.
    ///
    /// # Returns
    ///
    /// The descendants of the vertices.
    ///
    pub fn descendants(&self, x: &Set<usize>) -> Set<usize> {
        // Assert the vertices are within bounds.
        x.iter().for_each(|&v| {
            assert!(v < self.labels.len(), "Vertex `{v}` is out of bounds");
        });

        // Initialize a stack and a visited set.
        let mut stack = VecDeque::new();
        let mut visited = set![];

        // Start with the given vertices.
        stack.extend(x);

        // While there are vertices to visit ...
        while let Some(y) = stack.pop_back() {
            // For each outgoing edge ...
            for z in self.children(&set![y]) {
                // If z has not been visited ...
                if !visited.contains(&z) {
                    // Mark z as visited.
                    visited.insert(z);
                    // Add z to the stack to visit its descendants.
                    stack.push_back(z);
                }
            }
        }

        // Sort the visited set.
        visited.sort();

        // Return the visited set.
        visited
    }
}

impl Labelled for DiGraph {
    fn labels(&self) -> &Labels {
        &self.labels
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

    fn from_adjacency_matrix(mut labels: Labels, mut adjacency_matrix: Array2<bool>) -> Self {
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

        // Check if the labels are sorted.
        if !labels.is_sorted() {
            // Allocate the sorted indices.
            let mut indices: Vec<usize> = (0..labels.len()).collect();
            // Sort the indices based on the labels.
            indices.sort_by_key(|&i| &labels[i]);
            // Sort the labels.
            labels.sort();
            // Allocate a new adjacency matrix.
            let mut new_adjacency_matrix = adjacency_matrix.clone();
            // Fill the rows.
            for (i, &j) in indices.iter().enumerate() {
                new_adjacency_matrix
                    .row_mut(i)
                    .assign(&adjacency_matrix.row(j));
            }
            // Update the adjacency matrix.
            adjacency_matrix = new_adjacency_matrix;
            // Allocate a new adjacency matrix.
            let mut new_adjacency_matrix = adjacency_matrix.clone();
            // Fill the columns.
            for (i, &j) in indices.iter().enumerate() {
                new_adjacency_matrix
                    .column_mut(i)
                    .assign(&adjacency_matrix.column(j));
            }
            // Update the adjacency matrix.
            adjacency_matrix = new_adjacency_matrix;
        }

        // Create a new graph instance.
        Self {
            labels,
            adjacency_matrix,
        }
    }

    #[inline]
    fn to_adjacency_matrix(&self) -> Array2<bool> {
        self.adjacency_matrix.clone()
    }
}

impl Serialize for DiGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert adjacency matrix to a flat format.
        let edges: Vec<_> = self
            .edges()
            .into_iter()
            .map(|(x, y)| {
                (
                    self.index_to_label(x).to_owned(),
                    self.index_to_label(y).to_owned(),
                )
            })
            .collect();

        // Allocate the map.
        let mut map = serializer.serialize_map(Some(3))?;

        // Serialize labels.
        map.serialize_entry("labels", &self.labels)?;
        // Serialize edges.
        map.serialize_entry("edges", &edges)?;
        // Serialize type.
        map.serialize_entry("type", "digraph")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for DiGraph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Labels,
            Edges,
            Type,
        }

        struct DiGraphVisitor;

        impl<'de> Visitor<'de> for DiGraphVisitor {
            type Value = DiGraph;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct DiGraph")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DiGraph, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate fields
                let mut labels = None;
                let mut edges = None;
                let mut type_ = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Labels => {
                            if labels.is_some() {
                                return Err(E::duplicate_field("labels"));
                            }
                            labels = Some(map.next_value()?);
                        }
                        Field::Edges => {
                            if edges.is_some() {
                                return Err(E::duplicate_field("edges"));
                            }
                            edges = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(E::duplicate_field("type"));
                            }
                            type_ = Some(map.next_value()?);
                        }
                    }
                }

                // Check required fields.
                let labels = labels.ok_or_else(|| E::missing_field("labels"))?;
                let edges = edges.ok_or_else(|| E::missing_field("edges"))?;

                // Assert type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                assert_eq!(type_, "digraph", "Invalid type for DiGraph.");

                // Convert edges to an adjacency matrix.
                let labels: Labels = labels;
                let edges: Vec<(String, String)> = edges;
                let shape = (labels.len(), labels.len());
                let mut adjacency_matrix = Array2::from_elem(shape, false);
                for (x, y) in edges {
                    let x = labels
                        .get_index_of(&x)
                        .ok_or_else(|| E::custom(format!("Vertex `{x}` label does not exist")))?;
                    let y = labels
                        .get_index_of(&y)
                        .ok_or_else(|| E::custom(format!("Vertex `{y}` label does not exist")))?;
                    adjacency_matrix[(x, y)] = true;
                }

                Ok(DiGraph::from_adjacency_matrix(labels, adjacency_matrix))
            }
        }

        const FIELDS: &[&str] = &["labels", "edges", "type"];

        deserializer.deserialize_struct("DiGraph", FIELDS, DiGraphVisitor)
    }
}

// Implement `JsonIO` for `DiGraph`.
impl_json_io!(DiGraph);
