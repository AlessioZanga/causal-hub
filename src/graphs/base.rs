use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Base graph trait.
pub trait BaseGraph: Clone + Debug + Display {
    /// Vertex type.
    type Vertex: Clone + Debug + Eq + Ord + Hash;

    /// Vertex iterator type.
    type VertexIter<'a>: ExactSizeIterator<Item = &'a Self::Vertex>
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

    /// Edge iterator type.
    type EdgeIter<'a>: ExactSizeIterator<Item = Self::Edge<'a>>
    where
        Self: 'a;

    /// Order of the graph, i.e. |V|.
    fn order(&self) -> usize;

    /// Iterator over the vertices set.
    fn vertices<'a>(&'a self) -> Self::VertexIter<'a>;

    /// Checks if a vertex is in the graph.
    fn has_vertex(&self, x: &Self::Vertex) -> bool;

    /// Adds a vertex to the graph given its label, if not present.
    fn add_vertex(&mut self, x: String) -> Self::Vertex;

    /// Removes a vertex from the graph, if present.
    fn del_vertex(&mut self, x: &Self::Vertex) -> Self::Vertex;

    /// Size of the graph, i.e. |E|.
    fn size(&self) -> usize;

    /// Iterator over the edges set.
    fn edges<'a>(&'a self) -> Self::EdgeIter<'a>;

    /// Checks if an edge is in the graph.
    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Adds an edge to the graph, if not present.
    fn add_edge<'a>(&'a mut self, x: &Self::Vertex, y: &Self::Vertex) -> Self::Edge<'a>;

    /// Removes an edge from the graph, if present.
    fn del_edge<'a>(&'a mut self, x: &Self::Vertex, y: &Self::Vertex) -> Self::Edge<'a>;

    /// Checks if a vertex is adjacent to another vertex.
    fn is_adjacent(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;
}
