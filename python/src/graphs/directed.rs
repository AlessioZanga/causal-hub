use causal_hub::graphs::{DiGraph, Graph};
use pyo3::{prelude::*, types::PyType};

/// A struct representing a directed graph using an adjacency matrix.
#[pyclass(name = "DiGraph")]
pub struct PyDiGraph {
    inner: DiGraph,
}

impl From<DiGraph> for PyDiGraph {
    fn from(inner: DiGraph) -> Self {
        Self { inner }
    }
}

impl From<PyDiGraph> for DiGraph {
    fn from(outer: PyDiGraph) -> Self {
        outer.inner
    }
}

#[pymethods]
impl PyDiGraph {
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
    /// # Returns
    ///
    /// A new graph instance.
    ///
    #[classmethod]
    fn empty(_cls: &Bound<'_, PyType>, labels: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let labels: Vec<_> = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Create a new DiGraph with the labels.
        Ok(Self {
            inner: DiGraph::empty(labels),
        })
    }

    /// Returns the vertices of the graph.
    ///
    /// # Returns
    ///
    /// A list of vertices.
    ///
    fn vertices(&self) -> PyResult<Vec<&str>> {
        // Get the labels of the vertices in the graph.
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the edges of the graph.
    ///
    /// # Returns
    ///
    /// A list of edges.
    ///
    fn edges(&self) -> PyResult<Vec<(&str, &str)>> {
        // Get the edges of the graph.
        Ok(self
            .inner
            .edges()
            .into_iter()
            .map(|(x, y)| {
                // Get the labels of the vertices.
                let x = self.inner.index_to_label(x);
                let y = self.inner.index_to_label(y);
                // Return the labels as a tuple.
                (x, y)
            })
            .collect())
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
    fn has_edge(&self, x: &str, y: &str) -> PyResult<bool> {
        // Get the indices of the vertices.
        let x = self.inner.label_to_index(&x);
        let y = self.inner.label_to_index(&y);

        // Check if the edge exists in the graph.
        Ok(self.inner.has_edge(x, y))
    }

    /// Adds an edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Returns
    ///
    /// `true` if the edge was added, `false` if it already existed.
    ///
    fn add_edge(&mut self, x: &str, y: &str) -> PyResult<bool> {
        // Get the indices of the vertices.
        let x = self.inner.label_to_index(&x);
        let y = self.inner.label_to_index(&y);

        // Add the edge to the graph.
        Ok(self.inner.add_edge(x, y))
    }

    /// Deletes the edge between vertices `x` and `y`.
    ///
    /// # Arguments
    ///
    /// * `x` - The first vertex.
    /// * `y` - The second vertex.
    ///
    /// # Returns
    ///
    /// `true` if the edge was deleted, `false` if it did not exist.
    ///
    fn del_edge(&mut self, x: &str, y: &str) -> PyResult<bool> {
        // Get the indices of the vertices.
        let x = self.inner.label_to_index(&x);
        let y = self.inner.label_to_index(&y);

        // Delete the edge from the graph.
        Ok(self.inner.del_edge(x, y))
    }
}
