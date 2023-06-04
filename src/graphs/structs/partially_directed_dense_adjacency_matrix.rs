use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
    iter::{Enumerate, FilterMap, FusedIterator, Map},
    ops::Range,
};

use is_sorted::IsSorted;
use itertools::{iproduct, Itertools};
use ndarray::{iter::IndexedIter, prelude::*, OwnedRepr};
use serde::{Deserialize, Serialize};

use super::{DirectedDenseAdjacencyMatrixGraph, UndirectedDenseAdjacencyMatrixGraph};
use crate::{
    dE,
    graphs::{
        algorithms::traversal::{DFSEdge, DFSEdges, Traversal},
        direction::*,
        BaseGraph, DirectedGraph, IntoUndirectedGraph, PartialOrdGraph, PathGraph, SubGraph,
        UndirectedGraph,
    },
    io::DOT,
    models::MoralGraph,
    prelude::BFS,
    types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, FxIndexSet},
    uE, Adj, Ch, Ne, Pa, E, V,
};

/// Mixed graph struct based on a couple of dense adjacency matrix data structures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartiallyDenseAdjacencyMatrixGraph {
    labels: FxIndexSet<String>,
    undirected_adjacency_matrix: DenseAdjacencyMatrix,
    directed_adjacency_matrix: DenseAdjacencyMatrix,
    adjacency_matrix: DenseAdjacencyMatrix,
    undirected_size: usize,
    directed_size: usize,
    size: usize,
}

/* Implement BaseGraph trait. */

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
                .adjacency_matrix
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
            size: g.undirected_size,
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
            size: g.directed_size,
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
                .adjacency_matrix
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
                .map(|x| format!("\"{}\"", self.get_vertex_by_index(x)))
                .join(", ")
        )?;
        // Write undirected edge set.
        write!(
            f,
            "Undirected E = {{{}}}, ",
            uE!(self)
                .map(|(x, y)| format!(
                    "(\"{}\", \"{}\")",
                    self.get_vertex_by_index(x),
                    self.get_vertex_by_index(y)
                ))
                .join(", ")
        )?;
        // Write directed edge set.
        write!(
            f,
            "Directed E = {{{}}}, ",
            dE!(self)
                .map(|(x, y)| format!(
                    "(\"{}\", \"{}\")",
                    self.get_vertex_by_index(x),
                    self.get_vertex_by_index(y)
                ))
                .join(", ")
        )?;
        // Write all edge set.
        write!(
            f,
            "E = {{{}}}",
            E!(self)
                .map(|(x, y)| format!(
                    "(\"{}\", \"{}\")",
                    self.get_vertex_by_index(x),
                    self.get_vertex_by_index(y)
                ))
                .join(", ")
        )?;
        // Write ending character.
        write!(f, " }}")
    }
}

impl Hash for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.iter().for_each(|x| x.hash(state));
        self.directed_adjacency_matrix.hash(state);
        self.undirected_adjacency_matrix.hash(state);
        self.adjacency_matrix.hash(state);
    }
}

