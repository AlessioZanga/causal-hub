use rand::prelude::*;

use crate::{
    models::{BN, CatBN, CatSupport},
    random::{Random, RngCatCPD, RngDag},
    set,
    types::{Error, Labels, Result},
};

/// A struct for random categorical Bayesian network generation.
pub struct RngCatBN<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    support: &'a CatSupport,
    alpha: f64,
    probability: f64,
}

impl<'a, R> RngCatBN<'a, R>
where
    R: Rng,
{
    /// Creates a new `RngCatBN` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `support` - The support of the variables.
    /// * `alpha` - The parameter of the Dirichlet distribution.
    /// * `p` - The probability of generating an edge.
    ///
    /// # Errors
    ///
    /// * If `alpha` is not positive.
    /// * If `p` is not in [0, 1].
    ///
    /// # Returns
    ///
    /// A new `RngCatBN` instance.
    ///
    pub fn new(
        rng: &'a mut R,
        support: &'a CatSupport,
        alpha: f64,
        probability: f64,
    ) -> Result<Self> {
        // Check if alpha is positive.
        if alpha <= 0.0 {
            return Err(Error::InvalidParameter("alpha", "must be positive"));
        }
        // Check if the probability is in [0, 1].
        if !(0.0..=1.0).contains(&probability) {
            return Err(Error::InvalidParameter("p", "must be in [0, 1]"));
        }

        Ok(Self {
            rng,
            support,
            alpha,
            probability,
        })
    }
}

impl<R> Random for RngCatBN<'_, R>
where
    R: Rng,
{
    type Output = Result<CatBN>;

    fn random(&mut self) -> Self::Output {
        // Get the labels of the variables.
        let labels: Labels = self.support.keys().cloned().collect();

        // Generate a random DAG.
        let graph = RngDag::new(self.rng, &labels, self.probability)?.random()?;

        // Generate the CPDs.
        let cpds = labels
            .iter()
            .enumerate()
            .map(|(i, x)| {
                // Get the parents of the variable.
                let pa_i = graph.parents(&set![i])?;
                // Get the support of the variable.
                let mut support = CatSupport::default();
                support.insert(x.clone(), self.support[x].clone());
                // Get the support of the conditioning variables.
                let conditioning_support = pa_i
                    .iter()
                    .map(|&j| {
                        let y = &labels[j];
                        (y.clone(), self.support[y].clone())
                    })
                    .collect();
                // Generate the random CPD.
                RngCatCPD::new(self.rng, &support, &conditioning_support, self.alpha)?.random()
            })
            .collect::<Result<Vec<_>>>()?;

        // Return the categorical Bayesian network.
        CatBN::new(graph, cpds)
    }
}
