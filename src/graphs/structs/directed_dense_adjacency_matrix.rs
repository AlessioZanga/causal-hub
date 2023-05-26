use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
    iter::{Enumerate, FilterMap, FusedIterator, Map},
    ops::{Deref, Range},
};

use is_sorted::IsSorted;
use itertools::{iproduct, Itertools};
use ndarray::{iter::IndexedIter, prelude::*, OwnedRepr};
use serde::{Deserialize, Serialize};

use super::UndirectedDenseAdjacencyMatrixGraph;
use crate::{
    graphs::{
        algorithms::traversal::{DFSEdge, DFSEdges, Traversal},
        directions, BaseGraph, DirectedGraph, IntoUndirectedGraph, PartialOrdGraph, PathGraph,
        SubGraph,
    },
    io::DOT,
    models::MoralGraph,
    prelude::BFS,
    types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, FxIndexSet},
    Adj, Ch, Pa, E, V,
};

/// Directed graph struct based on dense adjacency matrix data structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectedDenseAdjacencyMatrixGraph {
    labels: FxIndexSet<String>,
    adjacency_matrix: DenseAdjacencyMatrix,
    size: usize,
}

/* Implement BaseGraph trait. */
impl Deref for DirectedDenseAdjacencyMatrixGraph {
    type Target = DenseAdjacencyMatrix;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.adjacency_matrix
    }
}

#[allow(dead_code, clippy::type_complexity)]
pub struct EdgesIterator<'a> {
    g: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> EdgesIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g.indexed_iter().filter_map(|((x, y), &f)| match f {
                true => Some((x, y)),
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
    g: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<<ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter>,
        fn((usize, bool)) -> Option<usize>,
    >,
}

impl<'a> AdjacentsIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: {
                let (row, col) = (g.row(x), g.column(x));

                (&row | &col)
                    .into_iter()
                    .enumerate()
                    .filter_map(|(x, f)| match f {
                        true => Some(x),
                        false => None,
                    })
            },
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

impl Display for DirectedDenseAdjacencyMatrixGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "DirectedGraph {{ ")?;
        // Write vertex set.
        write!(
            f,
            "V = {{{}}}, ",
            V!(self)
                .map(|x| format!("\"{}\"", self.get_vertex_by_index(x)))
                .join(", ")
        )?;
        // Write edge set.
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

impl Hash for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.iter().for_each(|x| x.hash(state));
        self.adjacency_matrix.hash(state);
    }
}

impl Default for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn default() -> Self {
        Self {
            labels: Default::default(),
            adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            size: 0,
        }
    }
}

