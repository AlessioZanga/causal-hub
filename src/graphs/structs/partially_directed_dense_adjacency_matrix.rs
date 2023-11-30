use std::{cmp::Ordering, fmt::Display, iter::FusedIterator, ops::Index};

use iter_set::{classify, intersection, Classify, Inclusion};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{DGraph, UGraph};
use crate::{
    graphs::{DirectedGraph, Graph, PartiallyDirected, PartiallyDirectedGraph, UndirectedGraph},
    E, L, V,
};

/// Define the `PartiallyDirectedDenseAdjacencyMatrix` struct.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PartiallyDirectedDenseAdjacencyMatrix {
    directed: DGraph,
    undirected: UGraph,
}

/// Alias for the `PartiallyDirectedDenseAdjacencyMatrix` struct.
pub type PGraph = PartiallyDirectedDenseAdjacencyMatrix;

// Implement the `Display` trait for the `PGraph` struct.
impl Display for PGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write graph type.
        write!(f, "PartiallyDirectedGraph {{ ")?;

        // Write vertex set.
        write!(
            f,
            "V = {{{}}}, ",
            V!(self).map(|x| format!("\"{}\"", &self[x])).join(", ")
        )?;

        // Write edge set.
        write!(
            f,
            "E = {{{}}}",
            E!(self)
                .map(|(x, y)| format!("(\"{}\", \"{}\")", &&self[x], &self[y]))
                .join(", ")
        )?;

        // Write ending character.
        write!(f, " }}")
    }
}

// Implement the `Index` trait for the `PGraph` struct.
impl Index<usize> for PGraph {
    type Output = str;

    #[inline]
    fn index(&self, x: usize) -> &Self::Output {
        // Get the vertex label.
        self.vertex_to_label(x)
    }
}

// Implement the `PartialOrd` trait for the `PGraph` struct.
impl PartialOrd for PGraph {
    /// Compare two graphs.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// Return `None` if the graphs are not comparable.
    ///
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare the undirected graphs.
        let undirected = self.undirected.partial_cmp(&other.undirected);
        // If the undirected graphs are not comparable, return `None`.
        if undirected.is_none() {
            return None;
        }

        // Compare the directed graphs.
        let directed = self.directed.partial_cmp(&other.directed);
        // If the directed graphs are not comparable, return `None`.
        if directed.is_none() {
            return None;
        }

        // Unwrap the undirected and directed comparison.
        let (undirected, directed) = (undirected.unwrap(), directed.unwrap());

        // If the undirected are equal, return the directed.
        if undirected.is_eq() {
            return Some(directed);
        }
        // If the edges are equal, return the undirected.
        if directed.is_eq() {
            return Some(undirected);
        }
        // If the undirected and the directed are the same, return arbitrarily.
        if undirected.eq(&directed) {
            return Some(undirected);
        }

        // Otherwise, return `None`.
        None
    }
}

/// Define the `EdgesIterator` iterator for the `PGraph` struct.
#[allow(clippy::type_complexity)]
pub struct EdgesIterator<'a> {
    // The edges indices iterator.
    iter: std::iter::Map<
        Classify<<DGraph as Graph>::EdgesIter<'a>, <UGraph as Graph>::EdgesIter<'a>>,
        fn(Inclusion<(usize, usize)>) -> (usize, usize),
    >,
    // The size of the iterator.
    size: usize,
}

// Implement the `EdgesIterator` iterator for the `PGraph` struct.
impl<'a> EdgesIterator<'a> {
    /// Create a new `EdgesIterator` iterator.
    #[inline]
    fn new(graph: &'a PGraph) -> Self {
        // Create the new `EdgesIterator` iterator.
        Self {
            iter: classify(E!(graph.directed), E!(graph.undirected)).map(Inclusion::union),
            size: graph.size(),
        }
    }
}

// Implement the `Iterator` trait for the `EdgesIterator` iterator.
impl<'a> Iterator for EdgesIterator<'a> {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next edge indices.
        let next = self.iter.next();

        // Debug assert the iterator size is zero if and only if the next edge indices is `None`.
        debug_assert_eq!(
            self.size == 0,
            next.is_none(),
            "The iterator size is not zero."
        );
        // Debug assert the iterator size is non zero if and only if the next edge indices is `Some(_)`.
        debug_assert_eq!(self.size != 0, next.is_some(), "The iterator size is zero.");

        // Decrement the iterator size.
        self.size = self.size.saturating_sub(1);

        next
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Get the iterator size hint.
        (self.size, Some(self.size))
    }

    #[inline]
    fn count(self) -> usize {
        // Get the iterator count.
        self.size
    }
}

