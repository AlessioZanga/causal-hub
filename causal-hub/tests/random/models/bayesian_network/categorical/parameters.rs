#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{CPD, CatCPD},
        random::{Random, RngCatCPD},
        states,
        types::Result,
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"])];
        let conditioning_states = states![];
        let alpha = 1.0;

        let res = RngCatCPD::new(&mut rng, &states, &conditioning_states, alpha);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn new_invalid_alpha() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"])];
        let conditioning_states = states![];
        let alpha = 0.0;

        let res = RngCatCPD::new(&mut rng, &states, &conditioning_states, alpha);
        match res {
            Err(err) => assert_eq!(err.to_string(), "Invalid parameter alpha: must be positive"),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"])];
        let conditioning_states = states![("B", ["no", "yes"])];
        let alpha = 1.0;

        let mut rng_cpd = RngCatCPD::new(&mut rng, &states, &conditioning_states, alpha)?;
        let cpd: CatCPD = rng_cpd.random()?;

        assert_eq!(cpd.states(), &states);
        assert_eq!(cpd.conditioning_states(), &conditioning_states);

        // Parameters should sum to 1.
        for row in cpd.parameters().rows() {
            let sum: f64 = row.iter().sum();
            assert!((sum - 1.0).abs() < 1e-10);
        }

        Ok(())
    }
}
