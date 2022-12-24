use std::iter::FusedIterator;

use super::BaseGraph;

/// Directed graph trait.
pub trait DirectedGraph: BaseGraph {
    /// Ancestors iterator type.
    type AncestorsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Parents iterator type.
    type ParentsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Children iterator type.
    type ChildrenIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Descendants iterator type.
    type DescendantsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Iterator over the ancestors set.
    // FIXME: Add docs.
    fn ancestors<'a>(self, x: usize) -> Self::AncestorsIter<'a>;

    /// Checks if a vertex is ancestor of another vertex.
    // FIXME: Add docs.
    fn is_ancestor(&self, x: usize, y: usize) -> bool;

    /// Iterator over the parents set.
    // FIXME: Add docs.
    fn parents<'a>(self, x: usize) -> Self::ParentsIter<'a>;

    /// Checks if a vertex is parent of another vertex.
    // FIXME: Add docs.
    fn is_parent(&self, x: usize, y: usize) -> bool;

    /// Iterator over the children set.
    // FIXME: Add docs.
    fn children<'a>(self, x: usize) -> Self::ChildrenIter<'a>;

    /// Checks if a vertex is child of another vertex.
    // FIXME: Add docs.
    fn is_child(&self, x: usize, y: usize) -> bool;

    /// Iterator over the descendants set.
    // FIXME: Add docs.
    fn descendants<'a>(self, x: usize) -> Self::DescendantsIter<'a>;

    /// Checks if a vertex is descendant of another vertex.
    // FIXME: Add docs.
    fn is_descendant(&self, x: usize, y: usize) -> bool;

    /// In-degree of a vertex.
    // FIXME: Add docs.
    fn in_degree(&self, x: usize) -> usize;

    /// Out-degree of a vertex.
    // FIXME: Add docs.
    fn out_degree(&self, x: usize) -> usize;
}
