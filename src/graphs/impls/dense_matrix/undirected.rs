use std::{
    cmp::Ordering,
    collections::{btree_set::Iter, BTreeSet},
    fmt::Display,
    iter::FilterMap,
    ops::Deref,
};

use itertools::{iproduct, Itertools};
use ndarray::{iter::IndexedIter, prelude::*};

use crate::{
    graphs::{BaseGraph, DefaultGraph, ErrorGraph as E, PartialOrdGraph},
    types::{AdjacencyMatrix, FnvBiHashMap},
};

/// Undirected graph struct based on dense adjacent matrix data structure.
#[derive(Clone, Debug)]
pub struct DenseMatrixUndirectedGraph {
    vertices: BTreeSet<String>,
    vertices_indexes: FnvBiHashMap<String, usize>,
    adjacency_matrix: AdjacencyMatrix,
    size: usize,
}

impl DenseMatrixUndirectedGraph {
    /// Gets vertex label given its index.
    pub fn label<'a>(&'a self, x: usize) -> &'a String {
        self.vertices_indexes
            .get_by_right(&x)
            .expect("Array index out of bounds")
    }
}

/* Implement BaseGraph trait. */

impl Deref for DenseMatrixUndirectedGraph {
    type Target = AdjacencyMatrix;

    fn deref(&self) -> &Self::Target {
        &self.adjacency_matrix
    }
}

pub struct EdgesIterator<'a> {
    graph: &'a DenseMatrixUndirectedGraph,
    iter: FilterMap<IndexedIter<'a, bool, Ix2>, fn(((usize, usize), &bool)) -> Option<(usize, usize)>>,
    size: usize,
}

impl<'a> EdgesIterator<'a> {
    /// Constructor.
    pub fn new(graph: &'a DenseMatrixUndirectedGraph) -> Self {
        Self {
            graph,
            iter: (*graph)
                .indexed_iter()
                .filter_map(|((i, j), flag)| match *flag && i <= j {
                    true => Some((i, j)),
                    false => None,
                }),
            size: graph.size(),
        }
    }
}

impl<'a> Iterator for EdgesIterator<'a> {
    type Item = <DenseMatrixUndirectedGraph as BaseGraph>::Edge<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(i, j)| (self.graph.label(i), self.graph.label(j)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a> ExactSizeIterator for EdgesIterator<'a> {}

impl Display for DenseMatrixUndirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "UndirectedGraph {{ ")?;
        // Write vertices set.
        write!(
            f,
            "V = {{{}}}, ",
            self.vertices().map(|x| format!("\"{}\"", x)).join(", ")
        )?;
        // Write edges set.
        write!(
            f,
            "E = {{{}}}",
            self.edges().map(|(x, y)| format!("(\"{}\", \"{}\")", x, y)).join(", ")
        )?;
        // Write ending character.
        write!(f, " }}")
    }
}

impl BaseGraph for DenseMatrixUndirectedGraph {
    type Data = AdjacencyMatrix;

    type Vertex = String;

    type VerticesIter<'a> = Iter<'a, Self::Vertex>;

    type Edge<'a> = (&'a Self::Vertex, &'a Self::Vertex);

    type EdgesIter<'a> = EdgesIterator<'a>;

    #[inline]
    fn order(&self) -> usize {
        // Assert vertices set and vertices map are consistent.
        debug_assert_eq!(self.vertices.len(), self.vertices_indexes.len());
        // Assert vertices set is consistent with adjacency matrix shape.
        debug_assert_eq!(self.vertices_indexes.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is square.
        debug_assert!(self.adjacency_matrix.is_square());

        self.vertices.len()
    }

    #[inline]
    fn vertices<'a>(&'a self) -> Self::VerticesIter<'a> {
        // Assert vertices set and vertices map are consistent.
        debug_assert!(self.vertices.iter().eq(self.vertices_indexes.left_values().sorted()));

