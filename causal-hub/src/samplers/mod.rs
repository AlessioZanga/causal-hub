mod forward;
pub use forward::*;

mod importance;
pub use importance::*;

use crate::models::{BN, CTBN};

/// A trait for sampling from a Bayesian network.
pub trait BayesianNetworkSampler<T>
where
    T: BN,
{
    /// Sample a single instance from a Bayesian network.
    ///
    /// # Returns
    ///
    /// A single sample from the Bayesian network.
    ///
    fn sample(&mut self) -> T::Sample;

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
    fn sample_n(&mut self, n: usize) -> T::Dataset;
}

pub use BayesianNetworkSampler as BNSampler;

/// A trait for sampling from a CTBN.
pub trait ContinuousTimeBayesianNetworkSampler<T>
where
    T: CTBN,
{
    /// Sample a single trajectory with a given length from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if `max_length` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_length(&mut self, max_length: usize) -> T::Trajectory;

    /// Sample a single trajectory with a given time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_time` - The ending time of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_time(&mut self, max_time: f64) -> T::Trajectory;

    /// Sample a single trajectory with a given length or time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectory.
    /// * `max_time` - The ending time of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * `max_length` is zero or negative.
    /// * `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_length_or_time(&mut self, max_length: usize, max_time: f64) -> T::Trajectory;

    /// Sample multiple trajectories with a given length from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `max_length` is zero or negative.
    ///     * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_length(&mut self, max_length: usize, n: usize) -> T::Trajectories;

    /// Sample multiple trajectories with a given time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///    * `max_time` is zero or negative.
    ///   * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_time(&mut self, max_time: f64, n: usize) -> T::Trajectories;

    /// Sample multiple trajectories with a given length or time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `max_length` is zero or negative.
    ///     * `max_time` is zero or negative.
    ///     * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_length_or_time(
        &mut self,
        max_length: usize,
        max_time: f64,
        n: usize,
    ) -> T::Trajectories;
}

pub use ContinuousTimeBayesianNetworkSampler as CTBNSampler;

/// A trait for sampling from a CTBN.
pub trait ParallelContinuousTimeBayesianNetworkSampler<T>
where
    T: CTBN,
{
    /// Sample multiple trajectories with a given length from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `max_length` is zero or negative.
    ///     * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_length(&mut self, max_length: usize, n: usize) -> T::Trajectories;

    /// Sample multiple trajectories with a given time from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `max_time` is zero or negative.
    ///     * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_time(&mut self, max_time: f64, n: usize) -> T::Trajectories;

    /// Sample multiple trajectories with a given length or time from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `max_length` is zero or negative.
    ///     * `max_time` is zero or negative.
    ///     * `n` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_length_or_time(
        &mut self,
        max_length: usize,
        max_time: f64,
        n: usize,
    ) -> T::Trajectories;
}

pub use ParallelContinuousTimeBayesianNetworkSampler as ParCTBNSampler;
