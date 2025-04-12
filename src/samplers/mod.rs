mod forward;
pub use forward::*;
use rand::Rng;

/// A trait for sampling from a Bayesian network.
pub trait BayesianNetworkSampler<BN, D> {
    /// Sample from a Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator.
    /// * `bn` - A Bayesian network.
    /// * `n` - The number of samples to generate.
    ///
    /// # Returns
    ///
    /// A dataset containing the samples.
    ///
    fn sample<R>(&self, rng: &mut R, bn: &BN, n: usize) -> D
    where
        R: Rng;
}

pub use BayesianNetworkSampler as BNSampler;
