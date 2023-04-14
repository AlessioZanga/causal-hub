/// Path checks on graph trait.
pub trait PathGraph {
    /// Checks if the graph contains a path.
    fn has_path_by_index(&self, x: usize, y: usize) -> bool;

    /// Checks if the graph contains no cycles.
    fn is_acyclic(&self) -> bool;
}
