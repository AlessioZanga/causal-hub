#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        datasets::{CatTable, CatWtdTable, Dataset},
        labels,
        models::Labelled,
        set, support,
        types::Result,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() -> Result<()> {
        let dataset = CatTable::new(
            support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
            array![[0, 0], [0, 1], [1, 0]],
        )?;
        let weights = array![0.5, 1.0, 1.5];
        let wtd = CatWtdTable::new(dataset, weights.clone())?;

        assert_eq!(wtd.labels(), &labels!["A", "B"]);
        assert_eq!(wtd.weights(), &weights);
        assert_relative_eq!(wtd.sample_size(), 3.0);

        Ok(())
    }

    #[test]
    fn from_table() -> Result<()> {
        let dataset = CatTable::new(support![("A", ["no", "yes"])], array![[0], [1]])?;
        let wtd: CatWtdTable = dataset.into();
        assert_relative_eq!(wtd.sample_size(), 2.0);
        assert!(wtd.weights().iter().all(|&w| w == 1.0));

        Ok(())
    }

    #[test]
    fn error_weight_mismatch() -> Result<()> {
        let dataset = CatTable::new(support![("A", ["no", "yes"])], array![[0], [1]])?;
        let weights = array![1.0];
        let result = CatWtdTable::new(dataset, weights);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn error_non_finite_weight() -> Result<()> {
        let dataset = CatTable::new(support![("A", ["no", "yes"])], array![[0]])?;
        let weights = array![f64::NAN];
        let result = CatWtdTable::new(dataset, weights);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn select_subset() -> Result<()> {
        let dataset = CatTable::new(
            support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
            array![[0, 0], [1, 1]],
        )?;
        let weights = array![0.5, 2.0];
        let wtd = CatWtdTable::new(dataset, weights)?;

        let sub = wtd.select(&set![0])?;
        assert_eq!(sub.labels(), &labels!["A"]);
        assert_relative_eq!(sub.sample_size(), 2.5);

        Ok(())
    }
}
