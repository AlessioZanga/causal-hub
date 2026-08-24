use backend::{
    estimators::{HC, PK},
    models::{CatBN, DiGraph, GaussBN},
    types::Cache,
};
use pyo3::{IntoPyObjectExt, prelude::*, types::PyDict};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::{PyDataset, PyMissingMechanism, PyMissingMethod},
    dispatch_parameters_estimator, dispatch_scorer,
    error::to_pyerr,
    estimators::{PyPK, PyParametersEstimator, PyScorer},
    kwarg,
    models::{PyCatBN, PyDiGraph, PyGaussBN},
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
/// scorer: Scorer | None
///     The scoring criterion to maximize, one of `Scorer.{LL, AIC,
///     AICC, BIC, BICC, HQC}` (default is `Scorer.BIC`).
/// **kwargs: dict | None
///     Optional keyword arguments:
///
/// - `parameters_estimator`: The parameter estimator used to fit the local
///   models, either `ParametersEstimator.MLE` or `ParametersEstimator.BE`
///   (default is `ParametersEstimator.BE`).
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
/// - `parallel`: Whether to run the algorithm in parallel (default is `True`).
/// - `prior_knowledge`: The prior knowledge (`PK`) constraining the search,
///   e.g., forbidden and required edges or temporal tiers
///   (default is `None`).
///
/// Returns
/// -------
/// CatBN | GaussBN
///     The fitted model over the learned structure: `CatBN` for categorical
///     datasets, `GaussBN` for Gaussian ones.
///
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[gen_stub(override_return_type(
    type_repr = "typing.Union[models.CatBN, models.GaussBN]",
    imports = ("typing")
))]
#[pyfunction]
#[pyo3(signature = (dataset, scorer = PyScorer::BIC, **kwargs))]
pub fn hc(
    py: Python<'_>,
    dataset: PyDataset,
    scorer: PyScorer,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    // Get the estimator method from the keyword arguments, or default to the BE estimator.
    let parameters_estimator = kwarg!(
        kwargs,
        "parameters_estimator",
        PyParametersEstimator,
        PyParametersEstimator::BE
    )?;

    // Get the initial graph from the keyword arguments, if any.
    let initial_graph: Option<PyDiGraph> = kwarg!(kwargs, "initial_graph", PyDiGraph)?;
    // Lock the initial graph, if any.
    let initial_graph_locks = initial_graph.as_ref().map(|x| x.lock());
    // Get the reference to the initial graph, if any.
    let initial_graph: Option<&DiGraph> = initial_graph_locks.as_deref();

    // Get the maximum number of iterations from the keyword arguments, if any.
    let max_iter = kwarg!(kwargs, "max_iter", usize)?;

    // Get the maximum number of parents from the keyword arguments, if any.
    let max_parents = kwarg!(kwargs, "max_parents", usize)?;

    // Get the missing data handling method from the keyword arguments, if any.
    let missing_method = kwarg!(kwargs, "missing_method", PyMissingMethod)?.map(Into::into);

    // Get the missing data mechanism from the keyword arguments, if any.
    let missing_mechanism =
        kwarg!(kwargs, "missing_mechanism", PyMissingMechanism)?.map(Into::into);

    // Get the parallel flag from the keyword arguments, or default to parallel execution.
    let parallel = kwarg!(kwargs, "parallel", bool, true)?;

    // Get the prior knowledge from the keyword arguments, if any.
    let prior_knowledge: Option<PyPK> = kwarg!(kwargs, "prior_knowledge", PyPK)?;
    // Lock the prior knowledge, if any.
    let prior_knowledge_locks = prior_knowledge.as_ref().map(|x| x.lock());
    // Get the reference to the prior knowledge, if any.
    let prior_knowledge: Option<&PK> = prior_knowledge_locks.as_deref();

    // Reject any unknown keyword arguments.
    crate::utils::ensure_kwargs_consumed(kwargs)?;

    // Macro to run the HC algorithm on the given table type.
    macro_rules! fit {
        ($dataset:expr, $model:ty, $pymodel:ty) => {{
            // Get a read lock on the table.
            let dataset = $dataset.lock();
            // Dispatch over the estimator method and scoring criterion, and run HC.
            dispatch_parameters_estimator!(
                &*dataset,
                parameters_estimator,
                missing_method,
                missing_mechanism,
                |estimator| {
                    // Cache the parameter estimator.
                    let cache = Cache::new(estimator);
                    // Dispatch over the scorer method and run HC.
                    dispatch_scorer!(&cache, scorer, |scorer| {
                        // Initialize the HC algorithm.
                        let mut hc = HC::new(scorer);
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
                        // Run the algorithm and fit the model over the learned structure.
                        let model: $model = if parallel {
                            py.detach(move || hc.par_fit())
                        } else {
                            hc.fit()
                        }
                        .map_err(to_pyerr)?;
                        // Convert the fitted model into a Python object.
                        let pymodel: $pymodel = model.into();
                        pymodel.into_py_any(py)
                    })
                }
            )
        }};
    }

    // Match the dataset type.
    match dataset {
        PyDataset::Categorical(dataset) => fit!(dataset, CatBN, PyCatBN),
        PyDataset::CategoricalIncomplete(dataset) => fit!(dataset, CatBN, PyCatBN),
        PyDataset::CategoricalWeighted(dataset) => fit!(dataset, CatBN, PyCatBN),
        PyDataset::Gaussian(dataset) => fit!(dataset, GaussBN, PyGaussBN),
        PyDataset::GaussianIncomplete(dataset) => fit!(dataset, GaussBN, PyGaussBN),
        PyDataset::GaussianWeighted(dataset) => fit!(dataset, GaussBN, PyGaussBN),
    }
}
