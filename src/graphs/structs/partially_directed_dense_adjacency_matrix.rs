#![allow(unused_imports, dead_code)] // FIXME: remove this line
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
    fmt::Display,
    hash::{Hash, Hasher},
    iter::{Chain, Enumerate, FilterMap, FusedIterator},
    ops::{Deref, Range},
};

use bimap::BiHashMap;
use itertools::{iproduct, Itertools};
use ndarray::{iter::IndexedIter, prelude::*, OwnedRepr};
use serde::{Deserialize, Serialize};

use super::UndirectedDenseAdjacencyMatrixGraph;
use crate::{
    dE,
    graphs::{
        algorithms::traversal::{DFSEdge, DFSEdges, Traversal},
        direction::*,
        BaseGraph, DefaultGraph, DirectedGraph, ErrorGraph as E, IntoUndirectedGraph,
        PartialOrdGraph, PathGraph, SubGraph, UndirectedGraph,
    },
    io::DOT,
    models::MoralGraph,
    prelude::BFS,
    types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, SparseAdjacencyMatrix},
    uE,
    utils::partial_cmp_sets,
    Adj, Ch, Pa, E, V,
};

/// Mixed graph struct based on a couple of dense adjacency matrix data structures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartiallyDenseAdjacencyMatrixGraph {
    labels: BTreeSet<String>,
    labels_indices: BiHashMap<String, usize>,
    undirected_adjacency_matrix: DenseAdjacencyMatrix,
    directed_adjacency_matrix: DenseAdjacencyMatrix,
    skeleton_adjacency_matrix: DenseAdjacencyMatrix,
    size: usize,
}

/* Implement BaseGraph trait. */
pub struct LabelsIterator<'a> {
    graph: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: Range<usize>,
}

impl<'a> LabelsIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph) -> Self {
        Self {
            graph: g,
            iter: Range {
                start: 0,
                end: g.labels.len(),
            },
        }
    }
}

impl<'a> Iterator for LabelsIterator<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| self.graph.label(x))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for LabelsIterator<'a> {}

impl<'a> FusedIterator for LabelsIterator<'a> {}

#[allow(dead_code, clippy::type_complexity)]
pub struct EdgesIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> EdgesIterator<'a> {
    /// Skeleton constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .skeleton_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match f && i <= j {
                    true => Some((i, j)),
                    false => None,
                }),
            size: g.size,
        }
    }
    /// Undirected edges constructor.
    #[inline]
    pub fn new_undirected(g: &'a PartiallyDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .undirected_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match f && i <= j {
                    true => Some((i, j)),
                    false => None,
                }),
            size: g.size,
        }
    }
    /// Directed edges constructor.
    #[inline]
    pub fn new_directed(g: &'a PartiallyDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .directed_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match f {
                    true => Some((i, j)),
                    false => None,
                }),
            size: g.size,
        }
    }
}

impl<'a> Iterator for EdgesIterator<'a> {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(x, y)| {
            // Decrement inner counter.
            self.size -= 1;

            (x, y)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a> ExactSizeIterator for EdgesIterator<'a> {}

impl<'a> FusedIterator for EdgesIterator<'a> {}

#[allow(dead_code, clippy::type_complexity)]
pub struct AdjacentsIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

impl<'a> AdjacentsIterator<'a> {
    /// General constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: g
                .skeleton_adjacency_matrix
                .row(x)
                .into_iter()
                .enumerate()
                .filter_map(|(i, &f)| match f {
                    true => Some(i),
                    false => None,
                }),
        }
    }
    /// Neighbours constructor.
    #[inline]
    pub fn new_undirected(g: &'a PartiallyDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: g
                .undirected_adjacency_matrix
                .row(x)
                .into_iter()
                .enumerate()
                .filter_map(|(i, &f)| match f {
                    true => Some(i),
                    false => None,
                }),
        }
    }
}

impl<'a> Iterator for AdjacentsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> FusedIterator for AdjacentsIterator<'a> {}

impl Display for PartiallyDenseAdjacencyMatrixGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "PartiallyDirectedGraph {{ ")?;
        // Write vertex set.
        write!(
            f,
            "V = {{{}}}, ",
            V!(self)
                .map(|x| format!("\"{}\"", self.label(x)))
                .join(", ")
        )?;
        // Write undirected edge set.
        write!(
            f,
            "Undirected E = {{{}}}",
            uE!(self)
                .map(|(x, y)| format!("(\"{}\", \"{}\")", self.label(x), self.label(y)))
                .join(", ")
        )?;
        // Write directed edge set.
        write!(
            f,
            "Directed E = {{{}}}",
            dE!(self)
                .map(|(x, y)| format!("(\"{}\", \"{}\")", self.label(x), self.label(y)))
                .join(", ")
        )?;
        // Write all edge set.
        write!(
            f,
            "E = {{{}}}",
            E!(self)
                .map(|(x, y)| format!("(\"{}\", \"{}\")", self.label(x), self.label(y)))
                .join(", ")
        )?;
        // Write ending character.
        write!(f, " }}")
    }
}

impl Hash for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.hash(state);
        self.directed_adjacency_matrix.hash(state);
        self.undirected_adjacency_matrix.hash(state);
        self.skeleton_adjacency_matrix.hash(state);
    }
}

