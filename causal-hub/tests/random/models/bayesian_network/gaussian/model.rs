#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{GaussBN, Labelled},
        random::{Random, RngGaussBN},
        types::{Error, Result},
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2", "X3"];
        let (s_a, s_b, e, p) = (1.0, 1.0, 1e-6, 0.5);

        let res = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn new_invalid_s_a() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2", "X3"];
        let (s_a, s_b, e, p) = (0.0, 1.0, 1e-6, 0.5);

        let res = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p);
        assert!(matches!(
            res,
            Err(Error::InvalidParameter(ref p, ref m)) if p == "s_a" && m == "must be positive"
        ));

        Ok(())
    }

    #[test]
    fn new_invalid_s_b() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2", "X3"];
        let (s_a, s_b, e, p) = (1.0, 0.0, 1e-6, 0.5);

        let res = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p);
        assert!(matches!(
            res,
            Err(Error::InvalidParameter(ref p, ref m)) if p == "s_b" && m == "must be positive"
        ));

        Ok(())
    }

    #[test]
    fn new_invalid_e() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2", "X3"];
        let (s_a, s_b, e, p) = (1.0, 1.0, 0.0, 0.5);

        let res = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p);
        assert!(matches!(
            res,
            Err(Error::InvalidParameter(ref p, ref m)) if p == "e" && m == "must be positive"
        ));

        Ok(())
    }

    #[test]
    fn new_invalid_p() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2", "X3"];

        let res = RngGaussBN::new(&mut rng, &labels, 1.0, 1.0, 1e-6, -0.1);
        assert!(matches!(
            res,
            Err(Error::InvalidParameter(ref p, ref m)) if p == "p" && m == "must be in [0, 1]"
        ));

        let res = RngGaussBN::new(&mut rng, &labels, 1.0, 1.0, 1e-6, 1.1);
        assert!(matches!(
            res,
            Err(Error::InvalidParameter(ref p, ref m)) if p == "p" && m == "must be in [0, 1]"
        ));

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["X1", "X2", "X3"];
        let (s_a, s_b, e, p) = (1.0, 1.0, 1e-6, 0.5);

        let mut rng_bn = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p)?;
        let bn: GaussBN = rng_bn.random()?;

        assert_eq!(bn.labels(), &labels);

        Ok(())
    }
}
