#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{CatBN, Labelled},
        random::{Random, RngCatBN},
        states,
        types::{Labels, Result},
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["a", "b", "c"])];
        let alpha = 1.0;
        let p = 0.5;

        let res = RngCatBN::new(&mut rng, &states, alpha, p);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["a", "b", "c"])];
        let alpha = 1.0;
        let p = 0.5;

        let mut rng_bn = RngCatBN::new(&mut rng, &states, alpha, p)?;
        let bn: CatBN = rng_bn.random()?;

        assert_eq!(bn.states(), &states);
        assert_eq!(bn.labels(), &states.keys().cloned().collect::<Labels>());

        Ok(())
    }
}
