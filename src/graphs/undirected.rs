use super::{BaseGraph, DefaultGraph, PartialOrdGraph};

/// Undirected graph trait.
pub trait UndirectedGraph: BaseGraph + DefaultGraph + PartialOrdGraph {
    /// Neighbors iterator type.
    type NeighborsIter<'a>: Iterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Iterator over the neighbors set.
    fn neighbors<'a>(&'a self, x: &'a Self::Vertex) -> Self::NeighborsIter<'a>;

    /// Checks if a vertex is neighbor of another vertex.
    fn is_neighbor(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Computes the degree of a vertex.
    fn degree(&self, x: &Self::Vertex) -> usize;
}
