use std::{
    fmt::{Debug, Display},
    hash::Hash, ops::Deref,
};

/// Base graph trait.
pub trait BaseGraph: Clone + Debug + Display + Deref<Target = Self::Data> {
    /// Data type.
    type Data;

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

    /// Order of the graph, i.e. |V|.
    #[inline]
    fn order(&self) -> usize {
        self.vertices().len()
    }

    /// Iterator over the vertices set.
    fn vertices<'a>(&'a self) -> Self::VerticesIter<'a>;

    /// Checks if a vertex is in the graph.
    fn has_vertex(&self, x: &Self::Vertex) -> bool;

    /// Adds a vertex to the graph given its label, if not present.
    fn add_vertex<V>(&mut self, x: V) -> Self::Vertex
    where
        V: Into<Self::Vertex>;

    /// Removes a vertex from the graph, if present.
    fn del_vertex(&mut self, x: &Self::Vertex);

    /// Size of the graph, i.e. |E|.
    #[inline]
    fn size(&self) -> usize {
        self.edges().len()
    }

    /// Iterator over the edges set.
    fn edges<'a>(&'a self) -> Self::EdgesIter<'a>;

    /// Checks if an edge is in the graph.
    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Adds an edge to the graph, if not present.
    fn add_edge<'a>(&'a mut self, x: &Self::Vertex, y: &Self::Vertex);

    /// Removes an edge from the graph, if present.
    fn del_edge<'a>(&'a mut self, x: &Self::Vertex, y: &Self::Vertex);

    /// Checks if a vertex is adjacent to another vertex.
    fn is_adjacent(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;
}
