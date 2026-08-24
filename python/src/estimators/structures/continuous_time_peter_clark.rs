use backend::{
    datasets::CatTrjs,
    estimators::{CTPC, ChiSquaredTest, FTest, PK},
    models::{CatCTBN, DiGraph},
    types::Cache,
};
use pyo3::{prelude::*, types::PyDict};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::{PyCatTrjs, PyMissingMechanism, PyMissingMethod},
    dispatch_parameters_estimator,
    error::to_pyerr,
    estimators::{PyPK, PyParametersEstimator},
    kwarg,
    models::{PyCatCTBN, PyDiGraph},
};

/// Perform structure learning using the Continuous Time Peter-Clark (CTPC) algorithm.
///
/// CTPC learns the structure of a Continuous Time Bayesian Network (CTBN)
/// from trajectories. It is a constraint-based algorithm: it starts from a
/// complete graph and removes edges whose end-points are conditionally
/// independent given suitable separating sets, as assessed by the
/// significance tests below.
///
/// Parameters
/// ----------
/// trajectories: CatTrjs
///     The trajectories to learn the structure from.
/// f_test: float | None
///     The significance level of the F-test for the transition rates
///     (default is `0.01`). It must be in `[0, 1]`.
/// c_test: float | None
///     The significance level of the chi-squared test for the initial
///     distributions (default is `0.01`). It must be in `[0, 1]`.
/// **kwargs: dict | None
///     Optional keyword arguments:
///
/// - `parameters_estimator`: The parameter estimator used to fit the local
///   models, either `ParametersEstimator.MLE` or `ParametersEstimator.BE`
///   (default is `ParametersEstimator.BE`).
/// - `initial_graph`: The initial graph (`DiGraph`) to start the search
///   from (default is a complete graph). Its labels must match the
///   trajectories.
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
///
/// Returns
/// -------
/// CatCTBN
///     The fitted model over the learned structure.
///
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[pyfunction]
#[pyo3(signature = (trajectories, f_test = 0.01, c_test = 0.01, **kwargs))]
pub fn ctpc(
    py: Python<'_>,
    trajectories: &Bound<'_, PyCatTrjs>,
    f_test: f64,
    c_test: f64,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyCatCTBN> {
    // Get the trajectories.
    let trajectories: PyCatTrjs = trajectories.extract()?;
    // Get the reference to the trajectories.
    let trajectories: &CatTrjs = &trajectories.lock();

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

    // Dispatch over the estimator method and run the CTPC algorithm.
    dispatch_parameters_estimator!(
        trajectories,
        parameters_estimator,
        missing_method,
        missing_mechanism,
        |estimator| {
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
            // Run the algorithm and fit the model over the learned structure.
            let model: CatCTBN = if parallel {
                py.detach(move || ctpc.par_fit())
            } else {
                ctpc.fit()
            }
            .map_err(to_pyerr)?;
            // Convert the fitted model into a Python object.
            Ok(model.into())
        }
    )
}
