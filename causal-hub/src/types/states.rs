use fxhash::FxBuildHasher;
use indexmap::{IndexMap, IndexSet};

/// A type alias for a hash map with a fast hash function.
pub type Map<K, V> = IndexMap<K, V, FxBuildHasher>;
/// A type alias for a hash set with a fast hash function.
pub type Set<T> = IndexSet<T, FxBuildHasher>;
/// A type alias for a set of labels, which are strings.
pub type Labels = Set<String>;
/// A type alias for a hash map of states, where keys are variable names and values are sets of states.
pub type States = Map<String, Set<String>>;
