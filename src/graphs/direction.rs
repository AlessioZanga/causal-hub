use std::iter::FusedIterator;

use super::{BaseGraph, DefaultGraph, PartialOrdGraph, SubGraph};

/// Directions pseudo-enumerator for generics algorithms.
pub mod directions {
    /// Undirected pseudo-enumerator for generics algorithms.
    #[derive(Clone, Copy, Debug)]
    pub struct Undirected;

    /// Directed pseudo-enumerator for generics algorithms.
    #[derive(Clone, Copy, Debug)]
    pub struct Directed;

    /// Mixed pseudo-enumerator for generics algorithms.
    #[derive(Clone, Copy, Debug)]
    pub struct Mixed;
}

/// Neighbors iterator.
///
/// Return the vertex iterator representing $Ne(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Ne {
    ($g:expr, $x:expr) => {
        $g.neighbors($x)
    };
}

/// Undirected graph trait.
pub trait UndirectedGraph: BaseGraph + DefaultGraph + PartialOrdGraph + SubGraph {
    /// Neighbors iterator type.
    type NeighborsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Neighbors iterator.
    ///
    /// Iterates over the vertex set $Ne(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = Graph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.vertex("A");
    ///
    /// // Use the neighbors iterator.
    /// assert!(g.neighbors(x).eq([0, 1, 2]));
    ///
    /// // Use the associated macro 'Ne!'.
    /// assert!(g.neighbors(x).eq(Ne!(g, x)));
    /// ```
    ///
    fn neighbors(&self, x: usize) -> Self::NeighborsIter<'_>;

    /// Checks neighbor vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is neighbor of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex indexes does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = Graph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.vertex("A"), g.vertex("B"));
    ///
    /// // Check edge.
    /// assert!(g.is_neighbor(x, y));
    /// assert!(Ne!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_neighbor(&self, x: usize, y: usize) -> bool {
        Ne!(self, x).any(|z| z == y)
    }

    /// Degree of a vertex.
    ///
    /// Computes the degree of a given vertex, i.e. $|Ne(\mathcal{G}, X)|$.
    ///
    /// # Panics
    ///
    /// The vertex index does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let mut g = Graph::from(e);
    ///
    /// // Choose a vertex.
    /// let x = g.vertex("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.degree(x), 3);
    /// assert_eq!(g.degree(x), Ne!(g, x).count());
    /// ```
    ///
    fn degree(&self, x: usize) -> usize {
        Ne!(self, x).count()
    }
}

/// Ancestors iterator.
///
/// Return the vertex iterator representing $An(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! An {
    ($g:expr, $x:expr) => {
        $g.ancestors($x)
    };
}

/// Parents iterator.
///
/// Return the vertex iterator representing $Pa(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Pa {
    ($g:expr, $x:expr) => {
        $g.parents($x)
    };
}

/// Children iterator.
///
/// Return the vertex iterator representing $Ch(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Ch {
    ($g:expr, $x:expr) => {
        $g.children($x)
    };
}

/// Descendants iterator.
///
/// Return the vertex iterator representing $De(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! De {
    ($g:expr, $x:expr) => {
        $g.descendants($x)
    };
}

/// Directed graph trait.
pub trait DirectedGraph: BaseGraph + DefaultGraph + PartialOrdGraph + SubGraph {
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

    /// Ancestors iterator.
    ///
    /// Iterates over the vertex set $An(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// The vertex label does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.vertex("B");
    ///
    /// // Use the ancestors iterator.
    /// assert!(g.ancestors(x).eq([0, 2]));
    ///
    /// // Use the associated macro 'An!'.
    /// assert!(g.ancestors(x).eq(An!(g, x)));
    /// ```
    ///
    fn ancestors(&self, x: usize) -> Self::AncestorsIter<'_>;

