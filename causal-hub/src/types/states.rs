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
            const CAPACITY: usize = <[()]>::len(&[$({ stringify!($value); }),*]);
            let mut set = $crate::types::Set::with_capacity_and_hasher(
                CAPACITY,
                fxhash::FxBuildHasher::default(),
            );
            $(
                set.insert($value);
            )*
            set
        }
    };
}

/// Create a `Labels` set from a list of string-like values.
#[macro_export]
macro_rules! labels {
    [] => { $crate::types::Labels::default() };
    [$($label:expr),+ $(,)?] => {
        {
            const CAPACITY: usize = <[()]>::len(&[$({ stringify!($label); }),*]);
            let mut labels = $crate::types::Labels::with_capacity_and_hasher(
                CAPACITY,
                fxhash::FxBuildHasher::default(),
            );
            $(
                labels.insert(::std::string::ToString::to_string(&$label));
            )*
            labels
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
                fxhash::FxBuildHasher::default(),
            );
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

/// Create a `States` map from a list of variable-state pairs.
#[macro_export]
macro_rules! states {
    [] => { $crate::types::States::default() };
    [$(($label:expr, [$($state:expr),* $(,)?]),)+] => { $crate::states!($(($label, [$($state),*])),+) };
    [$(($label:expr, [$($state:expr),* $(,)?])),*] => {
        {
            const CAPACITY: usize = <[()]>::len(&[$({ stringify!($label); $( stringify!($state); )* }),*]);
            let mut states = $crate::types::States::with_capacity_and_hasher(
                CAPACITY,
                fxhash::FxBuildHasher::default(),
            );
            $(
                // Convert the label to a string.
                let label = ::std::string::ToString::to_string(&$label);
                // Create a set of states for the current label.
                let state = $crate::set![$(
                    ::std::string::ToString::to_string(&$state)
                ),*];
                // Insert the label and the corresponding set of states into the map.
                states.insert(label, state);
            )*
            states
        }
    };
}
