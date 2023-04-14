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

/// Neighbors iterator for undirected graphs.
///
/// Return the vertex iterator representing $Ne(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Ne {
    ($g:expr, $x:expr) => {
        $g.get_neighbors_by_index($x)
    };
}
/// Undirected edges iterator for partially directed graphs.
///
/// Return the $E(\mathcal{G}, X)$ subset where edges are undirected as an iterator.///
#[macro_export]
macro_rules! uE {
    ($g:expr) => {
        $g.edges_of_type('u')
    };
}

/// Directed edges iterator for partially directed graphs.
///
/// Return the $E(\mathcal{G}, X)$ subset where edges are directed as an iterator.
///
#[macro_export]
macro_rules! dE {
    ($g:expr) => {
        $g.edges_of_type('d')
    };
}

/// Undirected graph trait.
pub trait UndirectedGraph: BaseGraph + PartialOrdGraph + SubGraph {
    /// Neighbors iterator type.
    type NeighborsIndexIter<'a>: Iterator<Item = usize> + FusedIterator
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

//TODO: Improve documentation
/// Partially directed graph trait.
pub trait PartiallyDirectedGraph:
    BaseGraph + PartialOrdGraph + SubGraph + DirectedGraph + UndirectedGraph
{
    /// Specilized new constructor. Pay attention: multiple types of edges between two nodes is not allowed
    fn new_partial<V, I, J, K>(vertices: I, undirected_edges: J, directed_edges: K) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
        K: IntoIterator<Item = (V, V)>;

    /// Specialized deferencing
    fn deref_of_type(&self, which: char) -> &Self::Data;

    /// Specilized edge iterator. Parameter `which` can be either `u` for undirected or `d` for directed edge type.
    fn edges_of_type(&self, which: char) -> Self::EdgesIndexIter<'_>;

    /// Specialized size of the graph. Parameter `which` can be either `u` for undirected or `d` for directed edge type.
    fn size_of_type(&self, which: char) -> usize;

    /// Type of the edge. It returns `None` if such edge doesn't exist, an `Option<char>` on the contrary. `char` can be `u` for undirected or `d` for directed edge type.
    fn type_of_edge(&self, x: usize, y: usize) -> Option<char>;

    /// Specilized edge adder. Parameter `which` can be either `u` for undirected or `d` for directed edge type.
    fn add_edge_of_type(&mut self, x: usize, y: usize, which: char) -> bool;

    /// Orient (or re-orient) an already present edge
    fn orient_edge(&mut self, x: usize, y: usize) -> bool;
}
