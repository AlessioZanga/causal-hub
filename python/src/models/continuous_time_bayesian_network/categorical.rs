use std::collections::BTreeMap;

use backend::{
    io::JsonIO,
    models::{CTBN, CatCTBN, DiGraph},
};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;

use crate::{
    impl_deref_from_into,
    models::{PyCatBN, PyCatCIM, PyDiGraph},
};

/// A continuous-time Bayesian network (CTBN).
#[gen_stub_pyclass]
#[pyclass(name = "CatCTBN", eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyCatCTBN {
    inner: CatCTBN,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatCTBN, CatCTBN);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatCTBN {
    /// Constructs a new continuous-time Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cims` - The conditional intensity matrices.
    ///
    /// # Returns
    ///
    /// A new continuous-time Bayesian network instance.
    ///
    #[new]
    pub fn new(graph: &Bound<'_, PyDiGraph>, cims: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert PyDiGraph to DiGraph.
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Convert PyAny to Vec<CatCPD>.
        let cims: Vec<_> = cims
            .try_iter()?
            .map(|x| x?.extract::<PyCatCIM>())
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyCatCPD> to Vec<CatCIM>.
        let cims = cims.into_iter().map(|x| x.into());
        // Create a new CatCTBN with the given parameters.
        Ok(CatCTBN::new(graph, cims).into())
    }

    /// Returns the name of the model, if any.
    ///
    /// # Returns
    ///
    /// The name of the model, if it exists.
    ///
    pub fn name(&self) -> PyResult<Option<&str>> {
        Ok(self.inner.name())
    }

    /// Returns the description of the model, if any.
    ///
    /// # Returns
    ///
    /// The description of the model, if it exists.
    ///
    pub fn description(&self) -> PyResult<Option<&str>> {
        Ok(self.inner.description())
    }

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the initial distribution.
    ///
    /// # Returns
    ///
    /// A reference to the initial distribution.
    ///
    pub fn initial_distribution(&self) -> PyResult<PyCatBN> {
        Ok(self.inner.initial_distribution().clone().into())
    }

    /// Returns the underlying graph.
    ///
    /// # Returns
    ///
    /// A reference to the graph.
    ///
    pub fn graph(&self) -> PyResult<PyDiGraph> {
        Ok(self.inner.graph().clone().into())
    }

    /// Returns the a map labels-distributions.
    ///
    /// # Returns
    ///
    /// A reference to the CIMs.
    ///
    pub fn cims(&self) -> PyResult<BTreeMap<&str, PyCatCIM>> {
        Ok(self
            .inner
            .cims()
            .iter()
            .map(|(label, cim)| {
                // Convert the label to a string slice.
                let label = label.as_ref();
                // Convert the CIM to a PyCatCIM.
                let cim = cim.clone().into();
                // Return the label and CIM as a tuple.
                (label, cim)
            })
            .collect())
    }

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.inner.parameters_size())
    }

    /// Read class from a JSON string.
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: CatCTBN::from_json(json),
        })
    }

    /// Write class to a JSON string.
    pub fn to_json(&self) -> PyResult<String> {
        Ok(self.inner.to_json())
    }

    /// Read class from a JSON file.
    #[classmethod]
    pub fn read_json(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: CatCTBN::read_json(path),
        })
    }

    /// Write class to a JSON file.
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.inner.write_json(path);
        Ok(())
    }
}
