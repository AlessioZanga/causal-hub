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
    evidence: f64,
    probability: f64,
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
        evidence: f64,
        probability: f64,
    ) -> Result<Self> {
        // Check parameters.
        if s_a <= 0.0 {
            return Err(Error::InvalidParameter("s_a", "must be positive"));
        }
        if s_b <= 0.0 {
            return Err(Error::InvalidParameter("s_b", "must be positive"));
        }
        if evidence <= 0.0 {
            return Err(Error::InvalidParameter("e", "must be positive"));
        }
        // Check if the probability is in [0, 1].
        if !(0.0..=1.0).contains(&probability) {
            return Err(Error::InvalidParameter("p", "must be in [0, 1]"));
        }

        Ok(Self {
            rng,
            labels,
            s_a,
            s_b,
            evidence,
            probability,
        })
    }
}

impl<R> Random for RngGaussBN<'_, R>
where
    R: Rng,
{
    type Output = Result<GaussBN>;

    fn random(&mut self) -> Self::Output {
        // Generate a random DAG.
        let graph = RngDag::new(self.rng, self.labels, self.probability)?.random()?;

        // Generate the CPDs.
        let cpds = self
            .labels
            .iter()
            .enumerate()
            .map(|(i, x)| {
                // Get the parents of the variable.
                let pa_i = graph.parents(&set![i])?;
                // Get the labels of the variable.
                let labels = labels![x.clone()];
                // Get the labels of the conditioning variables.
                let conditioning_labels =
                    pa_i.into_iter().map(|j| self.labels[j].clone()).collect();
                // Generate the random CPD.
                RngGaussCPD::new(
                    self.rng,
                    &labels,
                    &conditioning_labels,
                    self.s_a,
                    self.s_b,
                    self.evidence,
                )?
                .random()
            })
            .collect::<Result<Vec<_>>>()?;

        // Return the Gaussian Bayesian network.
        GaussBN::new(graph, cpds)
    }
}
