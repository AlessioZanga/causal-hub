#![allow(unused_imports, dead_code)] // FIXME: remove this line
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
    fmt::Display,
    hash::{Hash, Hasher},
    iter::{Enumerate, FilterMap, FusedIterator},
    ops::{Deref, Range},
};

use bimap::BiHashMap;
use itertools::{iproduct, Itertools};
use ndarray::{iter::IndexedIter, prelude::*, OwnedRepr};
use serde::{Deserialize, Serialize};
use std::iter::Chain;

use super::UndirectedDenseAdjacencyMatrixGraph;
use crate::{
    graphs::{
        algorithms::traversal::{DFSEdge, DFSEdges, Traversal},
        directions, BaseGraph, DefaultGraph, DirectedGraph, ErrorGraph as E, IntoUndirectedGraph,
        PartialOrdGraph, PathGraph, SubGraph,
    },
    io::DOT,
    models::MoralGraph,
    prelude::BFS,
    types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, SparseAdjacencyMatrix},
    utils::partial_cmp_sets,
    Adj, Ch, Pa, E, V,
};

/// Mixed graph struct based on a couple of dense adjacency matrix data structures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartiallyDenseAdjacencyMatrixGraph {
    labels: BTreeSet<String>,
    labels_indices: BiHashMap<String, usize>,
    directed_adjacency_matrix: DenseAdjacencyMatrix,
    undirected_adjacency_matrix: DenseAdjacencyMatrix,
    skeleton_adjacency_matrix: DenseAdjacencyMatrix,
    size: usize,
}

impl PartiallyDenseAdjacencyMatrixGraph {
    fn deref(&self) -> &DenseAdjacencyMatrix {
        todo!() // TODO:
    }
}

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
pub struct UndirectedEdgesIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> UndirectedEdgesIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph) -> Self {
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
}

impl<'a> Iterator for UndirectedEdgesIterator<'a> {
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

impl<'a> ExactSizeIterator for UndirectedEdgesIterator<'a> {}

impl<'a> FusedIterator for UndirectedEdgesIterator<'a> {}

#[allow(dead_code, clippy::type_complexity)]
pub struct DirectedEdgesIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> DirectedEdgesIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .directed_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match f && i <= j {
                    true => Some((i, j)),
                    false => None,
                }),
            size: g.size,
        }
    }
}

impl<'a> Iterator for DirectedEdgesIterator<'a> {
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

impl<'a> ExactSizeIterator for DirectedEdgesIterator<'a> {}

impl<'a> FusedIterator for DirectedEdgesIterator<'a> {}

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
    /// Constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .skeleton_adjacency_matrix
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
    /// Constructor.
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
        // Write edge set.
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

impl IntoUndirectedGraph for PartiallyDenseAdjacencyMatrixGraph {
    type UndirectedGraph = UndirectedDenseAdjacencyMatrixGraph;

    #[inline]
    fn to_undirected(&self) -> Self::UndirectedGraph {
        // Make the adjacent matrix symmetric.
        let adjacency_matrix = &self.undirected_adjacency_matrix
            | &self.directed_adjacency_matrix
            | &self.directed_adjacency_matrix.t();

        Self::UndirectedGraph::try_from((self.labels.clone(), adjacency_matrix)).unwrap()
    }
}

impl BaseGraph for PartiallyDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::Mixed;

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
        undirected_adjacency_matrix.diag_mut().map_inplace(|x| *x = false);

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

// TODO: Default, DefaultGraph
// TODO: From, TryFrom, Into
// TODO: PartialEq, Eq
// TODO: PartialOrd, PartialOrdGraph
// TODO: SubGraph
// TODO: AncestorsIterator, ParentsIterator, ChildrenIterator, DescendantsIterator
// TODO: UndirectedGraph, DirectedGraph
// TODO: PathGraph
// TODO: IntoUndirectedGraph
// TODO: MoralGraph
// TODO: From<DOT>