impl BaseGraph for PartiallyDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::PartiallyDirected;

    type VerticesIter<'a> = Map<indexmap::set::Iter<'a, String>, fn(&'a String) -> &'a str>;

    type VerticesIndexIter<'a> = Range<usize>;

    type EdgesIndexIter<'a> = EdgesIterator<'a>;

    type AdjacentsIndexIter<'a> = AdjacentsIterator<'a>;

    fn new<V, I, J>(vertices: I, edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
    {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Map edges iterator into edge list.
        let edges: EdgeList<_> = edges
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();
        // Add missing vertices from the edges.
        labels.extend(edges.iter().cloned().flat_map(|(x, y)| [x, y]));
        // Sort labels.
        labels.sort();

        // Compute new graph order.
        let order = labels.len();
        // Initialize skeleton adjacency matrix given graph order.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

        // Initialize the size.
        let mut size = 0;
        // Fill skeleton adjacency matrix given edge set.
        for (x, y) in edges {
            // Get associated vertices indices.
            let (i, j) = (
                labels.get_index_of(&x).unwrap(),
                labels.get_index_of(&y).unwrap(),
            );
            // Set edge given indices.
            if !adjacency_matrix[[i, j]] {
                // Add edge.
                adjacency_matrix[[i, j]] = true;
                adjacency_matrix[[j, i]] = true;
                // Increment size.
                size += 1;
            }
        }

        // Instantiate undirected adjacency matrix as a clone of skeleton adjacency matrix.
        let undirected_adjacency_matrix = adjacency_matrix.clone();

        // Instantiate empty directed adjacency matrix given graph order.
        let directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

        // Assert if undirected adjacency matrix and skeleton matrix are symmetric
        debug_assert_eq!(undirected_adjacency_matrix, undirected_adjacency_matrix.t());
        debug_assert_eq!(adjacency_matrix, adjacency_matrix.t());

        Self {
            labels,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            adjacency_matrix,
            undirected_size: size,
            directed_size: 0,
            size,
        }
    }

    fn null() -> Self {
        Default::default()
    }

    fn empty<V, I>(labels: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = labels.into_iter().map(|x| x.into()).collect();
        // Sort labels.
        labels.sort();
        // Compute new graph order.
        let order = labels.len();

        // Initialize adjacency matrix given graph order.
        let undirected_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);
        let directed_adjacency_matrix = undirected_adjacency_matrix.clone();
        let adjacency_matrix = undirected_adjacency_matrix.clone();

        Self {
            labels,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            adjacency_matrix,
            undirected_size: 0,
            directed_size: 0,
            size: 0,
        }
    }

    fn complete<V, I>(labels: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = labels.into_iter().map(|x| x.into()).collect();
        // Sort labels.
        labels.sort();
        // Compute new graph order.
        let order = labels.len();

        // Initialize directed adjacency matrix and undirected adjacency matrix given graph order.
        let directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);
        let mut undirected_adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), true);

        // Remove self loops.
        undirected_adjacency_matrix
            .diag_mut()
            .map_inplace(|x| *x = false);

        // Instantiate skeleton adjacency matrix as a clone of undirected adjacency matrix
        let adjacency_matrix = undirected_adjacency_matrix.clone();

        // Compute size.
        let size = (order * (order.saturating_sub(1))) / 2;

        Self {
            labels,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            adjacency_matrix,
            undirected_size: size,
            directed_size: 0,
            size,
        }
    }
    #[inline]
    fn clear(&mut self) {
        // Clear the vertices.
        self.labels.clear();
        // Clear all adjacency matrices.
        self.undirected_adjacency_matrix = Default::default();
        self.directed_adjacency_matrix = Default::default();
        self.adjacency_matrix = Default::default();
        // Clear the sizes.
        self.undirected_size = 0;
        self.directed_size = 0;
        self.size = 0;
    }

    #[inline]
    fn order(&self) -> usize {
        // Check iterator consistency.
        debug_assert_eq!(V!(self).len(), self.labels.len());
        // Assert vertex set is consistent with adjacency matrix shape.
        debug_assert_eq!(self.labels.len(), self.adjacency_matrix.nrows());

        // Assert vertex set is consistent with adjacency matrices shapes.
        debug_assert_eq!(self.labels.len(), self.undirected_adjacency_matrix.nrows());
        debug_assert_eq!(self.labels.len(), self.directed_adjacency_matrix.nrows());
        debug_assert_eq!(self.labels.len(), self.adjacency_matrix.nrows());

        // Assert adjacency matrices are square.
        debug_assert!(self.undirected_adjacency_matrix.is_square());
        debug_assert!(self.directed_adjacency_matrix.is_square());
        debug_assert!(self.adjacency_matrix.is_square());

        self.labels.len()
    }

    #[inline]
    fn get_vertices(&self) -> Self::VerticesIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }

    #[inline]
    fn get_vertex_by_index(&self, x: usize) -> &str {
        self.labels
            .get_index(x)
            .unwrap_or_else(|| panic!("No vertex with label `{x}`"))
    }

    fn add_vertex<V>(&mut self, x: V) -> usize
    where
        V: Into<String>,
    {
        // Cast vertex label.
        let x = x.into();
        // Try to insert vertex label.
        let (i, f) = self.labels.insert_full(x.clone());

        // If vertex was already present ...
        if !f {
            // ... return early.
            return i;
        }

        // Sort labels.
        self.labels.sort();

        // Assert vertex has been added.
        debug_assert!(self.labels.contains(&x));
        // Assert vertex set is still sorted.
        debug_assert!(self.labels.iter().is_sorted());

        // Get vertex index.
        let i = self.labels.get_index_of(&x).unwrap();
        // Compute the new size of adjacency matrix.
        let n = self.adjacency_matrix.nrows();

        // Allocate new adjacency matrices.
        let mut directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((n + 1, n + 1), false);
        let mut undirected_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem((n + 1, n + 1), false);
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((n + 1, n + 1), false);
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

            adjacency_matrix
                .slice_mut(s![p_start, p_end])
                .assign(&self.adjacency_matrix.slice(s![q_start, q_end]));
        }
        // Replace old with new adjacency matrices.
        self.directed_adjacency_matrix = directed_adjacency_matrix;
        self.undirected_adjacency_matrix = undirected_adjacency_matrix;
        self.adjacency_matrix = adjacency_matrix;

        // Assert vertex has been added.
        debug_assert!(self.labels.contains(&x));

        // Assert vertex set is still consistent with adjacency matrices shapes.
        debug_assert_eq!(self.labels.len(), self.directed_adjacency_matrix.nrows());
        debug_assert_eq!(self.labels.len(), self.undirected_adjacency_matrix.nrows());
        debug_assert_eq!(self.labels.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrices are still square.
        debug_assert!(self.directed_adjacency_matrix.is_square());
        debug_assert!(self.undirected_adjacency_matrix.is_square());
        debug_assert!(self.adjacency_matrix.is_square());
        // Assert adjacency matrices are still symmetric.
        debug_assert_eq!(
            self.undirected_adjacency_matrix,
            self.undirected_adjacency_matrix.t()
        );
        debug_assert_eq!(self.adjacency_matrix, self.adjacency_matrix.t());

        // Return new vertex.
        i
    }

    #[inline]
    fn get_vertices_index(&self) -> Self::VerticesIndexIter<'_> {
        0..self.labels.len()
    }

    #[inline]
    fn get_vertex_index(&self, x: &str) -> usize {
        self.labels
            .get_index_of(x)
            .unwrap_or_else(|| panic!("No vertex with identifier `{x}`"))
    }

    #[inline]
    fn has_vertex_by_index(&self, x: usize) -> bool {
        // Check vertex existence.
        let f = self.labels.get_index(x).is_some();

        // Check iterator consistency.
        debug_assert_eq!(V!(self).any(|y| y == x), f);
        // Assert vertex set and vertices map are consistent.
        debug_assert_eq!(x < self.order(), f);

        f
    }

    fn del_vertex_by_index(&mut self, x: usize) -> bool {
        // Get vertex label and identifier.
        let (x, i) = (self.labels.shift_remove_index(x), x);

        // If vertex was not present ...
        if x.is_none() {
            // ... then return early.
            return false;
        }

        // Assert vertex has been removed.
        debug_assert!(!self.labels.contains(&x.unwrap()));
        // Assert vertex set is still sorted.
        debug_assert!(self.labels.iter().is_sorted());

        // Compute the new size of adjacency matrix.
        let n = self.adjacency_matrix.nrows();
        // Allocate new adjacency matrices.
        let mut directed_adjacency_matrix = DenseAdjacencyMatrix::from_elem((n - 1, n - 1), false);
        let mut undirected_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem((n - 1, n - 1), false);
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((n - 1, n - 1), false);
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

            adjacency_matrix
                .slice_mut(s![p_start, p_end])
                .assign(&self.adjacency_matrix.slice(s![q_start, q_end]));
        }
        // Replace old with new adjacency matrices.
        self.directed_adjacency_matrix = directed_adjacency_matrix;
        self.undirected_adjacency_matrix = undirected_adjacency_matrix;
        self.adjacency_matrix = adjacency_matrix;

        // Assert vertex set is still consistent with adjacency matrices shapes.
        debug_assert_eq!(self.labels.len(), self.directed_adjacency_matrix.nrows());
        debug_assert_eq!(self.labels.len(), self.undirected_adjacency_matrix.nrows());
        debug_assert_eq!(self.labels.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrices are still square.
        debug_assert!(self.directed_adjacency_matrix.is_square());
        debug_assert!(self.undirected_adjacency_matrix.is_square());
        debug_assert!(self.adjacency_matrix.is_square());
        // Assert adjacency matrices are still symmetric.
        debug_assert_eq!(
            self.undirected_adjacency_matrix,
            self.undirected_adjacency_matrix.t()
        );
        debug_assert_eq!(self.adjacency_matrix, self.adjacency_matrix.t());

        true
    }
    #[inline]
    fn size(&self) -> usize {
        // Check iterator consistency.
        debug_assert_eq!(E!(self).len(), self.size);

        self.size
    }

    #[inline]
    fn get_edges_index(&self) -> Self::EdgesIndexIter<'_> {
        Self::EdgesIndexIter::new(self)
    }

    #[inline]
    fn has_edge_by_index(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]]
    }

    #[inline]
    fn add_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        // If edge already exists ...
        if self.adjacency_matrix[[x, y]] {
            // ... return early.
            return false;
        }

        // Add edge.
        self.undirected_adjacency_matrix[[x, y]] = true;
        self.undirected_adjacency_matrix[[y, x]] = true;

        self.adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[y, x]] = true;

        // Increment sizes.
        self.size += 1;
        self.undirected_size += 1;

        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.undirected_adjacency_matrix[[y, x]]
        );
        debug_assert_eq!(self.adjacency_matrix[[x, y]], self.adjacency_matrix[[y, x]]);

        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.adjacency_matrix[[x, y]]
        );
        // Assert size counter and adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_size,
            self.undirected_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );

        debug_assert_eq!(
            self.directed_size,
            self.directed_adjacency_matrix
                .iter()
                .map(|&f| f as usize)
                .sum::<usize>()
        );

        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );

        true
    }

    #[inline]
    fn del_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        if !self.has_edge_by_index(x, y) {
            return false;
        }
        match self.has_undirected_edge_by_index(x, y) {
            true => {
                // Remove edge from undirected adjacency matrix.
                self.undirected_adjacency_matrix[[x, y]] = false;
                self.undirected_adjacency_matrix[[y, x]] = false;
                // Decrement undirected size.
                self.undirected_size -= 1;
            }
            _ => {
                // Remove edge from directed adjacency matrix.
                self.directed_adjacency_matrix[[x, y]] = false;
                self.directed_adjacency_matrix[[y, x]] = false;
                // Decrement directed size.
                self.directed_size -= 1;
            }
        };

        // Remove edge from skeleton matrix.

        self.adjacency_matrix[[x, y]] = false;
        self.adjacency_matrix[[y, x]] = false;

        // Decrement size.
        self.size -= 1;

        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.undirected_adjacency_matrix[[y, x]]
        );
        debug_assert_eq!(self.adjacency_matrix[[x, y]], self.adjacency_matrix[[y, x]]);

        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.adjacency_matrix[[x, y]]
        );

        debug_assert_eq!(
            self.directed_adjacency_matrix[[x, y]],
            self.adjacency_matrix[[x, y]]
        );
        // Assert size counter and adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_size,
            self.undirected_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );

        debug_assert_eq!(
            self.directed_size,
            self.directed_adjacency_matrix
                .iter()
                .map(|&f| f as usize)
                .sum::<usize>()
        );

        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );

        true
    }

    #[inline]
    fn get_adjacents_index(&self, x: usize) -> Self::AdjacentsIndexIter<'_> {
        Self::AdjacentsIndexIter::new(self, x)
    }

    #[inline]
    fn is_adjacent_by_index(&self, x: usize, y: usize) -> bool {
        // Check using has_edge.
        let f = self.has_edge_by_index(x, y);

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
            undirected_adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            directed_adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            undirected_size: 0,
            directed_size: 0,
            size: 0,
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

impl From<UndirectedDenseAdjacencyMatrixGraph> for PartiallyDenseAdjacencyMatrixGraph {
    fn from(undirected_graph: UndirectedDenseAdjacencyMatrixGraph) -> Self {
        let vertices: Vec<_> = undirected_graph.get_vertices().collect();
        let edges: EdgeList<_> = undirected_graph
            .get_edges_index()
            .map(|(x, y)| {
                (
                    undirected_graph.get_vertex_by_index(x),
                    undirected_graph.get_vertex_by_index(y),
                )
            })
            .collect();
        Self::new(vertices, edges)
    }
}

impl From<DirectedDenseAdjacencyMatrixGraph> for PartiallyDenseAdjacencyMatrixGraph {
    fn from(directed_graph: DirectedDenseAdjacencyMatrixGraph) -> Self {
        let vertices: Vec<_> = directed_graph.get_vertices().collect();
        let edges: EdgeList<_> = directed_graph
            .get_edges_index()
            .map(|(x, y)| {
                (
                    directed_graph.get_vertex_by_index(x),
                    directed_graph.get_vertex_by_index(y),
                )
            })
            .collect();
        Self::new_pagraph(vertices, [], edges)
    }
}

impl<V, I> From<(I, DenseAdjacencyMatrix)> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    fn from((vertices, adjacency_matrix): (I, DenseAdjacencyMatrix)) -> Self {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Sort labels.
        labels.sort();
        // Compute order
        let order = labels.len();
        // Check if vertex set is not consistent with given adjacency matrix.
        if order != adjacency_matrix.nrows() {
            panic!("Matrix must be consistent with inputs");
        }
        // Check if adjacency matrix is not square.
        if !adjacency_matrix.is_square() {
            panic!("Matrix must be square");
        }
        // Check if adjacency matrix is symmetric
        if adjacency_matrix != adjacency_matrix.t() {
            panic!("Matrix must be symmetric");
        }

        // Cast to standard memory layout (i.e. C layout), if not already.
        let adjacency_matrix = adjacency_matrix.as_standard_layout().into_owned();

        // Compute size.
        let size = adjacency_matrix.mapv(|f| f as usize).sum();

        Self {
            labels,
            directed_adjacency_matrix: DenseAdjacencyMatrix::from_elem((order, order), false),
            undirected_adjacency_matrix: adjacency_matrix.clone(),
            adjacency_matrix,
            undirected_size: size,
            directed_size: 0,
            size,
        }
    }
}

