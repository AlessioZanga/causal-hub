mod directed;
pub use directed::*;

mod topological_order;
pub use topological_order::*;

mod undirected;
pub use undirected::*;

/// A trait for graphs.
pub trait Graph {
    /// The type of the labels.
    type Labels;
    /// The type of the vertices.
    type Vertices: IntoIterator<Item = usize>;
    /// The type of the edges.
    type Edges: IntoIterator<Item = (usize, usize)>;

    /// Creates an empty directed graph with the given labels.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the vertices in the graph.
    ///
    /// # Notes
    ///
    /// * Labels will be sorted in alphabetical order.
    ///
    /// # Panics
    ///
    /// * If the labels are not unique.
    ///
    /// # Returns
    ///
    /// A new graph instance.
    ///
    fn empty<I, V>(labels: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: AsRef<str>;

    /// Returns the labels of the vertices in the graph.
    ///
    /// # Returns
    ///
    /// A reference to the vector of labels.
    ///
    fn labels(&self) -> &Self::Labels;

    /// Return the vertex index for a given label.
    ///
    /// # Arguments
    ///
    /// * `x` - The label of the vertex.
    ///
    /// # Panics
    ///
    /// * If the label is not in the graph.
    ///
    /// # Returns
    ///
    /// The index of the vertex.
    ///
    fn label_to_index<V>(&self, x: &V) -> usize
    where
        V: AsRef<str>;

    /// Return the label for a given vertex index.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the vertex.
    ///
    /// # Panics
    ///
    /// * If the index is out of bounds.
    ///
    /// # Returns
    ///
    /// The label of the vertex.
    ///
    fn index_to_label(&self, x: usize) -> &str;

    /// Returns the iterator of vertices in the graph.
    ///
    /// # Returns
    ///
    /// A range representing the vertices in the graph.
    ///
    fn vertices(&self) -> Self::Vertices;

    /// Returns the iterator of edges in the graph.
    ///
    /// # Returns
    ///
    /// A vector of tuples representing the edges in the graph.
    ///
    fn edges(&self) -> Self::Edges;

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
