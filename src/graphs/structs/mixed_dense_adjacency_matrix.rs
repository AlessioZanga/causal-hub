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


// TODO: AdvGraph
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
