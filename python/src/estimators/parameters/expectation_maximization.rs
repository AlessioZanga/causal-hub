use std::cell::RefCell;

use approx::*;
use backend::{
    datasets::{CatTrjs, CatTrjsEv, CatWtdTrjs, Dataset},
    estimators::{BE, EMBuilder, ParCTBNEstimator, RAWE},
    models::{CTBN, CatCTBN, DiGraph},
    samplers::{ImportanceSampler, ParCTBNSampler},
};
use log::debug;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use pyo3_stub_gen::derive::*;
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

use crate::{
    datasets::{PyCatTrjsEv, PyCatWtdTrjs},
    models::{PyCatCTBN, PyDiGraph},
};

/// A function to perform parameter learning using the Expectation Maximization (EM) algorithm.
#[gen_stub_pyfunction(module = "causal_hub.estimation")]
#[pyfunction]
#[pyo3(signature = (
    evidence,
    graph,
    max_iter = 10,
    seed = 42
))]
pub fn em<'a>(
    py: Python<'a>,
    evidence: &Bound<'_, PyCatTrjsEv>,
    graph: &Bound<'_, PyDiGraph>,
    max_iter: usize,
    seed: u64,
) -> PyResult<Bound<'a, PyDict>> {
    // Get the evidence.
    let evidence: PyCatTrjsEv = evidence.extract()?;
    // Get the reference to the evidence.
    let evidence: &CatTrjsEv = &evidence.lock();

    // Get the graph.
    let graph: PyDiGraph = graph.extract()?;
    // Get the reference to the graph.
    let graph: &DiGraph = &graph.lock();

    // Release the GIL to allow parallel execution.
    let output = py.detach(|| {
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Log the raw estimator initialization.
        debug!("Initializing the raw estimator for the initial guess ...");
        // Initialize a raw estimator for an initial guess.
        let raw = RAWE::<'_, _, CatTrjsEv, CatTrjs>::par_new(&mut rng, evidence);
        // Log the initial model fitting.
        debug!("Fitting the initial model using the raw estimator ...");
        // Set the initial model.
        let model = raw.par_fit(graph.clone());

        // Wrap the random number generator in a RefCell to allow mutable borrowing.
        let rng = RefCell::new(rng);

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
        let em = EMBuilder::new(&model, evidence)
            .with_e_step(&e_step)
            .with_m_step(&m_step)
            .with_stop(&stop)
            .build();

        // Fit the model.
        em.fit()
    });

    // Convert each EM output.
    let result = PyDict::new(py);
    // Convert the models.
    let models = output.models.into_iter().map(Into::<PyCatCTBN>::into);
    let models = PyList::new(py, models).unwrap();
    result.set_item("models", models).unwrap();
    // Convert the expectations.
    let expectations = output
        .expectations
        .into_iter()
        .map(Into::<PyCatWtdTrjs>::into);
    let expectations = PyList::new(py, expectations).unwrap();
    result.set_item("expectations", expectations).unwrap();
    // Convert the last model.
    let last_model: PyCatCTBN = output.last_model.into();
    result.set_item("last_model", last_model).unwrap();
    // Set the number of iterations.
    let iterations = output.iterations;
    result.set_item("iterations", iterations).unwrap();
    // Return the converted EM output.
    Ok(result)
}
