use std::sync::{Arc, RwLock};

use backend::{
    io::{DotIO, GmlIO, JsonIO},
    models::{Graph, HasLabels, UnGraph},
    random::{Random, RngUnGraph},
    types::Labels,
};
use numpy::{PyArray2, prelude::*};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{error::to_pyerr, impl_from_into_lock, indices_from};

/// A struct representing an undirected graph using an adjacency matrix.
///
#[gen_stub_pyclass]
#[pyclass(name = "UnGraph", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyUnGraph {
    inner: Arc<RwLock<UnGraph>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyUnGraph, UnGraph);

impl PartialEq for PyUnGraph {
    fn eq(&self, other: &Self) -> bool {
        // Compare the adjacency matrices and the label sets.
        let self_lock = self.lock();
        let other_lock = other.lock();
        self_lock.to_adjacency_matrix() == other_lock.to_adjacency_matrix()
            && self_lock.labels() == other_lock.labels()
    }
}

impl Eq for PyUnGraph {}

#[gen_stub_pymethods]
#[pymethods]
impl PyUnGraph {
    /// Creates an empty undirected graph with the given vertices.
    ///
    /// Parameters
    /// ----------
    /// vertices: Iterable[str]
    ///     The vertices of the graph.
    ///     Vertices will be sorted in alphabetical order.
    ///
    /// Returns
    /// -------
    /// UnGraph
    ///     A new graph instance.
    ///
    #[classmethod]
    pub fn empty(_cls: &Bound<'_, PyType>, vertices: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let vertices: Vec<_> = vertices
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Create a new UnGraph with the labels.
        UnGraph::empty(vertices).map(Into::into).map_err(to_pyerr)
    }

    /// Creates a complete undirected graph with the given vertices.
    ///
    /// Parameters
    /// ----------
    /// vertices: Iterable[str]
    ///     The vertices of the graph.
    ///     Vertices will be sorted in alphabetical order.
    ///     No self-loops are created.
    ///
    /// Returns
    /// -------
    /// UnGraph
    ///     A new graph instance.
    ///
    #[classmethod]
    pub fn complete(_cls: &Bound<'_, PyType>, vertices: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let vertices: Vec<_> = vertices
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Create a new UnGraph with the labels.
        UnGraph::complete(vertices)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Returns the vertices of the graph.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of vertices.
    ///
    pub fn vertices(&self) -> PyResult<Vec<String>> {
        // Get the labels of the vertices in the graph.
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Checks if a vertex exists in the graph.
    ///
    /// Parameters
    /// ----------
    /// x: str
    ///     The vertex.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if the vertex exists, `false` otherwise.
    ///
    pub fn has_vertex(&self, x: &str) -> PyResult<bool> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the index of the vertex.
        let x = lock.label_to_index(x).map_err(to_pyerr)?;
        // Check if the vertex exists in the graph.
        Ok(lock.has_vertex(x))
    }

    /// Adds a new vertex with the given label to the graph.
    ///
    /// Parameters
    /// ----------
    /// x: str
    ///     The label of the vertex to add.
    ///
    /// Returns
    /// -------
    /// int
    ///     The index of the (possibly new) vertex.
    ///     Vertices are kept sorted in alphabetical order:
    ///     adding a vertex may shift the indices of other vertices.
    ///
    pub fn add_vertex(&mut self, x: &str) -> PyResult<usize> {
        // Get a mutable lock on the inner field.
        let mut lock = self.lock_mut();
        // Add the vertex to the graph.
        Ok(lock.add_vertex(x))
    }

    /// Deletes the vertex with the given label from the graph,
    /// together with all its incident edges.
    ///
    /// Parameters
    /// ----------
    /// x: str
    ///     The label of the vertex to delete.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if the vertex was deleted, `false` if it did not exist.
    ///     Vertices are kept sorted in alphabetical order:
    ///     deleting a vertex may shift the indices of other vertices.
    ///
    pub fn del_vertex(&mut self, x: &str) -> PyResult<bool> {
        // Get a mutable lock on the inner field.
        let mut lock = self.lock_mut();
        // Get the index of the vertex, if any.
        let Ok(x) = lock.label_to_index(x) else {
            // If the label does not exist, the vertex does not exist.
            return Ok(false);
        };
        // Delete the vertex from the graph.
        Ok(lock.del_vertex(x))
    }

    /// Returns the edges of the graph.
    ///
    /// Returns
    /// -------
    /// list[tuple[str, str]]
    ///     A list of edges.
    ///
    pub fn edges(&self) -> PyResult<Vec<(String, String)>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the edges of the graph.
        lock.edges()
            .into_iter()
            .map(|(x, y)| {
                // Get the labels of the vertices.
                let x = lock.index_to_label(x).map_err(to_pyerr)?.into();
                let y = lock.index_to_label(y).map_err(to_pyerr)?.into();
                // Return the labels as a tuple.
                Ok((x, y))
            })
            .collect::<PyResult<_>>()
    }

