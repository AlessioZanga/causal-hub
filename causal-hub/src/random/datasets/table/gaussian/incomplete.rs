use ndarray::prelude::*;
use ndarray_stats::{Quantile1dExt, interpolate::Nearest};
use noisy_float::prelude::*;
use rand::{Rng, RngExt};
use rand_distr::{Uniform, num_traits::ToPrimitive};

use crate::{
    datasets::{Dataset, GaussIncTable, GaussTable, GaussType, IncDataset, MissingMechanism},
    models::HasLabels,
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
                "missing_mechanism",
                "labels do not match dataset labels",
            ));
        }
        // Check that p_min and p_max are in [0, 1].
        if !(0.0..=1.0).contains(&p_min) {
            return Err(Error::InvalidParameter("p_min", "must be in [0, 1]"));
        }
        if !(0.0..=1.0).contains(&p_max) {
            return Err(Error::InvalidParameter("p_max", "must be in [0, 1]"));
        }
        // Check that p_min is less than or equal to p_max.
        if p_min > p_max {
            return Err(Error::InvalidParameter(
                "p_min",
                "must be less than or equal to p_max",
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

impl<R: Rng> Random for RngGaussIncTable<'_, R> {
    type Output = Result<GaussIncTable>;

    fn random(&mut self) -> Self::Output {
        // Get the missing indicator.
        const M: GaussType = GaussIncTable::MISSING;
        // Get dataset labels.
        let labels = self.dataset.labels().clone();
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
                // Get the threshold of the parent via quantile.
                let s_z = match c_z
                    .mapv(n64)
                    .quantile_mut(n64(0.2), &Nearest)
                    .map(|q| q.to_f64())
                {
                    Ok(Some(q)) => q,
                    // If quantile computation fails, skip this parent.
                    _ => continue,
                };
                // Modify the corresponding column.
                azip!((x in &mut c_x, z in &c_z) {
                    // Sample a missingness probability for X given parent Z.
                    if
                        (self.rng.sample(p_s) < self.p_max && *z <  s_z) ||
                        (self.rng.sample(p_s) < self.p_min && *z >= s_z)
                    {
                        *x = M;
                    }
                });
            }
        }

        // Return the incomplete dataset.
        GaussIncTable::new(labels, values)
    }
}
