use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::Display,
    iter::{Enumerate, FilterMap, FusedIterator},
    ops::{Deref, Range},
};

use itertools::{iproduct, Itertools};
use ndarray::{iter::IndexedIter, prelude::*, OwnedRepr};

use crate::{
    graphs::{directions, BaseGraph, DefaultGraph, ErrorGraph as E, PartialOrdGraph},
    types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, FnvBiHashMap, SparseAdjacencyMatrix},
    Adj, E, V,
};

/// Directed graph struct based on dense adjacency matrix data structure.
#[derive(Clone, Debug)]
pub struct DirectedDenseAdjacencyMatrixGraph {
    vertices: BTreeSet<String>,
    vertices_indexes: FnvBiHashMap<String, usize>,
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

#[allow(clippy::type_complexity)]
pub struct EdgesIterator<'a> {
    graph: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<IndexedIter<'a, bool, Ix2>, fn(((usize, usize), &bool)) -> Option<(usize, usize)>>,
    size: usize,
}

impl<'a> EdgesIterator<'a> {
    /// Constructor.
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph) -> Self {
        Self {
            graph: g,
            iter: g.indexed_iter().filter_map(|((x, y), &f)| match f {
                true => Some((x, y)),
                false => None,
            }),
            size: g.size(),
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

#[allow(clippy::type_complexity)]
pub struct AdjacentsIterator<'a> {
    graph: &'a DirectedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<<ArrayBase<OwnedRepr<bool>, Dim<[usize; 1]>> as IntoIterator>::IntoIter>,
        fn((usize, bool)) -> Option<usize>,
    >,
}

impl<'a> AdjacentsIterator<'a> {
    /// Constructor.
    pub fn new(g: &'a DirectedDenseAdjacencyMatrixGraph, x: usize) -> Self {
        Self {
            graph: g,
            iter: {
                let (row, col) = (g.row(x), g.column(x));

                (&row | &col).into_iter().enumerate().filter_map(|(x, f)| match f {
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
            V!(self).map(|x| format!("\"{}\"", self.vertex(x))).join(", ")
        )?;
        // Write edge set.
        write!(
            f,
            "E = {{{}}}",
            E!(self)
                .map(|(x, y)| format!("(\"{}\", \"{}\")", self.vertex(x), self.vertex(y)))
                .join(", ")
        )?;
        // Write ending character.
        write!(f, " }}")
    }
}

impl BaseGraph for DirectedDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::Directed;

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
        let mut vertices: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Map edges iterator into edge list.
        let edges: EdgeList<_> = edges.into_iter().map(|(x, y)| (x.into(), y.into())).collect();
        // Add missing vertices from the edges.
        for (x, y) in &edges {
            vertices.insert(x.clone());
            vertices.insert(y.clone());
        }

        // Compute new graph order.
        let order = vertices.len();
        // Map vertices labels to vertices indices.
        let vertices_indexes: FnvBiHashMap<_, _> = vertices.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();
        // Initialize adjacency matrix given graph order.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

        // Initialize the size.
        let mut size = 0;
        // Fill adjacency matrix given edge set.
        for (x, y) in edges {
            // Get associated vertices indices.
            let (i, j) = (
                *vertices_indexes.get_by_left(&x).unwrap(),
                *vertices_indexes.get_by_left(&y).unwrap(),
            );
            // Set edge given indices.
            adjacency_matrix[[i, j]] = true;
            // Increment size.
            size += 1;
        }

        Self {
            vertices,
            vertices_indexes,
            adjacency_matrix,
            size,
        }
    }

    fn clear(&mut self) {
        // Clear the vertices.
        self.vertices.clear();
        // Clear the vertices map.
        self.vertices_indexes.clear();
        // Clear the adjacency matrix.
        self.adjacency_matrix = Default::default();
        // Clear the size.
        self.size = 0;
    }

    #[inline]
    fn index(&self, x: &str) -> usize {
        *self
            .vertices_indexes
            .get_by_left(x)
            .unwrap_or_else(|| panic!("No vertex with index `{}`", x))
    }

    #[inline]
    fn vertex(&self, x: usize) -> &str {
        self.vertices_indexes
            .get_by_right(&x)
            .unwrap_or_else(|| panic!("No vertex with label `{}`", x))
    }

    #[inline]
    fn vertices(&self) -> Self::VerticesIter<'_> {
        // Assert vertex set and vertices map are consistent.
        debug_assert_eq!(self.vertices.len(), self.vertices_indexes.len());

        0..self.vertices.len()
    }

    #[inline]
    fn order(&self) -> usize {
        // Assert vertex set and vertices map are consistent.
        debug_assert_eq!(self.vertices.len(), self.vertices_indexes.len());
        // Assert vertex set is consistent with adjacency matrix shape.
        debug_assert_eq!(self.vertices_indexes.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is square.
        debug_assert!(self.adjacency_matrix.is_square());

        self.vertices.len()
    }

    #[inline]
    fn has_vertex(&self, x: usize) -> bool {
        // Assert vertex set and vertices map are consistent.
        debug_assert_eq!(self.vertices_indexes.contains_right(&x), x < self.order());

        self.vertices_indexes.contains_right(&x)
    }

    fn add_vertex<V>(&mut self, x: V) -> usize
    where
        V: Into<String>,
    {
        // Cast vertex label.
        let x = x.into();

        // If vertex was already present ...
        if !self.vertices.insert(x.clone()) {
            // ... return early.
            return self.index(&x);
        }

        // Get vertex index.
        let i = self.vertices.iter().position(|y| y == &x).unwrap();

        // Update the vertices map after the added vertex.
        for (j, y) in self.vertices.iter().skip(i).enumerate() {
            // Add the given vertex and increment subsequent ones by overwriting the entries.
            self.vertices_indexes.insert(y.clone(), i + j);
        }

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

        // Assert vertex has been added.
        debug_assert!(self.vertices.contains(&x));
        debug_assert!(self.vertices_indexes.contains_left(&x));
        // Assert vertex set is still consistent with vertices map.
        debug_assert!(self.vertices.iter().eq(self.vertices_indexes.left_values().sorted()));
        // Assert vertices labels are still associated to an ordered and
        // contiguous sequence of integers starting from zero, i.e in [0, n).
        debug_assert!(self
            .vertices_indexes
            .right_values()
            .cloned()
            .sorted()
            .eq(0..self.vertices_indexes.len()));
        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(self.vertices_indexes.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(self.adjacency_matrix.is_square());

        // Return new vertex.
        i
    }

    fn del_vertex(&mut self, x: usize) -> bool {
        // Get vertex label and index.
        let x_i = self.vertices_indexes.remove_by_right(&x);

        // If vertex was not present ...
        if x_i.is_none() {
            // ... then return early.
            return false;
        }

        // Get vertex label and index.
        let (x, i) = x_i.unwrap();

        // Remove vertex label.
        self.vertices.remove(&x);

        // Update the vertices map after the removed vertex.
        for (j, y) in self.vertices.iter().skip(i).enumerate() {
            // Decrement subsequent ones by overwriting the entries.
            self.vertices_indexes.insert(y.clone(), i + j);
        }

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

        // Assert vertex has been removed.
        debug_assert!(!self.vertices.contains(&x));
        debug_assert!(!self.vertices_indexes.contains_left(&x));
        // Assert vertex set is still consistent with vertices map.
        debug_assert!(self.vertices.iter().eq(self.vertices_indexes.left_values().sorted()));
        // Assert vertices labels are still associated to an ordered and
        // contiguous sequence of integers starting from zero, i.e in [0, n).
        debug_assert!(self
            .vertices_indexes
            .right_values()
            .cloned()
            .sorted()
            .eq(0..self.vertices_indexes.len()));
        // Assert vertex set is still consistent with adjacency matrix shape.
        debug_assert_eq!(self.vertices_indexes.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(self.adjacency_matrix.is_square());

        true
    }

    #[inline]
    fn edges(&self) -> Self::EdgesIter<'_> {
        Self::EdgesIter::new(self)
    }

    #[inline]
    fn size(&self) -> usize {
        self.size
    }

    #[inline]
    fn has_edge(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]]
    }

    fn add_edge(&mut self, x: usize, y: usize) -> bool {
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

    fn del_edge(&mut self, x: usize, y: usize) -> bool {
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
    fn adjacents(&self, x: usize) -> Self::AdjacentsIter<'_> {
        Self::AdjacentsIter::new(self, x)
    }

    #[inline]
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        self.has_edge(x, y)
    }

    #[inline]
    fn degree(&self, x: usize) -> usize {
        self.adjacency_matrix.row(x).mapv(|f| f as usize).sum()
    }
}

/* Implement DefaultGraph trait. */

impl Default for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            vertices_indexes: Default::default(),
            adjacency_matrix: DenseAdjacencyMatrix::from_elem((0, 0), false),
            size: 0,
        }
    }
}

