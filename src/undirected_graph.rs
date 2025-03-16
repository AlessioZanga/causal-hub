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
        // Check if the vertices are within bounds.
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
    pub fn add_edge(&mut self, x: usize, y: usize) -> bool {
        // Check if the vertices are within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        // Check if the edge already exists.
        if self.adjacency_matrix[[x, y]] {
            return false;
        }

        // Add the edge.
        self.adjacency_matrix[[x, y]] = true;
        self.adjacency_matrix[[y, x]] = true;

        true
    }

    /// Deletes the edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    pub fn del_edge(&mut self, x: usize, y: usize) -> bool {
        // Check if the vertices are within bounds.
        assert!(x < self.labels.len(), "Vertex {} index out of bounds", x);
        assert!(y < self.labels.len(), "Vertex {} index out of bounds", y);

        // Check if the edge exists.
        if !self.adjacency_matrix[[x, y]] {
            return false;
        }

        // Delete the edge.
        self.adjacency_matrix[[x, y]] = false;
        self.adjacency_matrix[[y, x]] = false;

        true
    }

    /// Returns the neighbors of a vertex.
    ///
    /// # Arguments
    ///
    /// * `vertex` - The vertex for which to find the neighbors.
    ///
    /// # Returns
    ///
    /// A vector of indices representing the neighbors of the vertex.
    ///
    pub fn neighbors(&self, vertex: usize) -> Vec<usize> {
        // Check if the vertex is within bounds.
        assert!(vertex < self.labels.len(), "Vertex {} index out of bounds", vertex);

        // Use functional code to find the neighbors.
        // Iterate over all vertices and filter the ones that are neighbors.
        (0..self.labels.len())
            .filter(|&i| self.adjacency_matrix[[vertex, i]])
            .collect()
    }
}