impl BaseGraph for PartiallyDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::PartiallyDirected;

    type LabelsIter<'a> = LabelsIterator<'a>;

    type VerticesIter<'a> = Range<usize>;

    type EdgesIter<'a> = EdgesIterator<'a>;

    type AdjacentsIter<'a> = AdjacentsIterator<'a>;

    fn new<V, I, J>(vertices: I, edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
    {
        // Remove duplicated vertices labels.
        let mut labels: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Map edges iterator into edge list.
        let edges: EdgeList<_> = edges
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();
        // Add missing vertices from the edges.
        labels.extend(edges.iter().cloned().flat_map(|(x, y)| [x, y]));

        // Compute new graph order.
        let order = labels.len();
        // Map vertices labels to vertices indices.
        let labels_indices: BiHashMap<_, _> = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        // Initialize skeleton adjacency matrix given graph order.
        let mut skeleton_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

        // Initialize the size.
        let mut size = 0;
        // Fill skeleton adjacency matrix given edge set.
        for (x, y) in edges {
            // Get associated vertices indices.
            let (i, j) = (
                *labels_indices.get_by_left(&x).unwrap(),
                *labels_indices.get_by_left(&y).unwrap(),
            );
            // Set edge given indices.
            if !skeleton_adjacency_matrix[[i, j]] {
                // Add edge.
                skeleton_adjacency_matrix[[i, j]] = true;
                skeleton_adjacency_matrix[[j, i]] = true;
                // Increment size.
                size += 1;
            }
        }

        // Instantiate undirected adjacency matrix as a clone of skeleton adjacency matrix.
        let undirected_adjacency_matrix = skeleton_adjacency_matrix.clone();

        // Instantiate empty directed adjacency matrix given graph order.
        let directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);
        Self {
            labels,
            labels_indices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            skeleton_adjacency_matrix,
            size,
        }
    }

    #[inline]
    fn clear(&mut self) {
        // Clear the vertices.
        self.labels.clear();
        // Clear the vertices map.
        self.labels_indices.clear();
        // Clear all adjacency matrices.
        self.undirected_adjacency_matrix = Default::default();
        self.directed_adjacency_matrix = Default::default();
        self.skeleton_adjacency_matrix = Default::default();
        // Clear the size.
        self.size = 0;
    }

    #[inline]
    fn label(&self, x: usize) -> &str {
        self.labels_indices
            .get_by_right(&x)
            .unwrap_or_else(|| panic!("No vertex with label `{x}`"))
    }

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        Self::LabelsIter::new(self)
    }

    #[inline]
    fn vertex(&self, x: &str) -> usize {
        *self
            .labels_indices
            .get_by_left(x)
            .unwrap_or_else(|| panic!("No vertex with identifier `{x}`"))
    }

    #[inline]
    fn vertices(&self) -> Self::VerticesIter<'_> {
        // Assert vertex set and vertices map are consistent.
        debug_assert_eq!(self.labels.len(), self.labels_indices.len());

        0..self.labels.len()
    }

    #[inline]
    fn order(&self) -> usize {
        // Check iterator consistency.
        debug_assert_eq!(V!(self).len(), self.labels.len());
        // Assert vertex set and vertices map are consistent.
        debug_assert_eq!(self.labels.len(), self.labels_indices.len());

        // Assert vertex set is consistent with adjacency matrices shapes.
        debug_assert_eq!(
            self.labels_indices.len(),
            self.undirected_adjacency_matrix.nrows()
        );
        debug_assert_eq!(
            self.labels_indices.len(),
            self.directed_adjacency_matrix.nrows()
        );
        debug_assert_eq!(
            self.labels_indices.len(),
            self.skeleton_adjacency_matrix.nrows()
        );

        // Assert adjacency matrices are square.
        debug_assert!(self.undirected_adjacency_matrix.is_square());
        debug_assert!(self.directed_adjacency_matrix.is_square());
        debug_assert!(self.skeleton_adjacency_matrix.is_square());

        self.labels.len()
    }

    #[inline]
    fn has_vertex(&self, x: usize) -> bool {
        // Check vertex existence.
        let f = self.labels_indices.contains_right(&x);

        // Check iterator consistency.
        debug_assert_eq!(V!(self).any(|y| y == x), f);
        // Assert vertex set and vertices map are consistent.
        debug_assert_eq!(x < self.order(), f);

        f
    }

    fn add_vertex<V>(&mut self, x: V) -> usize
    where
        V: Into<String>,
    {
        // Cast vertex label.
        let x = x.into();

        // If vertex was already present ...
        if !self.labels.insert(x.clone()) {
            // ... return early.
            return self.vertex(&x);
        }

        // Get vertex identifier.
        let i = self.labels.iter().position(|y| y == &x).unwrap();

        // Update the vertices map after the added vertex.
        for (j, y) in self.labels.iter().skip(i).enumerate() {
            // Add the given vertex and increment subsequent ones by overwriting the entries.
            self.labels_indices.insert(y.clone(), i + j);
        }

        // Compute the new size of adjacency matrix.
        let n = self.skeleton_adjacency_matrix.nrows();
        // Allocate new adjacency matrices.
        let mut directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((n + 1, n + 1), false);
        let mut undirected_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem((n + 1, n + 1), false);
        let mut skeleton_adjacency_matrix = DenseAdjacencyMatrix::from_elem((n + 1, n + 1), false);
        // Compute blocks.
        let (p, q) = ([0..i, (i + 1)..(n + 1)], [0..i, i..n]);
        let (p, q) = (iproduct!(p.clone(), p), iproduct!(q.clone(), q));
        // Copy old adjacency matrix using blocks operations.
        for ((p_start, p_end), (q_start, q_end)) in p.zip(q) {
            directed_adjacency_matrix
                .slice_mut(s![p_start.clone(), p_end.clone()])
                .assign(
                    &self
                        .directed_adjacency_matrix
                        .slice(s![q_start.clone(), q_end.clone()]),
                );

            undirected_adjacency_matrix
                .slice_mut(s![p_start.clone(), p_end.clone()])
                .assign(
                    &self
                        .undirected_adjacency_matrix
                        .slice(s![q_start.clone(), q_end.clone()]),
                );

            skeleton_adjacency_matrix
                .slice_mut(s![p_start, p_end])
                .assign(&self.skeleton_adjacency_matrix.slice(s![q_start, q_end]));
        }
        // Replace old with new adjacency matrices.
        self.directed_adjacency_matrix = directed_adjacency_matrix;
        self.undirected_adjacency_matrix = undirected_adjacency_matrix;
        self.skeleton_adjacency_matrix = skeleton_adjacency_matrix;

        // Assert vertex has been added.
        debug_assert!(self.labels.contains(&x));
        debug_assert!(self.labels_indices.contains_left(&x));
        // Assert vertex set is still consistent with vertices map.
        debug_assert!(self
            .labels
            .iter()
            .eq(self.labels_indices.left_values().sorted()));
        // Assert vertices labels are still associated to an ordered and
        // contiguous sequence of integers starting from zero, i.e in [0, n).
        debug_assert!(self
            .labels_indices
            .right_values()
            .cloned()
            .sorted()
            .eq(0..self.labels_indices.len()));
        // Assert vertex set is still consistent with adjacency matrices shapes.
        debug_assert_eq!(
            self.labels_indices.len(),
            self.directed_adjacency_matrix.nrows()
        );
        debug_assert_eq!(
            self.labels_indices.len(),
            self.undirected_adjacency_matrix.nrows()
        );
        debug_assert_eq!(
            self.labels_indices.len(),
            self.skeleton_adjacency_matrix.nrows()
        );
        // Assert adjacency matrices are still square.
        debug_assert!(self.directed_adjacency_matrix.is_square());
        debug_assert!(self.undirected_adjacency_matrix.is_square());
        debug_assert!(self.skeleton_adjacency_matrix.is_square());
        // Assert adjacency matrices are still symmetric.
        debug_assert_eq!(
            self.undirected_adjacency_matrix,
            self.undirected_adjacency_matrix.t()
        );
        debug_assert_eq!(
            self.skeleton_adjacency_matrix,
            self.skeleton_adjacency_matrix.t()
        );

        // Return new vertex.
        i
    }

    fn del_vertex(&mut self, x: usize) -> bool {
        // Get vertex label and identifier.
        let x_i = self.labels_indices.remove_by_right(&x);

        // If vertex was not present ...
        if x_i.is_none() {
            // ... then return early.
            return false;
        }

        // Get vertex label and identifier.
        let (x, i) = x_i.unwrap();

        // Remove vertex label.
        self.labels.remove(&x);

        // Update the vertices map after the removed vertex.
        for (j, y) in self.labels.iter().skip(i).enumerate() {
            // Decrement subsequent ones by overwriting the entries.
            self.labels_indices.insert(y.clone(), i + j);
        }

        // Compute the new size of adjacency matrix.
        let n = self.skeleton_adjacency_matrix.nrows();
        // Allocate new adjacency matrices.
        let mut directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((n - 1, n - 1), false);
        let mut undirected_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem((n - 1, n - 1), false);
        let mut skeleton_adjacency_matrix = DenseAdjacencyMatrix::from_elem((n - 1, n - 1), false);
        // Compute blocks.
        let (p, q) = ([0..i, i..(n - 1)], [0..i, (i + 1)..n]);
        let (p, q) = (iproduct!(p.clone(), p), iproduct!(q.clone(), q));
        // Copy old adjacency matrix using blocks operations.
        for ((p_start, p_end), (q_start, q_end)) in p.zip(q) {
            directed_adjacency_matrix
                .slice_mut(s![p_start.clone(), p_end.clone()])
                .assign(
                    &self
                        .directed_adjacency_matrix
                        .slice(s![q_start.clone(), q_end.clone()]),
                );

            undirected_adjacency_matrix
                .slice_mut(s![p_start.clone(), p_end.clone()])
                .assign(
                    &self
                        .undirected_adjacency_matrix
                        .slice(s![q_start.clone(), q_end.clone()]),
                );

            skeleton_adjacency_matrix
                .slice_mut(s![p_start, p_end])
                .assign(&self.skeleton_adjacency_matrix.slice(s![q_start, q_end]));
        }
        // Replace old with new adjacency matrices.
        self.directed_adjacency_matrix = directed_adjacency_matrix;
        self.undirected_adjacency_matrix = undirected_adjacency_matrix;
        self.skeleton_adjacency_matrix = skeleton_adjacency_matrix;

        // Assert vertex has been removed.
        debug_assert!(!self.labels.contains(&x));
        debug_assert!(!self.labels_indices.contains_left(&x));
        // Assert vertex set is still consistent with vertices map.
        debug_assert!(self
            .labels
            .iter()
            .eq(self.labels_indices.left_values().sorted()));
        // Assert vertices labels are still associated to an ordered and
        // contiguous sequence of integers starting from zero, i.e in [0, n).
        debug_assert!(self
            .labels_indices
            .right_values()
            .cloned()
            .sorted()
            .eq(0..self.labels_indices.len()));
        // Assert vertex set is still consistent with adjacency matrices shapes.
        debug_assert_eq!(
            self.labels_indices.len(),
            self.directed_adjacency_matrix.nrows()
        );
        debug_assert_eq!(
            self.labels_indices.len(),
            self.undirected_adjacency_matrix.nrows()
        );
        debug_assert_eq!(
            self.labels_indices.len(),
            self.skeleton_adjacency_matrix.nrows()
        );
        // Assert adjacency matrices are still square.
        debug_assert!(self.directed_adjacency_matrix.is_square());
        debug_assert!(self.undirected_adjacency_matrix.is_square());
        debug_assert!(self.skeleton_adjacency_matrix.is_square());
        // Assert adjacency matrices are still symmetric.
        debug_assert_eq!(
            self.undirected_adjacency_matrix,
            self.undirected_adjacency_matrix.t()
        );
        debug_assert_eq!(
            self.skeleton_adjacency_matrix,
            self.skeleton_adjacency_matrix.t()
        );

        true
    }

    #[inline]
    fn edges(&self) -> Self::EdgesIter<'_> {
        Self::EdgesIter::new(self)
    }

    #[inline]
    fn size(&self) -> usize {
        // Check iterator consistency.
        debug_assert_eq!(E!(self).len(), self.size);

        self.size
    }

    #[inline]
    fn has_edge(&self, x: usize, y: usize) -> bool {
        self.skeleton_adjacency_matrix[[x, y]]
    }

    #[inline]
    fn add_edge(&mut self, x: usize, y: usize) -> bool {
        // If edge already exists ...
        if self.skeleton_adjacency_matrix[[x, y]] {
            // ... return early.
            return false;
        }

        // Add edge.
        self.undirected_adjacency_matrix[[x, y]] = true;
        self.undirected_adjacency_matrix[[y, x]] = true;

        self.skeleton_adjacency_matrix[[x, y]] = true;
        self.skeleton_adjacency_matrix[[y, x]] = true;

        // Increment size.
        self.size += 1;

        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.undirected_adjacency_matrix[[y, x]]
        );
        debug_assert_eq!(
            self.skeleton_adjacency_matrix[[x, y]],
            self.skeleton_adjacency_matrix[[y, x]]
        );

        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.skeleton_adjacency_matrix[[x, y]]
        );
        // Assert size counter and adjacency matrices are still consistent.
        debug_assert_eq!(
            self.size,
            self.undirected_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum()
        );

        debug_assert_eq!(
            self.size,
            self.skeleton_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum()
        );

        true
    }

    #[inline]
    fn del_edge(&mut self, x: usize, y: usize) -> bool {
        // If edge does not exists ...
        if !self.skeleton_adjacency_matrix[[x, y]] {
            // ... return early.
            return false;
        }

        // Remove edge.
        self.undirected_adjacency_matrix[[x, y]] = false;
        self.undirected_adjacency_matrix[[y, x]] = false;

        self.directed_adjacency_matrix[[x, y]] = false;
        self.directed_adjacency_matrix[[y, x]] = false;

        self.skeleton_adjacency_matrix[[x, y]] = false;
        self.skeleton_adjacency_matrix[[y, x]] = false;

        // Decrement size.
        self.size -= 1;

        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.undirected_adjacency_matrix[[y, x]]
        );
        debug_assert_eq!(
            self.skeleton_adjacency_matrix[[x, y]],
            self.skeleton_adjacency_matrix[[y, x]]
        );

        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.skeleton_adjacency_matrix[[x, y]]
        );

        debug_assert_eq!(
            self.directed_adjacency_matrix[[x, y]],
            self.skeleton_adjacency_matrix[[x, y]]
        );
        // Assert size counter and adjacency matrices are still consistent.
        debug_assert_eq!(
            self.size,
            self.undirected_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum()
        );

        debug_assert_eq!(
            self.size,
            self.skeleton_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum()
        );

        true
    }

    #[inline]
    fn adjacents(&self, x: usize) -> Self::AdjacentsIter<'_> {
        Self::AdjacentsIter::new(self, x)
    }

    #[inline]
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        // Check using has_edge.
        let f = self.has_edge(x, y);

        // Check iterator consistency.
        debug_assert_eq!(Adj!(self, x).any(|z| z == y), f);

        f
    }
}

