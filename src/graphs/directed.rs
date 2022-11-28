use super::BaseGraph;

/// Directed graph trait.
pub trait DirectedGraph: BaseGraph {
    /// Ancestors iterator type.
    type AncestorsIter<'a>: Iterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Parents iterator type.
    type ParentsIter<'a>: Iterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Children iterator type.
    type ChildrenIter<'a>: Iterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Descendants iterator type.
    type DescendantsIter<'a>: Iterator<Item = &'a Self::Vertex>
    where
        Self: 'a,
        Self::Vertex: 'a;

    /// Iterator over the ancestors set.
    fn ancestors<'a>(&'a self, x: &'a Self::Vertex) -> Self::AncestorsIter<'a>;

    /// Checks if a vertex is ancestor of another vertex.
    fn is_ancestor(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Iterator over the parents set.
    fn parents<'a>(&'a self, x: &'a Self::Vertex) -> Self::ParentsIter<'a>;

    /// Checks if a vertex is parent of another vertex.
    fn is_parent(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Iterator over the children set.
    fn children<'a>(&'a self, x: &'a Self::Vertex) -> Self::ChildrenIter<'a>;

    /// Checks if a vertex is child of another vertex.
    fn is_child(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Iterator over the descendants set.
    fn descendants<'a>(&'a self, x: &'a Self::Vertex) -> Self::DescendantsIter<'a>;

    /// Checks if a vertex is descendant of another vertex.
    fn is_descendant(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// In-degree of a vertex.
    fn in_degree(&self, x: &Self::Vertex) -> usize;

    /// Out-degree of a vertex.
    fn out_degree(&self, x: &Self::Vertex) -> usize;
}