impl<V> From<(EdgeList<V>, EdgeList<V>)> for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
{
    #[inline]
    fn from((undirected_edges, directed_edges): (EdgeList<V>, EdgeList<V>)) -> Self {
        Self::new_pagraph([], undirected_edges, directed_edges)
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

        Self::new_pagraph(vertices, undirected_edges, directed_edges)
    }
}

impl<V, I> From<(I, DenseAdjacencyMatrix, DenseAdjacencyMatrix)>
    for PartiallyDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    fn from(
        (vertices, undirected_adjacency_matrix, directed_adjacency_matrix): (
            I,
            DenseAdjacencyMatrix,
            DenseAdjacencyMatrix,
        ),
    ) -> Self {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Sort labels.
        labels.sort();
        // Compute order
        let order = labels.len();
        // Check if vertex set is not consistent with given undirected adjacency matrix.
        if order != undirected_adjacency_matrix.nrows() {
            panic!("Matrix must be consistent with inputs");
        }
        // Check if undirected adjacency matrix is not square.
        if !undirected_adjacency_matrix.is_square() {
            panic!("Matrix must be square");
        }
        // Check if adjacency matrices have consistent dimensions.
        if undirected_adjacency_matrix.dim() != directed_adjacency_matrix.dim() {
            panic!("Matrix must be consistent with inputs");
        }
        // Check if undirected adjacency matrix is symmetric.
        if undirected_adjacency_matrix != undirected_adjacency_matrix.t() {
            panic!("Matrix must be symmetric");
        }

        // Check if adjacency matrices don't overlap.
        if (&undirected_adjacency_matrix & &directed_adjacency_matrix)
            .iter()
            .any(|x| *x)
        {
            panic!("Graph can only have one edge type between two nodes");
        }
        // Cast to standard memory layout (i.e. C layout), if not already.
        let adjacency_matrix = &undirected_adjacency_matrix
            | &directed_adjacency_matrix
            | directed_adjacency_matrix.t();
        let undirected_adjacency_matrix = undirected_adjacency_matrix
            .as_standard_layout()
            .into_owned();
        let directed_adjacency_matrix = directed_adjacency_matrix.as_standard_layout().into_owned();

        // Compute sizes.
        let undirected_size = undirected_adjacency_matrix
            .indexed_iter()
            .map(|((i, j), &f)| if i <= j { f as usize } else { 0 })
            .sum();
        let directed_size = directed_adjacency_matrix
            .iter()
            .map(|&f| f as usize)
            .sum::<usize>();
        let size = adjacency_matrix
            .indexed_iter()
            .map(|((i, j), &f)| if i <= j { f as usize } else { 0 })
            .sum();

        Self {
            labels,
            directed_adjacency_matrix,
            undirected_adjacency_matrix,
            adjacency_matrix,
            undirected_size,
            directed_size,
            size,
        }
    }
}

