use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use rand::Rng;
use rand_distr::Gamma;

use crate::{
    models::CatCPD,
    random::Random,
    types::{Error, Result, States},
};

/// A struct for random categorical CPD generation.
pub struct RngCatCPD<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    states: &'a States,
    conditioning_states: &'a States,
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
    /// * `states` - The states of the target variable.
    /// * `conditioning_states` - The states of the conditioning variables.
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
        states: &'a States,
        conditioning_states: &'a States,
        alpha: f64,
    ) -> Result<Self> {
        // Check if alpha is positive.
        if alpha <= 0.0 {
            return Err(Error::InvalidParameter("alpha", "must be positive"));
        }

        Ok(Self {
            rng,
            states,
            conditioning_states,
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
        let m = self.states.values().map(|v| v.len()).product();
        let n = self.conditioning_states.values().map(|v| v.len()).product();

        // Create the Gamma distribution.
        let gamma = Gamma::new(self.alpha, 1.0)
            .map_err(|e| Error::InvalidParameter("alpha", &e.to_string()))?;

        // Sample the parameters.
        let mut parameters = Array::random_using((n, m), gamma, self.rng);
        // Normalize the parameters row-wise.
        parameters /= &parameters.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Return the categorical CPD.
        CatCPD::new(
            self.states.clone(),
            self.conditioning_states.clone(),
            parameters,
        )
    }
}
