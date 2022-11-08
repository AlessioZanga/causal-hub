use super::{BaseGraph, DefaultGraph, PartialOrdGraph};

/// Undirected graph trait.
pub trait UndirectedGraph: BaseGraph + DefaultGraph + PartialOrdGraph {
    /// Neighbors iterator type.
    type NeighborsIter<'a>: ExactSizeIterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;
    
    /// Iterator over the neighbors set.
    fn neighbors<'a>(&'a self, x: &Self::Vertex) -> Self::NeighborsIter<'a>;

    /// Checks if a vertex is neighbor of another vertex.
    #[inline]
    fn is_neighbor(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
        self.is_adjacent(x, y)
    }

    /// Computes the degree of a vertex.
    #[inline]
    fn degree(&self, x: &Self::Vertex) -> usize {
        self.neighbors(x).len()
    }
}
