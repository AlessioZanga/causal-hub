use crate::{graphs::Graph, E};

/// Define the `Ne` neighbor macro.
#[macro_export]
macro_rules! Ne {
    ($g:expr, $x:expr) => {
        $g.neighbors_iter($x)
    };
}

/// Define the `Undirected` direction type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Undirected {}

/// Define the `UndirectedGraph` trait.
///
/// If the `Direction` associated type is `Undirected`, then
/// the methods of this trait are delegated to the `Graph` trait.
///
pub trait UndirectedGraph: Graph {
    /// Undirected edges indices iterator associated type.
    type UndirectedEdgesIter<'a>: Iterator<Item = (usize, usize)>
    where
        Self: 'a;
    /// Neighbors indices iterator associated type.
    type NeighborsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Get the undirected size.
    ///
    /// The undirected size is the number of undirected edges.
    ///
    /// # Returns
    /// The undirected size.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn undirected_size(&self) -> usize;

    /// Get the undirected edges indices iterator.
    ///
    /// # Returns
    /// The undirected edges indices iterator.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The undirected edges indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    fn undirected_edges_iter(&self) -> Self::UndirectedEdgesIter<'_>;

    /// Check if the undirected edge exists.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the undirected edge exists, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn has_undirected_edge(&self, x: usize, y: usize) -> bool;

    /// Add an undirected edge.
    ///
    /// The undirected edge is added only if it does not exist.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the undirected edge was added, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn add_undirected_edge(&mut self, x: usize, y: usize) -> bool;

    /// Delete an undirected edge.
    ///
    /// The undirected edge is deleted only if it exists.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// `true` if the undirected edge was deleted, otherwise `false`.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn del_undirected_edge(&mut self, x: usize, y: usize) -> bool;

    /// Get the vertex neighbors indices iterator.
    ///
    /// The vertex neighbors indices are the vertices with an undirected edge to the vertex.
    ///
    /// # Arguments
    /// * `x` - The vertex index.
    ///
    /// # Returns
    /// The vertex neighbors indices iterator.
    ///
    /// # Panics
    /// If the vertex index is out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    /// # Notes
    /// The vertex neighbors indices are:
    /// - Unique,
    /// - Sorted in ascending order.
    ///
    fn neighbors_iter(&self, x: usize) -> Self::NeighborsIter<'_>;

    /// Check if two vertices are neighbors.
    ///
    /// # Arguments
    /// * `x` - The first vertex index.
    /// * `y` - The second vertex index.
    ///
    /// # Returns
    /// If the two vertices are neighbors.
    ///
    /// # Panics
    /// If the vertex indices are out of bounds.
    ///
    /// # Complexity
    /// Check the implementation.
    ///
    fn is_neighbor(&self, x: usize, y: usize) -> bool;
}

/// Define blanket implementations for the `Graph` trait with the `Undirected` direction type.
impl<G> UndirectedGraph for G
where
    G: Graph<Direction = Undirected>,
{
    // Undirected graph edges iterator type.
    type UndirectedEdgesIter<'a> = <G as Graph>::EdgesIter<'a> where G: 'a;
    // Neighbors iterator type.
    type NeighborsIter<'a> = <G as Graph>::AdjacentsIter<'a> where G: 'a;

    // Get the undirected graph size.
    #[inline]
    fn undirected_size(&self) -> usize {
        // Delegate to the `size` method.
        self.size()
    }

    // Get the undirected graph edges indices iterator.
    #[inline]
    fn undirected_edges_iter(&self) -> Self::UndirectedEdgesIter<'_> {
        // Delegate to the `edges` method.
        E!(self)
    }

    // Check if the undirected edge exists.
    #[inline]
    fn has_undirected_edge(&self, x: usize, y: usize) -> bool {
        // Delegate to the `has_edge` method.
        self.has_edge(x, y)
    }

    // Add an undirected edge.
    #[inline]
    fn add_undirected_edge(&mut self, x: usize, y: usize) -> bool {
        // Delegate to the `add_edge` method.
        self.add_edge(x, y)
    }

    // Delete an undirected edge.
    #[inline]
    fn del_undirected_edge(&mut self, x: usize, y: usize) -> bool {
        // Delegate to the `del_edge` method.
        self.del_edge(x, y)
    }

    // Get the vertex neighbors indices iterator.
    #[inline]
    fn neighbors_iter(&self, x: usize) -> Self::NeighborsIter<'_> {
        // Delegate to the `adjacents` method.
        self.adjacents_iter(x)
    }

    // Check if two vertices are neighbors.
    #[inline]
    fn is_neighbor(&self, x: usize, y: usize) -> bool {
        // Delegate to the `is_adjacent` method.
        self.is_adjacent(x, y)
    }
}
