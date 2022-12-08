use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Directions pseudo-enumerator for generics algorithms.
pub mod directions {
    /// Undirected pseudo-enumerator for generics algorithms.
    pub struct Undirected;
    /// Directed pseudo-enumerator for generics algorithms.
    pub struct Directed;
}

/// Base graph trait.
pub trait BaseGraph: Clone + Debug + Display {
    /// Data type.
    type Data;

    /// Directional type.
    type Direction;

    /// Vertex type.
    type Vertex: Clone + Debug + Eq + Ord + Hash;

    /// Vertices iterator type.
    type VerticesIter<'a>: ExactSizeIterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Edge type.
    // TODO: Replace with "associated type defaults" once stabilized.
    type Edge<'a>: From<(&'a Self::Vertex, &'a Self::Vertex)>
        + Into<(&'a Self::Vertex, &'a Self::Vertex)>
        + Eq
        + Ord
        + Hash
    where
        Self: 'a;

    /// Edges iterator type.
    type EdgesIter<'a>: ExactSizeIterator<Item = Self::Edge<'a>>
    where
        Self: 'a;

    /// Adjacents vertices iterator type.
    type AdjacentsIter<'a>: Iterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Order of the graph, i.e. |V|.
    fn order(&self) -> usize;

    /// Iterator over the vertices set.
    fn vertices(&self) -> Self::VerticesIter<'_>;

    /// Checks if a vertex is in the graph.
    fn has_vertex(&self, x: &Self::Vertex) -> bool;

    /// Adds a vertex to the graph given its label, if not present.
    fn add_vertex<V>(&mut self, x: V) -> Self::Vertex
    where
        V: Into<Self::Vertex>;

    /// Removes a vertex from the graph, if present.
    fn del_vertex(&mut self, x: &Self::Vertex) -> bool;

    /// Size of the graph, i.e. |E|.
    fn size(&self) -> usize;

    /// Iterator over the edges set.
    fn edges(&self) -> Self::EdgesIter<'_>;

    /// Checks if an edge is in the graph.
    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Adds an edge to the graph, if not present.
    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Removes an edge from the graph, if present.
    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Iterator over the adjacents vertices set.
    fn adjacents<'a>(&'a self, x: &'a Self::Vertex) -> Self::AdjacentsIter<'a>;

    /// Checks if a vertex is adjacent to another vertex.
    fn is_adjacent(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;
}
