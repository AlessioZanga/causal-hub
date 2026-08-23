use backend::{
    datasets::{MissingMechanism, MissingMethod},
    estimators::{HC, PK},
    models::DiGraph,
    types::Cache,
};
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::{PyDataset, PyMissingMechanism, PyMissingMethod},
    dispatch_estimator_method, dispatch_scorer_method,
    error::to_pyerr,
    estimators::{PyEstimatorMethod, PyPK, PyScorerMethod},
    kwarg,
    models::PyDiGraph,
};

/// Perform structure learning using the Hill Climbing (HC) algorithm.
///
/// HC explores the space of directed acyclic graphs (DAGs) greedily: at each
/// iteration it evaluates all single-edge additions, deletions and reversals,
/// and applies the move that most increases the score of the model until no
/// improving move is found.
///
/// Parameters
/// ----------
/// dataset: CatTable | CatIncTable | CatWtdTable | GaussTable | GaussIncTable | GaussWtdTable
///     The dataset to learn the structure from.
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
///   from (default is an empty graph). Its labels must match the dataset.
/// - `max_parents`: The maximum number of parents for each vertex
///   (default is no limit).
/// - `max_iter`: The maximum number of iterations of the search
///   (default is unlimited).
/// - `missing_method`: The method (`MissingMethod`) used to handle missing
///   data, one of `MissingMethod.{LW, PW, IPW, AIPW}` (default is `None`).
/// - `missing_mechanism`: The missing data mechanism (`MissingMechanism`)
///   associated to the dataset (default is `None`). It is required by
///   `MissingMethod.IPW` and `MissingMethod.AIPW`, and it must be `None`
///   otherwise.
///
/// Returns
/// -------
/// DiGraph
///     The learned structure.
///
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[pyfunction]
#[pyo3(signature = (dataset, scorer_method = PyScorerMethod::BIC, parallel = true, **kwargs))]
pub fn hc(
    py: Python<'_>,
    dataset: PyDataset,
    scorer_method: PyScorerMethod,
    parallel: bool,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyDiGraph> {
    // Get the estimator method from the keyword arguments, if any.
    let estimator_method: Option<_> = kwarg!(kwargs, "estimator_method", PyEstimatorMethod)?;
    // Default to the BE estimator.
    let estimator_method = estimator_method.unwrap_or(PyEstimatorMethod::BE);
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

    // Macro to run the HC algorithm on the given table type.
    macro_rules! fit_hc {
        ($dataset:expr) => {{
            // Get a read lock on the table.
            let dataset = $dataset.lock();
            // Dispatch over the estimator method and scoring criterion, and run HC.
            dispatch_estimator_method!(
                &*dataset,
                estimator_method,
                missing_method,
                missing_mechanism,
                |estimator| {
                    // Cache the parameter estimator.
                    let cache = Cache::new(estimator);
                    // Dispatch over the scorer method and run HC.
                    dispatch_scorer_method!(&cache, scorer_method, |scorer_method| {
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
                }
            )
        }};
    }

    // Match the dataset type.
    let graph = match dataset {
        PyDataset::Categorical(dataset) => fit_hc!(dataset),
        PyDataset::CategoricalIncomplete(dataset) => fit_hc!(dataset),
        PyDataset::CategoricalWeighted(dataset) => fit_hc!(dataset),
        PyDataset::Gaussian(dataset) => fit_hc!(dataset),
        PyDataset::GaussianIncomplete(dataset) => fit_hc!(dataset),
        PyDataset::GaussianWeighted(dataset) => fit_hc!(dataset),
    }?;

    // Convert the fitted graph into a Python object.
    Ok(graph.into())
}