/* Implement DefaultGraph trait. */
impl Default for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn default() -> Self {
        Self {
            labels: Default::default(),
            labels_indices: Default::default(),
            undirected_adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            directed_adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            skeleton_adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            size: 0,
        }
    }
}

impl DefaultGraph for PartiallyDenseAdjacencyMatrixGraph {
    fn empty<V, I>(labels: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let labels: BTreeSet<_> = labels.into_iter().map(|x| x.into()).collect();

        // Compute new graph order.
        let order = labels.len();
        // Map vertices labels to vertices indices.
        let labels_indices = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        // Initialize adjacency matrix given graph order.
        let undirected_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);
        let directed_adjacency_matrix = undirected_adjacency_matrix.clone();
        let skeleton_adjacency_matrix = undirected_adjacency_matrix.clone();

        Self {
            labels,
            labels_indices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            skeleton_adjacency_matrix,
            size: 0,
        }
    }

    fn complete<V, I>(labels: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let labels: BTreeSet<_> = labels.into_iter().map(|x| x.into()).collect();

        // Compute new graph order.
        let order = labels.len();
        // Map vertices labels to vertices indices.
        let labels_indices = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        // Initialize directed adjacency matrix and undirected adjacency matrix given graph order.
        let directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);
        let mut undirected_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), true);

        // Remove self loops.
        undirected_adjacency_matrix
            .diag_mut()
            .map_inplace(|x| *x = false);

        // Instantiate skeleton adjacency matrix as a clone of undirected adjacency matrix
        let skeleton_adjacency_matrix = undirected_adjacency_matrix.clone();

        // Compute size.
        let size = (order * (order.saturating_sub(1))) / 2;

        Self {
            labels,
            labels_indices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            skeleton_adjacency_matrix,
            size,
        }
    }
}

