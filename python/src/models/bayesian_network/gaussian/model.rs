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
    random::{Random, RngGaussBN},
    samplers::{BNSampler, ForwardSampler, ParBNSampler},
    types::Labels,
};
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyAnyMethods, PyDict, PyType},
};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    datasets::{PyDataset, PyGaussEv, PyGaussTable, PyMissingMechanism, PyMissingMethod},
    error::to_pyerr,
    estimators::{PyBNEstimator, PyEstimatorMethod},
    impl_from_into_lock, indices_from, kwarg,
    models::{PyDiGraph, PyGaussCPD},
};

/// A Gaussian Bayesian network.
#[gen_stub_pyclass]
#[pyclass(name = "GaussBN", module = "causal_hub.models", eq, from_py_object)]
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
        GaussBN::new(graph, cpds).map(Into::into).map_err(to_pyerr)
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

    /// Returns the support (ranges) of the model variables.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[float, float]]
    ///     A mapping from each variable to its (low, high) range.
    ///
    pub fn support(&self) -> PyResult<BTreeMap<String, (f64, f64)>> {
        Ok(self
            .lock()
            .support()
            .iter()
            .map(|(label, (low, high))| (label.clone(), (*low, *high)))
            .collect())
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
    /// GaussBN
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

    /// Fit the model to a dataset and a given graph.
    ///
    /// Parameters
    /// ----------
    /// dataset: GaussTable
    ///     The dataset to fit the model to.
    /// graph: DiGraph
    ///     The graph to fit the model to.
    /// estimator: EstimatorMethod | None
    ///     The estimator to use for fitting (default is `EstimatorMethod.BE`).
    /// missing_method: MissingMethod | None
    ///     The method to use for handling missing data (default is `MissingMethod.PW`).
    /// missing_mechanism: MissingMechanism | None
    ///     The missing mechanism to use for handling missing data (default is `None`).
    /// parallel: bool
    ///     The flag to enable parallel fitting (default is `true`).
    /// **kwargs: dict | None
    ///     Optional keyword arguments:
    ///
    /// - `alpha`: The prior of the Bayesian estimator (float64).
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
        estimator_method = PyEstimatorMethod::BE,
        missing_method = PyMissingMethod::PW,
        missing_mechanism = None,
        parallel = true,
        **kwargs
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn fit(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dataset: PyDataset,
        graph: &Bound<'_, PyDiGraph>,
        estimator_method: PyEstimatorMethod,
        missing_method: PyMissingMethod,
        missing_mechanism: Option<PyMissingMechanism>,
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
                // Get the estimator method.
                // Initialize the estimator.
                let estimator: Box<dyn PyBNEstimator<GaussBN>> = match estimator_method {
                    // Initialize the maximum likelihood estimator.
                    PyEstimatorMethod::MLE => Box::new(
                        MLE::new(&dataset)
                            .with_missing_method(
                                Some(missing_method.into()),
                                missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                            )
                            .map_err(to_pyerr)?,
                    ),
                    // Initialize the Bayesian estimator.
                    PyEstimatorMethod::BE => {
                        // Initialize the Bayesian estimator.
                        let estimator = BE::new(&dataset)
                            .with_missing_method(
                                Some(missing_method.into()),
                                missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                            )
                            .map_err(to_pyerr)?;
                        // Set the prior `alpha`, if any.
                        match kwarg!(kwargs, "alpha", f64)? {
                            None => Box::new(estimator),
                            Some(alpha) => Box::new(estimator.with_prior(alpha)),
                        }
                    }
                };
                // Fit the model.
                let model = if parallel {
                    // Release the GIL to allow parallel execution.
                    py.detach(move || estimator.par_fit(graph))
                } else {
                    // Execute sequentially.
                    estimator.fit(graph)
                }
                .map_err(to_pyerr)?;
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
    #[pyo3(signature = (
        n,
        seed = 31,
        parallel = true
    ))]
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
        let sampler = ForwardSampler::new(&mut rng, &*lock).map_err(to_pyerr)?;
        // Sample from the model.
        let dataset = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || sampler.par_sample_n(n))
        } else {
            // Sample sequentially.
            sampler.sample_n(n)
        }
        .map_err(to_pyerr)?;
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
    /// w: GaussEv | dict[str, float] | None
    ///     Optional evidence to condition on during inference.
    /// estimator: EstimatorMethod | None
    ///     The estimator to use for estimation (default is `EstimatorMethod.BE`).
    /// missing_method: MissingMethod | None
    ///     The method to use for handling missing data (default is `MissingMethod.PW`).
    /// missing_mechanism: MissingMechanism | None
    ///     The missing mechanism to use for handling missing data (default is `None`).
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
    #[pyo3(signature = (
        x,
        z,
        w = None,
        estimator_method = PyEstimatorMethod::BE,
        missing_method = PyMissingMethod::PW,
        missing_mechanism = None,
        seed = 31,
        parallel = true
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn estimate(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        w: Option<&Bound<'_, PyAny>>,
        estimator_method: PyEstimatorMethod,
        missing_method: PyMissingMethod,
        missing_mechanism: Option<PyMissingMechanism>,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyGaussCPD> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the set of variables.
        let x = indices_from!(x, lock)?;
        let z = indices_from!(z, lock)?;
        // Get the evidence.
        let w = w
            .map(|w| PyGaussEv::from_any(w, lock.labels()).map(Into::into))
            .transpose()?;
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the inference engine.
        let engine = ApproximateInference::new(&mut rng, &*lock);
        // Estimate from the model.
        let estimate = match estimator_method {
            // Initialize the maximum likelihood estimator.
            PyEstimatorMethod::MLE => {
                // Estimate from the model.
                if parallel {
                    // Release the GIL to allow parallel execution.
                    py.detach(move || {
                        engine
                            .with_estimator(|d, x, z| {
                                MLE::new(d)
                                    .with_missing_method(
                                        Some(missing_method.into()),
                                        missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                                    )?
                                    .par_fit(x, z)
                            })
                            .par_estimate(&x, &z, w.as_ref())
                    })
                } else {
                    // Execute sequentially.
                    engine
                        .with_estimator(|d, x, z| {
                            MLE::new(d)
                                .with_missing_method(
                                    Some(missing_method.into()),
                                    missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                                )?
                                .fit(x, z)
                        })
                        .estimate(&x, &z, w.as_ref())
                }
            }
            // Initialize the Bayesian estimator.
            PyEstimatorMethod::BE => {
                // Estimate from the model.
                if parallel {
                    // Release the GIL to allow parallel execution.
                    py.detach(move || {
                        engine
                            .with_estimator(|d, x, z| {
                                BE::new(d)
                                    .with_missing_method(
                                        Some(missing_method.into()),
                                        missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                                    )?
                                    .par_fit(x, z)
                            })
                            .par_estimate(&x, &z, w.as_ref())
                    })
                } else {
                    // Execute sequentially.
                    engine
                        .with_estimator(|d, x, z| {
                            BE::new(d)
                                .with_missing_method(
                                    Some(missing_method.into()),
                                    missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                                )?
                                .fit(x, z)
                        })
                        .estimate(&x, &z, w.as_ref())
                }
            }
        };
        // Return the dataset.
        estimate.map(Into::into).map_err(to_pyerr)
    }

    /// Estimate a conditional population average causal effect (CPACE).
    ///
    /// Parameters
    /// ----------
    /// x: str | Iterable[str]
    ///     An intervention variable or an iterable of intervention variables.
    /// y: str | Iterable[str]
    ///     An outcome variable or an iterable of outcome variables.
    /// z: str | Iterable[str]
    ///     A conditioning variable or an iterable of conditioning variables.
    /// w: GaussEv | dict[str, float] | None
    ///     Optional evidence to condition on during inference.
    /// estimator: EstimatorMethod | None
    ///     The estimator to use for estimation (default is `EstimatorMethod.BE`).
    /// missing_method: MissingMethod | None
    ///     The method to use for handling missing data (default is `MissingMethod.PW`).
    /// missing_mechanism: MissingMechanism | None
    ///     The missing mechanism to use for handling missing data (default is `None`).
    /// seed: int
    ///     The seed of the random number generator (default is `31`).
    /// parallel: bool
    ///     The flag to enable parallel estimation (default is `true`).
    ///
    /// Returns
    /// -------
    /// GaussCPD | None
    ///     A new conditional population average causal effect (CPACE) distribution, if identifiable.
    ///
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        x,
        y,
        z,
        w = None,
        estimator_method = PyEstimatorMethod::BE,
        missing_method = PyMissingMethod::PW,
        missing_mechanism = None,
        seed = 31,
        parallel = true
    ))]
    pub fn do_estimate(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        w: Option<&Bound<'_, PyAny>>,
        estimator_method: PyEstimatorMethod,
        missing_method: PyMissingMethod,
        missing_mechanism: Option<PyMissingMechanism>,
        seed: u64,
        parallel: bool,
    ) -> PyResult<Option<PyGaussCPD>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the set of variables.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let z = indices_from!(z, lock)?;
        // Get the evidence.
        let w = w
            .map(|w| PyGaussEv::from_any(w, lock.labels()).map(Into::into))
            .transpose()?;
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the inference engine.
        let engine = ApproximateInference::new(&mut rng, &*lock);
        // Estimate from the model.
        let estimate = match estimator_method {
            // Initialize the maximum likelihood estimator.
            PyEstimatorMethod::MLE => {
                // Estimate from the model.
                if parallel {
                    // Release the GIL to allow parallel execution.
                    py.detach(move || {
                        let engine = engine.with_estimator(|d, x, z| {
                            MLE::new(d)
                                .with_missing_method(
                                    Some(missing_method.into()),
                                    missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                                )?
                                .par_fit(x, z)
                        });
                        CausalInference::new(&engine).par_cpace_estimate(&x, &y, &z, w.as_ref())
                    })
                } else {
                    // Execute sequentially.
                    let engine = engine.with_estimator(|d, x, z| {
                        MLE::new(d)
                            .with_missing_method(
                                Some(missing_method.into()),
                                missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                            )?
                            .fit(x, z)
                    });
                    CausalInference::new(&engine).cpace_estimate(&x, &y, &z, w.as_ref())
                }
            }
            // Initialize the Bayesian estimator.
            PyEstimatorMethod::BE => {
                // Estimate from the model.
                if parallel {
                    // Release the GIL to allow parallel execution.
                    py.detach(move || {
                        let engine = engine.with_estimator(|d, x, z| {
                            BE::new(d)
                                .with_missing_method(
                                    Some(missing_method.into()),
                                    missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                                )?
                                .par_fit(x, z)
                        });
                        CausalInference::new(&engine).par_cpace_estimate(&x, &y, &z, w.as_ref())
                    })
                } else {
                    // Execute sequentially.
                    let engine = engine.with_estimator(|d, x, z| {
                        BE::new(d)
                            .with_missing_method(
                                Some(missing_method.into()),
                                missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                            )?
                            .fit(x, z)
                    });
                    CausalInference::new(&engine).cpace_estimate(&x, &y, &z, w.as_ref())
                }
            }
        };
        // Return the dataset.
        estimate.map(|e| e.map(Into::into)).map_err(to_pyerr)
    }

    /// Generates a random Gaussian Bayesian network.
    ///
    /// Parameters
    /// ----------
    /// labels: Iterable[str]
    ///     The labels of the variables.
    /// s_a: float, default=1.0
    ///     The standard deviation of the regression coefficients.
    /// s_b: float, default=1.0
    ///     The standard deviation of the intercept.
    /// e: float, default=1e-6
    ///     A small positive constant for covariance regularization.
    /// p: float, default=0.1
    ///     The probability of generating an edge.
    /// seed: int, default=31
    ///     The seed for the random number generator.
    ///
    /// Returns
    /// -------
    /// GaussBN
    ///     A random Gaussian Bayesian network.
    ///
    #[classmethod]
    #[pyo3(signature = (
        labels,
        s_a = 1.0,
        s_b = 1.0,
        e = 1e-6,
        p = 0.1,
        seed = 31
    ))]
    pub fn random(
        _cls: &Bound<'_, PyType>,
        labels: &Bound<'_, PyAny>,
        s_a: f64,
        s_b: f64,
        e: f64,
        p: f64,
        seed: u64,
    ) -> PyResult<Self> {
        // Convert the PyAny to a Labels.
        let labels: Labels = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Create a new RngGaussBN and generate a random BN.
        RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p)
            .and_then(|mut x| x.random())
            .map(Into::into)
            .map_err(to_pyerr)
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
        GaussBN::from_json_string(json)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON string.
    ///
    /// Returns
    /// -------
    /// str
    ///     A JSON string representation of the instance.
    ///
    pub fn to_json_string(&self) -> PyResult<String> {
        self.lock().to_json_string().map_err(to_pyerr)
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
        GaussBN::from_json_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to write to.
    ///
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path).map_err(to_pyerr)
    }
}
