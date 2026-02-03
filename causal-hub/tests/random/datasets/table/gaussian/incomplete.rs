#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{GaussTable, MissingMechanism},
        labels, map,
        models::Labelled,
        random::{Random, RngGaussIncTable},
        set,
        types::{Error, Result},
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B"];
        let values = array![[0., 0.], [1., 1.]];
        let dataset = GaussTable::new(labels.clone(), values)?;
        let mechanism = MissingMechanism::new(labels, map![(0, set![1])])?;

        let res = RngGaussIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 0.2);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn new_invalid_labels() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B"];
        let values = array![[0., 0.], [1., 1.]];
        let dataset = GaussTable::new(labels, values)?;
        let mechanism = MissingMechanism::new(labels!["A", "C"], map![(0, set![1])])?;

        let res = RngGaussIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn new_invalid_p_min() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B"];
        let values = array![[0., 0.], [1., 1.]];
        let dataset = GaussTable::new(labels.clone(), values)?;
        let mechanism = MissingMechanism::new(labels, map![(0, set![1])])?;

        let res = RngGaussIncTable::new(&mut rng, &dataset, &mechanism, -0.1, 0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        let res = RngGaussIncTable::new(&mut rng, &dataset, &mechanism, 1.1, 1.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn new_invalid_p_max() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B"];
        let values = array![[0., 0.], [1., 1.]];
        let dataset = GaussTable::new(labels.clone(), values)?;
        let mechanism = MissingMechanism::new(labels, map![(0, set![1])])?;

        let res = RngGaussIncTable::new(&mut rng, &dataset, &mechanism, 0.1, -0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        let res = RngGaussIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 1.3);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn new_p_min_greater_than_p_max() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B"];
        let values = array![[0., 0.], [1., 1.]];
        let dataset = GaussTable::new(labels.clone(), values)?;
        let mechanism = MissingMechanism::new(labels, map![(0, set![1])])?;

        let res = RngGaussIncTable::new(&mut rng, &dataset, &mechanism, 0.5, 0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B"];
        let values = array![[0., 0.], [1., 1.]];
        let dataset = GaussTable::new(labels.clone(), values)?;
        let mechanism = MissingMechanism::new(labels, map![(0, set![1])])?;

        let mut rng_gauss_inc_table =
            RngGaussIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 0.2)?;
        let sample = rng_gauss_inc_table.random()?;

        assert_eq!(sample.labels(), dataset.labels());

        Ok(())
    }
}