/* Implement Into traits. */

#[allow(clippy::from_over_into)]
impl Into<EdgeList<String>> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> EdgeList<String> {
        E!(self)
            .map(|(x, y)| {
                (
                    self.get_vertex_by_index(x).into(),
                    self.get_vertex_by_index(y).into(),
                )
            })
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<AdjacencyList<String>> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> AdjacencyList<String> {
        V!(self)
            .map(|x| {
                (
                    self.get_vertex_by_index(x).into(),
                    iter_set::union(Ne!(self, x), Ch!(self, x))
                        .map(|y| self.get_vertex_by_index(y).into())
                        .collect(),
                )
            })
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<(FxIndexSet<String>, DenseAdjacencyMatrix)> for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn into(self) -> (FxIndexSet<String>, DenseAdjacencyMatrix) {
        (self.labels, self.adjacency_matrix)
    }
}

#[allow(clippy::from_over_into)]
impl Into<(EdgeList<String>, EdgeList<String>)> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> (EdgeList<String>, EdgeList<String>) {
        (
            uE!(self)
                .map(|(x, y)| {
                    (
                        self.get_vertex_by_index(x).into(),
                        self.get_vertex_by_index(y).into(),
                    )
                })
                .collect(),
            uE!(self)
                .map(|(x, y)| {
                    (
                        self.get_vertex_by_index(x).into(),
                        self.get_vertex_by_index(y).into(),
                    )
                })
                .collect(),
        )
    }
}

