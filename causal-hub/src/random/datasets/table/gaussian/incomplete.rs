use rand::Rng;

use crate::{
    datasets::{GaussIncTable, GaussTable, MissingMechanism},
    models::Labelled,
    random::Random,
    types::{Error, Result},
};

/// A struct representing a random incomplete gaussian table dataset generator.
pub struct RngGaussIncTable<'a, R> {
    rng: &'a mut R,
    dataset: &'a GaussTable,
    missing_mechanism: &'a MissingMechanism,
    p_min: f64,
    p_max: f64,
}

impl<'a, R: Rng> RngGaussIncTable<'a, R> {
    /// Creates a new `RngGaussIncTable` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `dataset` - A reference to the complete gaussian table dataset.
    /// * `missing_mechanism` - A reference to the missingness mechanism.
    /// * `p_min` - The minimum probability of missingness.
    /// * `p_max` - The maximum probability of missingness.
    ///
    /// # Returns
    ///
    /// A new `RngGaussIncTable` instance.
    pub fn new(
        rng: &'a mut R,
        dataset: &'a GaussTable,
        missing_mechanism: &'a MissingMechanism,
        p_min: f64,
        p_max: f64,
    ) -> Result<Self> {
        // Check that dataset labels are equals to missing mechanism labels.
        if dataset.labels() != missing_mechanism.labels() {
            return Err(Error::InvalidParameter(
                "missing_mechanism".to_string(),
                "labels do not match dataset labels".to_string(),
            ));
        }
        // Check that p_min and p_max are in [0, 1].
        if !(0.0..=1.0).contains(&p_min) {
            return Err(Error::InvalidParameter(
                "p_min".to_string(),
                "must be in [0, 1]".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&p_max) {
            return Err(crate::types::Error::InvalidParameter(
                "p_max".to_string(),
                "must be in [0, 1]".to_string(),
            ));
        }
        // Check that p_min is less than or equal to p_max.
        if p_min > p_max {
            return Err(crate::types::Error::InvalidParameter(
                "p_min".to_string(),
                "must be less than or equal to p_max".to_string(),
            ));
        }

        Ok(Self {
            rng,
            dataset,
            missing_mechanism,
            p_min,
            p_max,
        })
    }
}

impl<R: Rng> Random<Result<GaussIncTable>> for RngGaussIncTable<'_, R> {
    fn random(&mut self) -> Result<GaussIncTable> {
        todo!() // FIXME:
    }
}
