use rand::prelude::*;

use crate::{
    models::{BN, CatBN},
    random::{Random, RngCatCPD, RngDag},
    set,
    types::{Error, Labels, Result, States},
};

/// A struct for random categorical Bayesian network generation.
pub struct RngCatBN<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    states: &'a States,
    alpha: f64,
    p: f64,
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
    /// * `states` - The states of the variables.
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
    pub fn new(rng: &'a mut R, states: &'a States, alpha: f64, p: f64) -> Result<Self> {
        // Check if alpha is positive.
        if alpha <= 0.0 {
            return Err(Error::InvalidParameter("alpha", "must be positive"));
        }
        // Check if the probability is in [0, 1].
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter("p", "must be in [0, 1]"));
        }

        Ok(Self {
            rng,
            states,
            alpha,
            p,
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
        let labels: Labels = self.states.keys().cloned().collect();

        // Generate a random DAG.
        let graph = RngDag::new(self.rng, &labels, self.p)?.random()?;

        // Generate the CPDs.
        let cpds = labels
            .iter()
            .enumerate()
            .map(|(i, x)| {
                // Get the parents of the variable.
                let pa_i = graph.parents(&set![i])?;
                // Get the states of the variable.
                let mut states = States::default();
                states.insert(x.clone(), self.states[x].clone());
                // Get the states of the conditioning variables.
                let conditioning_states = pa_i
                    .iter()
                    .map(|&j| {
                        let y = &labels[j];
                        (y.clone(), self.states[y].clone())
                    })
                    .collect();
                // Generate the random CPD.
                RngCatCPD::new(self.rng, &states, &conditioning_states, self.alpha)?.random()
            })
            .collect::<Result<Vec<_>>>()?;

        // Return the categorical Bayesian network.
        CatBN::new(graph, cpds)
    }
}
