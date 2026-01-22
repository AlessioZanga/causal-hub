use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{GaussIncTable, GaussTable},
    estimators::{BE, CPDEstimator, MLE, ParCPDEstimator},
    inference::{
        ApproximateInference, BNCausalInference, BNInference, CausalInference,
        ParBNCausalInference, ParBNInference,
    },
    io::JsonIO,
    models::{BN, DiGraph, GaussBN, Labelled},
    samplers::{BNSampler, ForwardSampler, ParBNSampler},
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
    datasets::{PyDataset, PyGaussTable},
    estimators::PyBNEstimator,
    impl_from_into_lock, indices_from, kwarg,
    models::{PyDiGraph, PyGaussCPD},
};

/// A Gaussian Bayesian network.
#[gen_stub_pyclass]
#[pyclass(name = "GaussBN", module = "causal_hub.models", eq)]
#[derive(Clone, Debug)]
pub struct PyGaussBN {
    inner: Arc<RwLock<GaussBN>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyGaussBN, GaussBN);

impl PartialEq for PyGaussBN {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussBN {
    /// Constructs a new Bayesian network.
    ///
    /// Parameters
    /// ----------
    /// graph: DiGraph
    ///     The underlying graph.
    /// cpds: Iterable[GaussCPD]
    ///     The conditional probability distributions.
    ///
    /// Returns
    /// -------
    /// GaussBN
    ///     A new Bayesian network instance.
    ///
    #[new]
    pub fn new(graph: &Bound<'_, PyDiGraph>, cpds: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert PyDiGraph to DiGraph.
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Convert PyAny to Vec<CatCPD>.
        let cpds: Vec<PyGaussCPD> = cpds
            .try_iter()?
            .map(|x| x.and_then(|x| x.extract::<PyGaussCPD>().map_err(PyErr::from)))
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyGaussCPD> to Vec<GaussCPD>.
        let cpds = cpds.into_iter().map(|x: PyGaussCPD| x.into());
        // Create a new GaussBN with the given parameters.
        Ok(GaussBN::new(graph, cpds).into())
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
    /// dict[str, GaussCPD]
    ///     A reference to the CPDs.
    ///
    pub fn cpds(&self) -> PyResult<BTreeMap<String, PyGaussCPD>> {
        Ok(self
            .lock()
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                // Convert the label to a string slice.
                let label = label.clone();
                // Convert the CPD to a PyGaussCPD.
                let cpd = cpd.clone().into();
                // Return the label and CPD as a tuple.
                (label, cpd)
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
    /// dataset: GaussTable
    ///     The dataset to fit the model to.
    /// graph: DiGraph
    ///     The graph to fit the model to.
    /// method: str
    ///     The method to use for fitting (default is `be`).
    /// parallel: bool
    ///     The flag to enable parallel fitting (default is `true`).
    /// **kwargs: dict | None
    ///     Optional keyword arguments:
    ///
    ///         - `alpha`: The prior of the Bayesian estimator (float64).
    ///
    /// Returns
    /// -------
    /// GaussBN
    ///     A new fitted model.
    ///
    #[classmethod]
    #[pyo3(signature = (
        dataset,
        graph,
        method="be",
        parallel=true,
        **kwargs
    ))]
    pub fn fit(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dataset: PyDataset,
        graph: &Bound<'_, PyDiGraph>,
        method: &str,
        parallel: bool,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Get the graph.
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();

        // Macro to fit the model.
        macro_rules! fit {
            ($type: ty, $dataset: expr) => {{
                // Get the dataset.
                let dataset: $type = $dataset.into();
                // Initialize the estimator.
                let estimator: Box<dyn PyBNEstimator<GaussBN>> = match method {
                    // Initialize the maximum likelihood estimator.
                    "mle" => Box::new(MLE::new(&dataset)),
                    // Initialize the Bayesian estimator.
                    "be" => {
                        // Initialize the Bayesian estimator.
                        let estimator = BE::new(&dataset);
                        // Set the prior `alpha`, if any.
                        match kwarg!(kwargs, "alpha", f64) {
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
            }};
        }

        // Match the dataset type.
        match dataset {
            PyDataset::Gaussian(dataset) => fit!(GaussTable, dataset),
            PyDataset::GaussianIncomplete(dataset) => fit!(GaussIncTable, dataset),
            _ => Err(PyErr::new::<PyValueError, _>(
                "Expected a Gaussian dataset for a Gaussian Bayesian network.",
            )),
        }
    }

    /// Generate samples from the model.
    ///
    /// Parameters
    /// ----------
    /// n: int
    ///     The number of samples to generate.
    /// seed: int
    ///     The seed of the random number generator (default is `31`).
    /// parallel: bool
    ///     The flag to enable parallel sampling (default is `true`).
    ///
    /// Returns
    /// -------
    /// GaussTable
    ///     A new dataset containing the samples.
    ///
    #[pyo3(signature = (n, seed=31, parallel=true))]
    pub fn sample(
        &self,
        py: Python<'_>,
        n: usize,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyGaussTable> {
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Get a lock on the inner field.
        let lock = self.lock();
        // Initialize the sampler.
        let sampler = ForwardSampler::new(&mut rng, &*lock);
        // Sample from the model.
        let dataset = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || sampler.par_sample_n(n))
        } else {
            // Sample sequentially.
            sampler.sample_n(n)
        };
        // Return the dataset.
        Ok(dataset.into())
    }

    /// Estimate a conditional probability distribution (CPD).
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     A variable or an iterable of variables.
    /// z: str | Iterable[str]
    ///     A conditioning variable or an iterable of conditioning variables.
    /// method: str
    ///     The method to use for estimation (default is `be`).
    /// seed: int
    ///     The seed of the random number generator (default is `31`).
    /// parallel: bool
    ///     The flag to enable parallel estimation (default is `true`).
    ///
    /// Returns
    /// -------
    /// GaussCPD
    ///     A new conditional probability distribution.
    ///
    #[pyo3(signature = (x, z, method = "be", seed=31, parallel=true))]
    pub fn estimate(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        method: &str,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyGaussCPD> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the set of variables.
        let x = indices_from!(x, lock)?;
        let z = indices_from!(z, lock)?;
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the inference engine.
        let engine = ApproximateInference::new(&mut rng, &*lock);
        // Estimate from the model.
        let estimate = match method {
            // Initialize the maximum likelihood estimator.
            "mle" => {
                // Estimate from the model.
                if parallel {
                    // Release the GIL to allow parallel execution.
                    py.detach(move || {
                        engine
                            .with_estimator(|d, x, z| MLE::new(d).par_fit(x, z))
                            .par_estimate(&x, &z)
                    })
                } else {
                    // Execute sequentially.
                    engine
                        .with_estimator(|d, x, z| MLE::new(d).fit(x, z))
                        .estimate(&x, &z)
                }
            }
            // Initialize the Bayesian estimator.
            "be" => {
                // Estimate from the model.
                if parallel {
                    // Release the GIL to allow parallel execution.
                    py.detach(move || {
                        engine
                            .with_estimator(|d, x, z| BE::new(d).par_fit(x, z))
                            .par_estimate(&x, &z)
                    })
                } else {
                    // Execute sequentially.
                    engine
                        .with_estimator(|d, x, z| BE::new(d).fit(x, z))
                        .estimate(&x, &z)
                }
            }
            _ => {
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Unknown method: '{}', choose one of the following: \n\
                    \t- 'mle' - Maximum likelihood estimator, \n\
                    \t- 'be' - Bayesian estimator.",
                    method
                )));
            }
        };
        // Return the dataset.
        Ok(estimate.into())
    }

