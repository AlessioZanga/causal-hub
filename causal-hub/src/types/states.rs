use fxhash::FxBuildHasher;
use indexmap::{IndexMap, IndexSet};

/// A type alias for a hash map with a fast hash function.
pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
/// A type alias for a hash set with a fast hash function.
pub type FxIndexSet<T> = IndexSet<T, FxBuildHasher>;
/// A type alias for a set of labels, which are strings.
pub type Labels = FxIndexSet<String>;
/// A type alias for a hash map of states, where keys are variable names and values are sets of states.
pub type States = FxIndexMap<String, FxIndexSet<String>>;