/* Implement TryFrom traits. */
impl<V> From<EdgeList<V>> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
{
    #[inline]
    fn from(edge_list: EdgeList<V>) -> Self {
        Self::new([], edge_list)
    }
}

impl<V> From<AdjacencyList<V>> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Clone + Into<String>,
{
    fn from(adjacency_list: AdjacencyList<V>) -> Self {
        // Map into vertices.
        let vertices = adjacency_list.keys().cloned().collect_vec();
        // Map into edges.
        let edges = adjacency_list
            .into_iter()
            .flat_map(|(x, ys)| std::iter::repeat(x).take(ys.len()).zip(ys.into_iter()));

        Self::new(vertices, edges)
    }
}

impl<V, I> TryFrom<(I, DenseAdjacencyMatrix)> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    type Error = E;

    fn try_from(
        (vertices, adjacency_matrix): (I, DenseAdjacencyMatrix),
    ) -> Result<Self, Self::Error> {
        // Remove duplicated vertices labels.
        let labels: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Compute order
        let order = labels.len();
        // Check if vertex set is not consistent with given adjacency matrix.
        if order != adjacency_matrix.nrows() {
            return Err(E::InconsistentMatrix);
        }
        // Check if adjacency matrix is not square.
        if !adjacency_matrix.is_square() {
            return Err(E::NonSquareMatrix);
        }

        // Map vertices labels to vertices indices.
        let labels_indices = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();

        // Cast to standard memory layout (i.e. C layout), if not already.
        let adjacency_matrix = adjacency_matrix.as_standard_layout().into_owned();

        // Compute size.
        let size = adjacency_matrix.mapv(|f| f as usize).sum();

        Ok(Self {
            labels,
            labels_indices,
            directed_adjacency_matrix: DenseAdjacencyMatrix::from_elem((order, order), false),
            undirected_adjacency_matrix: adjacency_matrix.clone(),
            skeleton_adjacency_matrix: adjacency_matrix,
            size,
        })
    }
}

