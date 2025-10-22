use std::fmt::Display;

use itertools::Itertools;
use ndarray::prelude::*;

use crate::types::Labels;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
enum PKS {
    Unknown,
    Forbidden,
    Required,
}

impl PKS {
    #[inline]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    #[inline]
    pub const fn is_forbidden(&self) -> bool {
        matches!(self, Self::Forbidden)
    }

    #[inline]
    pub const fn is_required(&self) -> bool {
        matches!(self, Self::Required)
    }
}

impl Display for PKS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Forbidden => write!(f, "Forbidden"),
            Self::Required => write!(f, "Required"),
        }
    }
}

/// A structure representing prior knowledge for structure learning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PK {
    labels: Labels,
    adjacency_matrix: Array2<PKS>,
}

impl PK {
    /// Creates a new instance of prior knowledge.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the vertices.
    /// * `forbidden` - An iterator over forbidden edges.
    /// * `required` - An iterator over required edges.
    /// * `temporal_order` - An iterator over tiers of vertices, where each tier is an iterator of vertex indices.
    ///
    /// # Returns
    ///
    /// A new instance of prior knowledge.
    ///
    pub fn new<I, J, K, L>(labels: Labels, forbidden: I, required: J, temporal_order: K) -> Self
    where
        I: IntoIterator<Item = (usize, usize)>,
        J: IntoIterator<Item = (usize, usize)>,
        K: IntoIterator<Item = L>,
        L: IntoIterator<Item = usize>,
    {
        // FIXME: Check labels sorted.

        // Get the number of labels.
        let n = labels.len();
        // Initialize an adjacency matrix with `Unknown` state.
        let mut adjacency_matrix = Array::from_elem((n, n), PKS::Unknown);

        // Set the forbidden edges to `Forbidden`.
        forbidden.into_iter().for_each(|(i, j)| {
            // Set the edge to `Forbidden`.
            adjacency_matrix[[i, j]] = PKS::Forbidden;
        });

        // Set the required edges to `Required`.
        required.into_iter().for_each(|(i, j)| {
            // Assert that the edge is set to unknown.
            assert!(
                adjacency_matrix[[i, j]].is_unknown(),
                "Edge ({i}, {j}) is already set to a non-unknown state: \n\
                \t expected:    ({i}, {j}) set to 'Unknown', \n\",
                \t found:       ({i}, {j}) set to '{}'.",
                adjacency_matrix[[i, j]]
            );
            // Set the edge to `Required`.
            adjacency_matrix[[i, j]] = PKS::Required;
        });

        // Collect the tiered edges.
        let temporal_order: Vec<Vec<_>> = temporal_order
            .into_iter()
            .map(|tier| tier.into_iter().collect())
            .collect();
        // Edges from a vertex in a higher tier to a vertex in a lower tier are forbidden.
        temporal_order.iter().enumerate().for_each(|(t, tier)| {
            // Get the vertices in previous tiers.
            let previous_tiers = temporal_order[..t].iter().flatten();
            // For each vertex in the current tier, set edges to previous tiers as forbidden.
            tier.iter()
                .cartesian_product(previous_tiers)
                .for_each(|(&i, &j)| {
                    // Assert that the edge is not required.
                    assert!(
                        !adjacency_matrix[[i, j]].is_required(),
                        "Edge ({i}, {j}) is already set to a 'Required' state: \n\
                        \t expected:    ({i}, {j}) set to 'Unknown' or 'Forbidden', \n\",
                        \t found:       ({i}, {j}) set to '{}'.",
                        adjacency_matrix[[i, j]]
                    );
                    // Set the edge to `Forbidden`.
                    adjacency_matrix[[i, j]] = PKS::Forbidden;
                });
        });

        Self {
            labels,
            adjacency_matrix,
        }
    }

    /// Returns a reference to the labels of the prior knowledge.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    #[inline]
    pub const fn labels(&self) -> &Labels {
        &self.labels
    }

    /// Checks if an edge is unknown.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the first vertex.
    /// * `y` - The index of the second vertex.
    ///
    /// # Returns
    ///
    /// `true` if the edge is unknown, `false` otherwise.
    ///
    #[inline]
    pub fn is_unknown(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]].is_unknown()
    }

    /// Returns the unknown edges.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the indices of the unknown edges.
    ///
    pub fn unknown_edges(&self) -> Vec<(usize, usize)> {
        self.adjacency_matrix
            .indexed_iter()
            .filter_map(|((i, j), &state)| {
                if state.is_unknown() {
                    Some((i, j))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Checks if an edge is forbidden.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the first vertex.
    /// * `y` - The index of the second vertex.
    ///
    /// # Returns
    ///
    /// `true` if the edge is forbidden, `false` otherwise.
    ///
    #[inline]
    pub fn is_forbidden(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]].is_forbidden()
    }

    /// Returns the forbidden edges.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the indices of the forbidden edges.
    ///
    pub fn forbidden_edges(&self) -> Vec<(usize, usize)> {
        self.adjacency_matrix
            .indexed_iter()
            .filter_map(|((i, j), &state)| {
                if state.is_forbidden() {
                    Some((i, j))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Checks if an edge is required.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the first vertex.
    /// * `y` - The index of the second vertex.
    ///
    /// # Returns
    ///
    /// `true` if the edge is required, `false` otherwise.
    ///
    #[inline]
    pub fn is_required(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]].is_required()
    }

    /// Returns the required edges.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the indices of the required edges.
    ///
    pub fn required_edges(&self) -> Vec<(usize, usize)> {
        self.adjacency_matrix
            .indexed_iter()
            .filter_map(|((i, j), &state)| {
                if state.is_required() {
                    Some((i, j))
                } else {
                    None
                }
            })
            .collect()
    }
}
