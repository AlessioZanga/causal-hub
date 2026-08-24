#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_eating,
        estimators::{CTBNEstimator, ChiSquaredTest, FTest, HasEstimator, MLE},
        models::{CTBN, CatCTBN, HasLabels},
        samplers::{ForwardSampler, ParCTBNSampler},
        types::{Cache, Result},
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn ci_test_has_estimator() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Eating.
        let model = load_eating()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 1000 samples of length 100.
        let dataset = forward.par_sample_n_by_length(100, 100)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the F test and the chi-squared test.
        let f_test = FTest::new(&cache, 0.01)?;
        let chi_sq_test = ChiSquaredTest::new(&cache, 0.01)?;

        // Get the wrapped estimator from both tests.
        let f_estimator = f_test.estimator();
        let chi_sq_estimator = chi_sq_test.estimator();
        // Assert that the labels of the wrapped estimators match those of the tests.
        assert_eq!(f_estimator.labels(), f_test.labels());
        assert_eq!(chi_sq_estimator.labels(), chi_sq_test.labels());

        // Fit a CTBN over the true graph using the wrapped estimator ...
        let fitted: CatCTBN = f_estimator.fit(model.graph().clone())?;
        // ... and assert it matches the fit through the original estimator.
        let expected: CatCTBN = cache.fit(model.graph().clone())?;
        assert_eq!(fitted, expected);

        Ok(())
    }
}