impl<V, I> TryFrom<(I, SparseAdjacencyMatrix)> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    type Error = E;

    fn try_from(
        (vertices, adjacency_matrix): (I, SparseAdjacencyMatrix),
    ) -> Result<Self, Self::Error> {
        // Allocate dense adjacency matrix.
        let mut dense_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem(adjacency_matrix.shape(), false);
        // Fill dense adjacency matrix from sparse triplets.
        for (&f, (i, j)) in adjacency_matrix.triplet_iter() {
            dense_adjacency_matrix[[i, j]] = f;
        }
        // Delegate constructor to dense adjacency matrix constructor.
        Self::try_from((vertices, dense_adjacency_matrix))
    }
}

impl<V> From<(EdgeList<V>, EdgeList<V>)> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
{
    #[inline]
    fn from((undirected_edges, directed_edges): (EdgeList<V>, EdgeList<V>)) -> Self {
        Self::new_spec([], undirected_edges, directed_edges).unwrap()
    }
}

impl<V> From<(AdjacencyList<V>, AdjacencyList<V>)> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Clone + Into<String>,
{
    fn from(
        (undirected_adjacency_list, directed_adjacency_list): (AdjacencyList<V>, AdjacencyList<V>),
    ) -> Self {
        // Map into vertices.
        let mut vertices = undirected_adjacency_list.keys().cloned().collect_vec();
        vertices.append(&mut directed_adjacency_list.keys().cloned().collect_vec());
        // Map into edges.
        let undirected_edges = undirected_adjacency_list
            .into_iter()
            .flat_map(|(x, ys)| std::iter::repeat(x).take(ys.len()).zip(ys.into_iter()));
        let directed_edges = directed_adjacency_list
            .into_iter()
            .flat_map(|(x, ys)| std::iter::repeat(x).take(ys.len()).zip(ys.into_iter()));

        Self::new_spec(vertices, undirected_edges, directed_edges).unwrap()
    }
}

impl<V, I> TryFrom<(I, DenseAdjacencyMatrix, DenseAdjacencyMatrix)>
    for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    type Error = E;

    fn try_from(
        (vertices, undirected_adjacency_matrix, directed_adjacency_matrix): (
            I,
            DenseAdjacencyMatrix,
            DenseAdjacencyMatrix,
        ),
    ) -> Result<Self, Self::Error> {
        // Remove duplicated vertices labels.
        let labels: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Compute order
        let order = labels.len();
        // Check if vertex set is not consistent with given undirected adjacency matrix.
        if order != undirected_adjacency_matrix.nrows() {
            return Err(E::InconsistentMatrix);
        }
        // Check if undirected adjacency matrix is not square.
        if !undirected_adjacency_matrix.is_square() {
            return Err(E::NonSquareMatrix);
        }
        // Check if adjacency matrices have consistent dimensions.
        if undirected_adjacency_matrix.dim() != directed_adjacency_matrix.dim() {
            return Err(E::InconsistentMatrix);
        }

        // Check if adjacency matrices don't overlap.
        if (&undirected_adjacency_matrix & &directed_adjacency_matrix)
            .iter()
            .any(|x| *x)
        {
            return Err(E::MultipleTypesEdges);
        }

        // Map vertices labels to vertices indices.
        let labels_indices = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();

        // Cast to standard memory layout (i.e. C layout), if not already.
        let skeleton_adjacency_matrix = &undirected_adjacency_matrix
            | &directed_adjacency_matrix
            | directed_adjacency_matrix.t();
        let undirected_adjacency_matrix = undirected_adjacency_matrix
            .as_standard_layout()
            .into_owned();
        let directed_adjacency_matrix = directed_adjacency_matrix.as_standard_layout().into_owned();

        // Compute size.
        let size = skeleton_adjacency_matrix
            .indexed_iter()
            .map(|((i, j), &f)| if i <= j { f as usize } else { 0 })
            .sum();

        Ok(Self {
            labels,
            labels_indices,
            directed_adjacency_matrix,
            undirected_adjacency_matrix,
            skeleton_adjacency_matrix,
            size,
        })
    }
}

impl<V, I> TryFrom<(I, SparseAdjacencyMatrix, SparseAdjacencyMatrix)>
    for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    type Error = E;

    fn try_from(
        (vertices, undirected_adjacency_matrix, directed_adjacency_matrix): (
            I,
            SparseAdjacencyMatrix,
            SparseAdjacencyMatrix,
        ),
    ) -> Result<Self, Self::Error> {
        // Allocate dense adjacency matrices.
        let mut undirected_dense_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem(undirected_adjacency_matrix.shape(), false);
        let mut directed_dense_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem(directed_adjacency_matrix.shape(), false);
        // Fill dense adjacency matrices from sparse triplets.
        for (&f, (i, j)) in undirected_adjacency_matrix.triplet_iter() {
            undirected_dense_adjacency_matrix[[i, j]] = f;
        }
        for (&f, (i, j)) in directed_adjacency_matrix.triplet_iter() {
            directed_dense_adjacency_matrix[[i, j]] = f;
        }
        // Delegate constructor to dense adjacency matrix constructor.
        Self::try_from((
            vertices,
            undirected_dense_adjacency_matrix,
            directed_dense_adjacency_matrix,
        ))
    }
}

/* Implement Into traits. */

