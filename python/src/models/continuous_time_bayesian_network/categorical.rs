use std::collections::BTreeMap;

use causal_hub::{
    graphs::DiGraph,
    models::{CTBN, CatCTBN},
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{distributions::PyCatCIM, graphs::PyDiGraph, impl_deref_from_into};

#[pyclass(name = "CatCTBN")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PyCatCTBN {
    inner: CatCTBN,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatCTBN, CatCTBN);

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
    fn new(graph: &Bound<'_, PyDiGraph>, cims: &Bound<'_, PyAny>) -> PyResult<Self> {
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

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the underlying graph.
    ///
    /// # Returns
    ///
    /// A reference to the graph.
    ///
    fn graph(&self) -> PyResult<PyDiGraph> {
        Ok(self.inner.graph().clone().into())
    }

    /// Returns the a map labels-distributions.
    ///
    /// # Returns
    ///
    /// A reference to the CIMs.
    ///
    fn cims(&self) -> PyResult<BTreeMap<&str, PyCatCIM>> {
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
    fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.inner.parameters_size())
    }
}
