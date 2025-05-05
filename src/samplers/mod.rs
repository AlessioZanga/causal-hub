mod forward;
pub use forward::*;

use crate::models::{BayesianNetwork, ContinuousTimeBayesianNetwork};

/// A trait for sampling from a Bayesian network.
pub trait BayesianNetworkSampler<BN>
where
    BN: BayesianNetwork,
{
    /// Sample a single instance from a Bayesian network.
    ///
    /// # Returns
    ///
    /// A single sample from the Bayesian network.
    ///
    fn sample(&mut self) -> BN::Sample;

    /// Sample from a Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of samples to generate.
    ///
    /// # Returns
    ///
    /// A dataset containing the samples.
    ///
    fn sample_n(&mut self, n: usize) -> BN::Dataset;
}

pub use BayesianNetworkSampler as BNSampler;

/// A trait for sampling from a CTBN.
pub trait ContinuousTimeBayesianNetworkSampler<CTBN>
where
    CTBN: ContinuousTimeBayesianNetwork,
{
    /// Sample a single event from a CTBN.
    ///
    /// # Returns
    ///
    /// A single event from the CTBN.
    ///
    fn sample(&mut self) -> CTBN::Event;

    /// Sample a trajectory with a given number of events from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of events to sample.
    ///
    /// # Panics
    ///
    /// Panics if `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_n(&mut self, n: usize) -> CTBN::Trajectory;

    /// Sample a trajectory with a given time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `t` - The time to sample.
    ///
    /// # Panics
    ///
    /// Panics if `t` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_t(&mut self, t: f64) -> CTBN::Trajectory;

    /// Sample a trajectory with a given number of events or time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of events to sample.
    /// * `t` - The time to sample.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * `n` is zero or negative.
    /// * `t` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_n_or_t(&mut self, n: usize, t: f64) -> CTBN::Trajectory;
}

pub use ContinuousTimeBayesianNetworkSampler as CTBNSampler;
