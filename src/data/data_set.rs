use std::{collections::VecDeque, fmt::Debug, iter::FusedIterator};

use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_rand::rand_distr::num_traits::Zero;
use polars::prelude::*;
use rand::{distributions::Uniform, prelude::*, seq::index};
use rayon::{
    iter::plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer},
    prelude::*,
};
use serde::{Deserialize, Serialize};

/// Data set trait.
pub trait DataSet:
    Clone + Debug + From<DataFrame> + Into<DataFrame> + Sync + Serialize + for<'a> Deserialize<'a>
{
    /// Data set underlying structure.
    type Data: Clone;

    /// Labels underlying structure.
    type Labels: Clone;

    /// Labels iterator type.
    type LabelsIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Get reference to underlying data structure.
    fn data(&self) -> &Self::Data;

    /// Get the set of variables labels.
    fn labels(&self) -> &Self::Labels;

    /// Get the iterator over the set of variables labels.
    fn labels_iter(&self) -> Self::LabelsIter<'_>;

    /// Get sample size.
    fn sample_size(&self) -> usize;

    /// Construct a data set from data and labels.
    fn with_data_labels(data: Self::Data, labels: Self::Labels) -> Self;
}

/// Data set sample trait.
pub trait DataSetSample: DataSet {
    /// Bootstrap iterator type.
    type BootstrapIter<'a, 'b, R>: Iterator<Item = Self> + ExactSizeIterator + FusedIterator
    where
        Self: 'a,
        R: 'b + Rng;

    /// Draw `sample_size` samples without replacement.
    ///
    /// # Panics
    ///
    /// Panics if `sample_size` is greater than the total number of samples in the data set.
    ///
    fn sample<R: Rng>(&self, rng: &mut R, sample_size: usize) -> Self;

    /// Draw `sample_size` samples with replacement.
    fn sample_with_replacement<R: Rng>(&self, rng: &mut R, sample_size: usize) -> Self;

    /// Draw `sample_size` samples with replacement `bootstrap_size` times.
    fn bootstrap_iter<'a, 'b, R: Rng>(
        &'a self,
        rng: &'b mut R,
        sample_size: usize,
        bootstrap_size: usize,
    ) -> Self::BootstrapIter<'a, 'b, R>;
}

/// Data set bootstrap iterator.
pub struct BootstrapIterator<'a, 'b, D, R> {
    data_set: &'a D,
    rng: &'b mut R,
    sample_size: usize,
    bootstrap_size: usize,
}

impl<'a, 'b, D, R> BootstrapIterator<'a, 'b, D, R> {
    /// Construct a new bootstrap iterator.
    #[inline]
    pub fn new(data_set: &'a D, rng: &'b mut R, sample_size: usize, bootstrap_size: usize) -> Self {
        Self {
            data_set,
            rng,
            sample_size,
            bootstrap_size,
        }
    }
}

impl<'a, 'b, D, R> Iterator for BootstrapIterator<'a, 'b, D, R>
where
    D: DataSetSample,
    R: Rng,
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of bootstrap samples is zero ...
        if self.bootstrap_size == 0 {
            // ... return `None`.
            return None;
        }

        // Otherwise, draw a bootstrap sample.
        let sample = self
            .data_set
            .sample_with_replacement(self.rng, self.sample_size);
        // Decrement the number of bootstrap samples.
        self.bootstrap_size -= 1;

        Some(sample)
    }
}

impl<'a, 'b, D, R> ExactSizeIterator for BootstrapIterator<'a, 'b, D, R>
where
    D: DataSetSample,
    R: Rng,
{
    #[inline]
    fn len(&self) -> usize {
        self.bootstrap_size
    }
}

impl<'a, 'b, D, R> FusedIterator for BootstrapIterator<'a, 'b, D, R>
where
    D: DataSetSample,
    R: Rng,
{
}

impl<D, T> DataSetSample for D
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    type BootstrapIter<'a, 'b, R> = BootstrapIterator<'a, 'b, D, R> where D: 'a, R: 'b + Rng;

    fn sample<R: Rng>(&self, rng: &mut R, sample_size: usize) -> Self {
        // Check that the sample size is not greater than the total number of samples.
        assert!(
            sample_size <= self.sample_size(),
            "Sample size is greater than the total number of samples."
        );

        // Allocate memory for the samples.
        let mut data = Self::Data::zeros((sample_size, self.data().ncols()));

        // Initialize the sample indices.
        let indices = index::sample(rng, self.sample_size(), sample_size);

        // For each sample index ...
        for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the sample.
            row.assign(&self.data().row(i));
        }

        Self::with_data_labels(data, self.labels().clone())
    }

    fn sample_with_replacement<R: Rng>(&self, rng: &mut R, sample_size: usize) -> Self {
        // Allocate memory for the samples.
        let mut data = Self::Data::zeros((sample_size, self.data().ncols()));

        // Initialize the sample indices range.
        let indices = rng.sample_iter(Uniform::new(0, self.sample_size()));

        // For each sample ...
        for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the sample.
            row.assign(&self.data().row(i));
        }

        Self::with_data_labels(data, self.labels().clone())
    }

    #[inline]
    fn bootstrap_iter<'a, 'b, R: Rng>(
        &'a self,
        rng: &'b mut R,
        sample_size: usize,
        bootstrap_size: usize,
    ) -> Self::BootstrapIter<'a, 'b, R> {
        Self::BootstrapIter::new(self, rng, sample_size, bootstrap_size)
    }
}

/// Parallel data set bootstrap iterator.
pub struct ParallelBootstrapIterator<'a, D, R> {
    data_set: &'a D,
    rngs: VecDeque<R>,
    sample_size: usize,
}