#[allow(clippy::from_over_into)]
impl Into<EdgeList<String>> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> EdgeList<String> {
        E!(self)
            .map(|(x, y)| (self.label(x).into(), self.label(y).into()))
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<AdjacencyList<String>> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> AdjacencyList<String> {
        V!(self)
            .map(|x| {
                (
                    self.label(x).into(),
                    Adj!(self, x).map(|y| self.label(y).into()).collect(),
                )
            })
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<(BTreeSet<String>, DenseAdjacencyMatrix)> for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn into(self) -> (BTreeSet<String>, DenseAdjacencyMatrix) {
        (self.labels, self.skeleton_adjacency_matrix)
    }
}

#[allow(clippy::from_over_into)]
impl Into<(BTreeSet<String>, SparseAdjacencyMatrix)> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> (BTreeSet<String>, SparseAdjacencyMatrix) {
        // Get upper bound capacity.
        let size = self.size * 2;
        // Allocate triplets indices.
        let (mut rows, mut cols) = (Vec::with_capacity(size), Vec::with_capacity(size));
        // Build triplets indices.
        for ((i, j), &f) in self.skeleton_adjacency_matrix.indexed_iter() {
            if f {
                rows.push(i);
                cols.push(j);
            }
        }
        // Shrink triplets indices to actual capacity.
        rows.shrink_to_fit();
        cols.shrink_to_fit();
        // Build data vector.
        let data = std::iter::repeat(true).take(rows.len()).collect_vec();
        // Construct sparse adjacency matrix.
        let sparse_adjacency_matrix = SparseAdjacencyMatrix::from_triplets(
            self.skeleton_adjacency_matrix.dim(),
            rows,
            cols,
            data,
        );

        (self.labels, sparse_adjacency_matrix)
    }
}

/* Implement Into traits. */

#[allow(clippy::from_over_into)]
impl Into<(EdgeList<String>, EdgeList<String>)> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> (EdgeList<String>, EdgeList<String>) {
        (
            uE!(self)
                .map(|(x, y)| (self.label(x).into(), self.label(y).into()))
                .collect(),
            uE!(self)
                .map(|(x, y)| (self.label(x).into(), self.label(y).into()))
                .collect(),
        )
    }
}

#[allow(clippy::from_over_into)]
impl Into<(AdjacencyList<String>, AdjacencyList<String>)> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> (AdjacencyList<String>, AdjacencyList<String>) {
        /*
        V!(self)
            .map(|x| {
                (
                    self.label(x).into(),
                    Adj!(self, x).map(|y| self.label(y).into()).collect(),
                )
            })
            .collect()
        */
        todo!(); //TODO: First implement Ch! and Ne! for Partially directed graph struct
    }
}

#[allow(clippy::from_over_into)]
impl Into<(BTreeSet<String>, DenseAdjacencyMatrix, DenseAdjacencyMatrix)>
    for PartiallyDenseAdjacencyMatrixGraph
{
    #[inline]
    fn into(self) -> (BTreeSet<String>, DenseAdjacencyMatrix, DenseAdjacencyMatrix) {
        (
            self.labels,
            self.undirected_adjacency_matrix,
            self.directed_adjacency_matrix,
        )
    }
}

/* Implement PartialOrdGraph trait. */
impl PartialEq for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Check that V(\mathcal{G}) == V(\mathcal{H}) && E(\mathcal{G}) == E(\mathcal{H}).
        let labels: bool = self.labels.eq(&other.labels);
        let undirected: bool = self
            .undirected_adjacency_matrix
            .eq(&other.undirected_adjacency_matrix);
        let directed: bool = self
            .directed_adjacency_matrix
            .eq(&other.directed_adjacency_matrix);
        debug_assert!(self.skeleton_adjacency_matrix == other.skeleton_adjacency_matrix);
        labels && undirected && directed
    }
}

impl Eq for PartiallyDenseAdjacencyMatrixGraph {}

impl PartialOrd for PartiallyDenseAdjacencyMatrixGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare vertices sets.
        let lhs: HashSet<_> = V!(self).map(|x| self.label(x)).collect();
        let rhs: HashSet<_> = V!(other).map(|x| other.label(x)).collect();
        // If the vertices sets are comparable ...
        partial_cmp_sets!(lhs, rhs).and_then(|vertices| {
            // ... compare undirected edges sets.
            let lhs: HashSet<_> = uE!(self)
                .map(|(x, y)| (self.label(x), self.label(y)))
                .collect();
            let rhs: HashSet<_> = uE!(other)
                .map(|(x, y)| (other.label(x), other.label(y)))
                .collect();
            // If the undirected edges sets are comparable ...
            partial_cmp_sets!(lhs, rhs).and_then(|undirected_edges| {
                // ... compare directed edges sets.
                let lhs: HashSet<_> = dE!(self)
                    .map(|(x, y)| (self.label(x), self.label(y)))
                    .collect();
                let rhs: HashSet<_> = dE!(other)
                    .map(|(x, y)| (other.label(x), other.label(y)))
                    .collect();
                // If also the directed edges sets are comparable ...
                partial_cmp_sets!(lhs, rhs).and_then(|directed_edges| {
                    // ... then return ordering
                    match (vertices, undirected_edges, directed_edges) {
                        (_, Ordering::Greater, Ordering::Less) => None,
                        (_, Ordering::Less, Ordering::Greater) => None,
                        (Ordering::Greater, _, Ordering::Less) => None,
                        (Ordering::Less, _, Ordering::Greater) => None,
                        (Ordering::Greater, Ordering::Greater, _) => Some(Ordering::Greater),
                        (Ordering::Less, Ordering::Less, _) => Some(Ordering::Less),
                        (Ordering::Equal, Ordering::Equal, _) => Some(directed_edges),
                        (Ordering::Less, Ordering::Equal, _) => Some(Ordering::Less),
                        (Ordering::Equal, Ordering::Less, _) => Some(Ordering::Less),
                        (Ordering::Greater, Ordering::Equal, _) => Some(Ordering::Greater),
                        (Ordering::Equal, Ordering::Greater, _) => Some(Ordering::Greater),
                        _ => None,
                    }
                })
            })
        })
    }
}

impl PartialOrdGraph for PartiallyDenseAdjacencyMatrixGraph {}

