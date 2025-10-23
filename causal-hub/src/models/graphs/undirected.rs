use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    impl_json_io,
    models::{Graph, Labelled},
    types::{Labels, Set},
};

/// A struct representing an undirected graph using an adjacency matrix.
#[derive(Clone, Debug)]
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
    /// The neighbors of the vertex.
    ///
    pub fn neighbors(&self, x: &Set<usize>) -> Set<usize> {
        // Check if the vertices are within bounds.
        x.iter().for_each(|&v| {
            assert!(v < self.labels.len(), "Vertex `{v}` is out of bounds");
        });

        // Iterate over all vertices and filter the ones that are neighbors.
        let mut neighbors: Set<_> = x
            .into_iter()
            .flat_map(|&v| {
                self.adjacency_matrix
                    .row(v)
                    .into_iter()
                    .enumerate()
                    .filter_map(|(y, &has_edge)| if has_edge { Some(y) } else { None })
            })
            .collect();

        // Sort the neighbors.
        neighbors.sort();

        // Return the neighbors.
        neighbors
    }
}

impl Labelled for UnGraph {
    fn labels(&self) -> &Labels {
        &self.labels
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
        // Assert the adjacency matrix is symmetric.
        assert_eq!(
            adjacency_matrix,
            adjacency_matrix.t(),
            "Adjacency matrix must be symmetric."
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

impl Serialize for UnGraph {
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
        map.serialize_entry("type", "ungraph")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for UnGraph {
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

        struct UnGraphVisitor;

        impl<'de> Visitor<'de> for UnGraphVisitor {
            type Value = UnGraph;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct UnGraph")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UnGraph, V::Error>
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
                assert_eq!(type_, "ungraph", "Invalid type for UnGraph.");

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

                Ok(UnGraph::from_adjacency_matrix(labels, adjacency_matrix))
            }
        }

        const FIELDS: &[&str] = &["labels", "edges", "type"];

        deserializer.deserialize_struct("UnGraph", FIELDS, UnGraphVisitor)
    }
}

// Implement `JsonIO` for `UnGraph`.
impl_json_io!(UnGraph);
