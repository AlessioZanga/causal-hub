use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read},
};

use causal_hub::{
    graphs::DiGraph,
    io::BifReader,
    models::{BN, CatBN},
};
use pyo3::{prelude::*, types::PyType};
use serde::{Deserialize, Serialize};

use crate::{distributions::PyCatCPD, graphs::PyDiGraph, impl_deref_from_into};

#[pyclass(name = "CatBN")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PyCatBN {
    inner: CatBN,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatBN, CatBN);

#[pymethods]
impl PyCatBN {
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
            .map(|x| x?.extract::<PyCatCPD>())
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyCatCPD> to Vec<CatCPD>.
        let cpds = cpds.into_iter().map(|x| x.into());
        // Create a new CatBN with the given parameters.
        Ok(CatBN::new(graph, cpds).into())
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
    pub fn cpds(&self) -> PyResult<BTreeMap<&str, PyCatCPD>> {
        Ok(self
            .inner
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                // Convert the label to a string slice.
                let label = label.as_ref();
                // Convert the CPD to a PyCatCPD.
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

    /// Read a BIF file and return a CatBN.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the BIF file.
    ///
    /// # Returns
    ///
    /// A new CatBN instance.
    ///
    #[classmethod]
    pub fn read_bif(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        // Open file given by path.
        let file = File::open(path)?;
        // Read the file and parse it.
        let mut reader = BufReader::new(file);
        // Read the BIF file.
        let mut bif = String::new();
        reader.read_to_string(&mut bif)?;
        // Read the BIF file and return a CatBN.
        let bn = BifReader::read(&bif);
        // Convert the BifReader to a CatBN.
        Ok(bn.into())
    }

    /// Parse a JSON string.
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(serde_json::from_str(json).unwrap())
    }

    /// Serialize to a JSON string.
    pub fn to_json(&self) -> PyResult<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }
}
