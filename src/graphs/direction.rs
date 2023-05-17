use std::iter::FusedIterator;

use super::{BaseGraph, PartialOrdGraph, SubGraph};

/// Directions pseudo-enumerator for generics algorithms.
pub mod directions {
    /// Undirected pseudo-enumerator for generics algorithms.
    #[derive(Clone, Copy, Debug)]
    pub struct Undirected;

    /// Directed pseudo-enumerator for generics algorithms.
    #[derive(Clone, Copy, Debug)]
    pub struct Directed;

    /// Partially directed pseudo-enumerator for generics algorithms.
    #[derive(Clone, Copy, Debug)]
    pub struct PartiallyDirected;
}

/// Neighbors iterator.
///
/// Return the vertex iterator representing $Ne(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Ne {
    ($g:expr, $x:expr) => {
        $g.get_neighbors_by_index($x)
    };
}
/// Undirected edges iterator.
///
/// Return the $E(\mathcal{G}, X)$ subset where edges are undirected as an iterator.///
#[macro_export]
macro_rules! uE {
    ($g:expr) => {
        $g.get_undirected_edges_index()
    };
}

/// Directed edges iterator.
///
/// Return the $E(\mathcal{G}, X)$ subset where edges are directed as an iterator.
///
#[macro_export]
macro_rules! dE {
    ($g:expr) => {
        $g.get_directed_edges_index()
    };
}

/// Undirected graph trait.
pub trait UndirectedGraph: BaseGraph + PartialOrdGraph + SubGraph {
    /// Edges iterator type.
    type UndirectedEdgesIndexIter<'a>: Iterator<Item = (usize, usize)>
        + ExactSizeIterator
        + FusedIterator
    where
        Self: 'a;

    /// Neighbors iterator type.
    type NeighborsIndexIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Size of the undirected subgraph.
    fn size_of_maximal_undirected_subgraph(&self) -> usize;

    /// Undirected edges iterator
    fn get_undirected_edges_index(&self)
        -> <Self as UndirectedGraph>::UndirectedEdgesIndexIter<'_>;

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
    /// let x = g.get_vertex_index("A");
    ///
    /// // Use the neighbors iterator.
    /// assert!(g.get_neighbors_by_index(x).eq([0, 1, 2]));
    ///
    /// // Use the associated macro 'Ne!'.
    /// assert!(g.get_neighbors_by_index(x).eq(Ne!(g, x)));
    /// ```
    ///
    fn get_neighbors_by_index(&self, x: usize) -> Self::NeighborsIndexIter<'_>;

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
    /// let (x, y) = (g.get_vertex_index("A"), g.get_vertex_index("B"));
    ///
    /// // Check edge.
    /// assert!(g.is_neighbor_by_index(x, y));
    /// assert!(Ne!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_neighbor_by_index(&self, x: usize, y: usize) -> bool {
        Ne!(self, x).any(|z| z == y)
    }

    /// Checks whether the graph has a given undirected edge or not.
    fn has_undirected_edge_by_index(&self, x: usize, y: usize) -> bool {
        uE!(self).any(|z| z == (x, y) || z == (y, x))
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
    /// let x = g.get_vertex_index("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.get_degree_by_index(x), 3);
    /// assert_eq!(g.get_degree_by_index(x), Ne!(g, x).count());
    /// ```
    ///
    fn get_degree_by_index(&self, x: usize) -> usize {
        Ne!(self, x).count()
    }

    /// Undirected edge adder.
    fn add_undirected_edge_by_index(&mut self, x: usize, y: usize) -> bool;
}

/// Ancestors iterator.
///
/// Return the vertex iterator representing $An(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! An {
    ($g:expr, $x:expr) => {
        $g.get_ancestors_by_index($x)
    };
}

/// Parents iterator.
///
/// Return the vertex iterator representing $Pa(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Pa {
    ($g:expr, $x:expr) => {
        $g.get_parents_by_index($x)
    };
}

/// Children iterator.
///
/// Return the vertex iterator representing $Ch(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Ch {
    ($g:expr, $x:expr) => {
        $g.get_children_by_index($x)
    };
}

/// Descendants iterator.
///
/// Return the vertex iterator representing $De(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! De {
    ($g:expr, $x:expr) => {
        $g.get_descendants_by_index($x)
    };
}

/// Directed graph trait.
pub trait DirectedGraph: BaseGraph + PartialOrdGraph + SubGraph {
    /// Edges iterator type.
    type DirectedEdgesIndexIter<'a>: Iterator<Item = (usize, usize)>
        + ExactSizeIterator
        + FusedIterator
    where
        Self: 'a;

    /// Ancestors iterator type.
    type AncestorsIndexIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Parents iterator type.
    type ParentsIndexIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Children iterator type.
    type ChildrenIndexIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Descendants iterator type.
    type DescendantsIndexIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Size of the directed subgraph.
    fn size_of_maximal_directed_subgraph(&self) -> usize;

