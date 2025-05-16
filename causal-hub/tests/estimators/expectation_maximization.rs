#[cfg(test)]
mod tests {
    mod continuous_time_bayesian_network {
        use std::cell::RefCell;

        use approx::{assert_relative_eq, relative_eq};
        use causal_hub::{
            assets::load_eating,
            datasets::{CatTrjEv, CatWtdTrjs},
            estimators::{CTBNEstimator, EMBuilder, MLE},
            models::{CTBN, CatCTBN},
            samplers::{CTBNSampler, ImportanceSampler},
        };
        use rand::{RngCore, SeedableRng};
        use rand_xoshiro::Xoshiro256PlusPlus;
        use rayon::prelude::*;

        #[test]
        fn test_em_builder() {
            // Load eating.
            let model = load_eating();
            // Set the evidence.
            let evidence = vec![CatTrjEv::new(model.states(), Vec::<(String, _)>::new())];

            // Define the expectation step.
            let e_step = |_prev_model: &CatCTBN, _evidence: &Vec<CatTrjEv>| -> CatWtdTrjs {
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
            let evidence = vec![CatTrjEv::new(model.states(), Vec::<(String, _)>::new(),); 1_000];

            // Initialize a new random number generator.
            let rng = RefCell::new(Xoshiro256PlusPlus::seed_from_u64(42));

            // Define the expectation step.
            let e_step = |prev_model: &CatCTBN, evidence: &Vec<CatTrjEv>| -> CatWtdTrjs {
                // Reference the random number generator.
                let mut rng = rng.borrow_mut();
                // Sample the seeds to parallelize the sampling.
                let seeds: Vec<_> = (0..evidence.len()).map(|_| rng.next_u64()).collect();

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
                MLE::new(expectation).fit(prev_model.graph().clone())
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
    }
}
