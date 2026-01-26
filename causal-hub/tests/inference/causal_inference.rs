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
            [0.8669638384987691, 0.1330361615012309],
            [0.18308109137894216, 0.8169189086210579]
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
            [0.9761306532663316, 0.02386934673366834],
            [0.8873417721518987, 0.11265822784810127],
            [0.96, 0.04],
            [0.8, 0.2]
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
            [0.8548237217683241, 0.145176278231676],
            [0.18712474554378591, 0.8128752544562141]
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
            [0.9801184433164128, 0.01988155668358714],
            [0.8789144050104384, 0.12108559498956159],
            [0.9130434782608695, 0.08695652173913043],
            [0.6923076923076923, 0.3076923076923077]
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
