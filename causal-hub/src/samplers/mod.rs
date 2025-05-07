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
    /// Sample a single trajectory with a given length from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if `length` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_length(&mut self, length: usize) -> CTBN::Trajectory;

    /// Sample a single trajectory with a given time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `time` - The ending time of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if `time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_time(&mut self, time: f64) -> CTBN::Trajectory;

    /// Sample a single trajectory with a given length or time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the trajectory.
    /// * `time` - The ending time of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * `length` is zero or negative.
    /// * `time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_length_or_time(&mut self, length: usize, time: f64) -> CTBN::Trajectory;

    /// Sample multiple trajectories with a given length from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `length` is zero or negative.
    ///     * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_length(&mut self, length: usize, n: usize) -> CTBN::Trajectories;

    /// Sample multiple trajectories with a given time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///    * `time` is zero or negative.
    ///   * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_time(&mut self, time: f64, n: usize) -> CTBN::Trajectories;

    /// Sample multiple trajectories with a given length or time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the trajectories.
    /// * `time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///   * `length` is zero or negative.
    ///  * `time` is zero or negative.
    /// * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_length_or_time(
        &mut self,
        length: usize,
        time: f64,
        n: usize,
    ) -> CTBN::Trajectories;
}

pub use ContinuousTimeBayesianNetworkSampler as CTBNSampler;

/// A trait for sampling from a CTBN.
pub trait ParallelContinuousTimeBayesianNetworkSampler<CTBN>
where
    CTBN: ContinuousTimeBayesianNetwork,
{
    /// Sample multiple trajectories with a given length from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `length` is zero or negative.
    ///     * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_length(&mut self, length: usize, n: usize) -> CTBN::Trajectories;

    /// Sample multiple trajectories with a given time from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///    * `time` is zero or negative.
    ///   * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_time(&mut self, time: f64, n: usize) -> CTBN::Trajectories;

    /// Sample multiple trajectories with a given length or time from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the trajectories.
    /// * `time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///   * `length` is zero or negative.
    ///  * `time` is zero or negative.
    /// * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_length_or_time(
        &mut self,
        length: usize,
        time: f64,
        n: usize,
    ) -> CTBN::Trajectories;
}

pub use ParallelContinuousTimeBayesianNetworkSampler as ParCTBNSampler;
