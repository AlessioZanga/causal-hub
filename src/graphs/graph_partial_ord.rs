use super::GraphBase;

/// Partial order graph trait.
pub trait GraphPartialOrd: GraphBase + Eq + PartialOrd {
    /// Checks if `self` is a subgraph of `other`.
    fn is_subgraph_of(&self, other: &Self) -> bool {
        self <= other
    }

    /// Checks if `self` is a supergraph of `other`.
    fn is_supergraph_of(&self, other: &Self) -> bool {
        self >= other
    }
}