    /// Checks if there is an edge between vertices `x` and `y`.
    ///
    /// Parameters
    /// ----------
    /// x: str
    ///     The first vertex.
    /// y: str
    ///     The second vertex.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if there is an edge between `x` and `y`, `false` otherwise.
    ///
    pub fn has_edge(&self, x: &str, y: &str) -> PyResult<bool> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the indices of the vertices.
        let x = lock.label_to_index(x).map_err(to_pyerr)?;
        let y = lock.label_to_index(y).map_err(to_pyerr)?;
        // Check if the edge exists in the graph.
        lock.has_edge(x, y).map_err(to_pyerr)
    }

    /// Adds an edge between vertices `x` and `y`.
    ///
    /// Parameters
    /// ----------
    /// x: str
    ///     The first vertex.
    /// y: str
    ///     The second vertex.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if the edge was added, `false` if it already existed.
    ///
    pub fn add_edge(&mut self, x: &str, y: &str) -> PyResult<bool> {
        // Get a mutable lock on the inner field.
        let mut lock = self.lock_mut();
        // Get the indices of the vertices.
        let x = lock.label_to_index(x).map_err(to_pyerr)?;
        let y = lock.label_to_index(y).map_err(to_pyerr)?;
        // Add the edge to the graph.
        lock.add_edge(x, y).map_err(to_pyerr)
    }

    /// Deletes the edge between vertices `x` and `y`.
    ///
    /// Parameters
    /// ----------
    /// x: str
    ///     The first vertex.
    /// y: str
    ///     The second vertex.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if the edge was deleted, `false` if it did not exist.
    ///
    pub fn del_edge(&mut self, x: &str, y: &str) -> PyResult<bool> {
        // Get a mutable lock on the inner field.
        let mut lock = self.lock_mut();
        // Get the indices of the vertices.
        let x = lock.label_to_index(x).map_err(to_pyerr)?;
        let y = lock.label_to_index(y).map_err(to_pyerr)?;
        // Delete the edge from the graph.
        lock.del_edge(x, y).map_err(to_pyerr)
    }

    /// Returns the neighbors of a vertex `x`.
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A vertex or an iterable of vertices.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of neighbor vertices.
    ///
    pub fn neighbors(&self, x: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the index of the vertex.
        let x = indices_from!(x, lock)?;
        // Get the neighbors of the vertex.
        lock.neighbors(&x)
            .map_err(to_pyerr)?
            .iter()
            .map(|&i| {
                lock.index_to_label(i)
                    .map_err(to_pyerr)
                    .map(|label| label.into())
            })
            .collect()
    }

    /// Restrict the graph to the specified variables.
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A variable or an iterable of variables to select.
    ///
    /// Returns
    /// -------
    /// UnGraph
    ///     A graph restricted to the specified variables.
    ///
    pub fn select(&self, x: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert the Python iterable into a set of indices.
        let x = indices_from!(x, lock)?;
        // Restrict the graph.
        lock.select(&x).map(Into::into).map_err(to_pyerr)
    }

    /// Creates an undirected graph from an adjacency matrix and labels.
    ///
    /// Parameters
    /// ----------
    /// labels: Iterable[str]
    ///     The labels of the vertices.
    /// adjacency_matrix: numpy.ndarray
    ///     A 2D boolean array representing the (symmetric) adjacency matrix.
    ///
    /// Returns
    /// -------
    /// UnGraph
    ///     A new graph instance.
    ///
    #[classmethod]
    pub fn from_adjacency_matrix(
        _cls: &Bound<'_, PyType>,
        labels: &Bound<'_, PyAny>,
        adjacency_matrix: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Convert the PyIterator to a vector of labels.
        let labels: Vec<String> = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        let labels: Labels = labels.into_iter().collect();
        // Convert the adjacency matrix.
        let adjacency_matrix = adjacency_matrix.cast::<PyArray2<u8>>()?.to_owned_array();
        let adjacency_matrix = adjacency_matrix.mapv(|x| x != 0);
        // Create a new UnGraph from the adjacency matrix.
        UnGraph::from_adjacency_matrix(labels, adjacency_matrix)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Converts the graph to an adjacency matrix.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D boolean array representing the adjacency matrix.
    ///
    pub fn to_adjacency_matrix<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<u8>>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert the adjacency matrix to a NumPy array.
        Ok(lock.to_adjacency_matrix().mapv(|b| b as u8).to_pyarray(py))
    }

    /// Generates a random undirected graph.
    ///
    /// Parameters
    /// ----------
    /// labels: Iterable[str]
    ///     The labels of the graph.
    /// p: float, default=0.1
    ///     The probability of generating an edge.
    /// seed: int, default=31
    ///     The seed for the random number generator.
    ///
    /// Returns
    /// -------
    /// UnGraph
    ///     A random undirected graph.
    ///
    #[classmethod]
    #[pyo3(signature = (
        labels,
        p = 0.1,
        seed = 31
    ))]
    pub fn random(
        _cls: &Bound<'_, PyType>,
        labels: &Bound<'_, PyAny>,
        p: f64,
        seed: u64,
    ) -> PyResult<Self> {
        // Convert the PyIterator to a Labels.
        let labels: Labels = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Create a new RngUnGraph and generate a random graph.
        RngUnGraph::new(&mut rng, &labels, p)
            .and_then(|mut x| x.random())
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Read instance from a JSON string.
    ///
    #[classmethod]
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        UnGraph::from_json_string(json)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON string.
    ///
    pub fn to_json_string(&self) -> PyResult<String> {
        self.lock().to_json_string().map_err(to_pyerr)
    }

    /// Read instance from a JSON file.
    ///
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        UnGraph::from_json_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON file.
    ///
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path).map_err(to_pyerr)
    }

    /// Read instance from a DOT string.
    ///
    #[classmethod]
    pub fn from_dot_string(_cls: &Bound<'_, PyType>, dot: &str) -> PyResult<Self> {
        UnGraph::from_dot_string(dot)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a DOT string.
    ///
    pub fn to_dot_string(&self) -> PyResult<String> {
        self.lock().to_dot_string().map_err(to_pyerr)
    }

    /// Read instance from a DOT file.
    ///
    #[classmethod]
    pub fn from_dot_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        UnGraph::from_dot_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a DOT file.
    ///
    pub fn to_dot_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_dot_file(path).map_err(to_pyerr)
    }

    /// Read instance from a GML string.
    ///
    #[classmethod]
    pub fn from_gml_string(_cls: &Bound<'_, PyType>, gml: &str) -> PyResult<Self> {
        UnGraph::from_gml_string(gml)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a GML string.
    ///
    pub fn to_gml_string(&self) -> PyResult<String> {
        self.lock().to_gml_string().map_err(to_pyerr)
    }

    /// Read instance from a GML file.
    ///
    #[classmethod]
    pub fn from_gml_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        UnGraph::from_gml_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a GML file.
    ///
    pub fn to_gml_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_gml_file(path).map_err(to_pyerr)
    }
}