impl DefaultGraph for DirectedDenseAdjacencyMatrixGraph {
    fn empty<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Remove duplicated vertices labels.
        let vertices: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();

        // Compute new graph order.
        let order = vertices.len();
        // Map vertices labels to vertices indices.
        let vertices_indexes = vertices.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();
        // Initialize adjacency matrix given graph order.
        let adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

        Self {
            vertices,
            vertices_indexes,
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
        let vertices: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();

        // Compute new graph order.
        let order = vertices.len();
        // Map vertices labels to vertices indices.
        let vertices_indexes = vertices.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();
        // Initialize adjacency matrix given graph order.
        let mut adjacency_matrix = DenseAdjacencyMatrix::from_elem((order, order), true);
        // Remove self loops.
        adjacency_matrix.diag_mut().map_inplace(|x| *x = false);

        // Compute size.
        let size = order * (order.saturating_sub(1));

        Self {
            vertices,
            vertices_indexes,
            adjacency_matrix,
            size,
        }
    }
}

/* Implement TryFrom traits. */

impl<V> From<EdgeList<V>> for DirectedDenseAdjacencyMatrixGraph
where
    V: Into<String>,
{
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
        let vertices: Vec<_> = adjacency_list.keys().cloned().collect();
        // Map into edges.
        let edges = adjacency_list
            .into_iter()
            .flat_map(|(x, ys)| std::iter::repeat(x).take(ys.len()).zip(ys.into_iter()));

        Self::new(vertices, edges)
    }
}

