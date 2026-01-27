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
                assert_relative_eq!(true_query, pred_query, epsilon = 1e-2);

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
                assert_relative_eq!(true_query, pred_query, epsilon = 1e-2);

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
                    array![[0.9786268013452195, 0.0213731986547805]],
                )?;

                // Assert that the estimation is correct.
                assert_relative_eq!(true_query, pred_query, epsilon = 1e-2);

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
                assert_relative_eq!(true_query, pred_query, epsilon = 1e-2);

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
                assert_relative_eq!(true_query, pred_query, epsilon = 1e-2);

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

    mod gaussian {
        use causal_hub::{assets::load_ecoli70, models::CPD};

        use super::*;

        mod bayesian_network {
            use super::*;

            // =========================================================================
            // Error handling tests
            // =========================================================================

            #[test]
            fn estimate_empty_x() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict with empty X should fail.
                assert!(engine.estimate(&set![], &set![]).is_err());

                Ok(())
            }

            #[test]
            fn estimate_non_disjoint_xz() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict with overlapping X and Z should fail.
                assert!(engine.estimate(&set![0], &set![0]).is_err());

                Ok(())
            }

            #[test]
            fn estimate_x_not_in_model() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Predict with X index out of bounds should fail.
                assert!(engine.estimate(&set![100], &set![]).is_err());

                Ok(())
            }

            // =========================================================================
            // Single variable tests (1D X)
            // =========================================================================

            #[test]
            fn estimate_single_variable_without_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(10_000)?;

                // Get variable index for "eutG" (a root node with no parents).
                let x_idx = model.label_to_index("eutG")?;

                // Predict P(eutG) without conditioning.
                let pred_query = engine.estimate(&set![x_idx], &set![])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 1);
                assert!(pred_query.labels().contains("eutG"));
                assert!(pred_query.conditioning_labels().is_empty());

                // Check parameter dimensions: coefficients should be 1x0, intercept 1, covariance 1x1.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[1, 0]);
                assert_eq!(params.intercept().len(), 1);
                assert_eq!(params.covariance().shape(), &[1, 1]);

                // Check numerical values against the true model parameters.
                // True values from ecoli70: intercept=1.2654, variance=0.6911.
                assert_relative_eq!(params.intercept()[0], 1.2654, epsilon = 0.15);
                assert_relative_eq!(params.covariance()[[0, 0]], 0.6911, epsilon = 0.15);

                Ok(())
            }

            #[test]
            fn estimate_single_variable_with_single_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(10_000)?;

                // Get variable indices: aceB depends on icdA in the model.
                let x_idx = model.label_to_index("aceB")?;
                let z_idx = model.label_to_index("icdA")?;

                // Predict P(aceB | icdA).
                let pred_query = engine.estimate(&set![x_idx], &set![z_idx])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 1);
                assert!(pred_query.labels().contains("aceB"));
                assert_eq!(pred_query.conditioning_labels().len(), 1);
                assert!(pred_query.conditioning_labels().contains("icdA"));

                // Check parameter dimensions: coefficients should be 1x1, intercept 1, covariance 1x1.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[1, 1]);
                assert_eq!(params.intercept().len(), 1);
                assert_eq!(params.covariance().shape(), &[1, 1]);

                // Check numerical values against the true model parameters.
                // True values from ecoli70: coefficient=1.0464, intercept=0.1324, variance=0.0853.
                assert_relative_eq!(params.coefficients()[[0, 0]], 1.0464, epsilon = 0.2);
                assert_relative_eq!(params.intercept()[0], 0.1324, epsilon = 0.15);
                assert_relative_eq!(params.covariance()[[0, 0]], 0.0853, epsilon = 0.05);

                Ok(())
            }

            // =========================================================================
            // Multi-variable tests (higher dimensional X)
            // =========================================================================

            #[test]
            fn estimate_two_conditionally_independent_children() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;

                // Get variable indices: gltA and flgD are both children of sucA only.
                // When conditioned on sucA, they should be independent.
                let x1_idx = model.label_to_index("gltA")?;
                let x2_idx = model.label_to_index("flgD")?;
                let z_idx = model.label_to_index("sucA")?;

                // Predict P(gltA, flgD | sucA).
                let pred_query = engine.estimate(&set![x1_idx, x2_idx], &set![z_idx])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 2);
                assert!(pred_query.labels().contains("gltA"));
                assert!(pred_query.labels().contains("flgD"));
                assert_eq!(pred_query.conditioning_labels().len(), 1);
                assert!(pred_query.conditioning_labels().contains("sucA"));

                // Check parameter dimensions: coefficients should be 2x1.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[2, 1]);
                assert_eq!(params.intercept().len(), 2);
                assert_eq!(params.covariance().shape(), &[2, 2]);

                // The labels are returned in sorted order, check which is first.
                let labels: Vec<_> = pred_query.labels().iter().cloned().collect();
                let (glt_a_row, flg_d_row) = if labels[0] == "flgD" { (1, 0) } else { (0, 1) };

                // Check numerical values against the true model parameters.
                // True values from ecoli70:
                //   gltA|sucA: coef=0.379, intercept=-0.9572, var=0.6895
                //   flgD|sucA: coef=0.6362, intercept=-0.5167, var=0.3929
                assert_relative_eq!(params.coefficients()[[glt_a_row, 0]], 0.379, epsilon = 0.15);
                assert_relative_eq!(
                    params.coefficients()[[flg_d_row, 0]],
                    0.6362,
                    epsilon = 0.15
                );
                assert_relative_eq!(params.intercept()[glt_a_row], -0.9572, epsilon = 0.2);
                assert_relative_eq!(params.intercept()[flg_d_row], -0.5167, epsilon = 0.2);

                // Check variances (diagonal of covariance).
                assert_relative_eq!(
                    params.covariance()[[glt_a_row, glt_a_row]],
                    0.6895,
                    epsilon = 0.15
                );
                assert_relative_eq!(
                    params.covariance()[[flg_d_row, flg_d_row]],
                    0.3929,
                    epsilon = 0.15
                );

                // Since gltA and flgD are conditionally independent given sucA,
                // the off-diagonal covariance should be approximately zero.
                assert_relative_eq!(
                    params.covariance()[[glt_a_row, flg_d_row]],
                    0.0,
                    epsilon = 0.15
                );

                Ok(())
            }

            #[test]
            fn estimate_two_variables_without_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(10_000)?;

                // Get variable indices for two related variables.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacZ")?;

                // Predict P(lacA, lacZ) - joint distribution without conditioning.
                let pred_query = engine.estimate(&set![x1_idx, x2_idx], &set![])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 2);
                assert!(pred_query.labels().contains("lacA"));
                assert!(pred_query.labels().contains("lacZ"));
                assert!(pred_query.conditioning_labels().is_empty());

                // Check parameter dimensions: coefficients should be 2x0, intercept 2, covariance 2x2.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[2, 0]);
                assert_eq!(params.intercept().len(), 2);
                assert_eq!(params.covariance().shape(), &[2, 2]);

                // Covariance matrix should be symmetric.
                assert_relative_eq!(
                    params.covariance()[[0, 1]],
                    params.covariance()[[1, 0]],
                    epsilon = 1e-10
                );

                // Diagonal elements should be positive (variances).
                assert!(params.covariance()[[0, 0]] > 0.0);
                assert!(params.covariance()[[1, 1]] > 0.0);

                Ok(())
            }

            #[test]
            fn estimate_two_variables_with_single_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(10_000)?;

                // Get variable indices.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacZ")?;
                let z_idx = model.label_to_index("asnA")?;

                // Predict P(lacA, lacZ | asnA).
                let pred_query = engine.estimate(&set![x1_idx, x2_idx], &set![z_idx])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 2);
                assert_eq!(pred_query.conditioning_labels().len(), 1);

                // Check parameter dimensions: coefficients should be 2x1, intercept 2, covariance 2x2.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[2, 1]);
                assert_eq!(params.intercept().len(), 2);
                assert_eq!(params.covariance().shape(), &[2, 2]);

                // Covariance matrix should be symmetric.
                assert_relative_eq!(
                    params.covariance()[[0, 1]],
                    params.covariance()[[1, 0]],
                    epsilon = 1e-10
                );

                Ok(())
            }

            #[test]
            fn estimate_two_variables_with_two_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;

                // Get variable indices.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacZ")?;
                let z1_idx = model.label_to_index("asnA")?;
                let z2_idx = model.label_to_index("cspG")?;

                // Predict P(lacA, lacZ | asnA, cspG).
                let pred_query = engine.estimate(&set![x1_idx, x2_idx], &set![z1_idx, z2_idx])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 2);
                assert_eq!(pred_query.conditioning_labels().len(), 2);

                // Check parameter dimensions: coefficients should be 2x2, intercept 2, covariance 2x2.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[2, 2]);
                assert_eq!(params.intercept().len(), 2);
                assert_eq!(params.covariance().shape(), &[2, 2]);

                // Covariance matrix should be symmetric.
                assert_relative_eq!(
                    params.covariance()[[0, 1]],
                    params.covariance()[[1, 0]],
                    epsilon = 1e-10
                );

                // Diagonal elements should be positive (variances).
                assert!(params.covariance()[[0, 0]] > 0.0);
                assert!(params.covariance()[[1, 1]] > 0.0);

                Ok(())
            }

            #[test]
            fn estimate_three_variables_without_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(10_000)?;

                // Get variable indices for three variables in the lac operon cluster.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacY")?;
                let x3_idx = model.label_to_index("lacZ")?;

                // Predict P(lacA, lacY, lacZ) - joint distribution.
                let pred_query = engine.estimate(&set![x1_idx, x2_idx, x3_idx], &set![])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 3);
                assert!(pred_query.labels().contains("lacA"));
                assert!(pred_query.labels().contains("lacY"));
                assert!(pred_query.labels().contains("lacZ"));
                assert!(pred_query.conditioning_labels().is_empty());

                // Check parameter dimensions: coefficients should be 3x0, intercept 3, covariance 3x3.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[3, 0]);
                assert_eq!(params.intercept().len(), 3);
                assert_eq!(params.covariance().shape(), &[3, 3]);

                // Covariance matrix should be symmetric.
                for i in 0..3 {
                    for j in 0..3 {
                        assert_relative_eq!(
                            params.covariance()[[i, j]],
                            params.covariance()[[j, i]],
                            epsilon = 1e-10
                        );
                    }
                }

                // Diagonal elements should be positive (variances).
                for i in 0..3 {
                    assert!(params.covariance()[[i, i]] > 0.0);
                }

                Ok(())
            }

            #[test]
            fn estimate_three_variables_with_three_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;

                // Get variable indices.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacY")?;
                let x3_idx = model.label_to_index("lacZ")?;
                let z1_idx = model.label_to_index("asnA")?;
                let z2_idx = model.label_to_index("cspG")?;
                let z3_idx = model.label_to_index("eutG")?;

                // Predict P(lacA, lacY, lacZ | asnA, cspG, eutG).
                let pred_query = engine
                    .estimate(&set![x1_idx, x2_idx, x3_idx], &set![z1_idx, z2_idx, z3_idx])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 3);
                assert_eq!(pred_query.conditioning_labels().len(), 3);

                // Check parameter dimensions: coefficients should be 3x3, intercept 3, covariance 3x3.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[3, 3]);
                assert_eq!(params.intercept().len(), 3);
                assert_eq!(params.covariance().shape(), &[3, 3]);

                // Covariance matrix should be symmetric.
                for i in 0..3 {
                    for j in 0..3 {
                        assert_relative_eq!(
                            params.covariance()[[i, j]],
                            params.covariance()[[j, i]],
                            epsilon = 1e-10
                        );
                    }
                }

                // Diagonal elements should be positive (variances).
                for i in 0..3 {
                    assert!(params.covariance()[[i, i]] > 0.0);
                }

                Ok(())
            }

            // =========================================================================
            // Parallel estimation tests
            // =========================================================================

            #[test]
            fn par_estimate_empty_x() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Parallel predict with empty X should fail.
                assert!(engine.par_estimate(&set![], &set![]).is_err());

                Ok(())
            }

            #[test]
            fn par_estimate_non_disjoint_xz() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine.
                let engine = ApproximateInference::new(&mut rng, &model);

                // Parallel predict with overlapping X and Z should fail.
                assert!(engine.par_estimate(&set![0], &set![0]).is_err());

                Ok(())
            }

            #[test]
            fn par_estimate_two_variables_with_two_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;

                // Get variable indices.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacZ")?;
                let z1_idx = model.label_to_index("asnA")?;
                let z2_idx = model.label_to_index("cspG")?;

                // Parallel predict P(lacA, lacZ | asnA, cspG).
                let pred_query =
                    engine.par_estimate(&set![x1_idx, x2_idx], &set![z1_idx, z2_idx])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 2);
                assert_eq!(pred_query.conditioning_labels().len(), 2);

                // Check parameter dimensions.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[2, 2]);
                assert_eq!(params.intercept().len(), 2);
                assert_eq!(params.covariance().shape(), &[2, 2]);

                // Covariance matrix should be symmetric.
                assert_relative_eq!(
                    params.covariance()[[0, 1]],
                    params.covariance()[[1, 0]],
                    epsilon = 1e-10
                );

                // Diagonal elements should be positive (variances).
                assert!(params.covariance()[[0, 0]] > 0.0);
                assert!(params.covariance()[[1, 1]] > 0.0);

                Ok(())
            }

            #[test]
            fn par_estimate_three_variables_with_three_conditioning() -> Result<()> {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;
                // Initialize the inference engine with a fixed sample size.
                let engine =
                    ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;

                // Get variable indices.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacY")?;
                let x3_idx = model.label_to_index("lacZ")?;
                let z1_idx = model.label_to_index("asnA")?;
                let z2_idx = model.label_to_index("cspG")?;
                let z3_idx = model.label_to_index("eutG")?;

                // Parallel predict P(lacA, lacY, lacZ | asnA, cspG, eutG).
                let pred_query = engine
                    .par_estimate(&set![x1_idx, x2_idx, x3_idx], &set![z1_idx, z2_idx, z3_idx])?;

                // Check the CPD structure.
                assert_eq!(pred_query.labels().len(), 3);
                assert_eq!(pred_query.conditioning_labels().len(), 3);

                // Check parameter dimensions.
                let params = pred_query.parameters();
                assert_eq!(params.coefficients().shape(), &[3, 3]);
                assert_eq!(params.intercept().len(), 3);
                assert_eq!(params.covariance().shape(), &[3, 3]);

                // Covariance matrix should be symmetric.
                for i in 0..3 {
                    for j in 0..3 {
                        assert_relative_eq!(
                            params.covariance()[[i, j]],
                            params.covariance()[[j, i]],
                            epsilon = 1e-10
                        );
                    }
                }

                // Diagonal elements should be positive (variances).
                for i in 0..3 {
                    assert!(params.covariance()[[i, i]] > 0.0);
                }

                Ok(())
            }

            // =========================================================================
            // Consistency tests between sequential and parallel
            // =========================================================================

            #[test]
            fn estimate_vs_par_estimate_consistency() -> Result<()> {
                // Initialize RNG for sequential.
                let mut rng_seq = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize RNG for parallel.
                let mut rng_par = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_ecoli70()?;

                // Get variable indices.
                let x1_idx = model.label_to_index("lacA")?;
                let x2_idx = model.label_to_index("lacZ")?;
                let z_idx = model.label_to_index("asnA")?;

                // Sequential estimation.
                let engine_seq =
                    ApproximateInference::new(&mut rng_seq, &model).with_sample_size(5_000)?;
                let pred_seq = engine_seq.estimate(&set![x1_idx, x2_idx], &set![z_idx])?;

                // Parallel estimation.
                let engine_par =
                    ApproximateInference::new(&mut rng_par, &model).with_sample_size(5_000)?;
                let pred_par = engine_par.par_estimate(&set![x1_idx, x2_idx], &set![z_idx])?;

                // Both should have the same structure.
                assert_eq!(pred_seq.labels(), pred_par.labels());
                assert_eq!(
                    pred_seq.conditioning_labels(),
                    pred_par.conditioning_labels()
                );

                // Parameters should have the same shape.
                assert_eq!(
                    pred_seq.parameters().coefficients().shape(),
                    pred_par.parameters().coefficients().shape()
                );
                assert_eq!(
                    pred_seq.parameters().intercept().len(),
                    pred_par.parameters().intercept().len()
                );
                assert_eq!(
                    pred_seq.parameters().covariance().shape(),
                    pred_par.parameters().covariance().shape()
                );

                Ok(())
            }
        }
    }
}
