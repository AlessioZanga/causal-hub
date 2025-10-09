use std::collections::BTreeMap;

use backend::{
    datasets::CatTrjs,
    estimation::{BE, MLE},
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
    estimation::PyCTBNEstimator,
    impl_deref_from_into, kwarg,
    models::{PyCatBN, PyCatCIM, PyDiGraph},
};

/// A continuous-time Bayesian network (CTBN).
#[gen_stub_pyclass]
#[pyclass(name = "CatCTBN", module = "causal_hub.models", eq)]
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

    /// Fit the model to a dataset and a given graph.
    ///
    /// # Arguments
    ///
    /// * `dataset` - The dataset to fit the model to.
    /// * `graph` - The graph to fit the model to.
    /// * `method` - The method to use for fitting (default is `mle`).
    /// * `parallel` - The flag to enable parallel fitting (default is `true`).
    ///
    /// # Returns
    ///
    /// A new fitted model.
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
        // Initialize the sampler.
        let sampler = ForwardSampler::new(&mut rng, &self.inner);
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
