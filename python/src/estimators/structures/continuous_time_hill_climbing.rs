use backend::{
    datasets::{CatTrjs, MissingMechanism, MissingMethod},
    estimators::{CTHC, PK},
    models::{CatCTBN, DiGraph},
    types::Cache,
};
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::{PyCatTrjs, PyMissingMechanism, PyMissingMethod},
    dispatch_estimator_method, dispatch_scorer_method,
    error::to_pyerr,
    estimators::{PyEstimatorMethod, PyPK, PyScorerMethod},
    kwarg,
    models::{PyCatCTBN, PyDiGraph},
};

/// Perform structure learning using the Continuous Time Hill Climbing (CTHC) algorithm.
///
/// CTHC learns the structure of a Continuous Time Bayesian Network (CTBN)
/// from trajectories. It explores the space of directed graphs greedily: at
/// each iteration it evaluates all single-edge additions, deletions and
/// reversals, and applies the move that most increases the score of the
/// model until no improving move is found.
///
/// Parameters
/// ----------
/// trajectories: CatTrjs
///     The trajectories to learn the structure from.
/// scorer_method: ScorerMethod | None
///     The scoring criterion to maximize, one of `ScorerMethod.{LL, AIC,
///     AICC, BIC, BICC, HQC}` (default is `ScorerMethod.BIC`).
/// parallel: bool
///     Whether to run the algorithm in parallel (default is `True`).
/// **kwargs: dict | None
///     Optional keyword arguments:
///
/// - `estimator_method`: The parameter estimator used to fit the local
///   models, either `EstimatorMethod.MLE` or `EstimatorMethod.BE`
///   (default is `EstimatorMethod.BE`).
/// - `prior_knowledge`: The prior knowledge (`PK`) constraining the search,
///   e.g., forbidden and required edges or temporal tiers
///   (default is `None`).
/// - `initial_graph`: The initial graph (`DiGraph`) to start the search
///   from (default is an empty graph). Its labels must match the
///   trajectories.
/// - `max_parents`: The maximum number of parents for each vertex
///   (default is no limit).
/// - `missing_method`: The method (`MissingMethod`) used to handle missing
///   data, one of `MissingMethod.{LW, PW, IPW, AIPW}` (default is `None`).
/// - `missing_mechanism`: The missing data mechanism (`MissingMechanism`)
///   associated to the trajectories (default is `None`). It is required by
///   `MissingMethod.IPW` and `MissingMethod.AIPW`, and it must be `None`
///   otherwise.
///
/// Returns
/// -------
/// CatCTBN
///     The fitted model over the learned structure.
///
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[pyfunction]
#[pyo3(signature = (trajectories, scorer_method = PyScorerMethod::BIC, parallel = true, **kwargs))]
pub fn cthc(
    py: Python<'_>,
    trajectories: &Bound<'_, PyCatTrjs>,
    scorer_method: PyScorerMethod,
    parallel: bool,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyCatCTBN> {
    // Get the trajectories.
    let trajectories: PyCatTrjs = trajectories.extract()?;
    // Get the reference to the trajectories.
    let trajectories: &CatTrjs = &trajectories.lock();

    // Get the estimator method from the keyword arguments, if any.
    let estimator_method: Option<_> = kwarg!(kwargs, "estimator_method", PyEstimatorMethod)?;
    // Default to the BE estimator.
    let estimator_method = estimator_method.unwrap_or(PyEstimatorMethod::BE);
    // Get the maximum number of parents from the keyword arguments, if any.
    let max_parents = kwarg!(kwargs, "max_parents", usize)?;
    // Get the prior knowledge from the keyword arguments, if any.
    let prior_knowledge: Option<PyPK> = kwarg!(kwargs, "prior_knowledge", PyPK)?;
    // Lock the prior knowledge, if any.
    let prior_knowledge_locks = prior_knowledge.as_ref().map(|x| x.lock());
    // Get the reference to the prior knowledge, if any.
    let prior_knowledge: Option<&PK> = prior_knowledge_locks.as_deref();
    // Get the initial graph from the keyword arguments, if any.
    let initial_graph: Option<PyDiGraph> = kwarg!(kwargs, "initial_graph", PyDiGraph)?;
    // Lock the initial graph, if any.
    let initial_graph_locks = initial_graph.as_ref().map(|x| x.lock());
    // Get the reference to the initial graph, if any.
    let initial_graph: Option<&DiGraph> = initial_graph_locks.as_deref();
    // Get the missing data handling method from the keyword arguments, if any.
    let missing_method: Option<PyMissingMethod> =
        kwarg!(kwargs, "missing_method", PyMissingMethod)?;
    let missing_method: Option<MissingMethod> = missing_method.map(Into::into);
    // Get the missing data mechanism from the keyword arguments, if any.
    let missing_mechanism: Option<PyMissingMechanism> =
        kwarg!(kwargs, "missing_mechanism", PyMissingMechanism)?;
    let missing_mechanism: Option<MissingMechanism> = missing_mechanism.map(Into::into);
    // Reject any unknown keyword arguments.
    crate::utils::ensure_kwargs_consumed(kwargs)?;

    // Dispatch over the estimator method and scoring criterion, and run the CTHC algorithm.
    dispatch_estimator_method!(
        trajectories,
        estimator_method,
        missing_method,
        missing_mechanism,
        |estimator| {
            // Cache the parameter estimator.
            let cache = Cache::new(estimator);
            // Dispatch over the scoring criterion and run the CTHC algorithm.
            dispatch_scorer_method!(&cache, scorer_method, |scorer_method| {
                // Initialize the CTHC algorithm.
                let mut cthc = CTHC::new(scorer_method);
                // Set the initial graph, if any.
                if let Some(initial_graph) = initial_graph.as_ref() {
                    cthc = cthc.with_initial_graph(initial_graph).map_err(to_pyerr)?;
                }
                // Set the maximum number of parents, if any.
                if let Some(max_parents) = max_parents {
                    cthc = cthc.with_max_parents(max_parents);
                }
                // Set the prior knowledge, if any.
                if let Some(prior_knowledge) = prior_knowledge {
                    cthc = cthc
                        .with_prior_knowledge(prior_knowledge)
                        .map_err(to_pyerr)?;
                }
                // Run the algorithm and fit the model over the learned structure.
                let model: CatCTBN = if parallel {
                    py.detach(move || cthc.par_fit())
                } else {
                    cthc.fit()
                }
                .map_err(to_pyerr)?;
                // Convert the fitted model into a Python object.
                Ok(model.into())
            })
        }
    )
}
