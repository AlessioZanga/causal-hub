use std::sync::Mutex;

use approx::relative_eq;
use backend::{
    datasets::{CatTrjs, CatTrjsEv, CatWtdTrjs, Dataset},
    estimators::{
        BE, BIC, CTBNEstimator, CTHC, CTPC, ChiSquaredTest, EMBuilder, EMOutput, FTest, PK,
        ParCTBNEstimator, RAWE,
    },
    models::{CTBN, CatCTBN, DiGraph, Graph, HasLabels},
    samplers::{CTBNSampler, ImportanceSampler, ParCTBNSampler},
    types::{Cache, Error as BackendError, Result},
};
use log::debug;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use pyo3_stub_gen::derive::*;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

use crate::{
    datasets::{PyCatTrjsEv, PyCatWtdTrjs},
    error::to_pyerr,
    estimators::{PyPK, PyStructureEstimator},
    kwarg,
    models::{PyCatCTBN, PyDiGraph},
};

/// A function to perform structure learning using the Structural Expectation Maximization (SEM) algorithm.
///
/// Parameters
/// ----------
/// evidence: CatTrjsEv
///     The evidence to learn the structure from.
/// structure_estimator: StructureEstimator | None
///     The structure learning algorithm to use, one of
///     `StructureEstimator.{CTHC, CTPC}` (default is
///     `StructureEstimator.CTHC`).
///
/// **kwargs: dict | None
///     Optional keyword arguments:
///
/// - `parallel`: Whether to run the algorithm in parallel (default is `True`).
/// - `seed`: The seed of the random number generator (default is `42`).
#[gen_stub_pyfunction(module = "causal_hub.estimators")]
#[pyfunction]
#[pyo3(signature = (
    evidence,
    structure_estimator = PyStructureEstimator::CTHC,
    max_iter = 10,
    f_test = 0.01,
    c_test = 0.01,
    **kwargs
))]
pub fn sem<'a>(
    py: Python<'a>,
    evidence: &Bound<'_, PyCatTrjsEv>,
    structure_estimator: PyStructureEstimator,
    max_iter: usize,
    f_test: f64,
    c_test: f64,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'a, PyDict>> {
    // Get the evidence.
    let evidence: PyCatTrjsEv = evidence.extract()?;
    // Get the reference to the evidence.
    let evidence: &CatTrjsEv = &evidence.lock();

    // Get the initial graph from the keyword arguments, if any.
    let initial_graph: Option<PyDiGraph> = kwarg!(kwargs, "initial_graph", PyDiGraph)?;
    // Lock the initial graph, if any.
    let initial_graph_locks = initial_graph.as_ref().map(|x| x.lock());
    // Get the reference to the initial graph, if any.
    let initial_graph: Option<&DiGraph> = initial_graph_locks.as_deref();

    // Get the maximum number of parents from the keyword arguments, if any.
    let max_parents = kwarg!(kwargs, "max_parents", usize)?;

    // Get the parallel flag from the keyword arguments, or default to parallel execution.
    let parallel = kwarg!(kwargs, "parallel", bool, true)?;

    // Get the prior knowledge from the keyword arguments, if any.
    let prior_knowledge: Option<PyPK> = kwarg!(kwargs, "prior_knowledge", PyPK)?;
    // Lock the prior knowledge, if any.
    let prior_knowledge_locks = prior_knowledge.as_ref().map(|x| x.lock());
    // Get the reference to the prior knowledge, if any.
    let prior_knowledge: Option<&PK> = prior_knowledge_locks.as_deref();

    // Get the seed from the keyword arguments, or default to `42`.
    let seed = kwarg!(kwargs, "seed", u64, 42)?;

    // Reject any unknown keyword arguments.
    crate::utils::ensure_kwargs_consumed(kwargs)?;

    // Initialize the random number generator.
    let rng = Xoshiro256PlusPlus::seed_from_u64(seed);

    // Log the raw estimator initialization.
    debug!("Initializing the raw estimator for the initial guess ...");
    // Initialize a raw estimator for an initial guess.
    let raw = RAWE::<CatTrjs>::par_new(evidence).map_err(to_pyerr)?;
    // Log the initial model fitting.
    debug!("Fitting the initial model using the raw estimator ...");
    // Set the graph for the initial model guess, defaulting to an empty graph.
    let guess_graph = match initial_graph {
        Some(initial_graph) => initial_graph.clone(),
        None => DiGraph::empty(evidence.labels()).map_err(to_pyerr)?,
    };
    // Set the initial model.
    let initial_model = if parallel {
        py.detach(move || raw.par_fit(guess_graph))
    } else {
        raw.fit(guess_graph)
    }
    .map_err(to_pyerr)?;

    // Wrap the random number generator in a Mutex to allow mutable borrowing.
    let rng = Mutex::new(rng);

    // Define the expectation-maximization step.
    let em_step = |prev_model: &CatCTBN,
                   evidence: &CatTrjsEv|
     -> Result<EMOutput<CatCTBN, CatWtdTrjs>> {
        // Define the expectation step.
        let e_step = |prev_model: &CatCTBN, evidence: &CatTrjsEv| -> Result<CatWtdTrjs> {
            // Lock the random number generator.
            let mut rng = rng.lock().unwrap_or_else(|e| e.into_inner());
            // Get the maximum length of the trajectories.
            let max_length = evidence
                .evidences()
                .iter()
                .flat_map(|e| e.evidences())
                .map(|e| e.len())
                .max()
                .unwrap_or(0);
            // Sample the seeds to parallelize the sampling.
            let seeds: Vec<_> = (0..evidence.evidences().len())
                .map(|_| rng.next_u64())
                .collect();
            // Macro imputing the missing trajectories over the given zipped iterator.
            macro_rules! impute {
                ($zip:expr) => {{
                    $zip.map(|(s, e)| {
                        // Initialize a new random number generator.
                        let mut rng = Xoshiro256PlusPlus::seed_from_u64(s);
                        // Initialize a new sampler.
                        let importance = ImportanceSampler::new(&mut rng, prev_model, e)?;
                        // Perform multiple imputation.
                        let trjs = if parallel {
                            importance.par_sample_n_by_length(max_length, 10)?
                        } else {
                            importance.sample_n_by_length(max_length, 10)?
                        };
                        // Get the one with the highest weight.
                        trjs.values()
                            .iter()
                            .max_by(|a, b| {
                                a.weight()
                                    .partial_cmp(&b.weight())
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            })
                            .cloned()
                            .ok_or_else(|| BackendError::MissingData("No trajectories sampled"))
                    })
                    .collect()
                }};
            }
            // For each (seed, evidence) ...
            if parallel {
                impute!(seeds.into_par_iter().zip(evidence.par_iter()))
            } else {
                impute!(seeds.into_iter().zip(evidence.into_iter()))
            }
        };

        // Define the maximization step.
        let m_step = |prev_model: &CatCTBN, expectation: &CatWtdTrjs| -> Result<CatCTBN> {
            // Initialize the parameter estimator.
            let estimator = BE::new(expectation).with_prior((1, 1.));
            // Fit the model using the parameter estimator.
            if parallel {
                estimator.par_fit(prev_model.graph().clone())
            } else {
                estimator.fit(prev_model.graph().clone())
            }
        };

        // Define the stopping criteria.
        let stop = |prev_model: &CatCTBN, curr_model: &CatCTBN, counter: usize| -> Result<bool> {
            // Check if the models are equal or the counter is greater than the limit.
            Ok(relative_eq!(prev_model, curr_model, epsilon = 5e-2) || counter >= max_iter)
        };

        // Create a new EM.
        let em = EMBuilder::new(prev_model, evidence)
            .with_e_step(&e_step)
            .with_m_step(&m_step)
            .with_stop(&stop)
            .build()?;

        // Fit the model.
        em.fit()
    };

    // Define the structure learning step.
    let sl_step = |_prev_model: &CatCTBN, em: &EMOutput<CatCTBN, CatWtdTrjs>| -> Result<CatCTBN> {
        // Initialize the parameter estimator.
        let estimator = BE::new(
            em.expectations
                .last()
                .ok_or_else(|| BackendError::MissingData("No expectations in EM output"))?,
        )
        .with_prior((1, 1.));
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Learn the structure and fit the new model using the expectation.
        match structure_estimator {
            PyStructureEstimator::CTPC => {
                // Initialize the F test, shadowing the alpha value.
                let f_test = FTest::new(&cache, f_test)?;
                // Initialize the chi-squared test, shadowing the alpha value.
                let chi_sq_test = ChiSquaredTest::new(&cache, c_test)?;
                // Initialize the CTPC algorithm.
                let mut ctpc = CTPC::new(&f_test, &chi_sq_test)?;
                // Set the initial graph, if any.
                if let Some(initial_graph) = initial_graph.as_ref() {
                    ctpc = ctpc.with_initial_graph(initial_graph)?;
                }
                // Set the prior knowledge, if any.
                if let Some(prior_knowledge) = prior_knowledge {
                    ctpc = ctpc.with_prior_knowledge(prior_knowledge)?;
                }
                // Fit the new structure using CTPC.
                if parallel { ctpc.par_fit() } else { ctpc.fit() }
            }
            PyStructureEstimator::CTHC => {
                // Initialize the scoring criterion.
                let bic = BIC::new(&cache);
                // Initialize the CTHC algorithm.
                let mut cthc = CTHC::new(&bic);
                // Set the initial graph, if any.
                if let Some(initial_graph) = initial_graph.as_ref() {
                    cthc = cthc.with_initial_graph(initial_graph)?;
                }
                // Set the maximum number of parents, if any.
                if let Some(max_parents) = max_parents {
                    cthc = cthc.with_max_parents(max_parents);
                }
                // Set the prior knowledge, if any.
                if let Some(prior_knowledge) = prior_knowledge {
                    cthc = cthc.with_prior_knowledge(prior_knowledge)?;
                }
                // Fit the new structure using CTHC.
                if parallel { cthc.par_fit() } else { cthc.fit() }
            }
        }
    };

    // Define the stopping criteria.
    let sem_stop = |prev_model: &CatCTBN, curr_model: &CatCTBN, counter: usize| -> Result<bool> {
        // Check if the models are equal or the counter is greater than the limit.
        Ok(relative_eq!(prev_model, curr_model, epsilon = 5e-2) || counter >= max_iter)
    };

    // Create a new SEM.
    let sem = EMBuilder::new(&initial_model, evidence)
        .with_e_step(&em_step)
        .with_m_step(&sl_step)
        .with_stop(&sem_stop)
        .build()
        .map_err(to_pyerr)?;

    // Run the algorithm in parallel (releasing the GIL) or sequentially.
    let output = if parallel {
        py.detach(move || sem.fit())
    } else {
        sem.fit()
    }
    .map_err(to_pyerr)?;

    // Convert the output to a Python object.
    let result = PyDict::new(py);
    // Convert the intermediate models.
    let models = PyList::new(py, output.models.into_iter().map(Into::<PyCatCTBN>::into))?;
    result.set_item("models", models)?;
    // Convert the intermediate EM outputs.
    let expectations = PyList::new(
        py,
        output
            .expectations
            .into_iter()
            .map(|em| {
                // Convert each EM output.
                let result = PyDict::new(py);
                // Convert the models.
                let models = em.models.into_iter().map(Into::<PyCatCTBN>::into);
                let models = PyList::new(py, models)?;
                result.set_item("models", models)?;
                // Convert the expectations.
                let expectations = em.expectations.into_iter().map(Into::<PyCatWtdTrjs>::into);
                let expectations = PyList::new(py, expectations)?;
                result.set_item("expectations", expectations)?;
                // Convert the last model.
                let last_model: PyCatCTBN = em.last_model.into();
                result.set_item("last_model", last_model)?;
                // Set the number of iterations.
                let iterations = em.iterations;
                result.set_item("iterations", iterations)?;
                // Return the converted EM output.
                Ok::<_, PyErr>(result)
            })
            .collect::<PyResult<Vec<_>>>()?,
    )?;
    result.set_item("expectations", expectations)?;
    // Convert the last model.
    let last_model: PyCatCTBN = output.last_model.into();
    result.set_item("last_model", last_model)?;
    // Set the number of iterations.
    let iterations = output.iterations;
    result.set_item("iterations", iterations)?;

    // Convert the fitted model into a PyCatCTBN.
    Ok(result)
}
