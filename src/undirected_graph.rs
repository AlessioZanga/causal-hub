use ndarray::Array2;

/// A struct representing an undirected graph using an adjacency matrix.
///
pub struct UndirectedGraph {
    labels: Vec<String>,
    adjacency_matrix: Array2<bool>,
}

impl UndirectedGraph {
    /// Creates a new undirected graph with the given size.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the vertices in the graph.
    ///
    /// # Returns
    ///
    /// A new `UndirectedGraph` instance.
    ///
    pub fn new(labels: &[&str]) -> Self {
        // Convert the array of string slices to a vector of strings.
        let labels: Vec<_> = labels.iter().map(|s| s.to_string()).collect();
        // Get the size of the graph from the number of labels.
        let size = labels.len();
        // Initialize the adjacency matrix with `false` values.
        let adjacency_matrix = Array2::from_elem((size, size), false);

        Self {
            labels,
            adjacency_matrix,
        }
    }

    /// Returns the labels of the vertices in the graph.
    ///
    /// # Returns
    ///
    /// A reference to the vector of labels.
    ///
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Checks if there is an edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Returns
    ///
    /// `true` if there is an edge between `x` and `y`, `false` otherwise.
    ///
    pub fn has_edge(&self, x: usize, y: usize) -> bool {
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        self.adjacency_matrix[[x, y]]
    }

    /// Adds an edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    pub fn add_edge(&mut self, x: usize, y: usize) {
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        self.adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[y, x]] = true;
    }

    /// Removes the edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    pub fn del_edge(&mut self, x: usize, y: usize) {
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        self.adjacency_matrix[[x, y]] = false;
        self.adjacency_matrix[[y, x]] = false;
    }
}
