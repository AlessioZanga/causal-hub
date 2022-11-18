use super::{BaseGraph, ErrorGraph as E};
use crate::types::AdjacencyMatrix;

/// Default graph trait.
pub trait DefaultGraph: BaseGraph + Default {
    /// Null graph constructor.
    #[inline]
    fn null() -> Self {
        Default::default()
    }

    /// Empty graph constructor given vertices set.
    fn empty<I, V>(vertices: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>;

    /// Complete graph constructor given vertices set.
    fn complete<I, V>(vertices: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>;

    /// Adjacency matrix constructor.
    fn with_adjacency_matrix<I, V>(vertices: I, adjacency_matrix: AdjacencyMatrix) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>;
}
