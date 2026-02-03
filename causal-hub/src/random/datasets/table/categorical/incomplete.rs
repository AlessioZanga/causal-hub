use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use rand::Rng;
use rand_distr::Uniform;

use crate::{
    datasets::{CatIncTable, CatTable, CatType, Dataset, IncDataset, MissingMechanism},
    models::Labelled,
    random::Random,
    types::{Error, Result},
};

/// A struct representing a random incomplete categorical table dataset generator.
pub struct RngCatIncTable<'a, R> {
    rng: &'a mut R,
    dataset: &'a CatTable,
    missing_mechanism: &'a MissingMechanism,
    p_min: f64,
    p_max: f64,
}

impl<'a, R: Rng> RngCatIncTable<'a, R> {
    /// Creates a new `RngCatIncTable` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `dataset` - A reference to the complete categorical table dataset.
    /// * `missing_mechanism` - A reference to the missingness mechanism.
    /// * `p_min` - The minimum probability of missingness.
    /// * `p_max` - The maximum probability of missingness.
    ///
    /// # Returns
    ///
    /// A new `RngCatIncTable` instance.
    pub fn new(
        rng: &'a mut R,
        dataset: &'a CatTable,
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
            return Err(Error::InvalidParameter(
                "p_max".to_string(),
                "must be in [0, 1]".to_string(),
            ));
        }
        // Check that p_min is less than or equal to p_max.
        if p_min > p_max {
            return Err(Error::InvalidParameter(
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

impl<R: Rng> Random<Result<CatIncTable>> for RngCatIncTable<'_, R> {
    fn random(&mut self) -> Result<CatIncTable> {
        // Get the missing indicator.
        const M: CatType = CatIncTable::MISSING;
        // Get dataset states.
        let states = self.dataset.states().clone();
        // Get dataset values.
        let mut values = self.dataset.values().clone();

        // Define the uniform distributions for sampling.
        let p_s = Uniform::new_inclusive(0., 1.)?;
        let p_u = Uniform::new_inclusive(self.p_min, self.p_max)?;

        // Iterate over the missing mechanism.
        for (&x, pa_x) in self.missing_mechanism {
            // Get mutable reference to the column X.
            let mut c_x = values.column_mut(x);

            // Check if the variable has no parents.
            if pa_x.is_empty() {
                // Sample a missingness probability.
                let p_x = self.rng.sample(p_u);
                // Modify the corresponding column.
                c_x.iter_mut().for_each(|x| {
                    if self.rng.sample(p_s) < p_x {
                        *x = M;
                    }
                });
                // Continue to the next variable.
                continue;
            }

            // For each parent ...
            for &z in pa_x {
                // Get reference to the column Z.
                let c_z = self.dataset.values().column(z);
                // Get the mode of the parent via direct counting.
                let mut m_z = Array::from_elem(CatType::MAX as usize, 0);
                c_z.iter().for_each(|&z| m_z[z as usize] += 1);
                let s_z = match m_z.argmax() {
                    Ok(s_z) => s_z as CatType,
                    // If the mode cannot be found, skip this parent.
                    _ => continue,
                };
                // Modify the corresponding column.
                azip!((x in &mut c_x, z in &c_z) {
                    // Sample a missingness probability for X given parent Z.
                    if
                        (self.rng.sample(p_s) < self.p_max && *z == s_z) ||
                        (self.rng.sample(p_s) < self.p_min && *z != s_z)
                    {
                        *x = M;
                    }
                });
            }
        }

        // Return the incomplete dataset.
        CatIncTable::new(states, values)
    }
}
