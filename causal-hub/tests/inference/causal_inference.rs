#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        assets::load_asia,
        inference::{
            ApproximateInference, BNCausalInference, CausalInference, ParBNCausalInference,
        },
        models::{CatCPD, Labelled},
        set, states,
        types::{Error, Result},
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    mod categorical {
        use super::*;

        #[test]
        fn ace_estimate() -> Result<()> {
            // Load the model.
            let model = load_asia()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("bronc")?];
            let y = set![model.label_to_index("dysp")?];

            // Compute the ACE of "bronc" on "dysp".
            let pred_ace = engine.ace_estimate(&x, &y)?;

            // Set the true ACE.
            let true_x = states![("bronc", ["no", "yes"])];
            let true_y = states![("dysp", ["no", "yes"])];
            let true_p = array![
                [0.855219672329841, 0.14478032767015905],
                [0.2050478701174198, 0.7949521298825802]
            ];
            let true_ace = CatCPD::new(true_y, true_x, true_p)?;

            // Check that the ACE is correct.
            assert_relative_eq!(
                true_ace,
                pred_ace.ok_or(Error::IllegalArgument("No ACE".into()))?,
                epsilon = 1e-8
            );

            // Compute the ACE of "dysp" on "bronc".
            let pred_ace = engine.ace_estimate(&y, &x)?;

            // Check that the ACE does not exist.
            assert!(pred_ace.is_none());

            Ok(())
        }

        #[test]
        fn cace_estimate() -> Result<()> {
            // Load the model.
            let model = load_asia()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("smoke")?];
            let y = set![model.label_to_index("either")?];
            let z = set![model.label_to_index("asia")?];

            // Compute the ACE of "smoke" on "either" conditionally on "asia".
            let pred_ace = engine.cace_estimate(&x, &y, &z)?;

            // Set the true ACE.
            let true_x = states![("asia", ["no", "yes"]), ("smoke", ["no", "yes"])];
            let true_y = states![("either", ["no", "yes"])];
            let true_p = array![
                [0.978705636743215, 0.02129436325678497],
                [0.885738468049090, 0.11426153195090986],
                [0.909090909090909, 0.09090909090909091],
                [0.892857142857143, 0.10714285714285714]
            ];
            let true_ace = CatCPD::new(true_y, true_x, true_p)?;

            // Check that the ACE is correct.
            assert_relative_eq!(
                true_ace,
                pred_ace.ok_or(Error::IllegalArgument("No ACE".into()))?,
                epsilon = 1e-8
            );

            Ok(())
        }

        #[test]
        fn par_ace_estimate() -> Result<()> {
            // Load the model.
            let model = load_asia()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("bronc")?];
            let y = set![model.label_to_index("dysp")?];

            // Compute the ACE of "bronc" on "dysp".
            let pred_ace = engine.par_ace_estimate(&x, &y)?;

            // Set the true ACE.
            let true_x = states![("bronc", ["no", "yes"])];
            let true_y = states![("dysp", ["no", "yes"])];
            let true_p = array![
                [0.8522162481337745, 0.1477837518662255],
                [0.1872079354775384, 0.8127920645224616]
            ];
            let true_ace = CatCPD::new(true_y, true_x, true_p)?;

            // Check that the ACE is correct.
            assert_relative_eq!(
                true_ace,
                pred_ace.ok_or(Error::IllegalArgument("No ACE".into()))?,
                epsilon = 1e-8
            );

            Ok(())
        }

        #[test]
        fn par_cace_estimate() -> Result<()> {
            // Load the model.
            let model = load_asia()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("smoke")?];
            let y = set![model.label_to_index("either")?];
            let z = set![model.label_to_index("asia")?];

            // Compute the ACE of "smoke" on "either" conditionally on "asia".
            let pred_ace = engine.par_cace_estimate(&x, &y, &z)?;

            // Set the true ACE.
            let true_x = states![("asia", ["no", "yes"]), ("smoke", ["no", "yes"])];
            let true_y = states![("either", ["no", "yes"])];
            let true_p = array![
                [0.9763113367174281, 0.0236886632825719],
                [0.8872651356993737, 0.1127348643006263],
                [0.8695652173913043, 0.1304347826086957],
                [0.8076923076923077, 0.1923076923076923]
            ];
            let true_ace = CatCPD::new(true_y, true_x, true_p)?;

            // Check that the ACE is correct.
            assert_relative_eq!(
                true_ace,
                pred_ace.ok_or(Error::IllegalArgument("No ACE".into()))?,
                epsilon = 1e-8
            );

            Ok(())
        }
    }

    mod gaussian {
        use causal_hub::{assets::load_ecoli70, models::CPD};

        use super::*;

        // =========================================================================
        // Error handling tests
        // =========================================================================

        #[test]
        fn ace_estimate_empty_x() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let y = set![model.label_to_index("aceB")?];

            // ACE with empty X should fail.
            assert!(engine.ace_estimate(&set![], &y).is_err());

            Ok(())
        }

        #[test]
        fn ace_estimate_empty_y() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("icdA")?];

            // ACE with empty Y should fail.
            assert!(engine.ace_estimate(&x, &set![]).is_err());

            Ok(())
        }

        #[test]
        fn ace_estimate_non_disjoint_xy() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("aceB")?];

            // ACE with overlapping X and Y should fail.
            assert!(engine.ace_estimate(&x, &x).is_err());

            Ok(())
        }

        #[test]
        fn cace_estimate_non_disjoint_xz() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("icdA")?];
            let y = set![model.label_to_index("aceB")?];

            // CACE with overlapping X and Z should fail.
            assert!(engine.cace_estimate(&x, &y, &x).is_err());

            Ok(())
        }

        #[test]
        fn cace_estimate_non_disjoint_yz() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("icdA")?];
            let y = set![model.label_to_index("aceB")?];

            // CACE with overlapping Y and Z should fail.
            assert!(engine.cace_estimate(&x, &y, &y).is_err());

            Ok(())
        }

        // =========================================================================
        // Single variable ACE tests (1D X -> 1D Y)
        // =========================================================================

        #[test]
        fn ace_estimate_single_cause_single_effect() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(10_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables: icdA -> aceB (direct causal relationship).
            let x = set![model.label_to_index("icdA")?];
            let y = set![model.label_to_index("aceB")?];

            // Compute the ACE of icdA on aceB.
            let pred_ace = engine.ace_estimate(&x, &y)?;

            // The ACE should exist because icdA is a parent of aceB.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 1);
            assert!(ace.labels().contains("aceB"));
            assert_eq!(ace.conditioning_labels().len(), 1);
            assert!(ace.conditioning_labels().contains("icdA"));

            // Check parameter dimensions: coefficients should be 1x1.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[1, 1]);
            assert_eq!(params.intercept().len(), 1);
            assert_eq!(params.covariance().shape(), &[1, 1]);

            // Check numerical values against the true model parameters.
            // True values from ecoli70: coefficient=1.0464, intercept=0.1324, variance=0.0853.
            assert_relative_eq!(params.coefficients()[[0, 0]], 1.0464, epsilon = 0.3);
            assert_relative_eq!(params.intercept()[0], 0.1324, epsilon = 0.2);
            assert_relative_eq!(params.covariance()[[0, 0]], 0.0853, epsilon = 0.1);

            Ok(())
        }

        #[test]
        fn ace_estimate_no_causal_effect() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine.
            let engine = ApproximateInference::new(&mut rng, &model);
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables: aceB -> icdA (reversed, aceB is not a cause of icdA).
            let x = set![model.label_to_index("aceB")?];
            let y = set![model.label_to_index("icdA")?];

            // Compute the ACE of aceB on icdA.
            let pred_ace = engine.ace_estimate(&x, &y)?;

            // The ACE should not exist because aceB is not a cause of icdA.
            assert!(pred_ace.is_none());

            Ok(())
        }

        // =========================================================================
        // Multi-variable ACE tests (higher dimensional)
        // =========================================================================

        #[test]
        fn ace_estimate_single_cause_two_effects() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables: sucA affects multiple variables (gltA, flgD, etc.).
            let x = set![model.label_to_index("sucA")?];
            let y1_idx = model.label_to_index("gltA")?;
            let y2_idx = model.label_to_index("flgD")?;
            let y = set![y1_idx, y2_idx];

            // Compute the ACE of sucA on (gltA, flgD).
            let pred_ace = engine.ace_estimate(&x, &y)?;

            // The ACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 2);
            assert!(ace.labels().contains("gltA"));
            assert!(ace.labels().contains("flgD"));
            assert_eq!(ace.conditioning_labels().len(), 1);
            assert!(ace.conditioning_labels().contains("sucA"));

            // Check parameter dimensions: coefficients should be 2x1.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[2, 1]);
            assert_eq!(params.intercept().len(), 2);
            assert_eq!(params.covariance().shape(), &[2, 2]);

            // The labels are returned in sorted order, check which is first.
            let labels: Vec<_> = ace.labels().iter().cloned().collect();
            let (glt_a_row, flg_d_row) = if labels[0] == "flgD" { (1, 0) } else { (0, 1) };

            // Check numerical values against the true model parameters.
            // True values from ecoli70:
            //   ACE(sucA -> gltA) = 0.379
            //   ACE(sucA -> flgD) = 0.6362
            assert_relative_eq!(params.coefficients()[[glt_a_row, 0]], 0.379, epsilon = 0.2);
            assert_relative_eq!(params.coefficients()[[flg_d_row, 0]], 0.6362, epsilon = 0.2);

            // Covariance should be symmetric.
            assert_relative_eq!(
                params.covariance()[[0, 1]],
                params.covariance()[[1, 0]],
                epsilon = 1e-10
            );

            // Off-diagonal covariance should be close to zero (gltA and flgD are
            // conditionally independent given sucA).
            assert_relative_eq!(
                params.covariance()[[glt_a_row, flg_d_row]],
                0.0,
                epsilon = 0.15
            );

            Ok(())
        }

        #[test]
        fn ace_estimate_two_causes_single_effect() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables: asnA and ygcE both affect icdA.
            let x1_idx = model.label_to_index("asnA")?;
            let x2_idx = model.label_to_index("ygcE")?;
            let x = set![x1_idx, x2_idx];
            let y = set![model.label_to_index("icdA")?];

            // Compute the ACE of (asnA, ygcE) on icdA.
            let pred_ace = engine.ace_estimate(&x, &y)?;

            // The ACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 1);
            assert!(ace.labels().contains("icdA"));
            assert_eq!(ace.conditioning_labels().len(), 2);
            assert!(ace.conditioning_labels().contains("asnA"));
            assert!(ace.conditioning_labels().contains("ygcE"));

            // Check parameter dimensions: coefficients should be 1x2.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[1, 2]);
            assert_eq!(params.intercept().len(), 1);
            assert_eq!(params.covariance().shape(), &[1, 1]);

            // Check numerical values against the true model parameters.
            // True values from ecoli70: icdA | asnA, ygcE
            //   coef(asnA)=0.5228, coef(ygcE)=-1.0585, intercept=-0.4155, var=0.3179
            let cond_labels: Vec<_> = ace.conditioning_labels().iter().cloned().collect();
            let asn_a_col = cond_labels.iter().position(|x| x == "asnA").unwrap();
            let ygc_e_col = cond_labels.iter().position(|x| x == "ygcE").unwrap();

            assert_relative_eq!(params.coefficients()[[0, asn_a_col]], 0.5228, epsilon = 0.2);
            assert_relative_eq!(
                params.coefficients()[[0, ygc_e_col]],
                -1.0585,
                epsilon = 0.25
            );
            assert_relative_eq!(params.intercept()[0], -0.4155, epsilon = 0.2);
            assert_relative_eq!(params.covariance()[[0, 0]], 0.3179, epsilon = 0.15);

            Ok(())
        }

        #[test]
        fn ace_estimate_two_causes_two_effects() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(20_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables: sucA affects multiple outputs (gltA, flgD).
            // Using sucA and eutG as causes since eutG is a parent of sucA.
            let x1_idx = model.label_to_index("eutG")?;
            let x2_idx = model.label_to_index("sucA")?;
            let x = set![x1_idx, x2_idx];
            let y1_idx = model.label_to_index("gltA")?;
            let y2_idx = model.label_to_index("flgD")?;
            let y = set![y1_idx, y2_idx];

            // Compute the ACE of (eutG, sucA) on (gltA, flgD).
            let pred_ace = engine.ace_estimate(&x, &y)?;

            // The ACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 2);
            assert!(ace.labels().contains("gltA"));
            assert!(ace.labels().contains("flgD"));
            assert_eq!(ace.conditioning_labels().len(), 2);
            assert!(ace.conditioning_labels().contains("eutG"));
            assert!(ace.conditioning_labels().contains("sucA"));

            // Check parameter dimensions: coefficients should be 2x2.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[2, 2]);
            assert_eq!(params.intercept().len(), 2);
            assert_eq!(params.covariance().shape(), &[2, 2]);

            // Identify row/column indices based on label ordering.
            let labels: Vec<_> = ace.labels().iter().cloned().collect();
            let cond_labels: Vec<_> = ace.conditioning_labels().iter().cloned().collect();
            let glt_a_row = labels.iter().position(|x| x == "gltA").unwrap();
            let flg_d_row = labels.iter().position(|x| x == "flgD").unwrap();
            let eut_g_col = cond_labels.iter().position(|x| x == "eutG").unwrap();
            let suc_a_col = cond_labels.iter().position(|x| x == "sucA").unwrap();

            // Check numerical values. Graph: eutG -> sucA -> {gltA, flgD}
            // When we do(eutG, sucA), eutG has no direct effect on gltA/flgD.
            // True values: ACE(sucA -> gltA) = 0.379, ACE(sucA -> flgD) = 0.6362
            assert_relative_eq!(
                params.coefficients()[[glt_a_row, eut_g_col]],
                0.0,
                epsilon = 0.2
            );
            assert_relative_eq!(
                params.coefficients()[[flg_d_row, eut_g_col]],
                0.0,
                epsilon = 0.2
            );
            assert_relative_eq!(
                params.coefficients()[[glt_a_row, suc_a_col]],
                0.379,
                epsilon = 0.2
            );
            assert_relative_eq!(
                params.coefficients()[[flg_d_row, suc_a_col]],
                0.6362,
                epsilon = 0.2
            );

            // Covariance should be symmetric.
            assert_relative_eq!(
                params.covariance()[[0, 1]],
                params.covariance()[[1, 0]],
                epsilon = 1e-10
            );

            // Off-diagonal covariance should be approximately zero (conditional independence).
            assert_relative_eq!(
                params.covariance()[[glt_a_row, flg_d_row]],
                0.0,
                epsilon = 0.15
            );

            Ok(())
        }

        // =========================================================================
        // Conditional ACE (CACE) tests
        // =========================================================================

        #[test]
        fn cace_estimate_single_cause_single_effect_single_conditioning() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("sucA")?];
            let y = set![model.label_to_index("gltA")?];
            let z = set![model.label_to_index("eutG")?];

            // Compute the CACE of sucA on gltA given eutG.
            let pred_ace = engine.cace_estimate(&x, &y, &z)?;

            // The CACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 1);
            assert!(ace.labels().contains("gltA"));
            // Conditioning should include sucA (the do variable).
            assert!(ace.conditioning_labels().contains("sucA"));

            // Check that parameters have valid dimensions.
            let params = ace.parameters();
            assert!(params.coefficients().shape()[0] == 1);
            assert_eq!(params.intercept().len(), 1);
            assert_eq!(params.covariance().shape(), &[1, 1]);

            // Check numerical values. gltA depends only on sucA, so CACE equals ACE.
            // True value: ACE(sucA -> gltA) = 0.379, variance = 0.6895
            let cond_labels: Vec<_> = ace.conditioning_labels().iter().cloned().collect();
            let suc_a_col = cond_labels.iter().position(|x| x == "sucA").unwrap();
            assert_relative_eq!(params.coefficients()[[0, suc_a_col]], 0.379, epsilon = 0.2);
            assert_relative_eq!(params.covariance()[[0, 0]], 0.6895, epsilon = 0.2);

            Ok(())
        }

        #[test]
        fn cace_estimate_two_causes_two_effects_single_conditioning() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x1_idx = model.label_to_index("asnA")?;
            let x2_idx = model.label_to_index("cspG")?;
            let x = set![x1_idx, x2_idx];
            let y1_idx = model.label_to_index("lacA")?;
            let y2_idx = model.label_to_index("lacY")?;
            let y = set![y1_idx, y2_idx];
            let z = set![model.label_to_index("eutG")?];

            // Compute the CACE of (asnA, cspG) on (lacA, lacY) given eutG.
            let pred_ace = engine.cace_estimate(&x, &y, &z)?;

            // The CACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 2);
            assert!(ace.labels().contains("lacA"));
            assert!(ace.labels().contains("lacY"));

            // Check parameter dimensions: coefficients should be 2x(2+1) or similar depending on adjustment set.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape()[0], 2);
            assert_eq!(params.intercept().len(), 2);
            assert_eq!(params.covariance().shape(), &[2, 2]);

            // Covariance should be symmetric.
            assert_relative_eq!(
                params.covariance()[[0, 1]],
                params.covariance()[[1, 0]],
                epsilon = 1e-10
            );

            Ok(())
        }

        // =========================================================================
        // Parallel ACE tests
        // =========================================================================

        #[test]
        fn par_ace_estimate_single_cause_single_effect() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x = set![model.label_to_index("icdA")?];
            let y = set![model.label_to_index("aceB")?];

            // Compute the ACE in parallel.
            let pred_ace = engine.par_ace_estimate(&x, &y)?;

            // The ACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 1);
            assert!(ace.labels().contains("aceB"));
            assert_eq!(ace.conditioning_labels().len(), 1);
            assert!(ace.conditioning_labels().contains("icdA"));

            // Check parameter dimensions.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[1, 1]);
            assert_eq!(params.intercept().len(), 1);
            assert_eq!(params.covariance().shape(), &[1, 1]);

            // Check numerical values against true model parameters.
            // True values: coef=1.0464, intercept=0.1324, var=0.0853
            assert_relative_eq!(params.coefficients()[[0, 0]], 1.0464, epsilon = 0.25);
            assert_relative_eq!(params.intercept()[0], 0.1324, epsilon = 0.15);
            assert_relative_eq!(params.covariance()[[0, 0]], 0.0853, epsilon = 0.1);

            Ok(())
        }

        #[test]
        fn par_ace_estimate_two_causes_two_effects() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(20_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables: using eutG and sucA as causes, gltA and flgD as effects.
            let x1_idx = model.label_to_index("eutG")?;
            let x2_idx = model.label_to_index("sucA")?;
            let x = set![x1_idx, x2_idx];
            let y1_idx = model.label_to_index("gltA")?;
            let y2_idx = model.label_to_index("flgD")?;
            let y = set![y1_idx, y2_idx];

            // Compute the ACE in parallel.
            let pred_ace = engine.par_ace_estimate(&x, &y)?;

            // The ACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 2);
            assert!(ace.labels().contains("gltA"));
            assert!(ace.labels().contains("flgD"));
            assert_eq!(ace.conditioning_labels().len(), 2);

            // Check parameter dimensions.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[2, 2]);
            assert_eq!(params.intercept().len(), 2);
            assert_eq!(params.covariance().shape(), &[2, 2]);

            // Identify row/column indices based on label ordering.
            let labels: Vec<_> = ace.labels().iter().cloned().collect();
            let cond_labels: Vec<_> = ace.conditioning_labels().iter().cloned().collect();
            let glt_a_row = labels.iter().position(|x| x == "gltA").unwrap();
            let flg_d_row = labels.iter().position(|x| x == "flgD").unwrap();
            let suc_a_col = cond_labels.iter().position(|x| x == "sucA").unwrap();

            // Check numerical values for the direct causal effects.
            // True values: ACE(sucA -> gltA) = 0.379, ACE(sucA -> flgD) = 0.6362
            assert_relative_eq!(
                params.coefficients()[[glt_a_row, suc_a_col]],
                0.379,
                epsilon = 0.2
            );
            assert_relative_eq!(
                params.coefficients()[[flg_d_row, suc_a_col]],
                0.6362,
                epsilon = 0.2
            );

            // Covariance should be symmetric.
            assert_relative_eq!(
                params.covariance()[[0, 1]],
                params.covariance()[[1, 0]],
                epsilon = 1e-10
            );

            Ok(())
        }

        #[test]
        fn par_cace_estimate_two_causes_two_effects_single_conditioning() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(15_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x1_idx = model.label_to_index("asnA")?;
            let x2_idx = model.label_to_index("cspG")?;
            let x = set![x1_idx, x2_idx];
            let y1_idx = model.label_to_index("lacA")?;
            let y2_idx = model.label_to_index("lacY")?;
            let y = set![y1_idx, y2_idx];
            let z = set![model.label_to_index("eutG")?];

            // Compute the CACE in parallel.
            let pred_ace = engine.par_cace_estimate(&x, &y, &z)?;

            // The CACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 2);
            assert!(ace.labels().contains("lacA"));
            assert!(ace.labels().contains("lacY"));

            // Check parameter dimensions.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape()[0], 2);
            assert_eq!(params.intercept().len(), 2);
            assert_eq!(params.covariance().shape(), &[2, 2]);

            Ok(())
        }

        // =========================================================================
        // Consistency tests between sequential and parallel
        // =========================================================================

        #[test]
        fn ace_estimate_vs_par_ace_estimate_consistency() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize RNGs.
            let mut rng_seq = Xoshiro256PlusPlus::seed_from_u64(42);
            let mut rng_par = Xoshiro256PlusPlus::seed_from_u64(42);

            // Set variables.
            let x = set![model.label_to_index("icdA")?];
            let y = set![model.label_to_index("aceB")?];

            // Sequential estimation.
            let engine_seq =
                ApproximateInference::new(&mut rng_seq, &model).with_sample_size(5_000)?;
            let causal_seq = CausalInference::new(&engine_seq);
            let pred_seq = causal_seq.ace_estimate(&x, &y)?;

            // Parallel estimation.
            let engine_par =
                ApproximateInference::new(&mut rng_par, &model).with_sample_size(5_000)?;
            let causal_par = CausalInference::new(&engine_par);
            let pred_par = causal_par.par_ace_estimate(&x, &y)?;

            // Both should have the same existence.
            assert_eq!(pred_seq.is_some(), pred_par.is_some());

            if let (Some(seq), Some(par)) = (pred_seq, pred_par) {
                // Both should have the same structure.
                assert_eq!(seq.labels(), par.labels());
                assert_eq!(seq.conditioning_labels(), par.conditioning_labels());

                // Parameters should have the same shape.
                assert_eq!(
                    seq.parameters().coefficients().shape(),
                    par.parameters().coefficients().shape()
                );
                assert_eq!(
                    seq.parameters().intercept().len(),
                    par.parameters().intercept().len()
                );
                assert_eq!(
                    seq.parameters().covariance().shape(),
                    par.parameters().covariance().shape()
                );
            }

            Ok(())
        }

        // =========================================================================
        // High-dimensional tests (3+ variables)
        // =========================================================================

        #[test]
        fn ace_estimate_three_causes_three_effects() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(20_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables: three causes and three effects in the lac operon cluster.
            let x1_idx = model.label_to_index("asnA")?;
            let x2_idx = model.label_to_index("cspG")?;
            let x3_idx = model.label_to_index("eutG")?;
            let x = set![x1_idx, x2_idx, x3_idx];
            let y1_idx = model.label_to_index("lacA")?;
            let y2_idx = model.label_to_index("lacY")?;
            let y3_idx = model.label_to_index("lacZ")?;
            let y = set![y1_idx, y2_idx, y3_idx];

            // Compute the ACE of (asnA, cspG, eutG) on (lacA, lacY, lacZ).
            let pred_ace = engine.ace_estimate(&x, &y)?;

            // The ACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 3);
            assert!(ace.labels().contains("lacA"));
            assert!(ace.labels().contains("lacY"));
            assert!(ace.labels().contains("lacZ"));
            assert_eq!(ace.conditioning_labels().len(), 3);

            // Check parameter dimensions: coefficients should be 3x3.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[3, 3]);
            assert_eq!(params.intercept().len(), 3);
            assert_eq!(params.covariance().shape(), &[3, 3]);

            // Covariance should be symmetric.
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
        fn par_ace_estimate_three_causes_three_effects() -> Result<()> {
            // Load the model.
            let model = load_ecoli70()?;

            // Initialize a RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the inference engine with fixed sample size.
            let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(20_000)?;
            // Initialize the causal inference engine.
            let engine = CausalInference::new(&engine);

            // Set variables.
            let x1_idx = model.label_to_index("asnA")?;
            let x2_idx = model.label_to_index("cspG")?;
            let x3_idx = model.label_to_index("eutG")?;
            let x = set![x1_idx, x2_idx, x3_idx];
            let y1_idx = model.label_to_index("lacA")?;
            let y2_idx = model.label_to_index("lacY")?;
            let y3_idx = model.label_to_index("lacZ")?;
            let y = set![y1_idx, y2_idx, y3_idx];

            // Compute the ACE in parallel.
            let pred_ace = engine.par_ace_estimate(&x, &y)?;

            // The ACE should exist.
            assert!(pred_ace.is_some());

            let ace = pred_ace.unwrap();

            // Check the CPD structure.
            assert_eq!(ace.labels().len(), 3);
            assert_eq!(ace.conditioning_labels().len(), 3);

            // Check parameter dimensions.
            let params = ace.parameters();
            assert_eq!(params.coefficients().shape(), &[3, 3]);
            assert_eq!(params.intercept().len(), 3);
            assert_eq!(params.covariance().shape(), &[3, 3]);

            Ok(())
        }
    }
}
