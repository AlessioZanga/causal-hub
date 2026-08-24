use ndarray::prelude::*;
use rand::{Rng, RngExt};
use rand_distr::Gamma;

use crate::{
    models::{CatCPD, CatSupport},
    random::Random,
    types::{Error, Result},
};

/// A struct for random categorical CPD generation.
pub struct RngCatCPD<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    support: &'a CatSupport,
    conditioning_support: &'a CatSupport,
    alpha: f64,
}

impl<'a, R> RngCatCPD<'a, R>
where
    R: Rng,
{
    /// Creates a new `RngCatCPD` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `support` - The support of the target variable.
    /// * `conditioning_support` - The support of the conditioning variables.
    /// * `alpha` - The parameter of the Dirichlet distribution.
    ///
    /// # Errors
    ///
    /// * If `alpha` is not positive.
    ///
    /// # Returns
    ///
    /// A new `RngCatCPD` instance.
    ///
    pub fn new(
        rng: &'a mut R,
        support: &'a CatSupport,
        conditioning_support: &'a CatSupport,
        alpha: f64,
    ) -> Result<Self> {
        // Check if alpha is positive.
        if alpha <= 0.0 {
            return Err(Error::InvalidParameter("alpha", "must be positive"));
        }

        Ok(Self {
            rng,
            support,
            conditioning_support,
            alpha,
        })
    }
}

impl<R> Random for RngCatCPD<'_, R>
where
    R: Rng,
{
    type Output = Result<CatCPD>;

    fn random(&mut self) -> Self::Output {
        // Get the inner state sizes.
        let model = self.support.values().map(|v| v.len()).product();
        let n = self
            .conditioning_support
            .values()
            .map(|v| v.len())
            .product();

        // Create the Gamma distribution.
        let gamma = Gamma::new(self.alpha, 1.0)
            .map_err(|evidence| Error::InvalidParameter("alpha", &evidence.to_string()))?;

        // Sample the parameters.
        let mut parameters = Array::from_shape_fn((n, model), |_| self.rng.sample(gamma));
        // Normalize the parameters row-wise.
        parameters /= &parameters.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Return the categorical CPD.
        CatCPD::new(
            self.support.clone(),
            self.conditioning_support.clone(),
            parameters,
        )
    }
}
