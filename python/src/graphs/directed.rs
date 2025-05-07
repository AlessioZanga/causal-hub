use causal_hub::graphs::{DiGraph as _DiGraph, Graph};
use pyo3::{
    prelude::*,
    types::{PyIterator, PyString, PyType},
};

#[pyclass]
pub struct DiGraph {
    inner: _DiGraph,
}

#[pymethods]
impl DiGraph {
    #[classmethod]
    fn empty(_cls: &Bound<'_, PyType>, labels: &Bound<'_, PyIterator>) -> PyResult<Self> {
        // Convert the PyIterator to a Vec<String>.
        let labels: Vec<_> = labels
            .into_iter()
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Create a new DiGraph with the labels.
        Ok(Self {
            inner: _DiGraph::empty(labels),
        })
    }

    fn has_edge(&self, x: &Bound<'_, PyString>, y: &Bound<'_, PyString>) -> PyResult<bool> {
        // Extract the labels from the PyString.
        let x = x.extract::<String>()?;
        let y = y.extract::<String>()?;

        // Get the indices of the vertices.
        let x = self.inner.label_to_index(&x);
        let y = self.inner.label_to_index(&y);

        // Check if the edge exists in the graph.
        Ok(self.inner.has_edge(x, y))
    }
}
