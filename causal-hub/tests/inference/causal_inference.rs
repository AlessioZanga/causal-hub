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
