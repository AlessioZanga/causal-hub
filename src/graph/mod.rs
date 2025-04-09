mod directed;
pub use directed::*;

mod topological_sort;
pub use topological_sort::*;

mod undirected;
pub use undirected::*;

/// A trait for graphs.
pub trait Graph {
    /// The type of the labels.
    type Labels;
    /// The type of the vertices.
    type Vertices: IntoIterator<Item = usize>;

    /// Creates an empty directed graph with the given labels.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the vertices in the graph.
    ///
    /// # Panics
    ///
    /// * If the labels are not unique.
    ///
    /// # Returns
    ///
    /// A new graph instance.
    ///
    fn empty(labels: Vec<&str>) -> Self;

    /// Returns the labels of the vertices in the graph.
    ///
    /// # Returns
    ///
    /// A reference to the vector of labels.
    ///
    fn labels(&self) -> &Self::Labels;

    /// Returns the iterator of vertices in the graph.
    ///
    /// # Returns
    ///
    /// A range representing the vertices in the graph.
    ///
    fn vertices(&self) -> Self::Vertices;

    /// Checks if there is an edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Panics
    ///
    /// * If any of the vertices are out of bounds.
    ///
    /// # Returns
    ///
    /// `true` if there is an edge between `x` and `y`, `false` otherwise.
    ///
    fn has_edge(&self, x: usize, y: usize) -> bool;

    /// Adds an edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Panics
    ///
    /// * If any of the vertices are out of bounds.
    ///
    /// # Returns
    ///
    /// `true` if the edge was added, `false` if it already existed.
    ///
    fn add_edge(&mut self, x: usize, y: usize) -> bool;

    /// Deletes the edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Panics
    ///
    /// * If any of the vertices are out of bounds.
    ///
    /// # Returns
    ///
    /// `true` if the edge was deleted, `false` if it did not exist.
    ///
    fn del_edge(&mut self, x: usize, y: usize) -> bool;
}