// Implement the `ExactSizeIterator` trait for the `EdgesIterator` iterator.
impl<'a> ExactSizeIterator for EdgesIterator<'a> {}

// Implement the `FusedIterator` trait for the `EdgesIterator` iterator.
impl<'a> FusedIterator for EdgesIterator<'a> {}

/// Define the `AdjacentsIterator` iterator for the `PGraph` struct.
#[allow(clippy::type_complexity)]
pub struct AdjacentsIterator<'a> {
    // The edges indices iterator.
    iter: std::iter::Map<
        Classify<<DGraph as Graph>::AdjacentsIter<'a>, <UGraph as Graph>::AdjacentsIter<'a>>,
        fn(Inclusion<usize>) -> usize,
    >,
}

// Implement the `AdjacentsIterator` iterator for the `PGraph` struct.
impl<'a> AdjacentsIterator<'a> {
    /// Create a new `AdjacentsIterator` iterator.
    #[inline]
    fn new(graph: &'a PGraph, x: usize) -> Self {
        // Create the new `AdjacentsIterator` iterator.
        Self {
            iter: classify(
                graph.directed.adjacents_iter(x),
                graph.undirected.adjacents_iter(x),
            )
            .map(Inclusion::union),
        }
    }
}

// Implement the `Iterator` trait for the `AdjacentsIterator` iterator.
impl<'a> Iterator for AdjacentsIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next adjacent indices.
        self.iter.next()
    }
}

// Implement the `FusedIterator` trait for the `AdjacentsIterator` iterator.
impl<'a> FusedIterator for AdjacentsIterator<'a> {}

