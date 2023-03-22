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
        MultGraph, PartialOrdGraph, PathGraph, SubGraph,
    },
    io::DOT,
    models::MoralGraph,
    prelude::BFS,
    types::{
        AdjacencyList, DenseAdjacencyMatrix, EdgeList, MultipleDenseAdjacencyMatrix,
        SparseAdjacencyMatrix,
    },
    utils::partial_cmp_sets,
    Adj, Ch, Pa, E, V,
};

/// Mixed graph struct based on a couple of dense adjacency matrix data structures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MixedDenseAdjacencyMatrixGraph {
    labels: BTreeSet<String>,
    labels_indices: BiHashMap<String, usize>,
    adjacency_matrix: MultipleDenseAdjacencyMatrix,
    size: usize,
}

impl MixedDenseAdjacencyMatrixGraph {
    fn deref(&self, which: usize) -> &DenseAdjacencyMatrix {
        &self.adjacency_matrix[which].1 //FIXME: case of which > len(adjacency_matrix)
    }

    fn merged_matrix(&self) -> DenseAdjacencyMatrix {
        let mut merged_matrix: Array2<bool> = self.deref(0).clone();
        merged_matrix.indexed_iter_mut().for_each(|((i, j), f)| {
            if i >= j {
                *f = false;
            }
        });
        let merged_matrix = merged_matrix | self.deref(1);
        merged_matrix
    }
}

pub struct LabelsIterator<'a> {
    graph: &'a MixedDenseAdjacencyMatrixGraph,
    iter: Range<usize>,
}

impl<'a> LabelsIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a MixedDenseAdjacencyMatrixGraph) -> Self {
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
    g: &'a MixedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> UndirectedEdgesIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a MixedDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .deref(0)
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
    g: &'a MixedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> DirectedEdgesIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a MixedDenseAdjacencyMatrixGraph) -> Self {
        Self {
            g,
            iter: g
                .deref(1)
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
    g: &'a MixedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        IndexedIter<'a, bool, Ix2>,
        fn(((usize, usize), &bool)) -> Option<(usize, usize)>,
    >,
    size: usize,
}

impl<'a> EdgesIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a MixedDenseAdjacencyMatrixGraph) -> Self {
        let merged_matrix = g.merged_matrix();
        Self {
            g,
            iter: merged_matrix
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
    g: &'a MixedDenseAdjacencyMatrixGraph,
    iter: FilterMap<
        Enumerate<ndarray::iter::Iter<'a, bool, Dim<[usize; 1]>>>,
        fn((usize, &bool)) -> Option<usize>,
    >,
}

impl<'a> AdjacentsIterator<'a> {
    /// Constructor.
    #[inline]
    pub fn new(g: &'a MixedDenseAdjacencyMatrixGraph, x: usize) -> Self {
        let merged_matrix = g.merged_matrix();
        Self {
            g,
            iter: merged_matrix
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

impl Display for MixedDenseAdjacencyMatrixGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "MixedGraph {{ ")?;
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

impl Hash for MixedDenseAdjacencyMatrixGraph {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.hash(state);
        self.merged_matrix().hash(state); //FIXME: correct?
    }
}

// TODO: Implementing AdvGraph

impl MultGraph for MixedDenseAdjacencyMatrixGraph {
    type Data = DenseAdjacencyMatrix;

    type Direction = directions::Mixed;

    type LabelsIter<'a> = LabelsIterator<'a>;

    type VerticesIter<'a> = Range<usize>;

    type UndEdgesIter<'a> = UndirectedEdgesIterator<'a>;

    type DirEdgesIter<'a> = DirectedEdgesIterator<'a>;

    type EdgesIter<'a> = EdgesIterator<'a>;

    type AdjacentsIter<'a> = AdjacentsIterator<'a>;

    fn new<V, I, M, J>(vertices: I, edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        M: IntoIterator<Item = (V, V)>,
        J: IntoIterator<Item = (usize, M)>,
    {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn label(&self, x: usize) -> &str {
        todo!()
    }

    fn labels(&self) -> Self::LabelsIter<'_> {
        todo!()
    }

    fn vertex(&self, x: &str) -> usize {
        todo!()
    }

    fn vertices(&self) -> Self::VerticesIter<'_> {
        todo!()
    }

    fn order(&self) -> usize {
        V!(self).len()
    }

    fn has_vertex(&self, x: usize) -> bool {
        V!(self).any(|y| y == x)
    }

    fn add_vertex<V>(&mut self, x: V) -> usize
    where
        V: Into<String>,
    {
        todo!()
    }

    fn del_vertex(&mut self, x: usize) -> bool {
        todo!()
    }

    fn und_edges(&self) -> Self::UndEdgesIter<'_> {
        todo!()
    }

    fn dir_edges(&self) -> Self::DirEdgesIter<'_> {
        todo!()
    }

    fn edges(&self) -> Self::EdgesIter<'_> {
        todo!()
    }

    fn size(&self) -> usize {
        E!(self).len()
    }
    
    fn has_und_edge(&self, x: usize, y: usize) -> bool {
        self.und_edges().any(|z| z == (x, y))
    }

    fn has_dir_edge(&self, x: usize, y: usize) -> bool {
        self.dir_edges().any(|z| z == (x, y))
    }

    fn has_edge(&self, x: usize, y: usize) -> bool {
        E!(self).any(|z| z == (x, y))
    }

    fn add_edge(&mut self, which: usize, x: usize, y: usize) -> bool {
        todo!()
    }

    fn del_edge(&mut self, x: usize, y: usize) -> bool {
        todo!()
    }

    fn adjacents(&self, x: usize) -> Self::AdjacentsIter<'_> {
        todo!()
    }

    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        Adj!(self, x).any(|z| z == y)
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