        self.vertices.iter()
    }

    #[inline]
    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Assert vertices set and vertices map are consistent.
        debug_assert_eq!(self.vertices.contains(x), self.vertices_indexes.contains_left(x));

        self.vertices_indexes.contains_left(x)
    }

    fn add_vertex<V>(&mut self, x: V) -> Self::Vertex
    where
        V: Into<Self::Vertex>,
    {
        // Cast to vertex label.
        let x = x.into();

        // If label is not present ...
        if self.vertices.insert(x.clone()) {
            // ... then compute new index.
            let i = self.vertices.iter().position(|y| y == &x).unwrap();
            // Update the vertices map after the added vertex.
            for (j, y) in self.vertices.iter().skip(i).enumerate() {
                // Add the given vertex and increment subsequent ones by overwriting the entries.
                self.vertices_indexes.insert(y.clone(), i + j);
            }

            // Compute the new size of adjacency matrix.
            let n = self.adjacency_matrix.nrows();
            // Allocate new adjacency matrix.
            let mut adjacency_matrix = AdjacencyMatrix::from_elem((n + 1, n + 1), false);
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
        }

        // Assert vertex has been added.
        debug_assert!(self.vertices.contains(&x));
        debug_assert!(self.vertices_indexes.contains_left(&x));
        // Assert vertices set is still consistent with vertices map.
        debug_assert!(self.vertices.iter().eq(self.vertices_indexes.left_values().sorted()));
        // Assert vertices labels are still associated to an ordered and
        // contiguous sequence of integers starting from zero, i.e in [0, n).
        debug_assert!(self
            .vertices_indexes
            .right_values()
            .cloned()
            .sorted()
            .eq(0..self.vertices_indexes.len()));
        // Assert vertices set is still consistent with adjacency matrix shape.
        debug_assert_eq!(self.vertices_indexes.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(self.adjacency_matrix.is_square());
        // Assert adjacency matrix is still symmetric.
        debug_assert_eq!(self.adjacency_matrix, self.adjacency_matrix.t());

        // Return new vertex.
        x
    }

    fn del_vertex(&mut self, x: &Self::Vertex) {
        // If label is present ...
        if self.vertices.remove(x) {
            // ... then compute index.
            let i = self.vertices_indexes.remove_by_left(x).unwrap().1;
            // Update the vertices map after the removed vertex.
            for (j, y) in self.vertices.iter().skip(i).enumerate() {
                // Decrement subsequent ones by overwriting the entries.
                self.vertices_indexes.insert(y.clone(), i + j);
            }

            // Compute the new size of adjacency matrix.
            let n = self.adjacency_matrix.nrows();
            // Allocate new adjacency matrix.
            let mut adjacency_matrix = AdjacencyMatrix::from_elem((n - 1, n - 1), false);
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
        }

        // Assert vertex has been removed.
        debug_assert!(!self.vertices.contains(x));
        debug_assert!(!self.vertices_indexes.contains_left(x));
        // Assert vertices set is still consistent with vertices map.
        debug_assert!(self.vertices.iter().eq(self.vertices_indexes.left_values().sorted()));
        // Assert vertices labels are still associated to an ordered and
        // contiguous sequence of integers starting from zero, i.e in [0, n).
        debug_assert!(self
            .vertices_indexes
            .right_values()
            .cloned()
            .sorted()
            .eq(0..self.vertices_indexes.len()));
        // Assert vertices set is still consistent with adjacency matrix shape.
        debug_assert_eq!(self.vertices_indexes.len(), self.adjacency_matrix.nrows());
        // Assert adjacency matrix is still square.
        debug_assert!(self.adjacency_matrix.is_square());
        // Assert adjacency matrix is still symmetric.
        debug_assert_eq!(self.adjacency_matrix, self.adjacency_matrix.t());
    }

    #[inline]
    fn size(&self) -> usize {
        self.size
    }

    fn edges<'a>(&'a self) -> Self::EdgesIter<'a> {
        EdgesIterator::new(&self)
    }

    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        // Get associated vertices indices.
        let (i, j) = (
            *self.vertices_indexes.get_by_left(x).unwrap(),
            *self.vertices_indexes.get_by_left(y).unwrap(),
        );

        self.adjacency_matrix[[i, j]]
    }

    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) {
        // Get associated vertices indices.
        let (i, j) = (
            *self.vertices_indexes.get_by_left(x).unwrap(),
            *self.vertices_indexes.get_by_left(y).unwrap(),
        );

        // Check if edge not exists.
        if !self.adjacency_matrix[[i, j]] {
            // Add edge.
            self.adjacency_matrix[[i, j]] = true;
            self.adjacency_matrix[[j, i]] = true;
            // Increment size.
            self.size += 1;
        }

        // Assert adjacency matrix is still consistent.
        debug_assert_eq!(self.adjacency_matrix[[i, j]], self.adjacency_matrix[[j, i]]);
        // Assert size counter and adjacency matrix are still consistent.
        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), x)| match i <= j {
                    true => Some(*x as usize),
                    false => None,
                })
                .sum()
        )
    }

    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) {
        // Get associated vertices indices.
        let (i, j) = (
            *self.vertices_indexes.get_by_left(x).unwrap(),
            *self.vertices_indexes.get_by_left(y).unwrap(),
        );

        // Check if edge exists.
        if self.adjacency_matrix[[i, j]] {
            // Remove edge.
            self.adjacency_matrix[[i, j]] = false;
            self.adjacency_matrix[[j, i]] = false;
            // Decrement size.
            self.size -= 1;
        }

        // Assert adjacency matrix is still consistent.
        debug_assert_eq!(self.adjacency_matrix[[i, j]], self.adjacency_matrix[[j, i]]);
        // Assert size counter and adjacency matrix are still consistent.
        debug_assert_eq!(
            self.size,
            self.adjacency_matrix
                .indexed_iter()
                .filter_map(|((i, j), x)| match i <= j {
                    true => Some(*x as usize),
                    false => None,
                })
                .sum()
        )
    }

    #[inline]
    fn is_adjacent(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        self.has_edge(x, y)
    }
}

