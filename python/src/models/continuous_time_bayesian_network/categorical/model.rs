use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatTrjs, MissingMethod},
    estimators::{BE, MLE},
    io::JsonIO,
    models::{CTBN, CatCTBN, DiGraph, HasLabels},
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
    datasets::{PyCatTrjs, PyMissingMechanism, PyMissingMethod},
    error::to_pyerr,
    estimators::{
        PyCTBNEstimator, PyFitMethod, PyParametersEstimator, PyScorer, PyStructureEstimator, cthc,
        ctpc,
    },
    impl_from_into_lock, kwarg,
    models::{PyCatBN, PyCatCIM, PyDiGraph},
};

/// A continuous-time Bayesian network (CTBN).
///
#[gen_stub_pyclass]
#[pyclass(name = "CatCTBN", module = "causal_hub.models", eq, from_py_object)]
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
        let cims: Vec<PyCatCIM> = cims
            .try_iter()?
            .map(|x| x.and_then(|x| x.extract::<PyCatCIM>().map_err(PyErr::from)))
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyCatCPD> to Vec<CatCIM>.
        let cims = cims.into_iter().map(|x: PyCatCIM| x.into());
        // Create a new CatCTBN with the given parameters.
        CatCTBN::new(graph, cims).map(Into::into).map_err(to_pyerr)
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
            .map(|(label, intensity)| {
                // Convert the label to a string slice.
                let label = label.clone();
                // Convert the CIM to a PyCatCIM.
                let intensity = intensity.clone().into();
                // Return the label and CIM as a tuple.
                (label, intensity)
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

    /// Fit the model to trajectories, either fitting the parameters given the
    /// structure (`FitMethod.Parameters`) or learning the structure from data
    /// and fitting the parameters over it (`FitMethod.Structure`).
    ///
    /// Parameters
    /// ----------
    /// trajectories: CatTrjs
    ///     The trajectories to fit the model to.
    /// graph: DiGraph | None
    ///     The graph to fit the parameters to. It is required by
    ///     `FitMethod.Parameters`, and it is ignored by `FitMethod.Structure`.
    /// fit_method: FitMethod | None
    ///     The fitting method to use, one of `FitMethod.{Parameters,
    ///     Structure}` (default is `FitMethod.Parameters`).
    /// **kwargs: dict | None
    ///     Optional keyword arguments, forwarded to the underlying method:
    ///
    /// - `structure_estimator`: The structure learning algorithm used by
    ///   `FitMethod.Structure`, one of `StructureEstimator.{CTHC, CTPC}`
    ///   (default is `StructureEstimator.CTHC`).
    /// - `parameters_estimator`: The parameter estimator used to fit the local
    ///   models, either `ParametersEstimator.MLE` or `ParametersEstimator.BE`
    ///   (default is `ParametersEstimator.MLE`). Any further keyword
    ///   argument supported by the selected method is forwarded.
    ///
    /// Returns
    /// -------
    /// CatCTBN
    ///     A new fitted model.
    ///
    #[classmethod]
    #[pyo3(signature = (
        trajectories,
        graph = None,
        fit_method = PyFitMethod::Parameters,
        **kwargs
    ))]
    pub fn fit(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        trajectories: &Bound<'_, PyCatTrjs>,
        graph: Option<&Bound<'_, PyDiGraph>>,
        fit_method: PyFitMethod,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Match the fit method.
        match fit_method {
            PyFitMethod::Parameters => {
                // Get the graph, which is required to fit the parameters.
                let Some(graph) = graph else {
                    return Err(PyErr::new::<PyValueError, _>(
                        "A graph is required to fit the model parameters.",
                    ));
                };
                // Get the estimator method from the keyword arguments, or default to the MLE
                // estimator.
                let parameters_estimator = kwarg!(
                    kwargs,
                    "parameters_estimator",
                    PyParametersEstimator,
                    PyParametersEstimator::MLE
                )?;
                // Fit the model parameters over the given graph.
                Self::fit_parameters(_cls, py, trajectories, graph, parameters_estimator, kwargs)
            }
            PyFitMethod::Structure => {
                // Get the structure learning algorithm from the keyword arguments, or default to
                // the CTHC algorithm.
                let structure_estimator = kwarg!(
                    kwargs,
                    "structure_estimator",
                    PyStructureEstimator,
                    PyStructureEstimator::CTHC
                )?;
                // Learn the structure and fit the model over it, ignoring the given graph, if any.
                Self::fit_structure(_cls, py, trajectories, structure_estimator, kwargs)
            }
        }
    }

    /// Fit the model parameters to a dataset and a given graph.
    ///
    /// Parameters
    /// ----------
    /// dataset: CatTrjs
    ///     The dataset to fit the model to.
    /// graph: DiGraph
    ///     The graph to fit the model to.
    /// estimator: ParametersEstimator | None
    ///     The estimator to use for fitting (default is `ParametersEstimator.BE`).
    /// **kwargs: dict | None
    ///     Optional keyword arguments:
    ///
    /// - `alpha`: The prior of the Bayesian estimator (int, float64).
    /// - `missing_method`: The method (`MissingMethod`) used to handle missing
    ///   data (default is `MissingMethod.PW`).
    /// - `missing_mechanism`: The mechanism (`MissingMechanism`) associated to
    ///   the dataset (default is `None`). It is required by
    ///   `MissingMethod.IPW` and `MissingMethod.AIPW`, and it must be `None`
    ///   otherwise.
    /// - `parallel`: The flag to enable parallel fitting (default is `true`).
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
        parameters_estimator = PyParametersEstimator::BE,
        **kwargs
    ))]
    pub fn fit_parameters(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dataset: &Bound<'_, PyCatTrjs>,
        graph: &Bound<'_, PyDiGraph>,
        parameters_estimator: PyParametersEstimator,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Get the dataset and the graph.
        let dataset: CatTrjs = dataset.extract::<PyCatTrjs>()?.into();
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();

        // Get the alpha prior from the keyword arguments, if any.
        let alpha = kwarg!(kwargs, "alpha", (usize, f64))?;

        // Get the missing data handling method from the keyword arguments, or default to the PW
        // missing data handling method.
        let missing_method: MissingMethod = kwarg!(
            kwargs,
            "missing_method",
            PyMissingMethod,
            PyMissingMethod::PW
        )?
        .into();

        // Get the missing data mechanism from the keyword arguments, if any.
        let missing_mechanism: Option<PyMissingMechanism> =
            kwarg!(kwargs, "missing_mechanism", PyMissingMechanism)?;

        // Get the parallel flag from the keyword arguments, or default to parallel execution.
        let parallel = kwarg!(kwargs, "parallel", bool, true)?;
        // Reject any unknown keyword arguments.
        crate::utils::ensure_kwargs_consumed(kwargs)?;

        // Initialize the estimator.
        let estimator: Box<dyn PyCTBNEstimator<CatCTBN>> = match parameters_estimator {
            // Initialize the maximum likelihood estimator.
            PyParametersEstimator::MLE => Box::new(
                MLE::new(&dataset)
                    .with_missing_method(
                        Some(missing_method),
                        missing_mechanism.as_ref().map(|m| (*m.lock()).clone()),
                    )
                    .map_err(to_pyerr)?,
            ),
            // Initialize the Bayesian estimator.
            PyParametersEstimator::BE => {
                // Initialize the Bayesian estimator.
                let estimator = BE::new(&dataset)
                    .with_missing_method(
                        Some(missing_method),
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
    }

    /// Fit the model structure to trajectories using a structure learning
    /// algorithm, either Continuous Time Hill Climbing (`CTHC`) or Continuous
    /// Time Peter-Clark (`CTPC`).
    ///
    /// Parameters
    /// ----------
    /// trajectories: CatTrjs
    ///     The trajectories to learn the structure from.
    /// structure_estimator: StructureEstimator | None
    ///     The structure learning algorithm to use, one of
    ///     `StructureEstimator.{CTHC, CTPC}` (default is
    ///     `StructureEstimator.CTHC`).
    /// **kwargs: dict | None
    ///     Optional keyword arguments, forwarded to the selected algorithm:
    ///
    /// - `c_test`: The significance level of the chi-squared test for the
    ///   initial distributions, used by `CTPC` (default is `0.01`). It must be
    ///   in `[0, 1]`.
    /// - `parameters_estimator`: The parameter estimator used to fit the local
    ///   models, either `ParametersEstimator.MLE` or `ParametersEstimator.BE`
    ///   (default is `ParametersEstimator.BE`).
    /// - `f_test`: The significance level of the F-test for the transition
    ///   rates, used by `CTPC` (default is `0.01`). It must be in `[0, 1]`.
    /// - `initial_graph`: The initial graph (`DiGraph`) to start the search
    ///   from (default is an empty graph for `CTHC`, a complete graph for
    ///   `CTPC`). Its labels must match the trajectories.
    /// - `max_parents`: The maximum number of parents for each vertex, used
    ///   by `CTHC` (default is no limit).
    /// - `missing_method`: The method (`MissingMethod`) used to handle missing
    ///   data, one of `MissingMethod.{LW, PW, IPW, AIPW}` (default is `None`).
    /// - `missing_mechanism`: The missing data mechanism (`MissingMechanism`)
    ///   associated to the trajectories (default is `None`). It is required by
    ///   `MissingMethod.IPW` and `MissingMethod.AIPW`, and it must be `None`
    ///   otherwise.
    /// - `parallel`: Whether to run the algorithm in parallel (default is `True`).
    /// - `prior_knowledge`: The prior knowledge (`PK`) constraining the search,
    ///   e.g., forbidden and required edges or temporal tiers
    ///   (default is `None`).
    /// - `scorer`: The scoring criterion to maximize, used by `CTHC`,
    ///   one of `Scorer.{LL, AIC, AICC, BIC, BICC, HQC}` (default is
    ///   `Scorer.BIC`).
    ///
    /// Returns
    /// -------
    /// CatCTBN
    ///     A new fitted model over the learned structure.
    ///
    #[classmethod]
    #[pyo3(signature = (
        trajectories,
        structure_estimator = PyStructureEstimator::CTHC,
        **kwargs
    ))]
    pub fn fit_structure(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        trajectories: &Bound<'_, PyCatTrjs>,
        structure_estimator: PyStructureEstimator,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Match the structure learning algorithm.
        match structure_estimator {
            PyStructureEstimator::CTHC => {
                // Get the scoring criterion from the keyword arguments, or default to the BIC.
                let scorer = kwarg!(kwargs, "scorer", PyScorer, PyScorer::BIC)?;
                // Learn the structure and fit the model using CTHC.
                cthc(py, trajectories, scorer, kwargs)
            }
            PyStructureEstimator::CTPC => {
                // Get the significance level of the chi-squared test from the keyword arguments,
                // or default to `0.01`.
                let c_test = kwarg!(kwargs, "c_test", f64, 0.01)?;

                // Get the significance level of the F-test from the keyword arguments, or
                // default to `0.01`.
                let f_test = kwarg!(kwargs, "f_test", f64, 0.01)?;
                // Learn the structure and fit the model using CTPC.
                ctpc(py, trajectories, f_test, c_test, kwargs)
            }
        }
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
    /// **kwargs: dict | None
    ///     Optional keyword arguments:
    ///
    /// - `parallel`: The flag to enable parallel sampling (default is `true`).
    /// - `seed`: The seed of the random number generator (default is `31`).
    ///
    /// Returns
    /// -------
    /// CatTrjs
    ///     A new dataset containing the sampled trajectories.
    ///
    #[pyo3(signature = (
        n,
        max_len = None,
        max_time = None,
        **kwargs
    ))]
    pub fn sample(
        &self,
        py: Python<'_>,
        n: usize,
        max_len: Option<usize>,
        max_time: Option<f64>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyCatTrjs> {
        // Check at least one of max_len or max_time is set.
        if max_len.is_none() && max_time.is_none() {
            return Err(PyErr::new::<PyValueError, _>(
                "At least one of 'max_len' or 'max_time' must be set.",
            ));
        }
        // Get the parallel flag from the keyword arguments, or default to parallel execution.
        let parallel = kwarg!(kwargs, "parallel", bool, true)?;

        // Get the seed from the keyword arguments, or default to `31`.
        let seed = kwarg!(kwargs, "seed", u64, 31)?;
        // Reject any unknown keyword arguments.
        crate::utils::ensure_kwargs_consumed(kwargs)?;

        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Get a lock on the inner field.
        let lock = self.lock();
        // Initialize the sampler.
        let sampler = ForwardSampler::new(&mut rng, &*lock).map_err(to_pyerr)?;
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
        }
        .map_err(to_pyerr)?;
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
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        CatCTBN::from_json_string(json)
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
    /// CatCTBN
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        CatCTBN::from_json_file(path)
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
