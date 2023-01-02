use std::{
    collections::{BTreeMap, BTreeSet},
    hash::BuildHasherDefault,
};

use bimap::BiHashMap;
use ndarray::prelude::*;
use nohash_hasher::BuildNoHashHasher;
use rustc_hash::FxHasher;
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
/// [FxHasher](https://docs.rs/rustc_hash/latest) type.
pub type FxBiHashMap<L, R> = BiHashMap<L, R, BuildHasherDefault<FxHasher>, BuildNoHashHasher<R>>;
