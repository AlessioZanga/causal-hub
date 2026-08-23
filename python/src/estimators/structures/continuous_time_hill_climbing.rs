use backend::{
    datasets::CatTrjs,
    estimators::{CTHC, PK},
    models::DiGraph,
    types::Cache,
};
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::PyCatTrjs,
    dispatch_estimator_method, dispatch_scorer_method,
    error::to_pyerr,
    estimators::{PyEstimatorMethod, PyPK, PyScorerMethod},
    kwarg,
    models::PyDiGraph,
};

/// A function to perform structure learning using the Continuous Time Hill Climbing (CTHC) algorithm.
///
/// The scorer method can be selected through the `scorer_method` argument
/// (one of `ScorerMethod.{LL, AIC, AICC, BIC, BICC, HQC}`), defaulting to `BIC`.
///
/// **kwargs: dict | None
///     Optional keyword arguments:
///
/// - `estimator`: The parameter estimator to use (default is `EstimatorMethod.BE`).
/// - `prior_knowledge`: The prior knowledge to constrain the search (default is `None`).
/// - `initial_graph`: The initial graph to start the search from (default is an empty graph).
/// - `max_parents`: The maximum number of parents for each vertex (default is no limit).
///
/// parallel: bool
///     Whether to run the algorithm in parallel (default is `True`).
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[pyfunction]
#[pyo3(signature = (trajectories, estimator_method = PyEstimatorMethod::BE, scorer_method = PyScorerMethod::BIC, parallel = true, **kwargs))]
pub fn cthc(
    py: Python<'_>,
    trajectories: &Bound<'_, PyCatTrjs>,
    estimator_method: PyEstimatorMethod,
    scorer_method: PyScorerMethod,
    parallel: bool,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyDiGraph> {
    // Get the trajectories.
    let trajectories: PyCatTrjs = trajectories.extract()?;
    // Get the reference to the trajectories.
    let trajectories: &CatTrjs = &trajectories.lock();

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

    // Dispatch over the estimator method and scoring criterion, and run the CTHC algorithm.
    let graph = dispatch_estimator_method!(estimator_method, trajectories, |estimator| {
        // Cache the parameter estimator.
        let cache = Cache::new(estimator);
        dispatch_scorer_method!(scorer_method, &cache, |scorer_method| {
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
            // Run the algorithm.
            if parallel {
                py.detach(move || cthc.par_fit())
            } else {
                cthc.fit()
            }
            .map_err(to_pyerr)
        })
    })?;

    // Convert the fitted graph into a Python object.
    Ok(graph.into())
}
