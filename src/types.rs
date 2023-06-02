use std::{
    collections::{BTreeMap, BTreeSet},
    hash::BuildHasherDefault,
};

use indexmap::{IndexMap, IndexSet};
use ndarray::Array2 as Matrix;
use rustc_hash::FxHasher;

/// Edge list type.
pub type EdgeList<V> = BTreeSet<(V, V)>;

/// Adjacency list type.
pub type AdjacencyList<V> = BTreeMap<V, BTreeSet<V>>;

/// Separation sets type.
pub type SepSets = BTreeMap<(usize, usize), BTreeSet<usize>>;

/// Dense adjacency matrix type.
pub type DenseAdjacencyMatrix = Matrix<bool>;

/// IndexSet with FxHasher.
pub type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<FxHasher>>;

/// IndexMap with FxHasher.
pub type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;