/* Implement SubGraph trait. */
impl SubGraph for PartiallyDenseAdjacencyMatrixGraph {
    fn subgraph<I, J>(&self, vertices: I, edges: J) -> Self
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = (usize, usize)>,
    {
        // Initialize new indices.
        let mut indices = vec![false; self.order()];
        // Add the required vertices.
        for x in vertices {
            indices[x] = true;
        }

        // Initialize a new skeleton adjacency matrix.
        let mut skeleton_adjacency_matrix =
            Self::Data::from_elem(self.skeleton_adjacency_matrix.dim(), false);
        // Fill the adjacency matrix.
        for (x, y) in edges {
            // Add the edge.
            skeleton_adjacency_matrix[[x, y]] = true;
            // Add the vertices.
            indices[x] = true;
            indices[y] = true;
        }

        // Map the indices.
        let indices = indices
            .into_iter()
            .enumerate()
            .filter_map(|(i, f)| match f {
                true => Some(i),
                false => None,
            })
            .collect_vec();

        // Get minor of matrix.
        let skeleton_adjacency_matrix = skeleton_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // Initialize undirected and directed adjacences submatrices ...
        let mut undirected_adjacency_matrix = self
            .undirected_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        let mut directed_adjacency_matrix = self
            .directed_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // ... and filter their elements based on skeleton adjacency matrix
        undirected_adjacency_matrix =
            undirected_adjacency_matrix & skeleton_adjacency_matrix.clone();
        directed_adjacency_matrix = directed_adjacency_matrix & skeleton_adjacency_matrix.clone();

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.label(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), skeleton_adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(skeleton_adjacency_matrix.is_square());
        // Assert matrix dimensions are still consistent
        debug_assert!(undirected_adjacency_matrix.dim() == skeleton_adjacency_matrix.dim());
        debug_assert!(directed_adjacency_matrix.dim() == skeleton_adjacency_matrix.dim());

        // Build subgraph from vertices and adjacency matrix.
        Self::try_from((
            vertices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
        ))
        .unwrap()
    }

    fn subgraph_by_vertices<I>(&self, vertices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Remove duplicated vertices identifiers.
        let indices: BTreeSet<_> = vertices.into_iter().collect();
        // Cast to vector of indices.
        let indices = indices.into_iter().collect_vec();

        // Get minor of matrices.
        let undirected_adjacency_matrix = self
            .undirected_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);
        let directed_adjacency_matrix = self
            .directed_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);
        let skeleton_adjacency_matrix = self
            .skeleton_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.label(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), skeleton_adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(skeleton_adjacency_matrix.is_square());
        // Assert matrix dimensions are still consistent
        debug_assert!(undirected_adjacency_matrix.dim() == skeleton_adjacency_matrix.dim());
        debug_assert!(directed_adjacency_matrix.dim() == skeleton_adjacency_matrix.dim());

        // Build subgraph from vertices and adjacency matrix.
        Self::try_from((
            vertices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
        ))
        .unwrap()
    }

    fn subgraph_by_edges<J>(&self, edges: J) -> Self
    where
        J: IntoIterator<Item = (usize, usize)>,
    {
        // Initialize new indices.
        let mut indices = vec![false; self.order()];

        // Initialize a new adjacency matrix.
        let mut skeleton_adjacency_matrix =
            Self::Data::from_elem(self.skeleton_adjacency_matrix.dim(), false);
        // Fill the adjacency matrix.
        for (x, y) in edges {
            // Add the edge.
            skeleton_adjacency_matrix[[x, y]] = true;
            // Add the vertices.
            indices[x] = true;
            indices[y] = true;
        }

        // Map the indices.
        let indices = indices
            .into_iter()
            .enumerate()
            .filter_map(|(i, f)| match f {
                true => Some(i),
                false => None,
            })
            .collect_vec();

        // Get minor of matrix.
        let skeleton_adjacency_matrix = skeleton_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // Initialize undirected and directed adjacences submatrices ...
        let mut undirected_adjacency_matrix = self
            .undirected_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        let mut directed_adjacency_matrix = self
            .directed_adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // ... and filter their elements based on skeleton adjacency matrix
        undirected_adjacency_matrix =
            undirected_adjacency_matrix & skeleton_adjacency_matrix.clone();
        directed_adjacency_matrix = directed_adjacency_matrix & skeleton_adjacency_matrix.clone();

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.label(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), skeleton_adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(skeleton_adjacency_matrix.is_square());
        // Assert matrix dimensions are still consistent
        debug_assert!(undirected_adjacency_matrix.dim() == skeleton_adjacency_matrix.dim());
        debug_assert!(directed_adjacency_matrix.dim() == skeleton_adjacency_matrix.dim());

        // Build subgraph from vertices and adjacency matrix.
        Self::try_from((
            vertices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
        ))
        .unwrap()
    }
}

/* Implement UndirectedGraph trait. */
impl UndirectedGraph for PartiallyDenseAdjacencyMatrixGraph {
    type NeighborsIter<'a> = Self::AdjacentsIter<'a>;

    #[inline]
    fn neighbors(&self, x: usize) -> Self::NeighborsIter<'_> {
        Self::NeighborsIter::new_undirected(self, x)
    }

    #[inline]
    fn is_neighbor(&self, x: usize, y: usize) -> bool {
        self.undirected_adjacency_matrix[[x, y]]
    }

    #[inline]
    fn degree(&self, x: usize) -> usize {
        // Compute degree.
        let d = self
            .undirected_adjacency_matrix
            .row(x)
            .mapv(|f| f as usize)
            .sum();

        // Check iterator consistency.
        debug_assert_eq!(Adj!(self, x).count(), d);

        d
    }
}

/* Implement PartiallyGraph trait. */
impl IntoUndirectedGraph for PartiallyDenseAdjacencyMatrixGraph {
    type UndirectedGraph = UndirectedDenseAdjacencyMatrixGraph;

    #[inline]
    fn to_undirected(&self) -> Self::UndirectedGraph {
        Self::UndirectedGraph::try_from((
            self.labels.clone(),
            self.skeleton_adjacency_matrix.clone(),
        ))
        .unwrap()
    }
}

