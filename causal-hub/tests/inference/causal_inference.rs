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
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn ace_estimate() {
        // Load the model.
        let model = load_asia();

        // Initialize a RNG.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Initialize the inference engine.
        let engine = ApproximateInference::new(&mut rng, &model);
        // Initialize the causal inference engine.
        let engine = CausalInference::new(&engine);

        // Set variables.
        let x = set![model.label_to_index("bronc")];
        let y = set![model.label_to_index("dysp")];

        // Compute the ACE of "bronc" on "dysp".
        let pred_ace = engine.ace_estimate(&x, &y);

        // Set the true ACE.
        let true_x = states![("bronc", ["no", "yes"])];
        let true_y = states![("dysp", ["no", "yes"])];
        let true_p = array![[0.8675616185, 0.1324383815], [0.1824193800, 0.8175806200]];
        let true_ace = CatCPD::new(true_y, true_x, true_p);

        // Check that the ACE is correct.
        assert_relative_eq!(true_ace, pred_ace.unwrap(), epsilon = 1e-8);

        // Compute the ACE of "dysp" on "bronc".
        let pred_ace = engine.ace_estimate(&y, &x);

        // Check that the ACE does not exist.
        assert!(pred_ace.is_none());
    }

    #[test]
    fn cace_estimate() {
        // Load the model.
        let model = load_asia();

        // Initialize a RNG.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Initialize the inference engine.
        let engine = ApproximateInference::new(&mut rng, &model);
        // Initialize the causal inference engine.
        let engine = CausalInference::new(&engine);

        // Set variables.
        let x = set![model.label_to_index("smoke")];
        let y = set![model.label_to_index("either")];
        let z = set![model.label_to_index("asia")];

        // Compute the ACE of "smoke" on "either" conditionally on "asia".
        let pred_ace = engine.cace_estimate(&x, &y, &z);

        // Set the true ACE.
        let true_x = states![("asia", ["no", "yes"]), ("smoke", ["no", "yes"])];
        let true_y = states![("either", ["no", "yes"])];
        let true_p = array![
            [0.9765297569, 0.0234702431],
            [0.8876689189, 0.1123310811],
            [1.0000000000, 0.0000000000],
            [0.8260869565, 0.1739130435]
        ];
        let true_ace = CatCPD::new(true_y, true_x, true_p);

        // Check that the ACE is correct.
        assert_relative_eq!(true_ace, pred_ace.unwrap(), epsilon = 1e-8);
    }

    #[test]
    fn par_ace_estimate() {
        // Load the model.
        let model = load_asia();

        // Initialize a RNG.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Initialize the inference engine.
        let engine = ApproximateInference::new(&mut rng, &model);
        // Initialize the causal inference engine.
        let engine = CausalInference::new(&engine);

        // Set variables.
        let x = set![model.label_to_index("bronc")];
        let y = set![model.label_to_index("dysp")];

        // Compute the ACE of "bronc" on "dysp".
        let pred_ace = engine.par_ace_estimate(&x, &y);

        // Set the true ACE.
        let true_x = states![("bronc", ["no", "yes"])];
        let true_y = states![("dysp", ["no", "yes"])];
        let true_p = array![[0.8553890497, 0.1446109503], [0.1864779006, 0.8135220995]];
        let true_ace = CatCPD::new(true_y, true_x, true_p);

        // Check that the ACE is correct.
        assert_relative_eq!(true_ace, pred_ace.unwrap(), epsilon = 1e-8);
    }

    #[test]
    fn par_cace_estimate() {
        // Load the model.
        let model = load_asia();

        // Initialize a RNG.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Initialize the inference engine.
        let engine = ApproximateInference::new(&mut rng, &model);
        // Initialize the causal inference engine.
        let engine = CausalInference::new(&engine);

        // Set variables.
        let x = set![model.label_to_index("smoke")];
        let y = set![model.label_to_index("either")];
        let z = set![model.label_to_index("asia")];

        // Compute the ACE of "smoke" on "either" conditionally on "asia".
        let pred_ace = engine.par_cace_estimate(&x, &y, &z);

        // Set the true ACE.
        let true_x = states![("asia", ["no", "yes"]), ("smoke", ["no", "yes"])];
        let true_y = states![("either", ["no", "yes"])];
        let true_p = array![
            [0.9805249788, 0.0194750212],
            [0.8792310907, 0.1207689093],
            [0.9523809524, 0.0476190476],
            [0.7083333333, 0.2916666667]
        ];
        let true_ace = CatCPD::new(true_y, true_x, true_p);

        // Check that the ACE is correct.
        assert_relative_eq!(true_ace, pred_ace.unwrap(), epsilon = 1e-8);
    }
}
