use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::CatTrjs,
    estimators::{BE, MLE},
    io::JsonIO,
    models::{CTBN, CatCTBN, DiGraph, Labelled},
    samplers::{CTBNSampler, ForwardSampler, ParCTBNSampler},
};
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    datasets::PyCatTrjs,
    estimators::PyCTBNEstimator,
    impl_from_into_lock, kwarg,
    models::{PyCatBN, PyCatCIM, PyDiGraph},
};

/// A continuous-time Bayesian network (CTBN).
#[gen_stub_pyclass]
#[pyclass(name = "CatCTBN", module = "causal_hub.models", eq)]
#[derive(Clone, Debug)]
pub struct PyCatCTBN {
    inner: Arc<RwLock<CatCTBN>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatCTBN, CatCTBN);

impl PartialEq for PyCatCTBN {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCatCTBN {
    /// Constructs a new continuous-time Bayesian network.
    ///
    /// Parameters
    /// ----------
    /// graph: DiGraph
    ///     The underlying graph.
    /// cims: Iterable[CatCIM]
    ///     The conditional intensity matrices.
    ///
    /// Returns
    /// -------
    /// CatCTBN
    ///     A new continuous-time Bayesian network instance.
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
    /// Returns
    /// -------
    /// str | None
    ///     The name of the model, if it exists.
    ///
    pub fn name(&self) -> PyResult<Option<String>> {
        Ok(self.lock().name().map(Into::into))
    }

    /// Returns the description of the model, if any.
    ///
    /// Returns
    /// -------
    /// str | None
    ///     The description of the model, if it exists.
    ///
    pub fn description(&self) -> PyResult<Option<String>> {
        Ok(self.lock().description().map(Into::into))
    }

    /// Returns the labels of the variables.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels.
    ///
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the initial distribution.
    ///
    /// Returns
    /// -------
    /// CatBN
    ///     A reference to the initial distribution.
    ///
    pub fn initial_distribution(&self) -> PyResult<PyCatBN> {
        Ok(self.lock().initial_distribution().clone().into())
    }

    /// Returns the underlying graph.
    ///
    /// Returns
    /// -------
    /// DiGraph
    ///     A reference to the graph.
    ///
    pub fn graph(&self) -> PyResult<PyDiGraph> {
        Ok(self.lock().graph().clone().into())
    }

    /// Returns the a map labels-distributions.
    ///
    /// Returns
    /// -------
    /// dict[str, CatCIM]
    ///     A reference to the CIMs.
    ///
    pub fn cims(&self) -> PyResult<BTreeMap<String, PyCatCIM>> {
        Ok(self
            .lock()
            .cims()
            .iter()
            .map(|(label, cim)| {
                // Convert the label to a string slice.
                let label = label.clone();
                // Convert the CIM to a PyCatCIM.
                let cim = cim.clone().into();
                // Return the label and CIM as a tuple.
                (label, cim)
            })
            .collect())
    }

    /// Returns the parameters size.
    ///
    /// Returns
    /// -------
    /// int
    ///     The parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.lock().parameters_size())
    }

    /// Fit the model to a dataset and a given graph.
    ///
    /// Parameters
    /// ----------
    /// dataset: CatTrjs
    ///     The dataset to fit the model to.
    /// graph: DiGraph
    ///     The graph to fit the model to.
    /// method: str
    ///     The method to use for fitting (default is `mle`).
    /// parallel: bool
    ///     The flag to enable parallel fitting (default is `true`).
    /// **kwargs: dict | None
    ///     Optional keyword arguments:
    ///
    ///         - `alpha`: The prior of the Bayesian estimator (int, float64).
    ///
    /// Returns
    /// -------
    /// CatCTBN
    ///     A new fitted model.
    ///
    #[classmethod]
    #[pyo3(signature = (
        dataset,
        graph,
        method="mle",
        parallel=true,
        **kwargs
    ))]
    pub fn fit(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dataset: &Bound<'_, PyCatTrjs>,
        graph: &Bound<'_, PyDiGraph>,
        method: &str,
        parallel: bool,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Get the dataset and the graph.
        let dataset: CatTrjs = dataset.extract::<PyCatTrjs>()?.into();
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Initialize the estimator.
        let estimator: Box<dyn PyCTBNEstimator<CatCTBN>> = match method {
            // Initialize the maximum likelihood estimator.
            "mle" => Box::new(MLE::new(&dataset)),
            // Initialize the Bayesian estimator.
            "be" => {
                // Initialize the Bayesian estimator.
                let estimator = BE::new(&dataset);
                // Set the prior `alpha`, if any.
                match kwarg!(kwargs, "alpha", (usize, f64)) {
                    None => Box::new(estimator),
                    Some(alpha) => Box::new(estimator.with_prior(alpha)),
                }
            }
            // Raise an error if the method is unknown.
            method => {
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Unknown method: '{}', choose one of the following: \n\
                    \t- 'mle' - Maximum likelihood estimator, \n\
                    \t- 'be' - Bayesian estimator.",
                    method
                )));
            }
        };
        // Fit the model.
        let model = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || estimator.par_fit(graph))
        } else {
            // Execute sequentially.
            estimator.fit(graph)
        };
        // Return the fitted model.
        Ok(model.into())
    }

    /// Sample from the model.
    ///
    /// Parameters
    /// ----------
    /// n: int
    ///     The number of trajectories to sample.
    /// max_len: int | None
    ///     The maximum length of each trajectory (default is `None`).
    ///     Must be set if `max_time` is `None`.
    /// max_time: float | None
    ///     The maximum time of each trajectory (default is `None`).
    ///     Must be set if `max_len` is `None`.
    /// seed: int
    ///     The seed of the random number generator (default is `31`).
    /// parallel: bool
    ///     The flag to enable parallel sampling (default is `true`).
    ///
    /// Returns
    /// -------
    /// CatTrjs
    ///     A new dataset containing the sampled trajectories.
    ///
    #[pyo3(signature = (
        n,
        max_len=None,
        max_time=None,
        seed=31,
        parallel=true,
    ))]
    pub fn sample(
        &self,
        py: Python<'_>,
        n: usize,
        max_len: Option<usize>,
        max_time: Option<f64>,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyCatTrjs> {
        // Assert at least one of max_len or max_time is set.
        if max_len.is_none() && max_time.is_none() {
            return Err(PyErr::new::<PyValueError, _>(
                "At least one of 'max_len' or 'max_time' must be set.",
            ));
        }
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Get a lock on the inner field.
        let lock = self.lock();
        // Initialize the sampler.
        let sampler = ForwardSampler::new(&mut rng, &*lock);
        // Get the maximum length and time.
        let max_len = max_len.unwrap_or(usize::MAX);
        let max_time = max_time.unwrap_or(f64::INFINITY);
        // Sample from the model.
        let dataset = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || sampler.par_sample_n_by_length_or_time(max_len, max_time, n))
        } else {
            // Sample sequentially.
            sampler.sample_n_by_length_or_time(max_len, max_time, n)
        };
        // Return the dataset.
        Ok(dataset.into())
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
    /// CatCTBN
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(CatCTBN::from_json(json))),
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
    /// CatCTBN
    ///     A new instance.
    ///
    #[classmethod]
    pub fn read_json(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(CatCTBN::read_json(path))),
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