#[allow(clippy::from_over_into)]
impl Into<(AdjacencyList<String>, AdjacencyList<String>)> for PartiallyDenseAdjacencyMatrixGraph {
    fn into(self) -> (AdjacencyList<String>, AdjacencyList<String>) {
        let ch_map = V!(self)
            .map(|x| {
                (
                    self.get_vertex_by_index(x).into(),
                    Ch!(self, x)
                        .map(|y| self.get_vertex_by_index(y).into())
                        .collect(),
                )
            })
            .collect();

        let ne_map = V!(self)
            .map(|x| {
                (
                    self.get_vertex_by_index(x).into(),
                    Ne!(self, x)
                        .map(|y| self.get_vertex_by_index(y).into())
                        .collect(),
                )
            })
            .collect();
        (ch_map, ne_map)
    }
}

#[allow(clippy::from_over_into)]
impl
    Into<(
        FxIndexSet<String>,
        DenseAdjacencyMatrix,
        DenseAdjacencyMatrix,
    )> for PartiallyDenseAdjacencyMatrixGraph
{
    #[inline]
    fn into(
        self,
    ) -> (
        FxIndexSet<String>,
        DenseAdjacencyMatrix,
        DenseAdjacencyMatrix,
    ) {
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
        labels && undirected && directed
    }
}

impl Eq for PartiallyDenseAdjacencyMatrixGraph {}

impl PartialOrd for PartiallyDenseAdjacencyMatrixGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare vertices sets.
        let partial_cmp = iter_set::cmp(
            V!(self).map(|x| self.get_vertex_by_index(x)),
            V!(other).map(|x| other.get_vertex_by_index(x)),
        );
        // If the vertices sets are comparable ...
        partial_cmp.and_then(|vertices| {
            // ... compare undirected edges sets.
            let partial_cmp = iter_set::cmp(
                uE!(self).map(|(x, y)| (self.get_vertex_by_index(x), self.get_vertex_by_index(y))),
                uE!(other)
                    .map(|(x, y)| (other.get_vertex_by_index(x), other.get_vertex_by_index(y))),
            );
            // If the undirected edges sets are comparable ...
            partial_cmp.and_then(|undirected_edges| {
                // ... compare directed edges sets.
                let partial_cmp = iter_set::cmp(
                    dE!(self)
                        .map(|(x, y)| (self.get_vertex_by_index(x), self.get_vertex_by_index(y))),
                    dE!(other)
                        .map(|(x, y)| (other.get_vertex_by_index(x), other.get_vertex_by_index(y))),
                );
                // If also the directed edges sets are comparable ...
                partial_cmp.and_then(|directed_edges| {
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
        let mut adjacency_matrix = Self::Data::from_elem(self.adjacency_matrix.dim(), false);
        // Fill the adjacency matrix.
        for (x, y) in edges {
            // Add the edge.
            adjacency_matrix[[x, y]] = true;
            adjacency_matrix[[y, x]] = true;
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
        let adjacency_matrix = adjacency_matrix
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
        undirected_adjacency_matrix = undirected_adjacency_matrix & adjacency_matrix.clone();
        directed_adjacency_matrix = directed_adjacency_matrix & adjacency_matrix.clone();

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());
        // Assert matrix dimensions are still consistent
        debug_assert!(undirected_adjacency_matrix.dim() == adjacency_matrix.dim());
        debug_assert!(directed_adjacency_matrix.dim() == adjacency_matrix.dim());

        // Build subgraph from vertices and adjacency matrix.
        Self::from((
            vertices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
        ))
    }

    fn subgraph_by_vertices<I>(&self, vertices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Remove duplicated vertices identifiers.
        let indices: FxIndexSet<_> = vertices.into_iter().collect();
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
        let adjacency_matrix = self
            .adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());
        // Assert matrix dimensions are still consistent
        debug_assert!(undirected_adjacency_matrix.dim() == adjacency_matrix.dim());
        debug_assert!(directed_adjacency_matrix.dim() == adjacency_matrix.dim());

        // Build subgraph from vertices and adjacency matrix.
        Self::from((
            vertices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
        ))
    }

    fn subgraph_by_edges<J>(&self, edges: J) -> Self
    where
        J: IntoIterator<Item = (usize, usize)>,
    {
        // Initialize new indices.
        let mut indices = vec![false; self.order()];

        // Initialize a new adjacency matrix.
        let mut adjacency_matrix = Self::Data::from_elem(self.adjacency_matrix.dim(), false);
        // Fill the adjacency matrix.
        for (x, y) in edges {
            // Add the edge.
            adjacency_matrix[[x, y]] = true;
            adjacency_matrix[[y, x]] = true;
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
        let adjacency_matrix = adjacency_matrix
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
        undirected_adjacency_matrix = undirected_adjacency_matrix & adjacency_matrix.clone();
        directed_adjacency_matrix = directed_adjacency_matrix & adjacency_matrix.clone();

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());
        // Assert matrix dimensions are still consistent
        debug_assert!(undirected_adjacency_matrix.dim() == adjacency_matrix.dim());
        debug_assert!(directed_adjacency_matrix.dim() == adjacency_matrix.dim());

        // Build subgraph from vertices and adjacency matrix.
        Self::from((
            vertices,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
        ))
    }
}

/* Implement UndirectedGraph trait. */
impl UndirectedGraph for PartiallyDenseAdjacencyMatrixGraph {
    type UndirectedEdgesIndexIter<'a> = EdgesIterator<'a>;
    type NeighborsIndexIter<'a> = Self::AdjacentsIndexIter<'a>;

    #[inline]
    fn size_of_maximal_undirected_subgraph(&self) -> usize {
        self.undirected_size
    }

    #[inline]
    fn get_undirected_edges_index(&self) -> Self::UndirectedEdgesIndexIter<'_> {
        Self::UndirectedEdgesIndexIter::new_undirected(self)
    }

    #[inline]
    fn get_neighbors_by_index(&self, x: usize) -> Self::NeighborsIndexIter<'_> {
        Self::NeighborsIndexIter::new_undirected(self, x)
    }

    #[inline]
    fn is_neighbor_by_index(&self, x: usize, y: usize) -> bool {
        self.undirected_adjacency_matrix[[x, y]]
    }

    #[inline]
    fn has_undirected_edge_by_index(&self, x: usize, y: usize) -> bool {
        self.undirected_adjacency_matrix[[x, y]]
    }

    #[inline]
    fn get_degree_by_index(&self, x: usize) -> usize {
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
    #[inline]
    fn add_undirected_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        // If edge already exists ...
        if self.adjacency_matrix[[x, y]] {
            debug_assert!(self.adjacency_matrix[[x, y]] == self.adjacency_matrix[[y, x]]);
            // ... return early.
            return false;
        }
        // Add edge.
        self.undirected_adjacency_matrix[[x, y]] = true;
        self.undirected_adjacency_matrix[[y, x]] = true;
        self.adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[y, x]] = true;

        // Increment sizes.
        self.undirected_size += 1;
        self.size += 1;

        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.undirected_adjacency_matrix[[y, x]]
        );
        debug_assert_eq!(
            self.undirected_adjacency_matrix[[x, y]],
            self.adjacency_matrix[[x, y]]
        );
        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(self.adjacency_matrix[[x, y]], self.adjacency_matrix[[y, x]]);
        // Assert size counter and adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_size,
            self.undirected_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );
        debug_assert_eq!(
            self.directed_size,
            self.directed_adjacency_matrix
                .iter()
                .map(|&f| f as usize)
                .sum::<usize>()
        );
        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );

        true
    }
}

