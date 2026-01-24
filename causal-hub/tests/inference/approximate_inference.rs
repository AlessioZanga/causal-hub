#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        assets::load_asia,
        datasets::{CatEv, CatEvT},
        inference::{ApproximateInference, BNInference, ParBNInference},
        models::{CatCPD, Labelled},
        set, states,
        types::{Error, Result},
    };
    use ndarray::prelude::*;
    use rand::prelude::*;
    use rand_xoshiro::Xoshiro256PlusPlus;

    mod categorical {
        use super::*;

        mod bayesian_network {
            use super::*;

            #[test]
            fn estimate_empty_x() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                assert!(engine.estimate(&set![], &set![]).is_err());

                Ok(())
            }

            #[test]
            fn estimate_non_disjoint_xz() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                assert!(engine.estimate(&set![0], &set![0]).is_err());

                Ok(())
            }

            #[test]
            fn estimate_x_not_in_model() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                assert!(engine.estimate(&set![10], &set![]).is_err());

                Ok(())
            }

            #[test]
            fn estimate_zero_sample_size() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                assert!(
                    ApproximateInference::new(&mut rng, &model)
                        .with_sample_size(0)
                        .is_err()
                );

                Ok(())
            }

            #[test]
            fn estimate_with_sample_size() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(1000)?;

                // Predict P(asia) without evidence.
                let pred_query = engine.estimate(&set![0], &set![])?;
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                )?;

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);

                Ok(())
            }

            #[test]
            fn estimate_without_evidence() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                let pred_query = engine.estimate(&set![0], &set![])?;
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                )?;

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);

                Ok(())
            }

            #[test]
            fn estimate_with_evidence() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the evidence.
                let evidence = CatEv::new(
                    model.states().clone(),
                    [
                        CatEvT::CertainPositive {
                            event: model
                                .labels()
                                .get_index_of("lung")
                                .ok_or(Error::IllegalArgument("missing".into()))?, // lung
                            state: 1, // yes
                        },
                        CatEvT::CertainPositive {
                            event: model
                                .labels()
                                .get_index_of("tub")
                                .ok_or(Error::IllegalArgument("missing".into()))?, // tub
                            state: 0, // no
                        },
                    ],
                )?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model).with_evidence(&evidence);

                // Predict P(asia) without evidence.
                let pred_query = engine.estimate(&set![0], &set![])?;
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                )?;

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);

                Ok(())
            }

            #[test]
            fn par_estimate_empty_x() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                assert!(engine.par_estimate(&set![], &set![]).is_err());

                Ok(())
            }

            #[test]
            fn par_estimate_non_disjoint_xz() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                assert!(engine.estimate(&set![0], &set![0]).is_err());

                Ok(())
            }

            #[test]
            fn par_estimate_x_not_in_model() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict P(asia) without evidence.
                assert!(engine.par_estimate(&set![10], &set![]).is_err());

                Ok(())
            }

            #[test]
            fn par_estimate_with_sample_size() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(1000)?;

                // Predict P(asia) without evidence.
                let pred_query = engine.par_estimate(&set![0], &set![])?;
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                )?;

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);

                Ok(())
            }

            #[test]
            fn par_estimate_without_evidence() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict without evidence.
                let pred_query = engine.par_estimate(&set![0], &set![])?;
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.99, 0.01]],
                )?;

                // Assert that the estimation is correct.
                assert_relative_eq!(pred_query, true_query, epsilon = 1e-2);

                Ok(())
            }

            #[test]
            fn par_estimate_with_evidence() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_asia()?;
                // Initialize the evidence.
                let evidence = CatEv::new(
                    model.states().clone(),
                    [
                        CatEvT::CertainPositive {
                            event: model.label_to_index("lung")?, // lung
                            state: 1,                             // yes
                        },
                        CatEvT::CertainPositive {
                            event: model.label_to_index("tub")?, // tub
                            state: 0,                            // no
                        },
                    ],
                )?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model).with_evidence(&evidence);

                // Predict without evidence.
                let pred_query = engine.par_estimate(&set![0], &set![])?;
                // Set the expected results.
                let true_query = CatCPD::new(
                    // X
                    states![("asia", ["no", "yes"])],
                    // Z
                    states![],
                    // Theta
                    array![[0.9757204131016298, 0.024279586898370155]],
                )?;

                // Assert that the estimation is correct.
                assert_relative_eq!(true_query, pred_query, epsilon = 1e-2);

                Ok(())
            }
        }
    }
}
