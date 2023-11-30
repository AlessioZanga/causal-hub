use std::{
    collections::{BTreeMap, BTreeSet},
    hash::BuildHasherDefault,
};

use indexmap::{IndexMap, IndexSet};
use ndarray::Array2;
use rustc_hash::FxHasher;

pub type EdgeList<V> = BTreeSet<(V, V)>;

pub type AdjacencyList<V> = BTreeMap<V, BTreeSet<V>>;

pub type DenseAdjacencyMatrix = Array2<bool>;

pub type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<FxHasher>>;

pub type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

pub type SepSets = FxIndexMap<(usize, usize), FxIndexSet<usize>>;
