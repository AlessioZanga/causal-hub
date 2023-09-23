use super::Graph;

/// Partial order graph trait.
pub trait PartialOrdGraph: Graph + PartialOrd {
    /// Checks if `self` is a subgraph of `other`.
    #[inline]
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    /// Checks if `self` is a supergraph of `other`.
    #[inline]
    fn is_supergraph(&self, other: &Self) -> bool {
        self >= other
    }
}
