#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTable, MissingMechanism},
        labels, map,
        models::Labelled,
        random::{Random, RngCatIncTable},
        set, states,
        types::{Error, Result},
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["0", "1"])];
        let values = array![[0, 0], [1, 1]];
        let dataset = CatTable::new(states, values)?;
        let mechanism = MissingMechanism::new(labels!["A", "B"], map![(0, set![1])])?;

        let res = RngCatIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 0.2);
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn new_invalid_labels() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["0", "1"])];
        let values = array![[0, 0], [1, 1]];
        let dataset = CatTable::new(states, values)?;
        let mechanism = MissingMechanism::new(labels!["A", "C"], map![(0, set![1])])?;

        let res = RngCatIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn new_invalid_p_min() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["0", "1"])];
        let values = array![[0, 0], [1, 1]];
        let dataset = CatTable::new(states, values)?;
        let mechanism = MissingMechanism::new(labels!["A", "B"], map![(0, set![1])])?;

        let res = RngCatIncTable::new(&mut rng, &dataset, &mechanism, -0.1, 0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        let res = RngCatIncTable::new(&mut rng, &dataset, &mechanism, 1.1, 1.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn new_invalid_p_max() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["0", "1"])];
        let values = array![[0, 0], [1, 1]];
        let dataset = CatTable::new(states, values)?;
        let mechanism = MissingMechanism::new(labels!["A", "B"], map![(0, set![1])])?;

        let res = RngCatIncTable::new(&mut rng, &dataset, &mechanism, 0.1, -0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        let res = RngCatIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 1.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn new_p_min_greater_than_p_max() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["0", "1"])];
        let values = array![[0, 0], [1, 1]];
        let dataset = CatTable::new(states, values)?;
        let mechanism = MissingMechanism::new(labels!["A", "B"], map![(0, set![1])])?;

        let res = RngCatIncTable::new(&mut rng, &dataset, &mechanism, 0.5, 0.2);
        assert!(matches!(res, Err(Error::InvalidParameter(..))));

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let states = states![("A", ["0", "1"]), ("B", ["0", "1"])];
        let values = array![[0, 0], [1, 1]];
        let dataset = CatTable::new(states, values)?;
        let mechanism = MissingMechanism::new(labels!["A", "B"], map![(0, set![1])])?;

        let mut rng_cat_inc_table = RngCatIncTable::new(&mut rng, &dataset, &mechanism, 0.1, 0.2)?;
        let sample = rng_cat_inc_table.random()?;

        assert_eq!(sample.labels(), dataset.labels());
        assert_eq!(sample.states(), dataset.states());

        Ok(())
    }
}
