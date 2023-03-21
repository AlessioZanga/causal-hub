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
pub struct MixedDenseAdjacencyMatrixGraph {
    labels: BTreeSet<String>,
    labels_indices: BiHashMap<String, usize>,
    undirected_adjacency_matrix: DenseAdjacencyMatrix,
    directed_adjacency_matrix: DenseAdjacencyMatrix,
    size: usize,
}

// TODO: Deref
// TODO: LabelsIterator, EdgesIterator, AdjacentsIterator
// TODO: Hash
// TODO: BaseGraph
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