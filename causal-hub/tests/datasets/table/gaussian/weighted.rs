#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        datasets::{Dataset, GaussTable, GaussWtdTable},
        labels,
        models::HasLabels,
        set,
        types::Result,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() -> Result<()> {
        let dataset = GaussTable::new(
            labels!["A", "B"],
            array![[0.0, 0.0], [1.0, 2.0], [3.0, 4.0]],
        )?;
        let weights = array![0.5, 1.0, 1.5];
        let wtd = GaussWtdTable::new(dataset, weights.clone())?;

        assert_eq!(wtd.labels(), &labels!["A", "B"]);
        assert_eq!(wtd.weights(), &weights);
        assert_relative_eq!(wtd.sample_size(), 3.0);

        Ok(())
    }

    #[test]
    fn from_table() -> Result<()> {
        let dataset = GaussTable::new(labels!["A"], array![[1.0], [2.0]])?;
        let wtd: GaussWtdTable = dataset.into();
        assert_relative_eq!(wtd.sample_size(), 2.0);
        assert!(wtd.weights().iter().all(|&w| w == 1.0));

        Ok(())
    }

    #[test]
    fn error_weight_mismatch() -> Result<()> {
        let dataset = GaussTable::new(labels!["A"], array![[1.0], [2.0]])?;
        let weights = array![1.0];
        assert!(GaussWtdTable::new(dataset, weights).is_err());
        Ok(())
    }

    #[test]
    fn error_negative_weight() -> Result<()> {
        let dataset = GaussTable::new(labels!["A"], array![[1.0]])?;
        let weights = array![-1.0];
        assert!(GaussWtdTable::new(dataset, weights).is_err());
        Ok(())
    }

    #[test]
    fn select_subset() -> Result<()> {
        let dataset = GaussTable::new(labels!["A", "B"], array![[0.0, 0.0], [1.0, 10.0]])?;
        let weights = array![0.5, 2.0];
        let wtd = GaussWtdTable::new(dataset, weights)?;

        let sub = wtd.select(&set![0])?;
        assert_eq!(sub.labels(), &labels!["A"]);
        assert_relative_eq!(sub.sample_size(), 2.5);

        Ok(())
    }
}
