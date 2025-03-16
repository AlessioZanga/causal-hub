use ndarray::Array2;

/// A struct representing an undirected graph using an adjacency matrix.
///
pub struct UndirectedGraph {
    adjacency_matrix: Array2<bool>,
}

impl UndirectedGraph {
    /// Creates a new undirected graph with the given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of nodes in the graph.
    ///
    /// # Returns
    ///
    /// A new `UndirectedGraph` instance.
    ///
    pub fn new(size: usize) -> Self {
        Self {
            adjacency_matrix: Array2::from_elem((size, size), false),
        }
    }

    /// Checks if there is an edge between nodes `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first node.
    /// * `y` - The second node.
    ///
    /// # Returns
    ///
    /// `true` if there is an edge between `x` and `y`, `false` otherwise.
    ///
    pub fn has_edge(&self, x: usize, y: usize) -> bool {
        self.adjacency_matrix[[x, y]]
    }

    /// Adds an edge between nodes `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first node.
    /// * `y` - The second node.
    ///
    pub fn add_edge(&mut self, x: usize, y: usize) {
        self.adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[y, x]] = true;
    }

    /// Removes the edge between nodes `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first node.
    /// * `y` - The second node.
    ///
    pub fn del_edge(&mut self, x: usize, y: usize) {
        self.adjacency_matrix[[x, y]] = false;
        self.adjacency_matrix[[y, x]] = false;
    }
}
