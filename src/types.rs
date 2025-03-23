use fxhash::FxBuildHasher;
use indexmap::{IndexMap, IndexSet};

pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub type FxIndexSet<T> = IndexSet<T, FxBuildHasher>;
