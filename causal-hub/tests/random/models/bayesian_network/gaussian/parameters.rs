#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{CPD, GaussCPD, HasLabels},
        random::{Random, RngGaussCPD},
        types::Result,
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2"];
        let conditioning_labels = labels!["Z1", "Z2", "Z3"];
        let s_a = 0.5;
        let s_b = 1.0;
        let evidence = 1e-2;

        let res = RngGaussCPD::new(&mut rng, &labels, &conditioning_labels, s_a, s_b, evidence);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2"];
        let conditioning_labels = labels!["Z1", "Z2", "Z3"];
        let s_a = 0.5;
        let s_b = 1.0;
        let evidence = 1e-2;

        let mut rng_cpd =
            RngGaussCPD::new(&mut rng, &labels, &conditioning_labels, s_a, s_b, evidence)?;
        let distribution: GaussCPD = rng_cpd.random()?;

        assert_eq!(distribution.labels(), &labels);
        assert_eq!(distribution.conditioning_labels(), &conditioning_labels);

        Ok(())
    }
}
