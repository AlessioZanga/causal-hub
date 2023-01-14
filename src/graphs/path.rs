/// Path checks on graph trait.
pub trait PathGraph {
    /// Checks if the graph contains no cycles.
    fn is_acyclic(&self) -> bool;
}