impl PartiallyGraph for PartiallyDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::PartiallyDirected;

    type EdgesIter<'a> = EdgesIterator<'a>;

    type Error = E;

    fn new_spec<V, I, J, K>(vertices: I, undirected_edges: J, directed_edges: K) -> Result<Self, E>
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
        K: IntoIterator<Item = (V, V)>,
    {
        // Remove duplicated vertices labels.
        let mut labels: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Map undirected edges iterator into edge list.
        let undirected_edges: EdgeList<_> = undirected_edges
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();
        // Map undirected edges iterator into edge list.
        let directed_edges: EdgeList<_> = directed_edges
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();
        // Add missing vertices from the edges.
        labels.extend(
            undirected_edges
                .iter()
                .cloned()
                .flat_map(|(x, y)| [x, y])
                .chain(directed_edges.iter().cloned().flat_map(|(x, y)| [x, y])),
        );

        // Compute new graph order.
        let order = labels.len();
        // Map vertices labels to vertices indices.
        let labels_indices: BiHashMap<_, _> = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();

        // Initialize adjacency matrices given graph order.
        let mut undirected_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem((order, order), false);
        let mut directed_adjacency_matrix = undirected_adjacency_matrix.clone();
        let mut skeleton_adjacency_matrix = undirected_adjacency_matrix.clone();

        // Initialize the size.
        let mut size = 0;
        // Fill skeleton adjacency matrix given edge set.
        for (x, y) in undirected_edges {
            // Get associated vertices indices.
            let (i, j) = (
                *labels_indices.get_by_left(&x).unwrap(),
                *labels_indices.get_by_left(&y).unwrap(),
            );
            // Set edge given indices.
            if !skeleton_adjacency_matrix[[i, j]] {
                // Add edge.
                undirected_adjacency_matrix[[i, j]] = true;
                undirected_adjacency_matrix[[j, i]] = true;
                skeleton_adjacency_matrix[[i, j]] = true;
                skeleton_adjacency_matrix[[j, i]] = true;
                // Increment size.
                size += 1;
            }
        }
        for (x, y) in directed_edges {
            // Get associated vertices indices.
            let (i, j) = (
                *labels_indices.get_by_left(&x).unwrap(),
                *labels_indices.get_by_left(&y).unwrap(),
            );
            // Set edge given indices.
            if !skeleton_adjacency_matrix[[i, j]] {
                // Add edge.
                directed_adjacency_matrix[[i, j]] = true;
                skeleton_adjacency_matrix[[i, j]] = true;
                skeleton_adjacency_matrix[[j, i]] = true;
                // Increment size.
                size += 1;
            } else {
                // Panic if edges lists overlap
                return Err(E::MultipleTypesEdges);
            }
        }

        Ok(Self {
            labels,
            labels_indices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            skeleton_adjacency_matrix,
            size,
        })
    }

    #[inline]
    fn edges_of_type(&self, which: char) -> Self::EdgesIter<'_> {
        match which {
            'u' => Self::EdgesIter::new_undirected(self),
            'd' => Self::EdgesIter::new_directed(self),
            _ => panic!("Invalid edge type. Types: 'u' for undirected and 'd' for directed."),
        }
    }
    #[inline]
    fn size_of_type(&self, which: char) -> usize {
        let size = match which {
            'u' => self.edges_of_type('u').len(),
            'd' => self.edges_of_type('d').len(),
            _ => panic!("Invalid edge type. Types: 'u' for undirected and 'd' for directed."),
        };
        debug_assert!(size <= self.size());
        size
    }
    #[inline]
    fn type_of_edge(&self, x: usize, y: usize) -> Option<char> {
        // If edge is not present:
        if !self.skeleton_adjacency_matrix[[x, y]] {
            return None;
        }
        // If edge is present...
        match self.undirected_adjacency_matrix[[x, y]] {
            // ... is undirected
            true => Some('u'),
            // ... is directed
            _ => Some('d'),
        }
    }

    fn add_edge_of_type(&mut self, x: usize, y: usize, which: char) -> bool {
        // If edge already exists ...
        if self.skeleton_adjacency_matrix[[x, y]] {
            debug_assert!(
                self.skeleton_adjacency_matrix[[x, y]] == self.skeleton_adjacency_matrix[[y, x]]
            );
            // ... return early.
            return false;
        }

        match which {
            'u' => {
                // Add edge.
                self.undirected_adjacency_matrix[[x, y]] = true;
                self.undirected_adjacency_matrix[[y, x]] = true;
                self.skeleton_adjacency_matrix[[x, y]] = true;
                self.skeleton_adjacency_matrix[[y, x]] = true;

                // Increment size.
                self.size += 1;

                // Assert adjacency matrices are still consistent.
                debug_assert_eq!(
                    self.undirected_adjacency_matrix[[x, y]],
                    self.undirected_adjacency_matrix[[y, x]]
                );
                debug_assert_eq!(
                    self.undirected_adjacency_matrix[[x, y]],
                    self.skeleton_adjacency_matrix[[x, y]]
                );
            }
            'd' => {
                // Add edge.
                self.directed_adjacency_matrix[[x, y]] = true;
                self.skeleton_adjacency_matrix[[x, y]] = true;
                self.skeleton_adjacency_matrix[[y, x]] = true;

                // Increment size.
                self.size += 1;
                debug_assert_eq!(
                    self.directed_adjacency_matrix[[x, y]],
                    self.skeleton_adjacency_matrix[[x, y]]
                );
            }
            _ => panic!("Invalid edge type. Types: 'u' for undirected and 'd' for directed."),
        }

        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(
            self.skeleton_adjacency_matrix[[x, y]],
            self.skeleton_adjacency_matrix[[y, x]]
        );
        // Assert size counter and adjacency matrices are still consistent.
        debug_assert_eq!(
            self.size,
            self.skeleton_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum()
        );

        true
    }
    fn orient_edge(&mut self, x: usize, y: usize) -> bool {
        if !self.has_edge(x, y) {
            return false;
        }
        self.del_edge(x, y);
        self.add_edge_of_type(x, y, 'd');
        true
    }
}

// TODO: AncestorsIterator, ParentsIterator, ChildrenIterator, DescendantsIterator structs
// TODO: impl UndirectedGraph, DirectedGraph traits
// TODO: Write tests for all implemented traits
//          (especially: UnidrectedGraph, DirectedGraph, PartiallyGraph, ...)

// TODO: impl PathGraph trait
// TODO: (impl MoralGraph trait)
// TODO: From<DOT> trait