impl BaseGraph for DirectedDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::Directed;

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
        let mut labels: FxIndexSet<_> = vertices.into_iter().map_into().collect();
        // Map edges iterator into edge list.
        let edges: FxIndexSet<_> = edges
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect();
        // Add missing vertices from the edges.
        labels.extend(edges.iter().cloned().flat_map(|(x, y)| [x, y]));
        // Sort labels.
        labels.sort();

        // Compute new graph order.
        let order = labels.len();
        // Initialize adjacency matrix given graph order.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

        // Initialize the size.
        let mut size = 0;
        // Fill adjacency matrix given edge set.
        for (x, y) in edges {
            // Get associated vertices indices.
            let (i, j) = (
                labels.get_index_of(&x).unwrap(),
                labels.get_index_of(&y).unwrap(),
            );
            // Set edge given indices.
            adjacency_matrix[[i, j]] = true;
            // Increment size.
            size += 1;
        }

        // Assert vertex set is still sorted.
        debug_assert!(labels.iter().is_sorted());
        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(labels.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());

        Self {
            labels,
            adjacency_matrix,
            size,
        }
    }

    fn null() -> Self {
        Default::default()
    }

    fn empty<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = vertices.into_iter().map_into().collect();
        // Sort labels.
        labels.sort();

        // Compute new graph order.
        let order = labels.len();
        // Initialize adjacency matrix given graph order.
        let adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

        // Assert vertex set is still sorted.
        debug_assert!(labels.iter().is_sorted());
        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(labels.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());

        Self {
            labels,
            adjacency_matrix,
            size: 0,
        }
    }

    fn complete<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = vertices.into_iter().map_into().collect();
        // Sort labels.
        labels.sort();

        // Compute new graph order.
        let order = labels.len();
        // Initialize adjacency matrix given graph order.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), true);
        // Remove self loops.
        adjacency_matrix.diag_mut().map_inplace(|x| *x = false);

        // Compute size.
        let size = order * (order.saturating_sub(1));

        // Assert vertex set is still sorted.
        debug_assert!(labels.iter().is_sorted());
        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(labels.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());

        Self {
            labels,
            adjacency_matrix,
            size,
        }
    }

    #[inline]
    fn clear(&mut self) {
        // Clear the vertices map.
        self.labels.clear();
        // Clear the adjacency matrix.
        self.adjacency_matrix = Default::default();
        // Clear the size.
        self.size = 0;
    }

    #[inline]
    fn order(&self) -> usize {
        // Check iterator consistency.
        debug_assert_eq!(V!(self).len(), self.labels.len());
        // Assert vertex set is consistent with adjacency matrix shape.
        debug_assert_eq!(self.labels.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is square.
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
        // Allocate new adjacency matrix.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((n + 1, n + 1), false);
        // Compute blocks.
        let (p, q) = ([0..i, (i + 1)..(n + 1)], [0..i, i..n]);
        let (p, q) = (iproduct!(p.clone(), p), iproduct!(q.clone(), q));
        // Copy old adjacency matrix using blocks operations.
        for ((p_start, p_end), (q_start, q_end)) in p.zip(q) {
            adjacency_matrix
                .slice_mut(s![p_start, p_end])
                .assign(&self.adjacency_matrix.slice(s![q_start, q_end]));
        }
        // Replace old with new adjacency matrix.
        self.adjacency_matrix = adjacency_matrix;

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(self.labels.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(self.adjacency_matrix.is_square());

        // Return new vertex index.
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
        // Allocate new adjacency matrix.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((n - 1, n - 1), false);
        // Compute blocks.
        let (p, q) = ([0..i, i..(n - 1)], [0..i, (i + 1)..n]);
        let (p, q) = (iproduct!(p.clone(), p), iproduct!(q.clone(), q));
        // Copy old adjacency matrix using blocks operations.
        for ((p_start, p_end), (q_start, q_end)) in p.zip(q) {
            adjacency_matrix
                .slice_mut(s![p_start, p_end])
                .assign(&self.adjacency_matrix.slice(s![q_start, q_end]));
        }
        // Replace old with new adjacency matrix.
        self.adjacency_matrix = adjacency_matrix;

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(self.labels.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(self.adjacency_matrix.is_square());

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
        self.adjacency_matrix[[x, y]] = true;
        // Increment size.
        self.size += 1;

        // Assert size counter and adjacency matrix are still consistent.
        debug_assert_eq!(self.size, self.adjacency_matrix.mapv(|f| f as usize).sum());

        true
    }

    #[inline]
    fn del_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        // If edge does not exists ...
        if !self.adjacency_matrix[[x, y]] {
            // ... return early.
            return false;
        }

        // Remove edge.
        self.adjacency_matrix[[x, y]] = false;
        // Decrement size.
        self.size -= 1;

        // Assert size counter and adjacency matrix are still consistent.
        debug_assert_eq!(self.size, self.adjacency_matrix.mapv(|f| f as usize).sum());

        true
    }

    #[inline]
    fn get_adjacents_index(&self, x: usize) -> Self::AdjacentsIndexIter<'_> {
        Self::AdjacentsIndexIter::new(self, x)
    }

    #[inline]
    fn is_adjacent_by_index(&self, x: usize, y: usize) -> bool {
        // Check using has_edge.
        let f = self.has_edge_by_index(x, y) || self.has_edge_by_index(y, x);

        // Check iterator consistency.
        debug_assert_eq!(Adj!(self, x).any(|z| z == y), f);

        f
    }
}

/* Implement TryFrom traits. */
impl<V> From<EdgeList<V>> for DirectedDenseAdjacencyMatrixGraph
where
    V: Into<String>,
{
    #[inline]
    fn from(edge_list: EdgeList<V>) -> Self {
        Self::new([], edge_list)
    }
}

impl<V> From<AdjacencyList<V>> for DirectedDenseAdjacencyMatrixGraph
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

impl<V, I> From<(I, DenseAdjacencyMatrix)> for DirectedDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    fn from((vertices, adjacency_matrix): (I, DenseAdjacencyMatrix)) -> Self {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = vertices.into_iter().map_into().collect();
        // Sort labels.
        labels.sort();