/// Implement the `Graph` trait for the `PGraph` struct.
impl Graph for PGraph {
    // Direction associated type.
    type Direction = PartiallyDirected;
    // Vertex labels iterator associated type.
    type LabelsIter<'a> =
        std::iter::Map<indexmap::set::Iter<'a, String>, fn(&'a String) -> &'a str>;
    // Vertex indices iterator associated type.
    type VerticesIter<'a> = std::ops::Range<usize>;
    // Edge indices iterator associated type.
    type EdgesIter<'a> = EdgesIterator<'a>;
    // Adjacents indices iterator associated type.
    type AdjacentsIter<'a> = AdjacentsIterator<'a>;

    /// Create a new graph.
    ///
    /// # Complexity
    /// - Time: `O(|V| + |E|)`,
    /// - Space: `O(|V| + |E|)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    fn new<V, I, J>(vertices: I, edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
    {
        // Initialize the undirected graph.
        let undirected = UGraph::new(vertices, edges);
        // Initialize the directed graph.
        let directed = DGraph::empty(L!(undirected));

        // Return the new graph.
        Self {
            directed,
            undirected,
        }
    }

    // Create a new null graph.
    fn null() -> Self {
        // Initialize the undirected graph.
        let undirected = UGraph::null();
        // Initialize the directed graph.
        let directed = DGraph::null();

        // Return the new graph.
        Self {
            directed,
            undirected,
        }
    }

    /// Create a new empty graph.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    fn empty<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Initialize the undirected graph.
        let undirected = UGraph::empty(vertices);
        // Initialize the directed graph.
        let directed = DGraph::empty(L!(undirected));

        // Return the new graph.
        Self {
            directed,
            undirected,
        }
    }

    // Create a new complete graph.
    fn complete<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
    {
        // Initialize the undirected graph.
        let undirected = UGraph::complete(vertices);
        // Initialize the directed graph.
        let directed = DGraph::empty(L!(undirected));

        // Return the new graph.
        Self {
            directed,
            undirected,
        }
    }

    // Get the vertices labels iterator.
    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        // Delegate to the `labels` method of the undirected graph.
        L!(self.undirected)
    }

    // Get the vertex label.
    #[inline]
    fn vertex_to_label(&self, x: usize) -> &str {
        // Delegate to the `vertex_to_label` method of the undirected graph.
        &self.undirected[x]
    }

    // Get the vertex index.
    #[inline]
    fn label_to_vertex(&self, x: &str) -> usize {
        // Delegate to the `label_to_vertex` method of the undirected graph.
        self.undirected.label_to_vertex(x)
    }

    // Get the graph order.
    #[inline]
    fn order(&self) -> usize {
        // Delegate to the `order` method of the undirected graph.
        self.undirected.order()
    }

    // Get the vertices indices iterator.
    #[inline]
    fn vertices_iter(&self) -> Self::VerticesIter<'_> {
        // Delegate to the `vertices` method of the undirected graph.
        V!(self.undirected)
    }

    // Check if the vertex exists.
    #[inline]
    fn has_vertex(&self, x: usize) -> bool {
        // Delegate to the `has_vertex` method of the undirected graph.
        self.undirected.has_vertex(x)
    }

    /// Add a vertex.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    fn add_vertex<V>(&mut self, x: V) -> (usize, bool)
    where
        V: Into<String>,
    {
        // Convert the vertex label to a string.
        let v = x.into();

        // Add the vertex to the directed graph.
        let (x, added_x) = self.directed.add_vertex(v.clone());
        // Add the vertex to the undirected graph.
        let (y, added_y) = self.undirected.add_vertex(v);

        // Debug assert that the two vertices are the same.
        debug_assert_eq!(x, y, "The two vertices are not the same.");
        // Debug assert that the two booleans are the same.
        debug_assert_eq!(added_x, added_y, "The two booleans are not the same.");

        // Return the vertex index and if it was added.
        (x, added_x)
    }

    /// Delete a vertex.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|^2)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    fn del_vertex(&mut self, x: usize) -> bool {
        // Delete the vertex from the directed graph.
        let deleted_x = self.directed.del_vertex(x);
        // Delete the vertex from the undirected graph.
        let deleted_y = self.undirected.del_vertex(x);

        // Debug assert that the two booleans are the same.
        debug_assert_eq!(deleted_x, deleted_y, "The two booleans are not the same.");

        // Return if the vertex was deleted.
        deleted_x
    }

    // Get the graph size.
    #[inline]
    fn size(&self) -> usize {
        // Sum the directed and undirected graph sizes.
        self.directed.size() + self.undirected.size()
    }

    /// Get the edges indices iterator.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn edges_iter(&self) -> Self::EdgesIter<'_> {
        // Compute the union of the directed and undirected graph edges.
        Self::EdgesIter::new(self)
    }

    /// Check if the edge exists.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn has_edge(&self, x: usize, y: usize) -> bool {
        // Check if the edge exists in the graph.
        self.directed.has_edge(x, y) || self.undirected.has_edge(x, y)
    }

    /// Add an undirected edge, only if it does not exist neither directed nor undirected.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn add_edge(&mut self, x: usize, y: usize) -> bool {
        // Check if the edge exists in the graph.
        if !self.has_edge(x, y) {
            // Add the edge to the undirected graph.
            let added = self.undirected.add_edge(x, y);

            // Debug assert that the edge sets are disjoint.
            debug_assert!(
                intersection(self.directed_edges_iter(), self.undirected_edges_iter())
                    .next()
                    .is_none(),
                "The edge sets are not disjoint."
            );

            return added;
        }

        false
    }

    /// Delete an edge.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn del_edge(&mut self, x: usize, y: usize) -> bool {
        // Delete the edge from the graph.
        self.directed.del_edge(x, y) || self.undirected.del_edge(x, y)
    }

    /// Get the vertex degree.
    ///
    /// # Complexity
    /// - Time: `O(|V|)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn degree(&self, x: usize) -> usize {
        // Compute the sum of the directed and undirected graph degrees.
        self.directed.degree(x) + self.undirected.degree(x)
    }

    /// Get the vertices degrees.
    ///
    /// # Complexity
    /// - Time: `O(|V|^2)`,
    /// - Space: `O(|V|)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn degrees(&self) -> Vec<usize> {
        // Compute the sum of the directed and undirected graph degrees.
        let mut degrees = self.directed.degrees();
        // Compute the sum of the directed and undirected graph degrees.
        degrees
            .iter_mut()
            .zip(self.undirected.degrees())
            .for_each(|(x, y)| *x += y);

        // Return the vertices degrees.
        degrees
    }

    /// Get the vertex adjacents indices iterator.
    ///
    /// # Complexity
    /// - Time: `O(|V|)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn adjacents_iter(&self, x: usize) -> Self::AdjacentsIter<'_> {
        // Compute the union of the directed and undirected graph adjacents.
        Self::AdjacentsIter::new(self, x)
    }

    /// Check if two vertices are adjacent.
    ///
    /// # Complexity
    /// - Time: `O(1)`,
    /// - Space: `O(1)`.
    ///
    /// # Notes
    /// See the `Graph` trait for more details.
    ///
    #[inline]
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        // Check if the vertices are adjacent in the graph.
        self.directed.is_adjacent(x, y) || self.undirected.is_adjacent(x, y)
    }

    // FIXME:
    fn subgraph<I, J>(&self, vertices: I, edges: J) -> Self
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = (usize, usize)>,
    {
        todo!() // FIXME:
    }

    // FIXME:
    fn subgraph_by_vertices<I>(&self, vertices: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        todo!() // FIXME:
    }

    // FIXME:
    fn subgraph_by_edges<J>(&self, edges: J) -> Self
    where
        J: IntoIterator<Item = (usize, usize)>,
    {
        todo!() // FIXME:
    }

    // FIXME:
    #[inline]
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    // FIXME:
    #[inline]
    fn is_supergraph(&self, other: &Self) -> bool {
        self >= other
    }
}

