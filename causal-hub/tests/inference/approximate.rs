#[cfg(test)]
mod tests {
    mod categorical {
        mod bayesian_network {
            use approx::assert_relative_eq;
            use causal_hub::{
                assets::load_asia,
                inference::{BNApproxInference, ParBNApproxInference},
                map,
                models::CatCPD,
                samplers::ForwardSampler,
                set,
            };
            use ndarray::prelude::*;
            use rand::prelude::*;
            use rand_xoshiro::Xoshiro256PlusPlus;

            #[test]
            fn predict() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ForwardSampler::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let pred_query = engine.predict(&set![0], &set![], 1000);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    map![(
                        "asia".to_string(),
                        set!["no".to_string(), "yes".to_string()]
                    )],
                    // Z
                    map![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the prediction is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }

            #[test]
            fn par_predict() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ForwardSampler::new(&mut rng, &model);

                // Predict without evidence.
                let pred_query = engine.par_predict(&set![0], &set![], 1000);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    map![(
                        "asia".to_string(),
                        set!["no".to_string(), "yes".to_string()]
                    )],
                    // Z
                    map![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the prediction is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }
        }
    }
}
