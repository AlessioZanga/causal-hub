use bimap::BiHashMap;
use fnv::FnvBuildHasher;
use ndarray::prelude::*;

/// Adjacency matrix type.
pub type AdjacencyMatrix = Array2<bool>;

/// [Bidirectional map](https://docs.rs/bimap/latest) with
/// [Fowler-Noll-Vo hash function](https://docs.rs/fnv/latest) type.
pub type FnvBiHashMap<L, R> = BiHashMap<L, R, FnvBuildHasher>;
