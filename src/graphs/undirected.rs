use std::iter::FusedIterator;

use super::BaseGraph;

/// Undirected graph trait.
pub trait UndirectedGraph: BaseGraph {
    /// Neighbors iterator type.
    type NeighborsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Iterator over the neighbors set.
    fn neighbors(&self, x: usize) -> Self::NeighborsIter<'_>;

    /// Checks if a vertex is neighbor of another vertex.
    fn is_neighbor(&self, x: usize, y: usize) -> bool;
}