        // Check if vertex set is not consistent with given adjacency matrix.
        if labels.len() != adjacency_matrix.nrows() {
            panic!("Matrix must be consistent with inputs");
        }
        // Check if adjacency matrix is not square.
        if !adjacency_matrix.is_square() {
            panic!("Matrix must be square");
        }

        // Cast to standard memory layout (i.e. C layout), if not already.
        let adjacency_matrix = adjacency_matrix.as_standard_layout().into_owned();

        // Compute size.
        let size = adjacency_matrix.mapv(|f| f as usize).sum();

        // Assert vertex set is still sorted.
        debug_assert!(labels.iter().is_sorted());
        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(labels.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());

        Self {
            labels,
            adjacency_matrix,
            size,
        }
    }
}

/* Implement Into traits. */

#[allow(clippy::from_over_into)]
impl Into<EdgeList<String>> for DirectedDenseAdjacencyMatrixGraph {
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
impl Into<AdjacencyList<String>> for DirectedDenseAdjacencyMatrixGraph {
    fn into(self) -> AdjacencyList<String> {
        V!(self)
            .map(|x| {
                (
                    self.get_vertex_by_index(x).into(),
                    Ch!(self, x)
                        .map(|y| self.get_vertex_by_index(y).into())
                        .collect(),
                )
            })
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<(FxIndexSet<String>, DenseAdjacencyMatrix)> for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn into(self) -> (FxIndexSet<String>, DenseAdjacencyMatrix) {
        (self.labels, self.adjacency_matrix)
    }
}

/* Implement PartialOrdGraph trait. */
impl PartialEq for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Check that V(\mathcal{G}) == V(\mathcal{H}) && E(\mathcal{G}) == E(\mathcal{H}).
        self.labels.eq(&other.labels) && self.adjacency_matrix.eq(&other.adjacency_matrix)
    }
}

impl Eq for DirectedDenseAdjacencyMatrixGraph {}

impl PartialOrd for DirectedDenseAdjacencyMatrixGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare vertices sets.
        let partial_cmp = iter_set::cmp(
            V!(self).map(|x| self.get_vertex_by_index(x)),
            V!(other).map(|x| other.get_vertex_by_index(x)),
        );
        // If the vertices sets are comparable ...
        partial_cmp.and_then(|vertices| {
            // ... compare edges sets.
            let partial_cmp = iter_set::cmp(
                E!(self).map(|(x, y)| (self.get_vertex_by_index(x), self.get_vertex_by_index(y))),
                E!(other)
                    .map(|(x, y)| (other.get_vertex_by_index(x), other.get_vertex_by_index(y))),
            );
            // If the edges sets are comparable ...
            partial_cmp.and_then(|edges| {
                // ... then return ordering.
                match (vertices, edges) {
                    // If vertices and edges are the same, then ordering is determined.
                    (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
                    (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
                    // If either vertices or edges are equal, the rest determines the order.
                    (_, Ordering::Equal) => Some(vertices),
                    (Ordering::Equal, _) => Some(edges),
                    // Every other combination does not determine an order.
                    _ => None,
                }
            })
        })
    }
}

impl PartialOrdGraph for DirectedDenseAdjacencyMatrixGraph {}

/* Implement SubGraph trait. */
impl SubGraph for DirectedDenseAdjacencyMatrixGraph {
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

        // Initialize a new adjacency matrix.
        let mut adjacency_matrix = Self::Data::from_elem(self.adjacency_matrix.dim(), false);
        // Fill the adjacency matrix.
        for (x, y) in edges {
            // Add the edge.
            adjacency_matrix[[x, y]] = true;
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

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());

        // Build subgraph from vertices and adjacency matrix.
        Self::try_from((vertices, adjacency_matrix)).unwrap()
    }

    fn subgraph_by_vertices<I>(&self, vertices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Remove duplicated vertices identifiers.
        let indices: FxIndexSet<_> = vertices.into_iter().collect();
        // Cast to vector of indices.
        let indices = indices.into_iter().collect_vec();

        // Get minor of matrix.
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

        // Build subgraph from vertices and adjacency matrix.
        Self::try_from((vertices, adjacency_matrix)).unwrap()
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

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(vertices.len(), adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(adjacency_matrix.is_square());

        // Build subgraph from vertices and adjacency matrix.
        Self::try_from((vertices, adjacency_matrix)).unwrap()
    }
}

/* Implement DirectedGraph trait. */

