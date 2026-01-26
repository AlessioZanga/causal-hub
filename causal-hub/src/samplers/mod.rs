mod forward;
pub use forward::*;

mod importance;
pub use importance::*;

use crate::{
    models::{BN, CTBN},
    types::Result,
};

/// A trait for sampling from a Bayesian network.
pub trait BNSampler<T>
where
    T: BN,
{
    /// The sample type.
    type Sample;
    /// The samples type.
    type Samples;

    /// Sample a single instance from a Bayesian network.
    ///
    /// # Returns
    ///
    /// A single sample from the Bayesian network.
    ///
    fn sample(&self) -> Result<Self::Sample>;

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
    fn sample_n(&self, n: usize) -> Result<Self::Samples>;
}

/// A trait for parallel sampling from a Bayesian network.
pub trait ParBNSampler<T>
where
    T: BN,
{
    /// The samples type.
    type Samples;

    /// Sample from a Bayesian network in parallel.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of samples to generate.
    ///
    /// # Returns
    ///
    /// A dataset containing the samples.
    ///
    fn par_sample_n(&self, n: usize) -> Result<Self::Samples>;
}

/// A trait for sampling from a CTBN.
pub trait CTBNSampler<T>
where
    T: CTBN,
{
    /// The sample type.
    type Sample;
    /// The samples type.
    type Samples;

    /// Sample a single trajectory with a given length from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectory.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if `max_length` is zero.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_length(&self, max_length: usize) -> Result<Self::Sample>;

    /// Sample a single trajectory with a given time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_time` - The ending time of the trajectory.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_time(&self, max_time: f64) -> Result<Self::Sample>;

    /// Sample a single trajectory with a given length or time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectory.
    /// * `max_time` - The ending time of the trajectory.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if:
    ///
    /// * `max_length` is zero.
    /// * `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A trajectory containing the sampled events.
    ///
    fn sample_by_length_or_time(&self, max_length: usize, max_time: f64) -> Result<Self::Sample>;

    /// Sample multiple trajectories with a given length from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if `max_length` is zero.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_length(&self, max_length: usize, n: usize) -> Result<Self::Samples>;

    /// Sample multiple trajectories with a given time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_time(&self, max_time: f64, n: usize) -> Result<Self::Samples>;

    /// Sample multiple trajectories with a given length or time from a CTBN.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if:
    ///
    /// * `max_length` is zero.
    /// * `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn sample_n_by_length_or_time(
        &self,
        max_length: usize,
        max_time: f64,
        n: usize,
    ) -> Result<Self::Samples>;
}

/// A trait for parallel sampling from a CTBN.
pub trait ParCTBNSampler<T>
where
    T: CTBN,
{
    /// The samples type.
    type Samples;

    /// Sample multiple trajectories with a given length from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if `max_length` is zero.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_length(&self, max_length: usize, n: usize) -> Result<Self::Samples>;

    /// Sample multiple trajectories with a given time from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_time(&self, max_time: f64, n: usize) -> Result<Self::Samples>;

    /// Sample multiple trajectories with a given length or time from a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `max_length` - The length of the trajectories.
    /// * `max_time` - The ending time of the trajectories.
    /// * `n` - The number of trajectories to generate.
    ///
    /// # Errors
    ///
    /// Returns `IllegalArgument` if:
    ///
    /// * `max_length` is zero.
    /// * `max_time` is zero or negative.
    ///
    /// # Returns
    ///
    /// A collection of trajectories containing the sampled events.
    ///
    fn par_sample_n_by_length_or_time(
        &self,
        max_length: usize,
        max_time: f64,
        n: usize,
    ) -> Result<Self::Samples>;
}
