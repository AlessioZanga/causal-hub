use backend::{
    datasets::{CatTable, GaussTable},
    estimators::{HC, PK},
    models::DiGraph,
    types::{Cache, Error as BackendError},
};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyDict};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::PyDataset,
    dispatch_estimator_method, dispatch_scorer_method,
    error::to_pyerr,
    estimators::{PyEstimatorMethod, PyPK, PyScorerMethod},
    kwarg,
    models::PyDiGraph,
};

/// Perform structure learning using the Hill Climbing (HC) algorithm.
///
/// Parameters
/// ----------
/// dataset: CatTable | GaussTable
///     The complete dataset to learn the structure from.
/// estimator_method: EstimatorMethod | None
///     The parameter estimator to use (default is `EstimatorMethod.BE`).
/// scorer_method: ScorerMethod | None
///     The scorer method to use (default is `ScorerMethod.BIC`).
/// parallel: bool
///     Whether to run the algorithm in parallel (default is `True`).
/// **kwargs: dict | None
///     Optional keyword arguments:
///
/// - `prior_knowledge`: The prior knowledge to constrain the search (default is `None`).
/// - `initial_graph`: The initial graph to start the search from (default is an empty graph).
/// - `max_parents`: The maximum number of parents for each vertex (default is no limit).
/// - `max_iter`: The maximum number of iterations (default is unlimited).
///
/// Returns
/// -------
/// DiGraph
///     The learned structure.
///
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[pyfunction]
#[pyo3(signature = (dataset, estimator_method = PyEstimatorMethod::BE, scorer_method = PyScorerMethod::BIC, parallel = true, **kwargs))]
pub fn hc(
    py: Python<'_>,
    dataset: PyDataset,
    estimator_method: PyEstimatorMethod,
    scorer_method: PyScorerMethod,
    parallel: bool,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyDiGraph> {
    // Get the maximum number of parents from the keyword arguments, if any.
    let max_parents = kwarg!(kwargs, "max_parents", usize)?;
    // Get the maximum number of iterations from the keyword arguments, if any.
    let max_iter = kwarg!(kwargs, "max_iter", usize)?;
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

    // Check the dataset type is supported.
    if !matches!(dataset, PyDataset::Categorical(_) | PyDataset::Gaussian(_)) {
        return Err(PyErr::new::<PyValueError, _>(
            "Expected either a categorical or a Gaussian complete dataset for structure learning.",
        ));
    }

    // Macro to run the HC algorithm on the given table type.
    macro_rules! fit_hc {
        ($type:ty, $dataset:expr) => {{
            // Get the dataset.
            let dataset = <$type>::from($dataset);
            // Dispatch over the estimator method and scoring criterion, and run HC.
            dispatch_estimator_method!(estimator_method, &dataset, |estimator| {
                // Cache the parameter estimator.
                let cache = Cache::new(estimator);
                // Dispatch over the scorer method and run HC.
                dispatch_scorer_method!(scorer_method, &cache, |scorer_method| {
                    // Initialize the HC algorithm.
                    let mut hc = HC::new(scorer_method);
                    // Set the initial graph, if any.
                    if let Some(initial_graph) = initial_graph.as_ref() {
                        hc = hc.with_initial_graph(initial_graph).map_err(to_pyerr)?;
                    }
                    // Set the maximum number of parents, if any.
                    if let Some(max_parents) = max_parents {
                        hc = hc.with_max_parents(max_parents);
                    }
                    // Set the maximum number of iterations, if any.
                    if let Some(max_iter) = max_iter {
                        hc = hc.with_max_iter(max_iter);
                    }
                    // Set the prior knowledge, if any.
                    if let Some(prior_knowledge) = prior_knowledge {
                        hc = hc.with_prior_knowledge(prior_knowledge).map_err(to_pyerr)?;
                    }
                    // Run the algorithm.
                    if parallel {
                        py.detach(move || hc.par_fit())
                    } else {
                        hc.fit()
                    }
                    .map_err(to_pyerr)
                })
            })
        }};
    }

    // Match the dataset type.
    let graph = match dataset {
        PyDataset::Categorical(dataset) => fit_hc!(CatTable, dataset),
        PyDataset::Gaussian(dataset) => fit_hc!(GaussTable, dataset),
        _ => Err(to_pyerr(BackendError::Unreachable(
            "Unsupported dataset type.",
        ))),
    }?;

    // Convert the fitted graph into a Python object.
    Ok(graph.into())
}