impl<V, I> TryFrom<(I, DenseAdjacencyMatrix)> for DirectedDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    type Error = E;

    fn try_from((vertices, adjacency_matrix): (I, DenseAdjacencyMatrix)) -> Result<Self, Self::Error> {
        // Remove duplicated vertices labels.
        let vertices: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();

        // Check if vertex set is not consistent with given adjacency matrix.
        if vertices.len() != adjacency_matrix.nrows() {
            return Err(E::InconsistentMatrix);
        }
        // Check if adjacency matrix is not square.
        if !adjacency_matrix.is_square() {
            return Err(E::NonSquareMatrix);
        }

        // Map vertices labels to vertices indices.
        let vertices_indexes = vertices.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();

        // Cast to standard memory layout (i.e. C layout), if not already.
        let adjacency_matrix = adjacency_matrix.as_standard_layout().into_owned();

        // Compute size.
        let size = adjacency_matrix.mapv(|f| f as usize).sum();

        Ok(Self {
            vertices,
            vertices_indexes,
            adjacency_matrix,
            size,
        })
    }
}

impl<V, I> TryFrom<(I, SparseAdjacencyMatrix)> for DirectedDenseAdjacencyMatrixGraph
where
    V: Into<String>,
    I: IntoIterator<Item = V>,
{
    type Error = E;

    fn try_from((vertices, adjacency_matrix): (I, SparseAdjacencyMatrix)) -> Result<Self, Self::Error> {
        // Allocate dense adjacency matrix.
        let mut dense_adjacency_matrix = DenseAdjacencyMatrix::from_elem(adjacency_matrix.shape(), false);
        // Fill dense adjacency matrix from sparse triplets.
        for (&f, (i, j)) in adjacency_matrix.triplet_iter() {
            dense_adjacency_matrix[[i, j]] = f;
        }
        // Delegate constructor to dense adjacency matrix constructor.
        Self::try_from((vertices, dense_adjacency_matrix))
    }
}

/* Implement Into traits. */

