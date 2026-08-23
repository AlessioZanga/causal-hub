use backend::{
    datasets::CatTrjs,
    estimators::{CTPC, ChiSquaredTest, FTest, PK},
    models::DiGraph,
    types::Cache,
};
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::PyCatTrjs,
    dispatch_estimator_method,
    error::to_pyerr,
    estimators::{PyEstimatorMethod, PyPK},
    kwarg,
    models::PyDiGraph,
};

/// A function to perform structure learning using the Continuous Time Peter-Clark (CTPC) algorithm.
///
/// **kwargs: dict | None
///     Optional keyword arguments:
///
/// - `estimator`: The parameter estimator to use (default is `EstimatorMethod.BE`).
/// - `prior_knowledge`: The prior knowledge to constrain the search (default is `None`).
/// - `initial_graph`: The initial graph to start the search from (default is a complete graph).
///
/// parallel: bool
///     Whether to run the algorithm in parallel (default is `True`).
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[pyfunction]
#[pyo3(signature = (trajectories, f_test = 0.01, c_test = 0.01, estimator_method = PyEstimatorMethod::BE, parallel = true, **kwargs))]
pub fn ctpc(
    py: Python<'_>,
    trajectories: &Bound<'_, PyCatTrjs>,
    f_test: f64,
    c_test: f64,
    estimator_method: PyEstimatorMethod,
    parallel: bool,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyDiGraph> {
    // Get the trajectories.
    let trajectories: PyCatTrjs = trajectories.extract()?;
    // Get the reference to the trajectories.
    let trajectories: &CatTrjs = &trajectories.lock();

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
    // Reject any unknown keyword arguments.
    crate::utils::ensure_kwargs_consumed(kwargs)?;

    // Dispatch over the estimator method and run the CTPC algorithm.
    let graph = dispatch_estimator_method!(estimator_method, trajectories, |estimator| {
        // Cache the parameter estimator.
        let cache = Cache::new(estimator);
        // Initialize the F test, shadowing the alpha value.
        let f_test = FTest::new(&cache, f_test).map_err(to_pyerr)?;
        // Initialize the chi-squared test, shadowing the alpha value.
        let chi_sq_test = ChiSquaredTest::new(&cache, c_test).map_err(to_pyerr)?;

        // Initialize the CTPC algorithm.
        let mut ctpc = CTPC::new(&f_test, &chi_sq_test).map_err(to_pyerr)?;
        // Set the initial graph, if any.
        if let Some(initial_graph) = initial_graph.as_ref() {
            ctpc = ctpc.with_initial_graph(initial_graph).map_err(to_pyerr)?;
        }
        // Set the prior knowledge, if any.
        if let Some(prior_knowledge) = prior_knowledge {
            ctpc = ctpc
                .with_prior_knowledge(prior_knowledge)
                .map_err(to_pyerr)?;
        }
        // Run the algorithm.
        if parallel {
            py.detach(move || ctpc.par_fit())
        } else {
            ctpc.fit()
        }
        .map_err(to_pyerr)
    })?;

    // Convert the fitted graph into a Python object.
    Ok(graph.into())
}
