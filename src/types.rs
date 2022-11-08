use bimap::BiHashMap;
use fnv::FnvBuildHasher;

/// [Bidirectional map](https://docs.rs/bimap/latest) with
/// [Fowler-Noll-Vo hash function](https://docs.rs/fnv/latest).
pub type FnvBiHashMap<L, R> = BiHashMap<L, R, FnvBuildHasher, FnvBuildHasher>;
