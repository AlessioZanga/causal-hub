use std::sync::{Arc, RwLock};

use backend::{
    inference::{BackdoorCriterion, GraphicalSeparation},
    io::JsonIO,
    models::{DiGraph, Graph, Labelled},
    types::Labels,
};
use numpy::prelude::*;
use pyo3::{
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::{impl_from_into_lock, indices_from};

/// A struct representing a directed graph using an adjacency matrix.
#[gen_stub_pyclass]
#[pyclass(name = "DiGraph", module = "causal_hub.models", eq)]
#[derive(Clone, Debug)]
pub struct PyDiGraph {
    inner: Arc<RwLock<DiGraph>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyDiGraph, DiGraph);

impl PartialEq for PyDiGraph {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

impl Eq for PyDiGraph {}

#[gen_stub_pymethods]
#[pymethods]
impl PyDiGraph {
    /// Creates an empty directed graph with the given vertices.
    ///
    /// Parameters
    /// ----------
    /// vertices: Iterable[str]
    ///     The vertices of the graph.
    ///     Vertices will be sorted in alphabetical order.
    ///
    /// Returns
    /// -------
    /// DiGraph
    ///     A new graph instance.
    ///
    #[classmethod]
    pub fn empty(_cls: &Bound<'_, PyType>, vertices: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let vertices: Vec<_> = vertices
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Create a new DiGraph with the labels.
        Ok(DiGraph::empty(vertices).into())
    }

    /// Creates a complete directed graph with the given vertices.
    ///
    /// Parameters
    /// ----------
    /// vertices: Iterable[str]
    ///     The the vertices of the graph.
    ///     Vertices will be sorted in alphabetical order.
    ///     No self-loops are created.
    ///
    /// Returns
    /// -------
    /// DiGraph
    ///     A new graph instance.
    ///
    #[classmethod]
    pub fn complete(_cls: &Bound<'_, PyType>, vertices: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let vertices: Vec<_> = vertices
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Create a new DiGraph with the labels.
        Ok(DiGraph::complete(vertices).into())
    }

    /// Returns the vertices of the graph.
    ///
    /// # Returns
    ///
    /// A list of vertices.
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
        // Get the labels of the vertices.
        let x = lock.label_to_index(x);
        // Check if the vertex exists in the graph.
        Ok(lock.has_vertex(x))
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
        Ok(lock
            .edges()
            .into_iter()
            .map(|(x, y)| {
                // Get the labels of the vertices.
                let x = lock.index_to_label(x).into();
                let y = lock.index_to_label(y).into();
                // Return the labels as a tuple.
                (x, y)
            })
            .collect())
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
        let x = lock.label_to_index(x);
        let y = lock.label_to_index(y);
        // Check if the edge exists in the graph.
        Ok(lock.has_edge(x, y))
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
        let x = lock.label_to_index(x);
        let y = lock.label_to_index(y);
        // Add the edge to the graph.
        Ok(lock.add_edge(x, y))
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
        let x = lock.label_to_index(x);
        let y = lock.label_to_index(y);
        // Delete the edge from the graph.
        Ok(lock.del_edge(x, y))
    }

    /// Returns the parents of a vertex `x`.
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A vertex or an iterable of vertices.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of parent vertices.
    ///
    pub fn parents(&self, x: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the index of the vertex.
        let x = indices_from!(x, lock)?;
        // Get the parents of the vertex.
        Ok(lock
            .parents(&x)
            .iter()
            .map(|&i| lock.index_to_label(i).into())
            .collect())
    }

    /// Returns the ancestors of a vertex `x`.
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A vertex or an iterable of vertices.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of ancestor vertices.
    ///
    pub fn ancestors(&self, x: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the index of the vertex.
        let x = indices_from!(x, lock)?;
        // Get the ancestors of the vertex.
        Ok(lock
            .ancestors(&x)
            .iter()
            .map(|&i| lock.index_to_label(i).into())
            .collect())
    }

    /// Returns the children of a vertex `x`.
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A vertex or an iterable of vertices.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of child vertices.
    ///
    pub fn children(&self, x: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the index of the vertex.
        let x = indices_from!(x, lock)?;
        // Get the children of the vertex.
        Ok(lock
            .children(&x)
            .iter()
            .map(|&i| lock.index_to_label(i).into())
            .collect())
    }

    /// Returns the descendants of a vertex `x`.
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A vertex or an iterable of vertices.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of descendant vertices.
    ///
    pub fn descendants(&self, x: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the index of the vertex.
        let x = indices_from!(x, lock)?;
        // Get the descendants of the vertex.
        Ok(lock
            .descendants(&x)
            .iter()
            .map(|&i| lock.index_to_label(i).into())
            .collect())
    }

    /// Checks if the vertex set `Z` is a separator set for `X` and `Y`.
    ///
    /// Parameters
    /// ----------
    /// x: Iterable[str]
    ///     An iterable of vertices representing set `X`.
    /// y: Iterable[str]
    ///     An iterable of vertices representing set `Y`.
    /// z: Iterable[str]
    ///     An iterable of vertices representing set `Z`.
    ///
    /// Notes
    /// ----------
    /// Raises an exception if:
    ///
    ///     * Any of the vertex in `X`, `Y`, or `Z` are out of bounds.
    ///     * `X`, `Y` or `Z` are not disjoint sets.
    ///     * `X` and `Y` are empty sets.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if `X` and `Y` are separated by `Z`, `false` otherwise.
    ///
    pub fn is_separator_set(
        &self,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
    ) -> PyResult<bool> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert Python iterators into Rust iterators on indices.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let z = indices_from!(z, lock)?;
        // Delegate to the inner method.
        Ok(lock.is_separator_set(&x, &y, &z))
    }

    /// Checks if the vertex set `Z` is a minimal separator set for `X` and `Y`.
    ///
    /// Parameters
    /// ----------
    /// x: Iterable[str]
    ///     An iterable of vertices representing set `X`.
    /// y: Iterable[str]
    ///     An iterable of vertices representing set `Y`.
    /// z: Iterable[str]
    ///     An iterable of vertices representing set `Z`.
    /// w: Iterable[str] | None
    ///     An optional iterable of vertices representing set `W`.
    /// v: Iterable[str] | None
    ///     An optional iterable of vertices representing set `V`.
    ///
    /// Notes
    /// ----------
    /// Raises an exception if:
    ///
    ///     * Any of the vertex in `X`, `Y`, `Z`, `W` or `V` are out of bounds.
    ///     * `X`, `Y` or `Z` are not disjoint sets.
    ///     * `X` and `Y` are empty sets.
    ///     * Not `W` <= `Z` <= `V`.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if `Z` is a minimal separator set for `X` and `Y`, `false` otherwise.
    ///
    #[pyo3(signature = (x, y, z, w=None, v=None))]
    pub fn is_minimal_separator_set(
        &self,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        w: Option<&Bound<'_, PyAny>>,
        v: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert Python iterators into Rust iterators on indices.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let z = indices_from!(z, lock)?;
        let w = w.map(|w| indices_from!(w, lock)).transpose()?;
        let v = v.map(|v| indices_from!(v, lock)).transpose()?;
        // Delegate to the inner method.
        Ok(lock.is_minimal_separator_set(&x, &y, &z, w.as_ref(), v.as_ref()))
    }

    /// Finds a minimal separator set for the vertex sets `X` and `Y`, if any.
    ///
    /// Parameters
    /// ----------
    /// x: Iterable[str]
    ///     An iterable of vertices representing set `X`.
    /// y: Iterable[str]
    ///     An iterable of vertices representing set `Y`.
    /// w: Iterable[str] | None
    ///     An optional iterable of vertices representing set `W`.
    /// v: Iterable[str] | None
    ///     An optional iterable of vertices representing set `V`.
    ///
    /// Notes
    /// ----------
    /// Raises an exception if:
    ///
    ///     * Any of the vertex in `X`, `Y`, `W` or `V` are out of bounds.
    ///     * `X` and `Y` are not disjoint sets.
    ///     * `X` or `Y` are empty sets.
    ///     * Not `W` <= `V`.
    ///
    /// Returns
    /// -------
    /// list[str] | None
    ///     A minimal separator set, or `None` if no separator set exists.
    ///
    #[pyo3(signature = (x, y, w=None, v=None))]
    pub fn find_minimal_separator_set(
        &self,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        w: Option<&Bound<'_, PyAny>>,
        v: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Option<Vec<String>>> {
        // Get a lock on the inner field.
        let lock = self.lock();

        // Convert Python iterators into Rust iterators on indices.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let w = w.map(|w| indices_from!(w, lock)).transpose()?;
        let v = v.map(|v| indices_from!(v, lock)).transpose()?;

        // Find the minimal separator.
        let z = lock.find_minimal_separator_set(&x, &y, w.as_ref(), v.as_ref());

        // Convert the indices back to labels.
        let z = z.map(|z| {
            z.into_iter()
                .map(|i| lock.index_to_label(i).into())
                .collect()
        });

        // Return the result.
        Ok(z)
    }

    /// Checks if the vertex set `Z` is a backdoor set for `X` and `Y`.
    ///
    /// Parameters
    /// ----------
    /// x: Iterable[str]
    ///     An iterable of vertices representing set `X`.
    /// y: Iterable[str]
    ///     An iterable of vertices representing set `Y`.
    /// z: Iterable[str]
    ///     An iterable of vertices representing set `Z`.
    ///
    /// Notes
    /// ----------
    /// Raises an exception if:
    ///
    ///     * Any of the vertex in `X`, `Y`, or `Z` are out of bounds.
    ///     * `X`, `Y` or `Z` are not disjoint sets.
    ///     * `X` and `Y` are empty sets.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if `Z` is a backdoor set for `X` and `Y`, `false` otherwise.
    ///
    pub fn is_backdoor_set(
        &self,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
    ) -> PyResult<bool> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert Python iterators into Rust iterators on indices.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let z = indices_from!(z, lock)?;
        // Delegate to the inner method.
        Ok(lock.is_backdoor_set(&x, &y, &z))
    }

    /// Checks if the vertex set `Z` is a minimal backdoor set for `X` and `Y`.
    ///
    /// Parameters
    /// ----------
    /// x: Iterable[str]
    ///     An iterable of vertices representing set `X`.
    /// y: Iterable[str]
    ///     An iterable of vertices representing set `Y`.
    /// z: Iterable[str]
    ///     An iterable of vertices representing set `Z`.
    /// w: Iterable[str] | None
    ///     An optional iterable of vertices representing set `W`.
    /// v: Iterable[str] | None
    ///     An optional iterable of vertices representing set `V`.
    ///
    /// Notes
    /// ----------
    /// Raises an exception if:
    ///
    ///     * Any of the vertex in `X`, `Y`, `Z`, `W` or `V` are out of bounds.
    ///     * `X`, `Y` or `Z` are not disjoint sets.
    ///     * `X` and `Y` are empty sets.
    ///     * Not `W` <= `Z` <= `V`.
    ///
    /// Returns
    /// -------
    /// bool
    ///     `true` if `Z` is a minimal backdoor set for `X` and `Y`, `false` otherwise.
    ///
    #[pyo3(signature = (x, y, z, w=None, v=None))]
    pub fn is_minimal_backdoor_set(
        &self,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        w: Option<&Bound<'_, PyAny>>,
        v: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert Python iterators into Rust iterators on indices.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let z = indices_from!(z, lock)?;
        let w = w.map(|w| indices_from!(w, lock)).transpose()?;
        let v = v.map(|v| indices_from!(v, lock)).transpose()?;
        // Delegate to the inner method.
        Ok(lock.is_minimal_backdoor_set(&x, &y, &z, w.as_ref(), v.as_ref()))
    }

    /// Finds a minimal backdoor set for the vertex sets `X` and `Y`, if any.
    ///
    /// Parameters
    /// ----------
    /// x: Iterable[str]
    ///     An iterable of vertices representing set `X`.
    /// y: Iterable[str]
    ///     An iterable of vertices representing set `Y`.
    /// w: Iterable[str] | None
    ///     An optional iterable of vertices representing set `W`.
    /// v: Iterable[str] | None
    ///     An optional iterable of vertices representing set `V`.
    ///
    /// Notes
    /// ----------
    /// Raises an exception if:
    ///
    ///     * Any of the vertex in `X`, `Y`, `W` or `V` are out of bounds.
    ///     * `X` and `Y` are not disjoint sets.
    ///     * `X` or `Y` are empty sets.
    ///     * Not `W` <= `V`.
    ///
    /// Returns
    /// -------
    /// list[str] | None
    ///     A minimal backdoor set, or `None` if no backdoor set exists.
    ///
    #[pyo3(signature = (x, y, w=None, v=None))]
    pub fn find_minimal_backdoor_set(
        &self,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        w: Option<&Bound<'_, PyAny>>,
        v: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Option<Vec<String>>> {
        // Get a lock on the inner field.
        let lock = self.lock();

        // Convert Python iterators into Rust iterators on indices.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let w = w.map(|w| indices_from!(w, lock)).transpose()?;
        let v = v.map(|v| indices_from!(v, lock)).transpose()?;

        // Find the minimal backdoor.
        let z = lock.find_minimal_backdoor_set(&x, &y, w.as_ref(), v.as_ref());

        // Convert the indices back to labels.
        let z = z.map(|z| {
            z.into_iter()
                .map(|i| lock.index_to_label(i).into())
                .collect()
        });

        // Return the result.
        Ok(z)
    }

    /// Converts from a NetworkX DiGraph.
    ///
    /// Parameters
    /// ----------
    /// g: networkx.DiGraph
    ///     A NetworkX DiGraph to convert from.
    ///
    /// Returns
    /// -------
    /// DiGraph
    ///     A new instance.
    ///
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
    ///
    /// Returns
    /// -------
    /// networkx.DiGraph
    ///     A NetworkX DiGraph representation of the graph.
    ///
    pub fn to_networkx<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        // Load the NetworkX module.
        let nx = py.import("networkx")?;
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the adjacency matrix.
        let adjacency_matrix = lock.to_adjacency_matrix().to_pyarray(py);
        // Create a new PyDict for keyword arguments.
        let kwargs = PyDict::new(py);
        // Set the `create_using` argument to `nx.DiGraph`.
        kwargs.set_item("create_using", nx.getattr("DiGraph")?)?;
        // Create a NetworkX DiGraph from the adjacency matrix.
        let g = nx.call_method("from_numpy_array", (adjacency_matrix,), Some(&kwargs))?;
        // Create a new PyDict for index-label mapping.
        let labels = PyDict::new(py);
        // Set index-label pairs.
        for (i, x) in lock.labels().iter().enumerate() {
            labels.set_item(i, x)?;
        }
        // Relabel the nodes with the graph's labels.
        let g = nx.call_method1("relabel_nodes", (g, labels))?;

        Ok(g)
    }

    /// Read instance from a JSON string.
    ///
    /// Parameters
    /// ----------
    /// json: str
    ///     The JSON string to read from.
    ///
    /// Returns
    /// -------
    /// DiGraph
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(DiGraph::from_json(json))),
        })
    }

    /// Write instance to a JSON string.
    ///
    /// Returns
    /// -------
    /// str
    ///     A JSON string representation of the instance.
    ///
    pub fn to_json(&self) -> PyResult<String> {
        Ok(self.lock().to_json())
    }

    /// Read instance from a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to read from.
    ///
    /// Returns
    /// -------
    /// DiGraph
    ///     A new instance.
    ///
    #[classmethod]
    pub fn read_json(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(DiGraph::read_json(path))),
        })
    }

    /// Write instance to a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to write to.
    ///
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.lock().write_json(path);
        Ok(())
    }
}