#[allow(clippy::from_over_into)]
impl Into<EdgeList<String>> for DirectedDenseAdjacencyMatrixGraph {
    fn into(self) -> EdgeList<String> {
        E!(self)
            .map(|(x, y)| (self.vertex(x).to_string(), self.vertex(y).to_string()))
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<AdjacencyList<String>> for DirectedDenseAdjacencyMatrixGraph {
    fn into(self) -> AdjacencyList<String> {
        V!(self)
            .map(|x| {
                (
                    self.vertex(x).to_string(),
                    Adj!(self, x).map(|y| self.vertex(y).to_string()).collect(),
                )
            })
            .collect()
    }
}

#[allow(clippy::from_over_into)]
impl Into<(BTreeSet<String>, DenseAdjacencyMatrix)> for DirectedDenseAdjacencyMatrixGraph {
    fn into(self) -> (BTreeSet<String>, DenseAdjacencyMatrix) {
        (self.vertices, self.adjacency_matrix)
    }
}

#[allow(clippy::from_over_into)]
impl Into<(BTreeSet<String>, SparseAdjacencyMatrix)> for DirectedDenseAdjacencyMatrixGraph {
    fn into(self) -> (BTreeSet<String>, SparseAdjacencyMatrix) {
        // Get upper bound capacity.
        let size = self.size() * 2;
        // Allocate triplets indices.
        let (mut rows, mut cols) = (Vec::with_capacity(size), Vec::with_capacity(size));
        // Build triplets indices.
        for ((i, j), &f) in self.adjacency_matrix.indexed_iter() {
            if f {
                rows.push(i);
                cols.push(j);
            }
        }
        // Shrink triplets indices to actual capacity.
        rows.shrink_to_fit();
        cols.shrink_to_fit();
        // Build data vector.
        let data: Vec<_> = std::iter::repeat(true).take(rows.len()).collect();
        // Construct sparse adjacency matrix.
        let shape = self.adjacency_matrix.shape();
        let sparse_adjacency_matrix = SparseAdjacencyMatrix::from_triplets((shape[0], shape[1]), rows, cols, data);

        (self.vertices, sparse_adjacency_matrix)
    }
}

/* Implement PartialOrdGraph trait. */

impl PartialEq for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Check that V(G) == V(H) && E(G) == E(H).
        self.vertices.eq(&other.vertices) && self.adjacency_matrix.eq(&other.adjacency_matrix)
    }
}

impl Eq for DirectedDenseAdjacencyMatrixGraph {}

impl PartialOrd for DirectedDenseAdjacencyMatrixGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare vertices sets.
        let vertices = crate::utils::partial_cmp_sets(&self.vertices, &other.vertices);
        // If the vertices sets are comparable ...
        vertices.and_then(|vertices| {
            // ... compare edges sets.
            // TODO: Check if allocation is avoidable.
            let self_edges = self.edges().collect::<BTreeSet<_>>();
            let other_edges = other.edges().collect::<BTreeSet<_>>();
            let edges = crate::utils::partial_cmp_sets(&self_edges, &other_edges);
            // If the edges sets are comparable ...
            edges.and_then(|edges| {
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

/* Implement DirectedGraph trait. */

/* FIXME:
impl DirectedGraph for DirectedDenseAdjacencyMatrixGraph {
    type AncestorsIter<'a>
    where
        Self: 'a;

    type ParentsIter<'a>
    where
        Self: 'a;

    type ChildrenIter<'a>
    where
        Self: 'a;

    type DescendantsIter<'a>
    where
        Self: 'a;

    fn ancestors<'a>(self, x: usize) -> Self::AncestorsIter<'a> {
        todo!()
    }

    fn is_ancestor(&self, x: usize, y: usize) -> bool {
        todo!()
    }

    fn parents<'a>(self, x: usize) -> Self::ParentsIter<'a> {
        todo!()
    }

    fn is_parent(&self, x: usize, y: usize) -> bool {
        todo!()
    }

    fn children<'a>(self, x: usize) -> Self::ChildrenIter<'a> {
        todo!()
    }

    fn is_child(&self, x: usize, y: usize) -> bool {
        todo!()
    }

    fn descendants<'a>(self, x: usize) -> Self::DescendantsIter<'a> {
        todo!()
    }

    fn is_descendant(&self, x: usize, y: usize) -> bool {
        todo!()
    }

    fn in_degree(&self, x: usize) -> usize {
        todo!()
    }

    fn out_degree(&self, x: usize) -> usize {
        todo!()
    }
}
*/
