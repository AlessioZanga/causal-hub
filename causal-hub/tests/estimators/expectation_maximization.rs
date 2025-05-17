#[cfg(test)]
mod tests {
    mod continuous_time_bayesian_network {
        use std::cell::RefCell;

        use approx::{assert_relative_eq, relative_eq};
        use causal_hub::{
            assets::load_eating,
            datasets::{CatTrjEv, CatTrjsEv, CatWtdTrjs, Dataset},
            distributions::CatCIM,
            estimators::{BE, EMBuilder, MLE, ParCTBNEstimator},
            generators::RngEv,
            models::{CTBN, CatCTBN},
            samplers::{CTBNSampler, ForwardSampler, ImportanceSampler, ParCTBNSampler},
        };
        use ndarray::prelude::*;
        use rand::{RngCore, SeedableRng};
        use rand_xoshiro::Xoshiro256PlusPlus;
        use rayon::prelude::*;

        #[test]
        fn test_em_builder() {
            // Load eating.
            let model = load_eating();
            // Set the evidence.
            let evidence = CatTrjsEv::new([
                // A single empty evidence.
                CatTrjEv::new(model.states(), Vec::<(String, _)>::new()),
            ]);

            // Define the expectation step.
            let e_step = |_prev_model: &CatCTBN, _evidence: &CatTrjsEv| -> CatWtdTrjs {
                unreachable!() // Dummy implementation.
            };
            // Define the maximization step.
            let m_step = |_prev_model: &CatCTBN, _expectation: &CatWtdTrjs| -> CatCTBN {
                unreachable!() // Dummy implementation.
            };
            // Define the stopping criteria.
            let stop = |_prev_model: &CatCTBN, _next_model: &CatCTBN, _counter: usize| -> bool {
                unreachable!() // Dummy implementation.
            };

            // Create a new builder
            let _builder = EMBuilder::new(&model, &evidence)
                .with_e_step(&e_step)
                .with_m_step(&m_step)
                .with_stop(&stop)
                .build();
        }

        #[test]
        fn test_em_with_no_evidence() {
            // Load eating.
            let model = load_eating();
            // Set the evidence.
            let evidence = CatTrjsEv::new(vec![
                // A thousands empty evidence.
                CatTrjEv::new(model.states(), Vec::<(String, _)>::new()); 1_000
            ]);

            // Initialize a new random number generator.
            let rng = RefCell::new(Xoshiro256PlusPlus::seed_from_u64(42));

            // Define the expectation step.
            let e_step = |prev_model: &CatCTBN, evidence: &CatTrjsEv| -> CatWtdTrjs {
                // Reference the random number generator.
                let mut rng = rng.borrow_mut();
                // Sample the seeds to parallelize the sampling.
                let seeds: Vec<_> = (0..evidence.values().len())
                    .map(|_| rng.next_u64())
                    .collect();
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
                        // Sample the trajectories.
                        importance.sample_by_length(1_000)
                    })
                    .collect()
            };

            // Define the maximization step.
            let m_step = |prev_model: &CatCTBN, expectation: &CatWtdTrjs| -> CatCTBN {
                // Fit the new model using the expectation.
                MLE::new(expectation).par_fit(prev_model.graph().clone())
            };

            // Define the stopping criteria.
            let stop = |prev_model: &CatCTBN, curr_model: &CatCTBN, counter: usize| -> bool {
                // Check if the models are equal or the counter is greater than 10.
                relative_eq!(prev_model, curr_model, epsilon = 5e-2) || counter >= 10
            };

            // Create a new builder.
            let builder = EMBuilder::new(&model, &evidence)
                .with_e_step(&e_step)
                .with_m_step(&m_step)
                .with_stop(&stop)
                .build();

            // Fit the model.
            let fitted_model = builder.fit();

            // Check if the models are equal.
            assert_relative_eq!(fitted_model, model, epsilon = 5e-2);
        }

        #[test]
        #[ignore = "this test is slow and should be run manually in release mode."]
        fn test_em_with_evidence() {
            // Initialize a new random number generator.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

            // Load eating.
            let model = load_eating();
            // Initialize a new sampler with no evidence.
            let mut forward = ForwardSampler::new(&mut rng, &model);
            // Sample the fully-observed trajectories from the model.
            let trajectories = forward.par_sample_n_by_length(100, 10_000);

            // Set the probability of the evidence.
            let p = 0.5;
            // Initialize the evidence generator.
            let mut generator = RngEv::new(&mut rng, &trajectories, p);
            // Sample the evidence from the fully-observed trajectories.
            let evidence = generator.random();

            // Set uniform CIMs.
            const E: f64 = 1.;
            // Set the initial CIMs.
            let initial_cims = vec![
                CatCIM::new(
                    // P(Hungry | FullStomach)
                    ("Hungry", vec!["no", "yes"]),
                    [("FullStomach", vec!["no", "yes"])],
                    array![
                        [
                            [-E, E], //
                            [E, -E]  //
                        ],
                        [
                            [-E, E], //
                            [E, -E]  //
                        ],
                    ],
                ),
                CatCIM::new(
                    // P(Eating | Hungry)
                    ("Eating", vec!["no", "yes"]),
                    [("Hungry", vec!["no", "yes"])],
                    array![
                        [
                            [-E, E], //
                            [E, -E]  //
                        ],
                        [
                            [-E, E], //
                            [E, -E]  //
                        ],
                    ],
                ),
                CatCIM::new(
                    // P(FullStomach | Eating)
                    ("FullStomach", vec!["no", "yes"]),
                    [("Eating", vec!["no", "yes"])],
                    array![
                        [
                            [-E, E], //
                            [E, -E]  //
                        ],
                        [
                            [-E, E], //
                            [E, -E]  //
                        ],
                    ],
                ),
            ];
            // Set the initial model.
            let initial_model = CatCTBN::new(model.graph().clone(), initial_cims);

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
            let m_step = |prev_model: &CatCTBN, expectation: &CatWtdTrjs| -> CatCTBN {
                // Fit the new model using the expectation.
                BE::new(expectation, (1, 1.0)).par_fit(prev_model.graph().clone())
            };

            // Define the stopping criteria.
            let stop = |prev_model: &CatCTBN, curr_model: &CatCTBN, counter: usize| -> bool {
                // Check if the models are equal or the counter is greater than the limit.
                relative_eq!(prev_model, curr_model, epsilon = 5e-2) || counter >= 100
            };

            // Create a new builder.
            let builder = EMBuilder::new(&initial_model, &evidence)
                .with_e_step(&e_step)
                .with_m_step(&m_step)
                .with_stop(&stop)
                .build();

            // Fit the model.
            let fitted_model = builder.fit();

            // Check if the models are equal.
            assert_relative_eq!(model, fitted_model, epsilon = 5e-2);
        }
    }
}