/// Implement the `UndirectedGraph` trait for the `PGraph` struct.
impl UndirectedGraph for PGraph {
    // Undirected graph edges iterator type.
    type UndirectedEdgesIter<'a> = <UGraph as UndirectedGraph>::UndirectedEdgesIter<'a>;
    // Neighbors iterator type.
    type NeighborsIter<'a> = <UGraph as UndirectedGraph>::NeighborsIter<'a>;

    // Get the undirected graph size.
    #[inline]
    fn undirected_size(&self) -> usize {
        // Delegate to the `size` method.
        self.undirected.undirected_size()
    }

    // Get the undirected graph edges indices iterator.
    #[inline]
    fn undirected_edges_iter(&self) -> Self::UndirectedEdgesIter<'_> {
        // Delegate to the `edges` method.
        self.undirected.undirected_edges_iter()
    }

    // Check if the undirected edge exists.
    #[inline]
    fn has_undirected_edge(&self, x: usize, y: usize) -> bool {
        // Delegate to the `has_edge` method.
        self.undirected.has_undirected_edge(x, y)
    }

    /// Add an undirected edge, deleting existing directed edges, if any.
    #[inline]
    fn add_undirected_edge(&mut self, x: usize, y: usize) -> bool {
        // Delete existing edges to avoid mixed edges.
        self.del_directed_edge(x, y);
        self.del_directed_edge(y, x);
        // Delegate to the `add_edge` method.
        let added = self.undirected.add_undirected_edge(x, y);

        // Debug assert that the edge sets are disjoint.
        debug_assert!(
            intersection(self.directed_edges_iter(), self.undirected_edges_iter())
                .next()
                .is_none(),
            "The edge sets are not disjoint."
        );

        added
    }

    // Delete an undirected edge.
    #[inline]
    fn del_undirected_edge(&mut self, x: usize, y: usize) -> bool {
        // Delegate to the `del_edge` method.
        self.undirected.del_undirected_edge(x, y)
    }

    // Get the vertex neighbors indices iterator.
    #[inline]
    fn neighbors_iter(&self, x: usize) -> Self::NeighborsIter<'_> {
        // Delegate to the `adjacents` method.
        self.undirected.neighbors_iter(x)
    }

    // Check if two vertices are neighbors.
    #[inline]
    fn is_neighbor(&self, x: usize, y: usize) -> bool {
        // Delegate to the `is_adjacent` method.
        self.undirected.is_neighbor(x, y)
    }
}

/// Implement the `DirectedGraph` trait for the `PGraph` struct.
impl DirectedGraph for PGraph {
    // Directed graph edges iterator type.
    type DirectedEdgesIter<'a> = <DGraph as DirectedGraph>::DirectedEdgesIter<'a>;
    // Ancestors indices iterator associated type.
    type AncestorsIter<'a> = <DGraph as DirectedGraph>::AncestorsIter<'a>;
    // Parents iterator type.
    type ParentsIter<'a> = <DGraph as DirectedGraph>::ParentsIter<'a>;
    // Children iterator type.
    type ChildrenIter<'a> = <DGraph as DirectedGraph>::ChildrenIter<'a>;
    // Descendants indices iterator associated type.
    type DescendantsIter<'a> = <DGraph as DirectedGraph>::DescendantsIter<'a>;
    // FIXME:
    type UndirectedGraph = UGraph;

    // Get the directed graph size.
    #[inline]
    fn directed_size(&self) -> usize {
        // Delegate to the `size` method.
        self.directed.directed_size()
    }

