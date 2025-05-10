use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read},
};

use causal_hub::{
    graphs::DiGraph,
    io::BifReader,
    models::{BN, CategoricalBN},
};
use pyo3::{prelude::*, types::PyType};

use crate::{distributions::PyCategoricalCPD, graphs::PyDiGraph};

#[pyclass(name = "CategoricalBN")]
#[derive(Clone, Debug)]
pub struct PyCategoricalBN {
    inner: CategoricalBN,
}

impl From<CategoricalBN> for PyCategoricalBN {
    fn from(inner: CategoricalBN) -> Self {
        Self { inner }
    }
}

impl From<PyCategoricalBN> for CategoricalBN {
    fn from(outer: PyCategoricalBN) -> Self {
        outer.inner
    }
}

#[pymethods]
impl PyCategoricalBN {
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
    fn new(graph: &Bound<'_, PyDiGraph>, cpds: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert PyDiGraph to DiGraph.
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Convert PyAny to Vec<CategoricalCPD>.
        let cpds: Vec<_> = cpds
            .try_iter()?
            .map(|x| x?.extract::<PyCategoricalCPD>())
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyCategoricalCPD> to Vec<CategoricalCPD>.
        let cpds = cpds.into_iter().map(|x| x.into());
        // Create a new CategoricalBN with the given parameters.
        Ok(CategoricalBN::new(graph, cpds).into())
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
    /// A reference to the cpds.
    ///
    fn cpds(&self) -> PyResult<BTreeMap<&str, PyCategoricalCPD>> {
        Ok(self
            .inner
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                // Convert the label to a string slice.
                let label = label.as_ref();
                // Convert the CPD to a PyCategoricalCPD.
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
    fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.inner.parameters_size())
    }

    /// Read a BIF file and return a CategoricalBN.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the BIF file.
    ///
    /// # Returns
    ///
    /// A new CategoricalBN instance.
    ///
    #[classmethod]
    fn read_bif(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        // Open file given by path.
        let file = File::open(path)?;
        // Read the file and parse it.
        let mut reader = BufReader::new(file);
        // Read the BIF file.
        let mut bif = String::new();
        reader.read_to_string(&mut bif)?;
        // Read the BIF file and return a CategoricalBN.
        let bn = BifReader::read(&bif);
        // Convert the BifReader to a CategoricalBN.
        Ok(bn.into())
    }
}
