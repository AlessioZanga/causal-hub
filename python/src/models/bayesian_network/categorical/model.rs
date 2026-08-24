use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatIncTable, CatTable},
    estimators::{BE, CPDEstimator, MLE, ParCPDEstimator},
    inference::{
        ApproximateInference, BNCausalInference, BNInference, CausalInference,
        ParBNCausalInference, ParBNInference,
    },
    io::{BifIO, JsonIO},
    models::{BN, CatBN, CatSupport, DiGraph, HasLabels},
    random::{Random, RngCatBN},
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
    datasets::{PyCatEv, PyCatTable, PyDataset, PyMissingMechanism, PyMissingMethod},
    error::to_pyerr,
    estimators::{PyBNEstimator, PyEstimatorMethod},
    impl_from_into_lock, indices_from, kwarg,
    models::{PyCatCPD, PyDiGraph},
};

/// A categorical Bayesian network (BN).
///
#[gen_stub_pyclass]
#[pyclass(name = "CatBN", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCatBN {
    inner: Arc<RwLock<CatBN>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatBN, CatBN);

impl PartialEq for PyCatBN {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCatBN {
    /// Constructs a new Bayesian network.
    ///
    /// Parameters
    /// ----------
    /// graph: DiGraph
    ///     The underlying graph.
    /// cpds: Iterable[CatCPD]
    ///     The conditional probability distributions.
    ///
    /// Returns
    /// -------
    /// CatBN
    ///     A new Bayesian network instance.
    ///
    #[new]
    pub fn new(graph: &Bound<'_, PyDiGraph>, cpds: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert PyDiGraph to DiGraph.
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Convert PyAny to Vec<CatCPD>.
        let cpds: Vec<PyCatCPD> = cpds
            .try_iter()?
            .map(|x| x.and_then(|x| x.extract::<PyCatCPD>().map_err(PyErr::from)))
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyCatCPD> to Vec<CatCPD>.
        let cpds = cpds.into_iter().map(|x: PyCatCPD| x.into());
        // Create a new CatBN with the given parameters.
        CatBN::new(graph, cpds).map(Into::into).map_err(to_pyerr)
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
    /// dict[str, CatCPD]
    ///     A reference to the CPDs.
    ///
    pub fn cpds(&self) -> PyResult<BTreeMap<String, PyCatCPD>> {
        Ok(self
            .lock()
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                // Convert the label to a string slice.
                let label = label.clone();
                // Convert the CPD to a PyCatCPD.
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

    /// Returns the support (states) of the model variables.
    ///
    /// Returns
    /// -------
    /// dict[str, list[str]]
    ///     A mapping from each variable to its possible states.
    ///
    pub fn support(&self) -> PyResult<BTreeMap<String, Vec<String>>> {
        Ok(self
            .lock()
            .support()
            .iter()
            .map(|(label, states)| (label.clone(), states.iter().cloned().collect()))
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
    /// CatBN
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
    /// dataset: CatTable | CatIncTable
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
    /// CatBN
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
            ($type:ty, $dataset:expr) => {{
                // Get the dataset.
                let dataset: $type = $dataset.into();
                // Get the prior `alpha` from the keyword arguments, if any.
                let alpha = kwarg!(kwargs, "alpha", usize)?;
                // Reject any unknown keyword arguments.
                crate::utils::ensure_kwargs_consumed(kwargs)?;

                // Initialize the estimator.
                let estimator: Box<dyn PyBNEstimator<CatBN>> = match estimator_method {
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
                        match alpha {
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
            PyDataset::Categorical(dataset) => fit!(CatTable, dataset),
            PyDataset::CategoricalIncomplete(dataset) => fit!(CatIncTable, dataset),
            _ => Err(PyErr::new::<PyValueError, _>(
                "Expected a Categorical dataset for a Categorical Bayesian network.",
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
    /// CatTable
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
    ) -> PyResult<PyCatTable> {
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
            // Execute sequentially.
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
    /// w: CatEv | dict[str, str] | None
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
    /// CatCPD
    ///     A new conditional probability distribution.
    ///
    #[allow(clippy::too_many_arguments)]
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
    ) -> PyResult<PyCatCPD> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the set of variables.
        let x = indices_from!(x, lock)?;
        let z = indices_from!(z, lock)?;
        // Get the evidence.
        let w = w
            .map(|w| PyCatEv::from_any(w, lock.support()).map(Into::into))
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
    /// w: CatEv | dict[str, str] | None
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
    /// CatCPD | None
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
    ) -> PyResult<Option<PyCatCPD>> {
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the set of variables.
        let x = indices_from!(x, lock)?;
        let y = indices_from!(y, lock)?;
        let z = indices_from!(z, lock)?;
        // Get the evidence.
        let w = w
            .map(|w| PyCatEv::from_any(w, lock.support()).map(Into::into))
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

    /// Generates a random categorical Bayesian network.
    ///
    /// Parameters
    /// ----------
    /// states: dict[str, tuple[str, ...]]
    ///     The states of the variables.
    /// alpha: float, default=1.0
    ///     The parameter of the Dirichlet distribution.
    /// p: float, default=0.1
    ///     The probability of generating an edge.
    /// seed: int, default=31
    ///     The seed for the random number generator.
    ///
    /// Returns
    /// -------
    /// CatBN
    ///     A random categorical Bayesian network.
    ///
    #[classmethod]
    #[pyo3(signature = (
        states,
        alpha = 1.0,
        p = 0.1,
        seed = 31
    ))]
    pub fn random(
        _cls: &Bound<'_, PyType>,
        states: &Bound<'_, PyDict>,
        alpha: f64,
        p: f64,
        seed: u64,
    ) -> PyResult<Self> {
        // Convert the PyDict to a CatSupport.
        let mut inner_states = CatSupport::default();
        for (label, states) in states {
            let label = label.extract::<String>()?;
            let states = states.extract::<Vec<String>>()?;
            inner_states.insert(label, states.into_iter().collect());
        }

        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Create a new RngCatBN and generate a random BN.
        RngCatBN::new(&mut rng, &inner_states, alpha, p)
            .and_then(|mut x| x.random())
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Read class from a BIF string.
    ///
    /// Parameters
    /// ----------
    /// bif: str
    ///     The BIF string to read from.
    ///
    /// Returns
    /// -------
    /// CatBN
    ///     A new Bayesian network instance.
    ///
    #[classmethod]
    pub fn from_bif_string(_cls: &Bound<'_, PyType>, bif: &str) -> PyResult<Self> {
        CatBN::from_bif_string(bif)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write class to a BIF string.
    ///
    /// Returns
    /// -------
    /// str
    ///     A BIF string representation of the model.
    ///
    pub fn to_bif_string(&self) -> PyResult<String> {
        self.lock().to_bif_string().map_err(to_pyerr)
    }

    /// Read class from a BIF file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the BIF file to read from.
    ///
    /// Returns
    /// -------
    /// CatBN
    ///     A new Bayesian network instance.
    ///
    #[classmethod]
    pub fn from_bif_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        CatBN::from_bif_file(path).map(Into::into).map_err(to_pyerr)
    }

    /// Write class to a BIF file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the BIF file to write to.
    ///
    pub fn to_bif_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_bif_file(path).map_err(to_pyerr)
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
    /// CatBN
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        CatBN::from_json_string(json)
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
    /// CatBN
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        CatBN::from_json_file(path)
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
