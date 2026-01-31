#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{CPD, GaussCPD, Labelled},
        random::{Random, RngGaussCPD},
        set,
        types::Result,
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = set!["X1".to_string(), "X2".to_string()];
        let conditioning_labels = set!["Z1".to_string(), "Z2".to_string(), "Z3".to_string()];
        let s_a = 0.5;
        let s_b = 1.0;
        let e = 1e-2;

        let res = RngGaussCPD::new(&mut rng, &labels, &conditioning_labels, s_a, s_b, e);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = set!["X1".to_string(), "X2".to_string()];
        let conditioning_labels = set!["Z1".to_string(), "Z2".to_string(), "Z3".to_string()];
        let s_a = 0.5;
        let s_b = 1.0;
        let e = 1e-2;

        let mut rng_cpd = RngGaussCPD::new(&mut rng, &labels, &conditioning_labels, s_a, s_b, e)?;
        let cpd: GaussCPD = rng_cpd.random()?;

        assert_eq!(cpd.labels(), &labels);
        assert_eq!(cpd.conditioning_labels(), &conditioning_labels);

        Ok(())
    }
}
