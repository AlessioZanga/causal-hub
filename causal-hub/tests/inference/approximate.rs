#[cfg(test)]
mod tests {
    mod categorical {
        mod bayesian_network {
            use approx::assert_relative_eq;
            use causal_hub::{
                assets::load_asia,
                datasets::{CatEv, CatEvT},
                inference::{ApproximateInference, BNApproxInference, ParBNApproxInference},
                map,
                models::{BN, CatCPD},
                set,
            };
            use ndarray::prelude::*;
            use rand::prelude::*;
            use rand_xoshiro::Xoshiro256PlusPlus;

            #[test]
            fn predict_without_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

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
            fn par_predict_without_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

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

            #[test]
            fn predict_with_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the evidence.
                let evidence = CatEv::new(
                    model.states().clone(),
                    [
                        CatEvT::CertainPositive {
                            event: model.labels().get_index_of("lung").unwrap(), // lung
                            state: 1,                                            // yes
                        },
                        CatEvT::CertainPositive {
                            event: model.labels().get_index_of("tub").unwrap(), // tub
                            state: 0,                                           // no
                        },
                    ],
                );
                // Initialize the inference engine.
                let mut engine =
                    ApproximateInference::new(&mut rng, &model).with_evidence(&evidence);

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
            fn par_predict_with_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the evidence.
                let evidence = CatEv::new(
                    model.states().clone(),
                    [
                        CatEvT::CertainPositive {
                            event: model.labels().get_index_of("lung").unwrap(), // lung
                            state: 1,                                            // yes
                        },
                        CatEvT::CertainPositive {
                            event: model.labels().get_index_of("tub").unwrap(), // tub
                            state: 0,                                           // no
                        },
                    ],
                );
                // Initialize the inference engine.
                let mut engine =
                    ApproximateInference::new(&mut rng, &model).with_evidence(&evidence);

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
