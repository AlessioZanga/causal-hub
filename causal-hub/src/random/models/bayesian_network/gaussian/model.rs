use rand::prelude::*;

use crate::{
    labels,
    models::{BN, GaussBN},
    random::{Random, RngDag, RngGaussCPD},
    set,
    types::{Error, Labels, Result},
};

/// A struct for random Gaussian Bayesian network generation.
pub struct RngGaussBN<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    labels: &'a Labels,
    s_a: f64,
    s_b: f64,
    e: f64,
    p: f64,
}

impl<'a, R> RngGaussBN<'a, R>
where
    R: Rng,
{
    /// Creates a new `RngGaussBN` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `labels` - The labels of the variables.
    /// * `s_a` - The standard deviation of the regression coefficients.
    /// * `s_b` - The standard deviation of the intercept.
    /// * `e` - A small positive constant for covariance regularization.
    /// * `p` - The probability of generating an edge.
    ///
    /// # Errors
    ///
    /// * If `s_a` is not positive.
    /// * If `s_b` is not positive.
    /// * If `e` is not positive.
    /// * If `p` is not in [0, 1].
    ///
    /// # Returns
    ///
    /// A new `RngGaussBN` instance.
    ///
    pub fn new(
        rng: &'a mut R,
        labels: &'a Labels,
        s_a: f64,
        s_b: f64,
        e: f64,
        p: f64,
    ) -> Result<Self> {
        // Check parameters.
        if s_a <= 0.0 {
            return Err(Error::InvalidParameter(
                "s_a".to_string(),
                "must be positive".to_string(),
            ));
        }
        if s_b <= 0.0 {
            return Err(Error::InvalidParameter(
                "s_b".to_string(),
                "must be positive".to_string(),
            ));
        }
        if e <= 0.0 {
            return Err(Error::InvalidParameter(
                "e".to_string(),
                "must be positive".to_string(),
            ));
        }
        // Check if the probability is in [0, 1].
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter(
                "p".to_string(),
                "must be in [0, 1]".to_string(),
            ));
        }

        Ok(Self {
            rng,
            labels,
            s_a,
            s_b,
            e,
            p,
        })
    }
}

impl<R> Random<Result<GaussBN>> for RngGaussBN<'_, R>
where
    R: Rng,
{
    fn random(&mut self) -> Result<GaussBN> {
        // Generate a random DAG.
        let graph = RngDag::new(self.rng, self.labels, self.p)?.random()?;

        // Generate the CPDs.
        let cpds = self
            .labels
            .iter()
            .enumerate()
            .map(|(i, x)| {
                // Get the parents of the variable.
                let pa_i = graph.parents(&set![i])?;
                // Get the labels of the variable.
                let labels: Labels = labels![x.clone()];
                // Get the labels of the conditioning variables.
                let conditioning_labels: Labels =
                    pa_i.into_iter().map(|j| self.labels[j].clone()).collect();
                // Generate the random CPD.
                RngGaussCPD::new(
                    self.rng,
                    &labels,
                    &conditioning_labels,
                    self.s_a,
                    self.s_b,
                    self.e,
                )?
                .random()
            })
            .collect::<Result<Vec<_>>>()?;

        // Return the Gaussian Bayesian network.
        GaussBN::new(graph, cpds)
    }
}
