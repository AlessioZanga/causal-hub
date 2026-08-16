use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    io::JsonIO,
    models::{BN, DiGraph, Labelled, MixedBN},
    samplers::{BNSampler, ForwardSampler, ParBNSampler},
};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    datasets::PyMixedTable,
    error::to_pyerr,
    impl_from_into_lock, indices_from,
    models::{PyDiGraph, PyMixedCPD},
};

/// A mixed Bayesian network.
#[gen_stub_pyclass]
#[pyclass(name = "MixedBN", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyMixedBN {
    inner: Arc<RwLock<MixedBN>>,
}

impl_from_into_lock!(PyMixedBN, MixedBN);

impl PartialEq for PyMixedBN {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMixedBN {
    /// Constructs a new mixed Bayesian network.
    #[new]
    pub fn new(graph: &Bound<'_, PyDiGraph>, cpds: &Bound<'_, PyAny>) -> PyResult<Self> {
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        let cpds: Vec<PyMixedCPD> = cpds
            .try_iter()?
            .map(|x| x.and_then(|x| x.extract::<PyMixedCPD>().map_err(PyErr::from)))
            .collect::<PyResult<_>>()?;
        let cpds = cpds.into_iter().map(|x: PyMixedCPD| x.into());
        MixedBN::new(graph, cpds).map(Into::into).map_err(to_pyerr)
    }

    /// Returns the name of the model, if any.
    pub fn name(&self) -> PyResult<Option<String>> {
        Ok(self.lock().name().map(Into::into))
    }

    /// Returns the description of the model, if any.
    pub fn description(&self) -> PyResult<Option<String>> {
        Ok(self.lock().description().map(Into::into))
    }

    /// Returns the labels of the variables.
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the underlying graph.
    pub fn graph(&self) -> PyResult<PyDiGraph> {
        Ok(self.lock().graph().clone().into())
    }

    /// Returns a map of labels to CPDs.
    pub fn cpds(&self) -> PyResult<BTreeMap<String, PyMixedCPD>> {
        Ok(self
            .lock()
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                let label = label.clone();
                let cpd = cpd.clone().into();
                (label, cpd)
            })
            .collect())
    }

    /// Returns the parameters size.
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.lock().parameters_size())
    }

    /// Restrict the model to the specified variables.
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A variable or an iterable of variables to select.
    ///
    /// Returns
    /// -------
    /// MixedBN
    ///     A model restricted to the specified variables.
    ///
    pub fn select(&self, x: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert the Python iterable into a set of indices.
        let x = indices_from!(x, lock)?;
        // Restrict the model.
        lock.select(&x).map(Into::into).map_err(to_pyerr)
    }

    /// Returns the topological order of the underlying graph.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A topological ordering of the variables.
    ///
    pub fn topological_order(&self) -> PyResult<Vec<String>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Convert the indices to labels.
        lock.topological_order()
            .iter()
            .map(|&i| lock.index_to_label(i).map(str::to_owned))
            .collect::<Result<Vec<_>, _>>()
            .map_err(to_pyerr)
    }

    /// Generate samples from the model.
    #[pyo3(signature = (n, seed = 31, parallel = true))]
    pub fn sample(
        &self,
        py: Python<'_>,
        n: usize,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyMixedTable> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let lock = self.lock();
        let sampler = ForwardSampler::new(&mut rng, &*lock).map_err(to_pyerr)?;
        let dataset: backend::models::MixedTable = if parallel {
            py.detach(move || sampler.par_sample_n(n))
        } else {
            sampler.sample_n(n)
        }
        .map_err(to_pyerr)?;
        Ok(dataset.into())
    }

    /// Read instance from a JSON string.
    #[classmethod]
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        MixedBN::from_json_string(json)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON string.
    pub fn to_json_string(&self) -> PyResult<String> {
        self.lock().to_json_string().map_err(to_pyerr)
    }

    /// Read instance from a JSON file.
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        MixedBN::from_json_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON file.
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path).map_err(to_pyerr)
    }
}
