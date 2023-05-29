use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
    iter::{Enumerate, FilterMap, FusedIterator, Map},
    ops::{Deref, Range},
};

use is_sorted::IsSorted;
use itertools::{iproduct, Itertools};
use ndarray::{iter::IndexedIter, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    graphs::{
        algorithms::traversal::{DFSEdge, DFSEdges, Traversal},
        directions, BaseGraph, PartialOrdGraph, PathGraph, SubGraph, UndirectedGraph,
    },
    io::DOT,
    prelude::BFS,
    types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, FxIndexSet},
    Adj, E, V,
};

/// Undirected graph struct based on dense adjacency matrix data structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndirectedDenseAdjacencyMatrixGraph {
    labels: FxIndexSet<String>,
    adjacency_matrix: DenseAdjacencyMatrix,
    size: usize,
}

/* Implement BaseGraph trait. */
impl Deref for UndirectedDenseAdjacencyMatrixGraph {
    type Target = DenseAdjacencyMatrix;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.adjacency_matrix
    }
}

#[allow(dead_code, clippy::type_complexity)]
pub struct EdgesIterator<'a> {
    g: &'a UndirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> EdgesIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a UndirectedDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .indexed_iter()
                .filter_map(|((i, j), &f)| match f && i <= j {
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
    g: &'a UndirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

impl<'a> AdjacentsIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a UndirectedDenseAdjacencyMatrixGraph, x: usize) -> Self {
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

impl<'a> Iterator for AdjacentsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> FusedIterator for AdjacentsIterator<'a> {}

impl Display for UndirectedDenseAdjacencyMatrixGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "UndirectedGraph {{ ")?;
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

impl Hash for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.iter().for_each(|x| x.hash(state));
        self.adjacency_matrix.hash(state);
    }
}

impl Default for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn default() -> Self {
        Self {
            labels: Default::default(),
            adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            size: 0,
        }
    }
}

