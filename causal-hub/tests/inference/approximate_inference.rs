#[cfg(test)]
mod tests {
    mod categorical {
        mod bayesian_network {
            use approx::assert_relative_eq;
            use causal_hub::{
                assets::load_asia,
                datasets::{CatEv, CatEvT},
                inference::{ApproximateInference, BNInference, ParBNInference},
                models::{CatCPD, Labelled},
                set, states,
            };
            use ndarray::prelude::*;
            use rand::prelude::*;
            use rand_xoshiro::Xoshiro256PlusPlus;

            #[test]
            #[should_panic(expected = "Variables X must not be empty.")]
            fn estimate_empty_x() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.estimate(&set![], &set![]);
            }
            #[test]
            #[should_panic(expected = "Variables X and Z must be disjoint.")]
            fn estimate_non_disjoint_xz() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.estimate(&set![0], &set![0]);
            }

            #[test]
            #[should_panic(expected = "Variables X and Z must be in the model.")]
            fn estimate_x_not_in_model() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.estimate(&set![10], &set![]);
            }

            #[test]
            #[should_panic(expected = "Sample size must be positive.")]
            fn estimate_zero_sample_size() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let _ = ApproximateInference::new(&mut rng, &model).with_sample_size(0);
            }

            #[test]
            fn estimate_with_sample_size() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(1000);

                // Predict P(asia) without evidence.
                let pred_query = engine.estimate(&set![0], &set![]);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }

            #[test]
            fn estimate_without_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let pred_query = engine.estimate(&set![0], &set![]);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }

            #[test]
            fn estimate_with_evidence() {
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
                let engine = ApproximateInference::new(&mut rng, &model).with_evidence(&evidence);

                // Predict P(asia) without evidence.
                let pred_query = engine.estimate(&set![0], &set![]);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }

            #[test]
            #[should_panic(expected = "Variables X must not be empty.")]
            fn par_estimate_empty_x() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.par_estimate(&set![], &set![]);
            }

            #[test]
            #[should_panic(expected = "Variables X and Z must be disjoint.")]
            fn par_estimate_non_disjoint_xz() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.estimate(&set![0], &set![0]);
            }

            #[test]
            #[should_panic(expected = "Variables X and Z must be in the model.")]
            fn par_estimate_x_not_in_model() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let _ = engine.par_estimate(&set![10], &set![]);
            }

            #[test]
            fn par_estimate_with_sample_size() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(1000);

                // Predict P(asia) without evidence.
                let pred_query = engine.par_estimate(&set![0], &set![]);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }

            #[test]
            fn par_estimate_without_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict without evidence.
                let pred_query = engine.par_estimate(&set![0], &set![]);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }

            #[test]
            fn par_estimate_with_evidence() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia();
                // Initialize the evidence.
                let evidence = CatEv::new(
                    model.states().clone(),
                    [
                        CatEvT::CertainPositive {
                            event: model.label_to_index("lung"), // lung
                            state: 1,                            // yes
                        },
                        CatEvT::CertainPositive {
                            event: model.label_to_index("tub"), // tub
                            state: 0,                           // no
                        },
                    ],
                );
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model).with_evidence(&evidence);

                // Predict without evidence.
                let pred_query = engine.par_estimate(&set![0], &set![]);
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                );

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);
            }
        }
    }
}