    /// Checks ancestor vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is ancestor of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.vertex("A"), g.vertex("C"));
    ///
    /// // Check edge.
    /// assert!(g.is_ancestor(x, y));
    /// assert!(An!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_ancestor(&self, x: usize, y: usize) -> bool {
        An!(self, x).any(|z| z == y)
    }

    /// Parents iterator.
    ///
    /// Iterates over the vertex set $Pa(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// The vertex label does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.vertex("A");
    ///
    /// // Use the parents iterator.
    /// assert!(g.parents(x).eq([0, 2]));
    ///
    /// // Use the associated macro 'Pa!'.
    /// assert!(g.parents(x).eq(Pa!(g, x)));
    /// ```
    ///
    fn parents(&self, x: usize) -> Self::ParentsIter<'_>;

    /// Checks parent vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is parent of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.vertex("A"), g.vertex("C"));
    ///
    /// // Check edge.
    /// assert!(g.is_parent(x, y));
    /// assert!(Pa!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_parent(&self, x: usize, y: usize) -> bool {
        Pa!(self, x).any(|z| z == y)
    }

    /// Children iterator.
    ///
    /// Iterates over the vertex set $Ch(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// The vertex label does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.vertex("A");
    ///
    /// // Use the children iterator.
    /// assert!(g.children(x).eq([0, 1]));
    ///
    /// // Use the associated macro 'Ch!'.
    /// assert!(g.children(x).eq(Ch!(g, x)));
    /// ```
    ///
    fn children(&self, x: usize) -> Self::ChildrenIter<'_>;

    /// Checks children vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is child of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.vertex("C"), g.vertex("A"));
    ///
    /// // Check edge.
    /// assert!(g.is_child(x, y));
    /// assert!(Ch!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_child(&self, x: usize, y: usize) -> bool {
        Ch!(self, x).any(|z| z == y)
    }

    /// Descendants iterator.
    ///
    /// Iterates over the vertex set $De(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// The vertex label does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.vertex("C");
    ///
    /// // Use the descendants iterator.
    /// assert!(g.descendants(x).eq([0, 1]));
    ///
    /// // Use the associated macro 'De!'.
    /// assert!(g.descendants(x).eq(De!(g, x)));
    /// ```
    ///
    fn descendants(&self, x: usize) -> Self::DescendantsIter<'_>;

    /// Checks descendant vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is descendant of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.vertex("C"), g.vertex("A"));
    ///
    /// // Check edge.
    /// assert!(g.is_descendant(x, y));
    /// assert!(De!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_descendant(&self, x: usize, y: usize) -> bool {
        De!(self, x).any(|z| z == y)
    }

    /// In-degree of a given vertex.
    ///
    /// Computes the in-degree of a given vertex, i.e. $|Pa(\mathcal{G}, X)|$.
    ///
    /// # Panics
    ///
    /// The vertex label does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let mut g = DiGraph::from(e);
    ///
    /// // Choose a vertex.
    /// let x = g.vertex("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.in_degree(x), 2);
    /// assert_eq!(g.in_degree(x), Pa!(g, x).count());
    /// ```
    ///
    fn in_degree(&self, x: usize) -> usize {
        Pa!(self, x).count()
    }

    /// Out-degree of a given vertex.
    ///
    /// Computes the out-degree of a given vertex, i.e. $|Ch(\mathcal{G}, X)|$.
    ///
    /// # Panics
    ///
    /// The vertex label does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let mut g = DiGraph::from(e);
    ///
    /// // Choose a vertex.
    /// let x = g.vertex("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.out_degree(x), 2);
    /// assert_eq!(g.out_degree(x), Ch!(g, x).count());
    /// ```
    ///
    fn out_degree(&self, x: usize) -> usize {
        Ch!(self, x).count()
    }
}

/// Convert to undirected graph trait.
pub trait IntoUndirectedGraph {
    /// Associated undirected graph type.
    type UndirectedGraph: UndirectedGraph<Direction = directions::Undirected>;

    /// Make an undirected copy of the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a directed graph.
    /// let g = DiGraph::new(
    ///     ["A", "B", "C"],
    ///     [("A", "B")]
    /// );
    ///
    /// // Get an undirected copy.
    /// let h = g.to_undirected();
    ///
    /// // The undirected copy has the same vertex set.
    /// assert_eq!(V!(g), V!(h));
    ///
    /// // The vertices are still connected.
    /// assert!(h.has_edge(0, 1));
    ///
    /// ```
    ///
    fn to_undirected(&self) -> Self::UndirectedGraph;
}
