use causal_hub_rust::{
    graphs::{DiGraph, Graph},
    types::Labels,
};
use numpy::{PyArray2, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

use crate::impl_deref_from_into;

/// A struct representing a directed graph using an adjacency matrix.
#[gen_stub_pyclass]
#[pyclass(name = "DiGraph", eq)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PyDiGraph {
    inner: DiGraph,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyDiGraph, DiGraph);

#[gen_stub_pymethods]
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
    pub fn empty(_cls: &Bound<'_, PyType>, labels: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let labels: Vec<_> = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Create a new DiGraph with the labels.
        Ok(DiGraph::empty(labels).into())
    }

    /// Creates a complete directed graph with the given labels.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the vertices in the graph.
    ///
    /// # Notes
    ///
    /// * Labels will be sorted in alphabetical order.
    /// * No self-loops are created.
    ///
    /// # Returns
    ///
    /// A new graph instance.
    ///
    #[classmethod]
    pub fn complete(_cls: &Bound<'_, PyType>, labels: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let labels: Vec<_> = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Create a new DiGraph with the labels.
        Ok(DiGraph::complete(labels).into())
    }

    /// Returns the vertices of the graph.
    ///
    /// # Returns
    ///
    /// A list of vertices.
    ///
    pub fn vertices(&self) -> PyResult<Vec<&str>> {
        // Get the labels of the vertices in the graph.
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the edges of the graph.
    ///
    /// # Returns
    ///
    /// A list of edges.
    ///
    pub fn edges(&self) -> PyResult<Vec<(&str, &str)>> {
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
    pub fn has_edge(&self, x: &str, y: &str) -> PyResult<bool> {
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
    pub fn add_edge(&mut self, x: &str, y: &str) -> PyResult<bool> {
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
    pub fn del_edge(&mut self, x: &str, y: &str) -> PyResult<bool> {
        // Get the indices of the vertices.
        let x = self.inner.label_to_index(&x);
        let y = self.inner.label_to_index(&y);
        // Delete the edge from the graph.
        Ok(self.inner.del_edge(x, y))
    }

    /// Returns the parents of a vertex `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The vertex whose parents are to be returned.
    ///
    /// # Returns
    ///
    /// A list of parent vertices.
    ///
    pub fn parents(&self, x: &str) -> PyResult<Vec<&str>> {
        // Get the index of the vertex.
        let x = self.inner.label_to_index(&x);
        // Get the parents of the vertex.
        Ok(self
            .inner
            .parents(x)
            .iter()
            .map(|&i| self.inner.index_to_label(i))
            .collect())
    }

    /// Returns the children of a vertex `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The vertex whose children are to be returned.
    ///
    /// # Returns
    ///
    /// A list of child vertices.
    ///
    pub fn children(&self, x: &str) -> PyResult<Vec<&str>> {
        // Get the index of the vertex.
        let x = self.inner.label_to_index(&x);
        // Get the children of the vertex.
        Ok(self
            .inner
            .children(x)
            .iter()
            .map(|&i| self.inner.index_to_label(i))
            .collect())
    }

    /// Creates a graph from an adjacency matrix and labels.
    ///
    /// # Arguments
    ///
    /// * `labels` - An iterator over the labels of the vertices.
    /// * `adjacency_matrix` - A reference to a 2D array representing the adjacency matrix.
    ///
    /// # Returns
    ///
    /// A new graph instance.
    ///
    #[classmethod]
    pub fn from_adjacency_matrix(
        _cls: &Bound<'_, PyType>,
        labels: &Bound<'_, PyAny>,
        adjacency_matrix: &Bound<'_, PyArray2<i64>>,
    ) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let labels: Vec<_> = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Convert the adjacency matrix to a 2D array.
        let adjacency_matrix = adjacency_matrix.readonly().as_array().mapv(|x| x > 0);
        // Create a new DiGraph from the adjacency matrix.
        Ok(DiGraph::from_adjacency_matrix(labels, adjacency_matrix).into())
    }

    /// Returns the adjacency matrix of the graph.
    ///
    /// # Returns
    ///
    /// A 2D array representing the adjacency matrix.
    ///
    pub fn to_adjacency_matrix<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<i64>>> {
        // Convert the matrix to a PyArray2 and return as PyResult.
        Ok(self
            .inner
            .to_adjacency_matrix()
            .mapv(|x| x as i64)
            .to_pyarray(py))
    }

    /// Converts from a NetworkX DiGraph.
    #[classmethod]
    pub fn from_networkx(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        g: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Load the NetworkX module.
        let nx = py.import("networkx")?;

        // Assume the input is a NetworkX DiGraph.
        assert!(
            g.is_instance(&nx.getattr("DiGraph")?)?,
            "Expected a NetworkX DiGraph, but '{}' found.",
            g.get_type().name()?
        );

        // Get the labels of the vertices.
        let labels: Labels = g
            .getattr("nodes")?
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Get the adjacency matrix from the NetworkX graph.
        let mut graph = DiGraph::empty(labels);
        // Iterate over the edges and add them to the graph.
        for edge in g.getattr("edges")?.try_iter()? {
            // Extract the edge as a tuple of strings.
            let (x, y): (String, String) = edge?.extract()?;
            // Get the indices of the vertices.
            let x = graph.label_to_index(&x);
            let y = graph.label_to_index(&y);
            // Add the edge to the graph.
            graph.add_edge(x, y);
        }

        // Create a new DiGraph from the adjacency matrix.
        Ok(graph.into())
    }

    /// Converts to a NetworkX DiGraph.
    pub fn to_networkx<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        // Load the NetworkX module.
        let nx = py.import("networkx")?;
        // Get the adjacency matrix.
        let adjacency_matrix = self.to_adjacency_matrix(py)?;
        // Create a new PyDict for keyword arguments.
        let kwargs = PyDict::new(py);
        // Set the `create_using` argument to `nx.DiGraph`.
        kwargs.set_item("create_using", nx.getattr("DiGraph")?)?;
        // Create a NetworkX DiGraph from the adjacency matrix.
        let g = nx.call_method("from_numpy_array", (adjacency_matrix,), Some(&kwargs))?;
        // Create a new PyDict for index-label mapping.
        let labels = PyDict::new(py);
        // Set index-label pairs.
        for (i, x) in self.inner.labels().iter().enumerate() {
            labels.set_item(i, x)?;
        }
        // Relabel the nodes with the graph's labels.
        let g = nx.call_method1("relabel_nodes", (g, labels))?;

        Ok(g)
    }
}