impl BaseGraph for UndirectedDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::Undirected;

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
            if !adjacency_matrix[[i, j]] {
                // Add edge.
                adjacency_matrix[[i, j]] = true;
                adjacency_matrix[[j, i]] = true;
                // Increment size.
                size += 1;
            }
        }

        Self {
            labels,
            adjacency_matrix,
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
        let mut labels: FxIndexSet<_> = labels.into_iter().map_into().collect();
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

    fn complete<V, I>(labels: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<_> = labels.into_iter().map_into().collect();
        // Sort labels.
        labels.sort();

        // Compute new graph order.
        let order = labels.len();
        // Initialize adjacency matrix given graph order.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), true);
        // Remove self loops.
        adjacency_matrix.diag_mut().map_inplace(|x| *x = false);

        // Compute size.
        let size = (order * (order.saturating_sub(1))) / 2;

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
        self.adjacency_matrix[[y, x]] = true;
        // Increment size.
        self.size += 1;

        // Assert adjacency matrix is still consistent.
        debug_assert_eq!(self.adjacency_matrix[[x, y]], self.adjacency_matrix[[y, x]]);
        // Assert size counter and adjacency matrix are still consistent.
        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
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
    fn del_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        // If edge does not exists ...
        if !self.adjacency_matrix[[x, y]] {
            // ... return early.
            return false;
        }

        // Remove edge.
        self.adjacency_matrix[[x, y]] = false;
        self.adjacency_matrix[[y, x]] = false;
        // Decrement size.
        self.size -= 1;

        // Assert adjacency matrix is still consistent.
        debug_assert_eq!(self.adjacency_matrix[[x, y]], self.adjacency_matrix[[y, x]]);
        // Assert size counter and adjacency matrix are still consistent.
        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
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

/* Implement TryFrom traits. */
impl<V> From<EdgeList<V>> for UndirectedDenseAdjacencyMatrixGraph
where
    V: Into<String>,
{
    #[inline]
    fn from(edge_list: EdgeList<V>) -> Self {
        Self::new([], edge_list)
    }
}

impl<V> From<AdjacencyList<V>> for UndirectedDenseAdjacencyMatrixGraph
where
    V: Clone + Into<String>,
{
    fn from(adjacency_list: AdjacencyList<V>) -> Self {
        // Map into vertices.
        let vertices: Vec<_> = adjacency_list.keys().cloned().collect();
        // Map into edges.
        let edges = adjacency_list
            .into_iter()
            .flat_map(|(x, ys)| std::iter::repeat(x).take(ys.len()).zip(ys.into_iter()));

        Self::new(vertices, edges)
    }
}

impl<I, V> From<(I, DenseAdjacencyMatrix)> for UndirectedDenseAdjacencyMatrixGraph
where
    I: IntoIterator<Item = V>,
    V: Into<String>,
{
    fn from((labels, adjacency_matrix): (I, DenseAdjacencyMatrix)) -> Self {
        // Remove duplicated vertices labels.
        let mut labels: FxIndexSet<String> = labels.into_iter().map_into().collect();
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
        // Check if adjacency matrix is not symmetric.
        if adjacency_matrix != adjacency_matrix.t() {
            panic!("Matrix must be symmetric");
        }

        // Cast to standard memory layout (i.e. C layout), if not already.
        let adjacency_matrix = adjacency_matrix.as_standard_layout().into_owned();

        // Compute size.
        let size = adjacency_matrix.mapv(|f| f as usize).sum();
        let size = size + adjacency_matrix.diag().mapv(|f| f as usize).sum();
        let size = size / 2;

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
impl Into<EdgeList<String>> for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
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
impl Into<AdjacencyList<String>> for UndirectedDenseAdjacencyMatrixGraph {
    fn into(self) -> AdjacencyList<String> {
        V!(self)
            .map(|x| {
                (
                    self.get_vertex_by_index(x).into(),
                    Adj!(self, x)
                        .map(|y| self.get_vertex_by_index(y).into())
                        .collect(),
                )
            })
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<(FxIndexSet<String>, DenseAdjacencyMatrix)> for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn into(self) -> (FxIndexSet<String>, DenseAdjacencyMatrix) {
        (self.labels, self.adjacency_matrix)
    }
}

/* Implement PartialOrdGraph trait. */
impl PartialEq for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Check that V(\mathcal{G}) == V(\mathcal{H}) && E(\mathcal{G}) == E(\mathcal{H}).
        self.labels.eq(&other.labels) && self.adjacency_matrix.eq(&other.adjacency_matrix)
    }
}

impl Eq for UndirectedDenseAdjacencyMatrixGraph {}

impl PartialOrd for UndirectedDenseAdjacencyMatrixGraph {
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

impl PartialOrdGraph for UndirectedDenseAdjacencyMatrixGraph {}

/* Implement SubGraph trait. */
impl SubGraph for UndirectedDenseAdjacencyMatrixGraph {
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
        // Fill the adjacency matrix, symmetrically.
        for (x, y) in edges {
            // Add the edge.
            adjacency_matrix[[x, y]] = true;
            adjacency_matrix[[y, x]] = true;
            // Add the vertices.
            indices[x] = true;
            indices[y] = true;
        }

        // Map the indices.
        let indices: Vec<_> = indices
            .into_iter()
            .enumerate()
            .filter_map(|(i, f)| match f {
                true => Some(i),
                false => None,
            })
            .collect();

        // Get minor of matrix.
        let adjacency_matrix = adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

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
        let indices: Vec<_> = indices.into_iter().collect();

        // Get minor of matrix.
        let adjacency_matrix = self
            .adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

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
        // Fill the adjacency matrix, symmetrically.
        for (x, y) in edges {
            // Add the edge.
            adjacency_matrix[[x, y]] = true;
            adjacency_matrix[[y, x]] = true;
            // Add the vertices.
            indices[x] = true;
            indices[y] = true;
        }

        // Map the indices.
        let indices: Vec<_> = indices
            .into_iter()
            .enumerate()
            .filter_map(|(i, f)| match f {
                true => Some(i),
                false => None,
            })
            .collect();

        // Get minor of matrix.
        let adjacency_matrix = adjacency_matrix
            .select(Axis(0), &indices)
            .select(Axis(1), &indices);

        // Get vertices labels.
        let vertices = indices.into_iter().map(|x| self.get_vertex_by_index(x));

        // Build subgraph from vertices and adjacency matrix.
        Self::try_from((vertices, adjacency_matrix)).unwrap()
    }
}

/* Implement UndirectedGraph trait. */
impl UndirectedGraph for UndirectedDenseAdjacencyMatrixGraph {
    type UndirectedEdgesIndexIter<'a> = EdgesIterator<'a>;
    type NeighborsIndexIter<'a> = Self::AdjacentsIndexIter<'a>;

    #[inline]
    fn size_of_maximal_undirected_subgraph(&self) -> usize {
        self.size()
    }

    #[inline]
    fn get_undirected_edges_index(&self) -> Self::UndirectedEdgesIndexIter<'_> {
        self.get_edges_index()
    }

    #[inline]
    fn get_neighbors_by_index(&self, x: usize) -> Self::NeighborsIndexIter<'_> {
        Self::NeighborsIndexIter::new(self, x)
    }

    #[inline]
    fn is_neighbor_by_index(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]]
    }

    #[inline]
    fn has_undirected_edge_by_index(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]]
    }

    #[inline]
    fn get_degree_by_index(&self, x: usize) -> usize {
        // Compute degree.
        let d = self.adjacency_matrix.row(x).mapv(|f| f as usize).sum();

        // Check iterator consistency.
        debug_assert_eq!(Adj!(self, x).count(), d);

        d
    }

    #[inline]
    fn add_undirected_edge_by_index(&mut self, x: usize, y: usize) -> bool {
        self.add_edge_by_index(x, y)
    }
}

/* Implement PathGraph */
impl PathGraph for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn has_path_by_index(&self, x: usize, y: usize) -> bool {
        self.has_edge_by_index(x, y) || BFS::from((self, x)).skip(1).any(|z| z == y)
    }

    #[inline]
    fn is_acyclic(&self) -> bool {
        !DFSEdges::new(self, None, Traversal::Forest).any(|e| matches!(e, DFSEdge::Back(_, _)))
    }
}

impl From<DOT> for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn from(other: DOT) -> Self {
        // Assert graph type.
        assert_eq!(
            other.graph_type, "graph",
            "DOT graph type must match direction"
        );

        Self::new(other.vertices.into_keys(), other.edges.into_keys())
    }
}