    /// Estimate a conditional causal effect (CACE).
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     An intervention variable or an iterable of intervention variables.
    /// y: str | Iterable[str]
    ///     An outcome variable or an iterable of outcome variables.
    /// z: str | Iterable[str]
    ///     A conditioning variable or an iterable of conditioning variables.
    /// seed: int
    ///     The seed of the random number generator (default is `31`).
    /// parallel: bool
    ///     The flag to enable parallel estimation (default is `true`).
    ///
    /// Returns
    /// -------
    /// GaussCPD | None
    ///     A new conditional causal effect (CACE) distribution, if identifiable.
    ///
    #[pyo3(signature = (x, y, z, seed=31, parallel=true))]
    pub fn do_estimate(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        seed: u64,
        parallel: bool,
    ) -> PyResult<Option<PyGaussCPD>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the set of variables.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let z = indices_from!(z, lock)?;
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the inference engine.
        let estimator = ApproximateInference::new(&mut rng, &*lock);
        // Estimate from the model.
        let estimate = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || CausalInference::new(&estimator).par_cace_estimate(&x, &y, &z))
        } else {
            // Execute sequentially.
            CausalInference::new(&estimator).cace_estimate(&x, &y, &z)
        };
        // Return the dataset.
        Ok(estimate.map(|e| e.into()))
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
    /// GaussBN
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(GaussBN::from_json_string(json))),
        })
    }

    /// Write instance to a JSON string.
    ///
    /// Returns
    /// -------
    /// str
    ///     A JSON string representation of the instance.
    ///
    pub fn to_json_string(&self) -> PyResult<String> {
        Ok(self.lock().to_json_string())
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
    /// GaussBN
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(GaussBN::from_json_file(path))),
        })
    }

    /// Write instance to a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to write to.
    ///
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path);
        Ok(())
    }
}
