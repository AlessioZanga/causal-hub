#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{CatBN, Labelled},
        random::{Random, RngCatBN},
        support,
        types::{Labels, Result},
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let support = support![("A", ["0", "1"]), ("B", ["a", "b", "c"])];
        let alpha = 1.0;
        let probability = 0.5;

        let res = RngCatBN::new(&mut rng, &support, alpha, probability);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let support = support![("A", ["0", "1"]), ("B", ["a", "b", "c"])];
        let alpha = 1.0;
        let probability = 0.5;

        let mut rng_bn = RngCatBN::new(&mut rng, &support, alpha, probability)?;
        let bayesian_network: CatBN = rng_bn.random()?;

        assert_eq!(bayesian_network.support(), &support);
        assert_eq!(
            bayesian_network.labels(),
            &support.keys().cloned().collect::<Labels>()
        );

        Ok(())
    }
}
