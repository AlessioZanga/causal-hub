/// Acyclic graph trait.
pub trait AcyclicGraph {
    /// Checks if the graph contains no cycles.
    fn is_acyclic(&self) -> bool;
}
