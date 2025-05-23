use fxhash::FxBuildHasher;
use indexmap::{IndexMap, IndexSet};

/// A type alias for a hash map with a fast hash function.
pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
/// A type alias for a hash set with a fast hash function.
pub type FxIndexSet<T> = IndexSet<T, FxBuildHasher>;