    /// Directed edges iterator
    fn get_directed_edges_index(&self) -> Self::DirectedEdgesIndexIter<'_>;

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
    /// let x = g.get_vertex_index("B");
    ///
    /// // Use the ancestors iterator.
    /// assert!(g.get_ancestors_by_index(x).eq([0, 2]));
    ///
    /// // Use the associated macro 'An!'.
    /// assert!(g.get_ancestors_by_index(x).eq(An!(g, x)));
    /// ```
    ///
    fn get_ancestors_by_index(&self, x: usize) -> Self::AncestorsIndexIter<'_>;

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
    /// let (x, y) = (g.get_vertex_index("A"), g.get_vertex_index("C"));
    ///
    /// // Check edge.
    /// assert!(g.is_ancestor_by_index(x, y));
    /// assert!(An!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_ancestor_by_index(&self, x: usize, y: usize) -> bool {
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
    /// let x = g.get_vertex_index("A");
    ///
    /// // Use the parents iterator.
    /// assert!(g.get_parents_by_index(x).eq([0, 2]));
    ///
    /// // Use the associated macro 'Pa!'.
    /// assert!(g.get_parents_by_index(x).eq(Pa!(g, x)));
    /// ```
    ///
    fn get_parents_by_index(&self, x: usize) -> Self::ParentsIndexIter<'_>;

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
    /// let (x, y) = (g.get_vertex_index("A"), g.get_vertex_index("C"));
    ///
    /// // Check edge.
    /// assert!(g.is_parent_by_index(x, y));
    /// assert!(Pa!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_parent_by_index(&self, x: usize, y: usize) -> bool {
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
    /// let x = g.get_vertex_index("A");
    ///
    /// // Use the children iterator.
    /// assert!(g.get_children_by_index(x).eq([0, 1]));
    ///
    /// // Use the associated macro 'Ch!'.
    /// assert!(g.get_children_by_index(x).eq(Ch!(g, x)));
    /// ```
    ///
    fn get_children_by_index(&self, x: usize) -> Self::ChildrenIndexIter<'_>;

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
    /// let (x, y) = (g.get_vertex_index("C"), g.get_vertex_index("A"));
    ///
    /// // Check edge.
    /// assert!(g.is_child_by_index(x, y));
    /// assert!(Ch!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_child_by_index(&self, x: usize, y: usize) -> bool {
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
    /// let x = g.get_vertex_index("C");
    ///
    /// // Use the descendants iterator.
    /// assert!(g.get_descendants_by_index(x).eq([0, 1]));
    ///
    /// // Use the associated macro 'De!'.
    /// assert!(g.get_descendants_by_index(x).eq(De!(g, x)));
    /// ```
    ///
    fn get_descendants_by_index(&self, x: usize) -> Self::DescendantsIndexIter<'_>;

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
    /// let (x, y) = (g.get_vertex_index("C"), g.get_vertex_index("A"));
    ///
    /// // Check edge.
    /// assert!(g.is_descendant_by_index(x, y));
    /// assert!(De!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_descendant_by_index(&self, x: usize, y: usize) -> bool {
        De!(self, x).any(|z| z == y)
    }

    /// Checks whether the graph has a given directed edge or not.
    fn has_directed_edge_by_index(&self, x: usize, y: usize) -> bool {
        dE!(self).any(|z| z == (x, y))
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
    /// let x = g.get_vertex_index("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.get_in_degree_by_index(x), 2);
    /// assert_eq!(g.get_in_degree_by_index(x), Pa!(g, x).count());
    /// ```
    ///
    fn get_in_degree_by_index(&self, x: usize) -> usize {
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
    /// let x = g.get_vertex_index("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.get_out_degree_by_index(x), 2);
    /// assert_eq!(g.get_out_degree_by_index(x), Ch!(g, x).count());
    /// ```
    ///
    fn get_out_degree_by_index(&self, x: usize) -> usize {
        Ch!(self, x).count()
    }

    /// Directed edge adder.
    fn add_directed_edge_by_index(&mut self, x: usize, y: usize) -> bool;
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
    /// assert!(h.has_edge_by_index(0, 1));
    ///
    /// ```
    ///
    fn to_undirected(&self) -> Self::UndirectedGraph;
}

//TODO: Improve documentation with examples and panics
/// Partially directed graph trait.
pub trait PartiallyDirectedGraph:
    BaseGraph + PartialOrdGraph + SubGraph + DirectedGraph + UndirectedGraph
{
    /// Edges iterator type.
    type EdgesIndexIter<'a>: Iterator<Item = (usize, usize)> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Partially directed graph constructor.
    ///
    /// # Pay attention:
    /// multiple types of edges between two nodes are not allowed.
    ///
    ///
    fn new_pagraph<V, I, J, K>(vertices: I, undirected_edges: J, directed_edges: K) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
        K: IntoIterator<Item = (V, V)>;

    /// Orient (or re-orient) an already present edge
    fn orient_edge(&mut self, x: usize, y: usize) -> bool;
}

/// Meek's orientation rules
pub trait MeekRules: PartiallyDirectedGraph {
    /// Meek's rule 1
    fn meek_1(&mut self) -> bool;
    /// Meek's rule 2
    fn meek_2(&mut self) -> bool;
    /// Meek's rule 3
    fn meek_3(&mut self) -> bool;
    /// Meek's rule 4
    fn meek_4(&mut self) -> bool;
    /// Meek's procedure untile Meek's rule 3
    fn meek_procedure_until_3(self) -> Self;
    /// Meek's procedure untile Meek's rule 4
    fn meek_procedure_until_4(self) -> Self;
}
