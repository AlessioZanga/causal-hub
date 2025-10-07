#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        assets::load_asia,
        inference::{ApproximateInference, BNCausalInference, CausalInference},
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
        let true_x = states![("bronc", ["no", "yes"]),];
        let true_y = states![("dysp", ["no", "yes"]),];
        let true_p = array![[0.8675616185, 0.1324383815], [0.1824193800, 0.8175806200]];
        let true_ace = CatCPD::new(true_y, true_x, true_p);

        // Check that the ACE is correct.
        assert_relative_eq!(true_ace, pred_ace.unwrap(), epsilon = 1e-8);

        // Compute the ACE of "dysp" on "bronc".
        let pred_ace = engine.ace_estimate(&y, &x);

        // Check that the ACE does not exist.
        assert!(pred_ace.is_none());
    }
}