impl<'a, D, R> ParallelBootstrapIterator<'a, D, R>
where
    R: Rng + SeedableRng + Send,
{
    /// Construct a new parallel bootstrap iterator.
    #[inline]
    pub fn new(data_set: &'a D, rng: &mut R, sample_size: usize, bootstrap_size: usize) -> Self {
        // Allocate the thread-local RNGs.
        let mut rngs = Vec::with_capacity(bootstrap_size);
        // Draw `bootstrap_size` seeds.
        let seeds = (0..bootstrap_size).map(|_| rng.next_u64()).collect_vec();
        // Initialize the thread-local RNGs.
        seeds
            .into_par_iter()
            .map(|seed| R::seed_from_u64(seed))
            .collect_into_vec(&mut rngs);
        // Convert the thread-local RNGs to a queue.
        let rngs = rngs.into();

        Self {
            data_set,
            rngs,
            sample_size,
        }
    }
}

impl<'a, D, R> ParallelIterator for ParallelBootstrapIterator<'a, D, R>
where
    D: DataSetSample + Send,
    R: Rng + SeedableRng + Send,
{
    type Item = D;

    #[inline]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        // Delegate to more specific implementation.
        self.drive(consumer)
    }

    #[inline]
    fn opt_len(&self) -> Option<usize> {
        Some(self.rngs.len())
    }
}

struct ParallelBootstrapProducer<'a, D, R> {
    data_set: &'a D,
    rngs: VecDeque<R>,
    sample_size: usize,
}

impl<'a, D, R> From<ParallelBootstrapIterator<'a, D, R>> for ParallelBootstrapProducer<'a, D, R> {
    #[inline]
    fn from(producer: ParallelBootstrapIterator<'a, D, R>) -> Self {
        Self {
            data_set: producer.data_set,
            rngs: producer.rngs,
            sample_size: producer.sample_size,
        }
    }
}

impl<'a, D, R> Iterator for ParallelBootstrapProducer<'a, D, R>
where
    D: DataSetSample + Send,
    R: Rng + SeedableRng + Send,
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of bootstrap samples is zero ...
        if self.rngs.is_empty() {
            // ... return `None`.
            return None;
        }

        // Pop the next RNG.
        let mut rng = self.rngs.pop_front().unwrap();
        // Otherwise, draw a bootstrap sample.
        let sample = self
            .data_set
            .sample_with_replacement(&mut rng, self.sample_size);

        Some(sample)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rngs.len(), Some(self.rngs.len()))
    }
}

impl<'a, D, R> ExactSizeIterator for ParallelBootstrapProducer<'a, D, R>
where
    D: DataSetSample + Send,
    R: Rng + SeedableRng + Send,
{
    #[inline]
    fn len(&self) -> usize {
        self.rngs.len()
    }
}

impl<'a, D, R> DoubleEndedIterator for ParallelBootstrapProducer<'a, D, R>
where
    D: DataSetSample + Send,
    R: Rng + SeedableRng + Send,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // If the remaining number of bootstrap samples is zero ...
        if self.rngs.is_empty() {
            // ... return `None`.
            return None;
        }

        // Pop the next RNG.
        let mut rng = self.rngs.pop_back().unwrap();
        // Otherwise, draw a bootstrap sample.
        let sample = self
            .data_set
            .sample_with_replacement(&mut rng, self.sample_size);

        Some(sample)
    }
}

impl<'a, D, R> Producer for ParallelBootstrapProducer<'a, D, R>
where
    D: DataSetSample + Send,
    R: Rng + SeedableRng + Send,
{
    type Item = D;

    type IntoIter = Self;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        // Split the RNGs.
        let mut a = self.rngs;
        let b = a.split_off(index);
        a.shrink_to_fit();

        // Construct the producers.
        (Self { rngs: a, ..self }, Self { rngs: b, ..self })
    }
}

impl<'a, D, R> IndexedParallelIterator for ParallelBootstrapIterator<'a, D, R>
where
    D: DataSetSample + Send,
    R: Rng + SeedableRng + Send,
{
    #[inline]
    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(ParallelBootstrapProducer::from(self))
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        // For each RNGs ...
        let producer = self.rngs.into_par_iter().map(move |mut rng| {
            // Draw a bootstrap sample.
            self.data_set
                .sample_with_replacement(&mut rng, self.sample_size)
        });

        bridge(producer, consumer)
    }

    #[inline]
    fn len(&self) -> usize {
        self.rngs.len()
    }
}

/// Parallel data set sample trait.
pub trait ParallelDataSetSample: DataSet + Send {
    /// Parallel bootstrap iterator type.
    type ParallelBootstrapIter<'a, R>: ParallelIterator<Item = Self>
    where
        Self: 'a + Send,
        R: Rng + SeedableRng + Send;

    /// Draw `sample_size` samples with replacement `bootstrap_size` times in parallel.
    fn par_bootstrap_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
        sample_size: usize,
        bootstrap_size: usize,
    ) -> Self::ParallelBootstrapIter<'a, R>;
}

impl<D, T> ParallelDataSetSample for D
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type ParallelBootstrapIter<'a, R> = ParallelBootstrapIterator<'a, D, R>
    where
        D: 'a + Send,
        R: Rng + SeedableRng + Send;

    #[inline]
    fn par_bootstrap_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
        sample_size: usize,
        bootstrap_size: usize,
    ) -> Self::ParallelBootstrapIter<'a, R> {
        Self::ParallelBootstrapIter::new(self, rng, sample_size, bootstrap_size)
    }
}
