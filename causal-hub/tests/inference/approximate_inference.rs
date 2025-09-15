#[cfg(test)]
mod tests {
    mod categorical {
        mod bayesian_network {
            use approx::assert_relative_eq;
            use causal_hub::{
                assets::load_asia,
                datasets::{CatEv, CatEvT},
                inference::{ApproximateInference, BNInference, ParBNInference},
                map,
                models::{BN, CatCPD},
                set,
            };
            use ndarray::prelude::*;
            use rand::prelude::*;
            use rand_xoshiro::Xoshiro256PlusPlus;

            #[test]
            #[should_panic(expected = "Variables X must not be empty.")]
            fn predict_empty_x() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.predict(&set![], &set![]);
            }
            #[test]
            #[should_panic(expected = "Variables X and Z must be disjoint.")]
            fn predict_non_disjoint_xz() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.predict(&set![0], &set![0]);
            }

            #[test]
            #[should_panic(expected = "Variables X and Z must be in the model.")]
            fn predict_x_not_in_model() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.predict(&set![10], &set![]);
            }

            #[test]
            fn predict_with_sample_size() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model).with_sample_size(1000);

                // Predict P(asia) without evidence.
                let pred_query = engine.predict(&set![0], &set![]);
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
            fn predict_without_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let pred_query = engine.predict(&set![0], &set![]);
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
                let pred_query = engine.predict(&set![0], &set![]);
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
            #[should_panic(expected = "Variables X must not be empty.")]
            fn par_predict_empty_x() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.par_predict(&set![], &set![]);
            }

            #[test]
            #[should_panic(expected = "Variables X and Z must be disjoint.")]
            fn par_predict_non_disjoint_xz() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.predict(&set![0], &set![0]);
            }

            #[test]
            #[should_panic(expected = "Variables X and Z must be in the model.")]
            fn par_predict_x_not_in_model() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.par_predict(&set![10], &set![]);
            }

            #[test]
            fn par_predict_with_sample_size() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let mut engine = ApproximateInference::new(&mut rng, &model).with_sample_size(1000);

                // Predict P(asia) without evidence.
                let pred_query = engine.par_predict(&set![0], &set![]);
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
                let pred_query = engine.par_predict(&set![0], &set![]);
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
                let pred_query = engine.par_predict(&set![0], &set![]);
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