#[allow(dead_code, clippy::type_complexity)]
pub struct AncestorsIterator<'a> {
    g: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<<ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter>,
        fn((usize, bool)) -> Option<usize>,
    >,
}

impl<'a> AncestorsIterator<'a> {
    /// Constructor.
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: {
                // Get underlying adjacency matrix.
                let adjacency_matrix = g.deref();
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
    g: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

impl<'a> ParentsIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: g
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
    g: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

impl<'a> ChildrenIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: g
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
    g: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<<ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter>,
        fn((usize, bool)) -> Option<usize>,
    >,
}

impl<'a> DescendantsIterator<'a> {
    /// Constructor.
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            g,
            iter: {
                // Get underlying adjacency matrix.
                let adjacency_matrix = g.deref();
                // Initialize previous solution.
                let mut prev = Array1::from_elem((adjacency_matrix.ncols(),), false);
                // Get current ancestors set, i.e. parents set.
                let mut curr = adjacency_matrix.row(x).to_owned();

                // Check stopping criterion.
                while curr != prev {
                    // Update previous.
                    prev.assign(&curr);
                    // Select current parents.
                    let next = &adjacency_matrix.t() & &curr;
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

impl DirectedGraph for DirectedDenseAdjacencyMatrixGraph {
    type DirectedEdgesIndexIter<'a> = EdgesIterator<'a>;

    type AncestorsIndexIter<'a> = AncestorsIterator<'a>;

    type ParentsIndexIter<'a> = ParentsIterator<'a>;

    type ChildrenIndexIter<'a> = ChildrenIterator<'a>;

    type DescendantsIndexIter<'a> = DescendantsIterator<'a>;

    #[inline]
    fn size_of_maximal_directed_subgraph(&self) -> usize {
        self.size()
    }

    #[inline]
    fn get_directed_edges_index(&self) -> Self::DirectedEdgesIndexIter<'_> {
        self.get_edges_index()
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
        self.adjacency_matrix[[y, x]]
    }

    #[inline]
    fn get_children_by_index(&self, x: usize) -> Self::ChildrenIndexIter<'_> {
        Self::ChildrenIndexIter::new(self, x)
    }

    #[inline]
    fn is_child_by_index(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]]
    }

    #[inline]
    fn get_descendants_by_index(&self, x: usize) -> Self::DescendantsIndexIter<'_> {
        Self::DescendantsIndexIter::new(self, x)
    }

    #[inline]
    fn has_directed_edge_by_index(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]]
    }

    #[inline]
    fn get_in_degree_by_index(&self, x: usize) -> usize {
        // Compute in-degree.
        let d = self.adjacency_matrix.column(x).mapv(|f| f as usize).sum();

        // Check iterator consistency.
        debug_assert_eq!(Pa!(self, x).count(), d);

        d
    }

    #[inline]
    fn get_out_degree_by_index(&self, x: usize) -> usize {
        // Compute out-degree.
        let d = self.adjacency_matrix.row(x).mapv(|f| f as usize).sum();

        // Check iterator consistency.
        debug_assert_eq!(Ch!(self, x).count(), d);

        d
    }

    #[inline]
    fn add_directed_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        self.add_edge_by_index(x, y)
    }
}

/* Implement PathGraph */
impl PathGraph for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn has_path_by_index(&self, x: usize, y: usize) -> bool {
        self.has_edge_by_index(x, y) || BFS::from((self, x)).skip(1).any(|z| z == y)
    }

    #[inline]
    fn is_acyclic(&self) -> bool {
        !DFSEdges::new(self, None, Traversal::Forest).any(|e| matches!(e, DFSEdge::Back(_, _)))
    }
}

impl IntoUndirectedGraph for DirectedDenseAdjacencyMatrixGraph {
    type UndirectedGraph = UndirectedDenseAdjacencyMatrixGraph;

    #[inline]
    fn to_undirected(&self) -> Self::UndirectedGraph {
        // Make the adjacent matrix symmetric.
        let adjacency_matrix = &self.adjacency_matrix | &self.adjacency_matrix.t();

        Self::UndirectedGraph::from((self.labels.clone(), adjacency_matrix))
    }
}

impl MoralGraph for DirectedDenseAdjacencyMatrixGraph {
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

impl From<DOT> for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn from(other: DOT) -> Self {
        // Assert graph type.
        assert_eq!(
            other.graph_type, "digraph",
            "DOT graph type must match direction"
        );

        Self::new(other.vertices.into_keys(), other.edges.into_keys())
    }
}