    // Get the directed graph edges indices iterator.
    #[inline]
    fn directed_edges_iter(&self) -> Self::DirectedEdgesIter<'_> {
        // Delegate to the `edges` method.
        self.directed.directed_edges_iter()
    }

    // Check if the directed edge exists.
    #[inline]
    fn has_directed_edge(&self, x: usize, y: usize) -> bool {
        // Delegate to the `has_edge` method.
        self.directed.has_directed_edge(x, y)
    }

    /// Add a directed edge, deleting an existing undirected edge, if any.
    #[inline]
    fn add_directed_edge(&mut self, x: usize, y: usize) -> bool {
        // Delete existing edges to avoid mixed edges.
        self.del_undirected_edge(x, y);
        // Delegate to the `add_edge` method.
        let added = self.directed.add_directed_edge(x, y);

        // Debug assert that the edge sets are disjoint.
        debug_assert!(
            intersection(self.directed_edges_iter(), self.undirected_edges_iter())
                .next()
                .is_none(),
            "The edge sets are not disjoint."
        );

        added
    }

    // Delete a directed edge.
    #[inline]
    fn del_directed_edge(&mut self, x: usize, y: usize) -> bool {
        // Delegate to the `del_edge` method.
        self.directed.del_directed_edge(x, y)
    }

    // Get the vertex in-degree.
    #[inline]
    fn in_degree(&self, x: usize) -> usize {
        // Delegate to the `in_degree` method.
        self.directed.in_degree(x)
    }

    // Get the vertices in-degrees.
    #[inline]
    fn in_degrees(&self) -> Vec<usize> {
        // Delegate to the `in_degrees` method.
        self.directed.in_degrees()
    }

    // Get the vertex out-degree.
    #[inline]
    fn out_degree(&self, x: usize) -> usize {
        // Delegate to the `out_degree` method.
        self.directed.out_degree(x)
    }

    // Get the vertices out-degrees.
    #[inline]
    fn out_degrees(&self) -> Vec<usize> {
        // Delegate to the `out_degrees` method.
        self.directed.out_degrees()
    }

    // Get the vertex ancestors indices iterator.
    #[inline]
    fn ancestors_iter(&self, x: usize) -> Self::AncestorsIter<'_> {
        // Delegate to the `ancestors` method.
        self.directed.ancestors_iter(x)
    }

    // Check if two vertices are ancestors.
    #[inline]
    fn is_ancestor(&self, x: usize, y: usize) -> bool {
        // Delegate to the `is_ancestor` method.
        self.directed.is_ancestor(x, y)
    }

    // Get the vertex parents indices iterator.
    #[inline]
    fn parents_iter(&self, x: usize) -> Self::ParentsIter<'_> {
        // Delegate to the `parents` method.
        self.directed.parents_iter(x)
    }

    // Check if two vertices are parents.
    #[inline]
    fn is_parent(&self, x: usize, y: usize) -> bool {
        // Delegate to the `is_parent` method.
        self.directed.is_parent(x, y)
    }

    // Get the vertex children indices iterator.
    #[inline]
    fn children_iter(&self, x: usize) -> Self::ChildrenIter<'_> {
        // Delegate to the `children` method.
        self.directed.children_iter(x)
    }

    // Check if two vertices are children.
    #[inline]
    fn is_child(&self, x: usize, y: usize) -> bool {
        // Delegate to the `is_child` method.
        self.directed.is_child(x, y)
    }

    // Get the vertex descendants indices iterator.
    #[inline]
    fn descendants_iter(&self, x: usize) -> Self::DescendantsIter<'_> {
        // Delegate to the `descendants` method.
        self.directed.descendants_iter(x)
    }

    // Check if two vertices are descendants.
    #[inline]
    fn is_descendant(&self, x: usize, y: usize) -> bool {
        // Delegate to the `is_descendant` method.
        self.directed.is_descendant(x, y)
    }
}

/// Implement the `PartiallyDirectedGraph` trait for the `PGraph` struct.
impl PartiallyDirectedGraph for PGraph {
    // Set an already existing edge as directed.
    fn set_directed_edge(&mut self, x: usize, y: usize) -> bool {
        // Remove the edge from the undirected graph.
        if self.del_undirected_edge(x, y) {
            // Add the edge to the directed graph.
            let added = self.add_directed_edge(x, y);

            // Debug assert that the edge sets are disjoint.
            debug_assert!(
                intersection(self.directed_edges_iter(), self.undirected_edges_iter())
                    .next()
                    .is_none(),
                "The edge sets are not disjoint."
            );

            return added;
        }

        false
    }

    // Set an already existing edge as undirected.
    fn set_undirected_edge(&mut self, x: usize, y: usize) -> bool {
        // Remove the edge from the directed graph.
        if self.del_directed_edge(x, y) {
            // Remove the reversed edge from the directed graph to avoid mixed edges.
            self.del_directed_edge(y, x);
            // Add the edge to the undirected graph.
            let added = self.add_undirected_edge(x, y);

            // Debug assert that the edge sets are disjoint.
            debug_assert!(
                intersection(self.directed_edges_iter(), self.undirected_edges_iter())
                    .next()
                    .is_none(),
                "The edge sets are not disjoint."
            );

            return added;
        }

        false
    }
}