/* Implement DefaultGraph trait. */

impl Default for DenseMatrixUndirectedGraph {
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            vertices_indexes: Default::default(),
            adjacency_matrix: AdjacencyMatrix::from_elem((0, 0), false),
            size: 0,
        }
    }
}

impl DefaultGraph for DenseMatrixUndirectedGraph {
    fn empty<I, V>(vertices: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>,
    {
        // Remove duplicated vertices labels.
        let vertices: BTreeSet<Self::Vertex> = vertices.into_iter().map(|x| x.into()).collect();

        // Check if vertices labels are non empty.
        if vertices.contains("") {
            return Err(E::EmptyVertexLabel);
        }

        // Map vertices labels to vertices indices.
        let vertices_indexes: FnvBiHashMap<Self::Vertex, usize> =
            vertices.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();
        // Compute new graph order.
        let order = vertices.len();
        // Initialize adjacency matrix given graph order.
        let adjacency_matrix = AdjacencyMatrix::from_elem((order, order), false);

        Ok(Self {
            vertices,
            vertices_indexes,
            adjacency_matrix,
            size: 0,
        })
    }

    fn complete<I, V>(vertices: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>,
    {
        // Remove duplicated vertices labels.
        let vertices: BTreeSet<Self::Vertex> = vertices.into_iter().map(|x| x.into()).collect();

        // Check if vertices labels are non empty.
        if vertices.contains("") {
            return Err(E::EmptyVertexLabel);
        }

        // Map vertices labels to vertices indices.
        let vertices_indexes: FnvBiHashMap<Self::Vertex, usize> =
            vertices.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();
        // Compute new graph order.
        let order = vertices.len();
        // Initialize adjacency matrix given graph order.
        let mut adjacency_matrix = AdjacencyMatrix::from_elem((order, order), true);
        // Remove self loops.
        adjacency_matrix.diag_mut().map_inplace(|i| *i = false);
        // Compute size.
        let size = adjacency_matrix
            .indexed_iter()
            .filter(|((i, j), flag)| **flag && i <= j)
            .count();

        Ok(Self {
            vertices,
            vertices_indexes,
            adjacency_matrix,
            size: size,
        })
    }

    fn with_adjacency_matrix<I, V>(vertices: I, adjacency_matrix: AdjacencyMatrix) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>,
    {
        // Remove duplicated vertices labels.
        let vertices: BTreeSet<Self::Vertex> = vertices.into_iter().map(|x| x.into()).collect();

        // Check if vertices labels are non empty.
        if vertices.contains("") {
            return Err(E::EmptyVertexLabel);
        }
        // Check if vertices set is not consistent with given adjacency matrix.
        if vertices.len() != adjacency_matrix.nrows() {
            return Err(E::InconsistentMatrix);
        }
        // Check if adjacency matrix is not square.
        if !adjacency_matrix.is_square() {
            return Err(E::NonSquareMatrix);
        }
        // Check if adjacency matrix is not symmetric.
        if adjacency_matrix != adjacency_matrix.t() {
            return Err(E::NonSymmetricMatrix);
        }

        // Map vertices labels to vertices indices.
        let vertices_indexes: FnvBiHashMap<Self::Vertex, usize> =
            vertices.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();

        // Cast to standard memory layout (i.e. C layout), if not already.
        let adjacency_matrix = adjacency_matrix.as_standard_layout().into_owned();

        // Compute size.
        let size = adjacency_matrix
            .indexed_iter()
            .filter(|((i, j), flag)| **flag && i <= j)
            .count();

        Ok(Self {
            vertices,
            vertices_indexes,
            adjacency_matrix,
            size: size,
        })
    }
}

/* Implement PartialOrdGraph trait. */

impl PartialEq for DenseMatrixUndirectedGraph {
    fn eq(&self, other: &Self) -> bool {
        // Check that V(G) == V(H) && E(G) == E(H).
        self.vertices.eq(&other.vertices) && self.adjacency_matrix.eq(&other.adjacency_matrix)
    }
}

impl Eq for DenseMatrixUndirectedGraph {}

impl PartialOrd for DenseMatrixUndirectedGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare vertices sets.
        let vertices = self.vertices.partial_cmp(&other.vertices);
        // Match vertices ordering.
        match vertices {
            // If the vertices sets are comparable ...
            Some(vertices) => {
                // ... compare edges sets.
                let edges = self.edges().into_iter().partial_cmp(other.edges().into_iter());
                // Match edges ordering.
                match edges {
                    // If the edges sets are comparable ...
                    Some(edges) => match vertices == edges {
                        // ... and vertices and edges are consistent, then return ordering.
                        true => Some(vertices),
                        // ... else return none.
                        false => None,
                    },
                    // ... else return none.
                    None => None,
                }
            }
            // ... else return none.
            None => None,
        }
    }
}

impl PartialOrdGraph for DenseMatrixUndirectedGraph {}
