use std::collections::{BTreeMap, BTreeSet};

use bimap::BiHashMap;
use fnv::FnvBuildHasher;
use ndarray::prelude::*;
use sprs::TriMat;

/// Edge list type.
pub type EdgeList<V> = BTreeSet<(V, V)>;

/// Adjacency list type.
pub type AdjacencyList<V> = BTreeMap<V, BTreeSet<V>>;

/// Dense adjacency matrix type.
pub type DenseAdjacencyMatrix = Array2<bool>;

/// Sparse adjacency matrix type.
pub type SparseAdjacencyMatrix = TriMat<bool>;

/// [Bidirectional map](https://docs.rs/bimap/latest) with
/// [Fowler-Noll-Vo hash function](https://docs.rs/fnv/latest) type.
pub type FnvBiHashMap<L, R> = BiHashMap<L, R, FnvBuildHasher>;
