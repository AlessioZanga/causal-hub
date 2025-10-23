#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_eating,
        estimators::{CTPC, ChiSquaredTest, FTest, MLE},
        models::{CTBN, DiGraph, Graph, Labelled},
        samplers::{ForwardSampler, ParCTBNSampler},
        types::Cache,
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn ctpc_fit() {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Eating.
        let model = load_eating();
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model);
        // Sample 1000 samples.
        let dataset = forward.par_sample_n_by_length(100, 100);

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the F test.
        let f_test = FTest::new(&cache, 0.01);
        // Initialize the chi-squared test.
        let chi_sq_test = ChiSquaredTest::new(&cache, 0.01);

        // Set the initial graph.
        let initial_graph = DiGraph::complete(dataset.labels());
        // Initialize the CTPC algorithm.
        let ctpc = CTPC::new(&initial_graph, &f_test, &chi_sq_test);
        // Run the CTPC algorithm.
        let fitted_graph = ctpc.fit();

        // Assert that the fitted model is equal to the original model.
        assert_eq!(model.graph(), &fitted_graph);
    }

    #[test]
    fn ctpc_par_fit() {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Eating.
        let model = load_eating();
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model);
        // Sample 1000 samples.
        let dataset = forward.par_sample_n_by_length(100, 1_000);

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the F test.
        let f_test = FTest::new(&cache, 0.01);
        // Initialize the chi-squared test.
        let chi_sq_test = ChiSquaredTest::new(&cache, 0.01);

        // Set the initial graph.
        let initial_graph = DiGraph::complete(dataset.labels());
        // Initialize the CTPC algorithm.
        let ctpc = CTPC::new(&initial_graph, &f_test, &chi_sq_test);
        // Run the CTPC algorithm.
        let fitted_graph = ctpc.par_fit();

        // Assert that the fitted model is equal to the original model.
        assert_eq!(model.graph(), &fitted_graph);
    }
}
