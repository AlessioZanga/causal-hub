#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_asia,
        estimators::{BIC, BNEstimator, HasEstimator, MLE},
        models::{BN, CatBN, CatCPD, HasLabels},
        samplers::{ForwardSampler, ParBNSampler},
        types::{Cache, Result},
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn score_has_estimator() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic: BIC<'_, _, CatCPD> = BIC::new(&cache);

        // Get the wrapped estimator from the scoring criterion.
        let estimator = bic.estimator();
        // Assert that the labels of the wrapped estimator match those of the score.
        assert_eq!(estimator.labels(), bic.labels());

        // Fit a BN over the true graph using the wrapped estimator ...
        let fitted: CatBN = BNEstimator::fit(estimator, model.graph().clone())?;
        // ... and assert it matches the fit through the original estimator.
        let expected: CatBN = cache.fit(model.graph().clone())?;
        assert_eq!(fitted, expected);

        Ok(())
    }
}
