use std::collections::BTreeMap;

use backend::{
    io::JsonIO,
    models::{BN, DiGraph, GaussBN, Labelled},
};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;

use crate::{
    impl_deref_from_into,
    models::{PyDiGraph, PyGaussCPD},
};

/// A Gaussian Bayesian network.
#[gen_stub_pyclass]
#[pyclass(name = "GaussBN", eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyGaussBN {
    inner: GaussBN,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyGaussBN, GaussBN);

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussBN {
    /// Constructs a new Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cpds` - The conditional probability distributions.
    ///
    /// # Returns
    ///
    /// A new Bayesian network instance.
    ///
    #[new]
    pub fn new(graph: &Bound<'_, PyDiGraph>, cpds: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert PyDiGraph to DiGraph.
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Convert PyAny to Vec<CatCPD>.
        let cpds: Vec<_> = cpds
            .try_iter()?
            .map(|x| x?.extract::<PyGaussCPD>())
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyGaussCPD> to Vec<GaussCPD>.
        let cpds = cpds.into_iter().map(|x| x.into());
        // Create a new GaussBN with the given parameters.
        Ok(GaussBN::new(graph, cpds).into())
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
    /// A reference to the CPDs.
    ///
    pub fn cpds(&self) -> PyResult<BTreeMap<&str, PyGaussCPD>> {
        Ok(self
            .inner
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                // Convert the label to a string slice.
                let label = label.as_ref();
                // Convert the CPD to a PyGaussCPD.
                let cpd = cpd.clone().into();
                // Return the label and CPD as a tuple.
                (label, cpd)
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
            inner: GaussBN::from_json(json),
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
            inner: GaussBN::read_json(path),
        })
    }

    /// Write class to a JSON file.
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.inner.write_json(path);
        Ok(())
    }
}
