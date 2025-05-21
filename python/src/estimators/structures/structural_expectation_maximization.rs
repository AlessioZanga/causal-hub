use std::{cell::RefCell, ops::Deref};

use approx::relative_eq;
use causal_hub::{
    datasets::{CatTrjs, CatTrjsEv, CatWtdTrjs, Dataset},
    estimators::{
        BE, CTPC, ChiSquaredTest, EMBuilder, FTest, ParCPDEstimator, ParCTBNEstimator, RE,
    },
    graphs::{DiGraph, Graph},
    models::{CTBN, CatCTBN},
    samplers::{CTBNSampler, ImportanceSampler},
    types::Cache,
};
use pyo3::prelude::*;
use rand::{RngCore, SeedableRng, seq::SliceRandom};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

use crate::{datasets::PyCatTrjsEv, models::PyCatCTBN};

#[pyfunction]
#[pyo3(signature = (
    evidence,
    f_test = 1e-2,
    c_test = 1e-2,
    max_iter = 10,
    max_parents = 10,
    seed = 42
))]
pub fn sem(
    evidence: &Bound<'_, PyCatTrjsEv>,
    f_test: f64,
    c_test: f64,
    max_iter: usize,
    max_parents: usize,
    seed: u64,
) -> PyResult<PyCatCTBN> {
    // Get the evidence.
    let evidence: PyCatTrjsEv = evidence.extract()?;
    // Get the reference to the evidence.
    let evidence: &CatTrjsEv = evidence.deref();

    // Initialize the random number generator.
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    // Set the initial graph.
    let mut inital_graph = DiGraph::complete(evidence.labels());

    // Set the parents of the initial graph to max_parents.
    for i in 0..inital_graph.vertices().len() {
        // Get the parents.
        let mut pa_i = inital_graph.parents(i);
        // Choose nodes randomly.
        pa_i.shuffle(&mut rng);
        // Remove the excess parents.
        for j in pa_i.split_off(max_parents) {
            // Remove the edge.
            inital_graph.del_edge(j, i);
        }
    }

    // Initialize a raw estimator for an initial guess.
    let raw = RE::<CatTrjs>::new(&evidence);
    // Set the initial CIMs.
    let initial_cims: Vec<_> = inital_graph
        .vertices()
        .into_par_iter()
        .map(|i| {
            // Get the parents.
            let pa_i = inital_graph.parents(i);
            // Fit the raw estimates.
            ParCPDEstimator::par_fit(&raw, i, &pa_i)
        })
        .collect();
    // Set the initial model.
    let initial_model = CatCTBN::new(inital_graph.clone(), initial_cims);

    // Wrap the random number generator in a RefCell to allow mutable borrowing.
    let rng = RefCell::new(rng);

    // Define the expectation step.
    let e_step = |prev_model: &CatCTBN, evidence: &CatTrjsEv| -> CatWtdTrjs {
        // Reference the random number generator.
        let mut rng = rng.borrow_mut();
        // Sample the seeds to parallelize the sampling.
        let seeds: Vec<_> = (0..evidence.values().len())
            .map(|_| rng.next_u64())
            .collect();
        // Get the max length of the evidence.
        let max_len = evidence
            .values()
            .iter()
            .map(|e| e.values().len())
            .max()
            .unwrap_or(10);
        // Fore each (seed, evidence) ...
        seeds
            .iter()
            .zip(evidence)
            .par_bridge()
            .map(|(&s, e)| {
                // Initialize a new random number generator.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(s);
                // Initialize a new sampler.
                let mut importance = ImportanceSampler::new(&mut rng, prev_model, e);
                // Perform multiple imputation.
                let trjs = importance.sample_n_by_length(2 * max_len, 10);
                // Get the one with the highest weight.
                trjs.values()
                    .iter()
                    .max_by(|a, b| a.weight().partial_cmp(&b.weight()).unwrap())
                    .unwrap()
                    .clone()
            })
            // Reject trajectories with low weight.
            .filter(|trj| trj.weight() >= 1e-3)
            .collect()
    };

    // Define the maximization step.
    let m_step = |_prev_model: &CatCTBN, expectation: &CatWtdTrjs| -> CatCTBN {
        // Initialize the parameter estimator.
        let estimator = BE::new(expectation, (1, 1.));
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the F test.
        let f_test = FTest::new(&cache, f_test);
        // Initialize the chi-squared test.
        let chi_sq_test = ChiSquaredTest::new(&cache, c_test);
        // Initialize the CTPC algorithm.
        let ctpc = CTPC::new(&inital_graph, &f_test, &chi_sq_test);
        // Fit the new structure using CTPC.
        let fitted_graph = ctpc.par_fit();
        // Fit the new model using the expectation.
        ParCTBNEstimator::par_fit(&estimator, fitted_graph)
    };

    // Define the stopping criteria.
    let stop = |prev_model: &CatCTBN, curr_model: &CatCTBN, counter: usize| -> bool {
        // Check if the models are equal or the counter is greater than the limit.
        relative_eq!(prev_model, curr_model, epsilon = 5e-2) || counter >= max_iter
    };

    // Create a new builder.
    let builder = EMBuilder::new(&initial_model, evidence)
        .with_e_step(&e_step)
        .with_m_step(&m_step)
        .with_stop(&stop)
        .build();

    // Fit the model.
    let fitted_model = builder.fit();

    // Convert the fitted model into a PyDiGraph.
    Ok(fitted_model.into())
}
