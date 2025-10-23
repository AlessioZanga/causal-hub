use std::cell::RefCell;

use approx::relative_eq;
use backend::{
    datasets::{CatTrjs, CatTrjsEv, CatWtdTrjs, Dataset},
    estimators::{
        BE, BIC, CTHC, CTPC, ChiSquaredTest, EMBuilder, EMOutput, FTest, PK, ParCTBNEstimator, RAWE,
    },
    models::{CTBN, CatCTBN, DiGraph, Graph, Labelled},
    samplers::{ImportanceSampler, ParCTBNSampler},
    set,
    types::Cache,
};
use log::debug;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use pyo3_stub_gen::derive::*;
use rand::{RngCore, SeedableRng, seq::SliceRandom};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

use crate::{
    datasets::{PyCatTrjsEv, PyCatWtdTrjs},
    estimators::PyPK,
    kwarg,
    models::PyCatCTBN,
};

/// A function to perform structure learning using the Structural Expectation Maximization (SEM) algorithm.
#[gen_stub_pyfunction(module = "causal_hub.estimation")]
#[pyfunction]
#[pyo3(signature = (
    evidence,
    prior_knowledge,
    algorithm,
    max_iter = 10,
    seed = 42,
    **kwargs
))]
pub fn sem<'a>(
    py: Python<'a>,
    evidence: &Bound<'_, PyCatTrjsEv>,
    prior_knowledge: &Bound<'_, PyPK>,
    algorithm: &str,
    max_iter: usize,
    seed: u64,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Bound<'a, PyDict>> {
    // Get the evidence.
    let evidence: PyCatTrjsEv = evidence.extract()?;
    // Get the reference to the evidence.
    let evidence: &CatTrjsEv = &evidence.lock();

    // Get the prior knowledge.
    let prior_knowledge: PyPK = prior_knowledge.extract()?;
    // Convert the prior knowledge into a PK.
    let prior_knowledge: &PK = &prior_knowledge.lock();

    // Get the maximum number of parents from the keyword arguments or set the maximum.
    let max_parents = kwarg!(kwargs, "max_parents", usize).unwrap_or(evidence.labels().len());
    // Get f_test and c_test from the keyword arguments or set defaults.
    let f_test = kwarg!(kwargs, "f_test", f64).unwrap_or(0.01);
    let c_test = kwarg!(kwargs, "c_test", f64).unwrap_or(0.01);

    // Release the GIL to allow parallel execution.
    let output = py.detach(|| {
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Set the initial graph depending on the algorithm.
        let initial_graph = match algorithm {
            "ctpc" => {
                // Log the graph initialization.
                debug!("Setting initial graph for CTPC algorithm to a complete graph ...");
                // Set the initial graph to a complete graph.
                let mut initial_graph = DiGraph::complete(evidence.labels());
                // Apply the prior knowledge to the initial graph.
                for (i, j) in prior_knowledge.forbidden_edges() {
                    // Remove the edge if it is forbidden.
                    initial_graph.del_edge(i, j);
                }
                // Check if the number of vertices is less than or equal to the maximum number of parents.
                if initial_graph.vertices().len() > max_parents + 1 {
                    // Log the maximum number of parents.
                    debug!("Reducing the number of parents to {max_parents}, when needed.");
                    // Set the parents of the initial graph to max_parents.
                    for i in 0..initial_graph.vertices().len() {
                        // Get the parents.
                        let pa_i = initial_graph.parents(&set![i]);
                        // Check the maximum number of parents.
                        if pa_i.len() > max_parents + 1 {
                            // Convert to a mutable vector.
                            let mut pa_i: Vec<_> = pa_i.into_iter().collect();
                            // Choose nodes randomly.
                            pa_i.shuffle(&mut rng);
                            // Remove the excess parents.
                            for j in pa_i.split_off(max_parents) {
                                // Remove the edge.
                                initial_graph.del_edge(j, i);
                            }
                        }
                    }
                }
                // Return the initial graph.
                initial_graph
            }
            "cthc" => {
                // Log the graph initialization.
                debug!("Setting initial graph for CTHC algorithm to an empty graph ...");
                // Set the initial graph to an empty graph.
                let mut initial_graph = DiGraph::empty(evidence.labels());
                // Apply the prior knowledge to the initial graph.
                for (i, j) in prior_knowledge.required_edges() {
                    // Add the edge if it is required.
                    initial_graph.add_edge(i, j);
                }
                // Return the initial graph.
                initial_graph
            }
            _ => panic!(
                "Failed to get the structure learning algorithm: \n\
                \t expected:   'ctpc' or 'cthc', \n\
                \t found:      '{algorithm}'"
            ),
        };

        // Log the raw estimator initialization.
        debug!("Initializing the raw estimator for the initial guess ...");
        // Initialize a raw estimator for an initial guess.
        let raw = RAWE::<'_, _, CatTrjsEv, CatTrjs>::par_new(&mut rng, evidence);
        // Log the initial model fitting.
        debug!("Fitting the initial model using the raw estimator ...");
        // Set the initial model.
        let initial_model = raw.par_fit(initial_graph.clone());

        // Wrap the random number generator in a RefCell to allow mutable borrowing.
        let rng = RefCell::new(rng);

        // Define the expectation-maximization step.
        let em_step =
            |prev_model: &CatCTBN, evidence: &CatTrjsEv| -> EMOutput<CatCTBN, CatWtdTrjs> {
                // Define the expectation step.
                let e_step = |prev_model: &CatCTBN, evidence: &CatTrjsEv| -> CatWtdTrjs {
                    // Reference the random number generator.
                    let mut rng = rng.borrow_mut();
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
                    // For each (seed, evidence) ...
                    seeds
                        .into_par_iter()
                        .zip(evidence.par_iter())
                        .map(|(s, e)| {
                            // Initialize a new random number generator.
                            let mut rng = Xoshiro256PlusPlus::seed_from_u64(s);
                            // Initialize a new sampler.
                            let importance = ImportanceSampler::new(&mut rng, prev_model, e);
                            // Perform multiple imputation.
                            let trjs = importance.par_sample_n_by_length(max_length, 10);
                            // Get the one with the highest weight.
                            trjs.values()
                                .iter()
                                .max_by(|a, b| a.weight().partial_cmp(&b.weight()).unwrap())
                                .unwrap()
                                .clone()
                        })
                        .collect()
                };

                // Define the maximization step.
                let m_step = |prev_model: &CatCTBN, expectation: &CatWtdTrjs| -> CatCTBN {
                    // Initialize the parameter estimator.
                    let estimator = BE::new(expectation).with_prior((1, 1.));
                    // Fit the model using the parameter estimator.
                    estimator.par_fit(prev_model.graph().clone())
                };

                // Define the stopping criteria.
                let stop = |prev_model: &CatCTBN, curr_model: &CatCTBN, counter: usize| -> bool {
                    // Check if the models are equal or the counter is greater than the limit.
                    relative_eq!(prev_model, curr_model, epsilon = 5e-2) || counter >= max_iter
                };

                // Create a new EM.
                let em = EMBuilder::new(prev_model, evidence)
                    .with_e_step(&e_step)
                    .with_m_step(&m_step)
                    .with_stop(&stop)
                    .build();

                // Fit the model.
                em.fit()
            };

        // Define the structure learning step.
        let sl_step = |_prev_model: &CatCTBN, em: &EMOutput<CatCTBN, CatWtdTrjs>| -> CatCTBN {
            // Initialize the parameter estimator.
            let estimator = BE::new(em.expectations.last().unwrap()).with_prior((1, 1.));
            // Cache the parameter estimator.
            let cache = Cache::new(&estimator);
            // Learn the graph.
            let fitted_graph = match algorithm {
                "ctpc" => {
                    // Initialize the F test.
                    let f_test = FTest::new(&cache, f_test);
                    // Initialize the chi-squared test.
                    let chi_sq_test = ChiSquaredTest::new(&cache, c_test);
                    // Initialize the CTPC algorithm.
                    let ctpc = CTPC::new(&initial_graph, &f_test, &chi_sq_test);
                    // Set prior knowledge.
                    let ctpc = ctpc.with_prior_knowledge(prior_knowledge);
                    // Fit the new structure using CTPC.
                    ctpc.par_fit()
                }
                "cthc" => {
                    // Initialize the scoring criterion.
                    let bic = BIC::new(&cache);
                    // Initialize the CTHC algorithm and set the maximum number of parents.
                    let cthc = CTHC::new(&initial_graph, &bic).with_max_parents(max_parents);
                    // Set prior knowledge.
                    let cthc = cthc.with_prior_knowledge(prior_knowledge);
                    // Fit the new structure using CTHC.
                    cthc.par_fit()
                }
                _ => panic!(
                    "Failed to get the structure learning algorithm: \n\
                    \t expected:   'ctpc' or 'cthc', \n\
                    \t found:      '{algorithm}'"
                ),
            };
            // Fit the new model using the expectation.
            estimator.par_fit(fitted_graph)
        };

        // Define the stopping criteria.
        let sem_stop = |prev_model: &CatCTBN, curr_model: &CatCTBN, counter: usize| -> bool {
            // Check if the models are equal or the counter is greater than the limit.
            relative_eq!(prev_model, curr_model, epsilon = 5e-2) || counter >= max_iter
        };

        // Create a new SEM.
        let sem = EMBuilder::new(&initial_model, evidence)
            .with_e_step(&em_step)
            .with_m_step(&sl_step)
            .with_stop(&sem_stop)
            .build();

        // Fit the model.
        sem.fit()
    });

    // Convert the output to a Python object.
    let result = PyDict::new(py);
    // Convert the intermediate models.
    let models = PyList::new(py, output.models.into_iter().map(Into::<PyCatCTBN>::into));
    result.set_item("models", models?)?;
    // Convert the intermediate EM outputs.
    let expectations = PyList::new(
        py,
        output.expectations.into_iter().map(|em| {
            // Convert each EM output.
            let result = PyDict::new(py);
            // Convert the models.
            let models = em.models.into_iter().map(Into::<PyCatCTBN>::into);
            let models = PyList::new(py, models).unwrap();
            result.set_item("models", models).unwrap();
            // Convert the expectations.
            let expectations = em.expectations.into_iter().map(Into::<PyCatWtdTrjs>::into);
            let expectations = PyList::new(py, expectations).unwrap();
            result.set_item("expectations", expectations).unwrap();
            // Convert the last model.
            let last_model: PyCatCTBN = em.last_model.into();
            result.set_item("last_model", last_model).unwrap();
            // Set the number of iterations.
            let iterations = em.iterations;
            result.set_item("iterations", iterations).unwrap();
            // Return the converted EM output.
            result
        }),
    );
    result.set_item("expectations", expectations?)?;
    // Convert the last model.
    let last_model: PyCatCTBN = output.last_model.into();
    result.set_item("last_model", last_model)?;
    // Set the number of iterations.
    let iterations = output.iterations;
    result.set_item("iterations", iterations)?;

    // Convert the fitted model into a PyCatCTBN.
    Ok(result)
}