/* Implement DirectedGraph trait. */

#[allow(dead_code, clippy::type_complexity)]
pub struct AncestorsIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<<ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter>,
        fn((usize, bool)) -> Option<usize>,
    >,
}

impl<'a> AncestorsIterator<'a> {
    /// Constructor.
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: {
                // Get underlying directed adjacency matrix.
                let directed_adjacency_matrix = &g.directed_adjacency_matrix;
                // Initialize previous solution.
                let mut prev = Array1::from_elem((directed_adjacency_matrix.ncols(),), false);
                // Get current ancestors set, i.e. parents set.
                let mut curr = directed_adjacency_matrix.column(x).to_owned();

                // Check stopping criterion.
                while curr != prev {
                    // Update previous.
                    prev.assign(&curr);
                    // Select current parents.
                    let next = directed_adjacency_matrix & &curr;
                    // Collapse new parents.
                    let next = next.fold_axis(Axis(1), false, |acc, f| acc | f);
                    // Accumulate new parents.
                    curr = curr | next;
                }

                curr.into_iter().enumerate().filter_map(|(x, f)| match f {
                    true => Some(x),
                    false => None,
                })
            },
        }
    }
}

impl<'a> Iterator for AncestorsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> FusedIterator for AncestorsIterator<'a> {}

#[allow(dead_code, clippy::type_complexity)]
pub struct ParentsIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

impl<'a> ParentsIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: g
                .directed_adjacency_matrix
                .column(x)
                .into_iter()
                .enumerate()
                .filter_map(|(i, &f)| match f {
                    true => Some(i),
                    false => None,
                }),
        }
    }
}

impl<'a> Iterator for ParentsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> FusedIterator for ParentsIterator<'a> {}

#[allow(dead_code, clippy::type_complexity)]
pub struct ChildrenIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

impl<'a> ChildrenIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: g
                .directed_adjacency_matrix
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

impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> FusedIterator for ChildrenIterator<'a> {}

#[allow(dead_code, clippy::type_complexity)]
pub struct DescendantsIterator<'a> {
    g: &'a PartiallyDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<<ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter>,
        fn((usize, bool)) -> Option<usize>,
    >,
}

impl<'a> DescendantsIterator<'a> {
    /// Constructor.
    pub fn new(g: &'a PartiallyDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: {
                // Get underlying directed adjacency matrix.
                let directed_adjacency_matrix = &g.directed_adjacency_matrix;
                // Initialize previous solution.
                let mut prev = Array1::from_elem((directed_adjacency_matrix.ncols(),), false);
                // Get current ancestors set, i.e. parents set.
                let mut curr = directed_adjacency_matrix.row(x).to_owned();

                // Check stopping criterion.
                while curr != prev {
                    // Update previous.
                    prev.assign(&curr);
                    // Select current parents.
                    let next = &directed_adjacency_matrix.t() & &curr;
                    // Collapse new parents.
                    let next = next.fold_axis(Axis(1), false, |acc, f| acc | f);
                    // Accumulate new parents.
                    curr = curr | next;
                }

                curr.into_iter().enumerate().filter_map(|(x, f)| match f {
                    true => Some(x),
                    false => None,
                })
            },
        }
    }
}

impl<'a> Iterator for DescendantsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> FusedIterator for DescendantsIterator<'a> {}

impl DirectedGraph for PartiallyDenseAdjacencyMatrixGraph {
    type DirectedEdgesIndexIter<'a> = EdgesIterator<'a>;

    type AncestorsIndexIter<'a> = AncestorsIterator<'a>;

    type ParentsIndexIter<'a> = ParentsIterator<'a>;

    type ChildrenIndexIter<'a> = ChildrenIterator<'a>;

    type DescendantsIndexIter<'a> = DescendantsIterator<'a>;

    #[inline]
    fn size_of_maximal_directed_subgraph(&self) -> usize {
        self.directed_size
    }

