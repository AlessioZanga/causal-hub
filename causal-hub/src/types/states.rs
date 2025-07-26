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

/// Create a `Set` from a list of values.
#[macro_export]
macro_rules! set {
    [] => { $crate::types::Set::default() };
    [$($value:expr,)+] => { $crate::set!($($value),+) };
    [$($value:expr),*] => {
        {
            // Note: `stringify!($value)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const CAPACITY: usize = <[()]>::len(&[$({ stringify!($value); }),*]);
            let mut set = $crate::types::Set::with_capacity_and_hasher(
                CAPACITY,
                fxhash::FxBuildHasher::default())
            ;
            $(
                set.insert($value);
            )*
            set
        }
    };
}

/// Create a `Map` from a list of key-value pairs.
#[macro_export]
macro_rules! map {
    [] => { $crate::types::Map::default() };
    [$(($key:expr, $value:expr),)+] => { $crate::map!($(($key, $value)),+) };
    [$(($key:expr, $value:expr)),*] => {
        {
            const CAPACITY: usize = <[()]>::len(&[$({ stringify!($key); stringify!($value); }),*]);
            let mut map = $crate::types::Map::with_capacity_and_hasher(
                CAPACITY,
                fxhash::FxBuildHasher::default())
            ;
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}