    #[inline]
    fn get_directed_edges_index(&self) -> Self::DirectedEdgesIndexIter<'_> {
        Self::DirectedEdgesIndexIter::new_directed(self)
    }

    #[inline]
    fn get_ancestors_by_index(&self, x: usize) -> Self::AncestorsIndexIter<'_> {
        Self::AncestorsIndexIter::new(self, x)
    }

    #[inline]
    fn get_parents_by_index(&self, x: usize) -> Self::ParentsIndexIter<'_> {
        Self::ParentsIndexIter::new(self, x)
    }

    #[inline]
    fn is_parent_by_index(&self, x: usize, y: usize) -> bool {
        self.directed_adjacency_matrix[[y, x]]
    }

    #[inline]
    fn get_children_by_index(&self, x: usize) -> Self::ChildrenIndexIter<'_> {
        Self::ChildrenIndexIter::new(self, x)
    }

    #[inline]
    fn is_child_by_index(&self, x: usize, y: usize) -> bool {
        self.directed_adjacency_matrix[[x, y]]
    }

    #[inline]
    fn get_descendants_by_index(&self, x: usize) -> Self::DescendantsIndexIter<'_> {
        Self::DescendantsIndexIter::new(self, x)
    }

    #[inline]
    fn has_directed_edge_by_index(&self, x: usize, y: usize) -> bool {
        self.directed_adjacency_matrix[[x, y]]
    }

    #[inline]
    fn get_in_degree_by_index(&self, x: usize) -> usize {
        // Compute in-degree.
        let d = self
            .directed_adjacency_matrix
            .column(x)
            .mapv(|f| f as usize)
            .sum();

        // Check iterator consistency.
        debug_assert_eq!(Pa!(self, x).count(), d);

        d
    }

    #[inline]
    fn get_out_degree_by_index(&self, x: usize) -> usize {
        // Compute out-degree.
        let d = self
            .directed_adjacency_matrix
            .row(x)
            .mapv(|f| f as usize)
            .sum();

        // Check iterator consistency.
        debug_assert_eq!(Ch!(self, x).count(), d);

        d
    }

    #[inline]
    fn add_directed_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        // If edge already exists ...
        if self.adjacency_matrix[[x, y]] {
            debug_assert!(self.adjacency_matrix[[x, y]] == self.adjacency_matrix[[y, x]]);
            // ... return early.
            return false;
        } // Add edge.
        self.directed_adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[y, x]] = true;

        // Increment size.
        self.directed_size += 1;
        self.size += 1;
        debug_assert_eq!(
            self.directed_adjacency_matrix[[x, y]],
            self.adjacency_matrix[[x, y]]
        );
        // Assert adjacency matrices are still consistent.
        debug_assert_eq!(self.adjacency_matrix[[x, y]], self.adjacency_matrix[[y, x]]);
        // Assert size counter and adjacency matrices are still consistent.
        debug_assert_eq!(
            self.undirected_size,
            self.undirected_adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );
        debug_assert_eq!(
            self.directed_size,
            self.directed_adjacency_matrix
                .iter()
                .map(|&f| f as usize)
                .sum::<usize>()
        );
        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), &f)| match i <= j {
                    true => Some(f as usize),
                    false => None,
                })
                .sum::<usize>()
        );

        true
    }
}

/* Implement PartiallyDirectedGraph trait. */
impl IntoUndirectedGraph for PartiallyDenseAdjacencyMatrixGraph {
    type UndirectedGraph = UndirectedDenseAdjacencyMatrixGraph;

    #[inline]
    fn to_undirected(&self) -> Self::UndirectedGraph {
        Self::UndirectedGraph::from((self.labels.clone(), self.adjacency_matrix.clone()))
    }
}

impl PartiallyDirectedGraph for PartiallyDenseAdjacencyMatrixGraph {
    type EdgesIndexIter<'a> = EdgesIterator<'a>;

    fn new_pagraph<V, I, J, K>(vertices: I, undirected_edges: J, directed_edges: K) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
        K: IntoIterator<Item = (V, V)>,
    {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = vertices.into_iter().map(|x| x.into()).collect();
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
        // Sort labels.
        labels.sort();
        // Compute new graph order.
        let order = labels.len();
        // Initialize adjacency matrices given graph order.
        let mut undirected_adjacency_matrix =
            DenseAdjacencyMatrix::from_elem((order, order), false);
        let mut directed_adjacency_matrix = undirected_adjacency_matrix.clone();
        let mut adjacency_matrix = undirected_adjacency_matrix.clone();

        // Initialize the sizes.
        let mut size = 0;
        let mut undirected_size = 0;
        let mut directed_size = 0;
        // Fill skeleton adjacency matrix given edge set.
        for (x, y) in undirected_edges {
            // Get associated vertices indices.
            let (i, j) = (
                labels.get_index_of(&x).unwrap(),
                labels.get_index_of(&y).unwrap(),
            );
            // Set edge given indices.
            if !adjacency_matrix[[i, j]] {
                // Add edge.
                undirected_adjacency_matrix[[i, j]] = true;
                undirected_adjacency_matrix[[j, i]] = true;
                adjacency_matrix[[i, j]] = true;
                adjacency_matrix[[j, i]] = true;
                // Increment sizes.
                undirected_size += 1;
                size += 1;
            }
        }
        for (x, y) in directed_edges {
            // Get associated vertices indices.
            let (i, j) = (
                labels.get_index_of(&x).unwrap(),
                labels.get_index_of(&y).unwrap(),
            );
            // Set edge given indices.
            if !adjacency_matrix[[i, j]] {
                // Add edge.
                directed_adjacency_matrix[[i, j]] = true;
                adjacency_matrix[[i, j]] = true;
                adjacency_matrix[[j, i]] = true;
                // Increment size.
                directed_size += 1;
                size += 1;
            } else {
                // Panic if edges lists overlap
                panic!("Graph can only have one edge type between two nodes");
            }
        }
        // Assert if undirected adjacency matrix and skeleton matrix are symmetric

        debug_assert_eq!(undirected_adjacency_matrix, undirected_adjacency_matrix.t());
        debug_assert_eq!(adjacency_matrix, adjacency_matrix.t());

        Self {
            labels,
            undirected_adjacency_matrix,
            directed_adjacency_matrix,
            adjacency_matrix,
            undirected_size,
            directed_size,
            size,
        }
    }

    fn orient_edge(&mut self, x: usize, y: usize) -> bool {
        // If such edge exists and it is undirected
        if self.has_undirected_edge_by_index(x, y) {
            // Delete edge
            self.del_edge_by_index(x, y);

            // Add directed edge
            self.add_directed_edge_by_index(x, y);

            // Check if sizes are still consistent
            debug_assert!(self.size == (self.undirected_size + self.directed_size));

            // Check if directed adjacency matrix is built correctly
            debug_assert!(self.directed_adjacency_matrix[[x, y]]);
            debug_assert!(!self.directed_adjacency_matrix[[y, x]]);

            return true;
        }

        false
    }
}

/* Implement PathGraph */
impl PathGraph for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn has_path_by_index(&self, x: usize, y: usize) -> bool {
        let has_edge =
            self.has_undirected_edge_by_index(x, y) || self.has_directed_edge_by_index(x, y);
        has_edge || BFS::from((self, x)).skip(1).any(|z| z == y)
    }

    #[inline]
    fn is_acyclic(&self) -> bool {
        !DFSEdges::new(self, None, Traversal::Forest).any(|e| matches!(e, DFSEdge::Back(_, _)))
    }
}

impl MoralGraph for PartiallyDenseAdjacencyMatrixGraph {
    type MoralGraph = UndirectedDenseAdjacencyMatrixGraph;

    #[inline]
    fn moral(&self) -> Self::MoralGraph {
        // Make an undirected copy of the current graph.
        let mut h = self.to_undirected();
        // For each vertex ...
        for x in V!(self) {
            // ... for each pair of parents ...
            for e in Pa!(self, x).combinations(2) {
                // ... add an edge between them.
                h.add_edge_by_index(e[0], e[1]);
            }
        }

        h
    }
}

impl From<DOT> for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn from(other: DOT) -> Self {
        // Assert graph type.
        assert_eq!(
            other.graph_type, "digraph",
            "DOT graph type must match direction"
        );

        let undirected_edges: Vec<_> = other
            .edges
            .iter()
            .filter_map(|(t, e)| match e.attributes.get_edge_dir() {
                Some(x) => {
                    if x.as_str() == "none" {
                        Some((*t).clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        let directed_edges: Vec<_> = other
            .edges
            .iter()
            .filter_map(|(t, e)| match e.attributes.get_edge_dir() {
                Some(x) => {
                    if x.as_str() == "none" {
                        None
                    } else {
                        Some((*t).clone())
                    }
                }
                _ => Some((*t).clone()),
            })
            .collect();

        Self::new_pagraph(other.vertices.into_keys(), undirected_edges, directed_edges)
    }
}

impl MeekRules for PartiallyDenseAdjacencyMatrixGraph {
    #[inline]
    fn meek_1(&mut self) -> bool {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;
        for x in V!(self).collect::<Vec<_>>() {
            if Pa!(self, x).next().is_none() {
                continue;
            }
            for z in Ne!(self, x).collect::<Vec<_>>() {
                if iter_set::intersection(Adj!(self, z), Pa!(self, x))
                    .next()
                    .is_none()
                {
                    self.orient_edge(x, z);
                    is_closed = false;
                }
            }
        }
        is_closed
    }

    #[inline]
    fn meek_2(&mut self) -> bool {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;
        for x in V!(self).collect::<Vec<_>>() {
            if Pa!(self, x).next().is_none() {
                continue;
            }
            for z in Ch!(self, x).collect::<Vec<_>>() {
                for y in iter_set::intersection(Ne!(self, z), Pa!(self, x)).collect::<Vec<_>>() {
                    self.orient_edge(y, z);
                    is_closed = false;
                }
            }
        }
        is_closed
    }

    #[inline]
    fn meek_3(&mut self) -> bool {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;
        for x in V!(self).collect::<Vec<_>>() {
            for z in Ne!(self, x).collect::<Vec<_>>() {
                let intersection = iter_set::intersection(Ne!(self, z), Pa!(self, x));
                // Look for a non-adjacent couple of parents of `x`
                if intersection
                    .combinations(2)
                    .any(|ab| !self.is_adjacent_by_index(ab[0], ab[1]))
                {
                    self.orient_edge(z, x);
                    is_closed = false;
                }
            }
        }
        is_closed
    }

    #[inline]
    fn meek_4(&mut self) -> bool {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;
        for x in V!(self).collect::<Vec<_>>() {
            if Pa!(self, x).next().is_none() {
                continue;
            }
            for z in Ne!(self, x).collect::<Vec<_>>() {
                if iter_set::intersection(
                    Ne!(self, z),
                    Pa!(self, x).flat_map(|parent| {
                        Pa!(self, parent).filter(|&y| !self.is_adjacent_by_index(y, x))
                    }),
                )
                .next()
                .is_some()
                {
                    self.orient_edge(z, x);
                    is_closed = false;
                }
            }
        }
        is_closed
    }

    #[inline]
    fn meek_procedure_until_3(mut self) -> Self {
        let mut is_closed = false;
        while !is_closed {
            is_closed = self.meek_1();
            is_closed &= self.meek_2();
            is_closed &= self.meek_3();
        }
        self
    }

    #[inline]
    fn meek_procedure_until_4(mut self) -> Self {
        let mut is_closed = false;
        while !is_closed {
            is_closed = self.meek_1();
            is_closed &= self.meek_2();
            is_closed &= self.meek_3();
            is_closed &= self.meek_4();
        }
        self
    }
}
