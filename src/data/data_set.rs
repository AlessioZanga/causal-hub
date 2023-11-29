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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.bootstrap_size, Some(self.bootstrap_size))
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

/* Test the `DataSetSample` trait using `CategoricalDataMatrix`. */
#[cfg(test)]
mod test_data_set_sample {
    use ndarray::prelude::*;
    use rand::prelude::*;
    use rand_xoshiro::Xoshiro256StarStar;

    use crate::prelude::*;

    #[test]
    #[should_panic]
    fn test_sample_panic() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        data_set.sample(&mut rng, 11);
    }

    #[test]
    fn test_sample() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let sample = data_set.sample(&mut rng, 5);
        assert_eq!(sample.sample_size(), 5);
        assert_eq!(sample.labels(), data_set.labels());
    }

    #[test]
    fn test_sample_with_replacement() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let sample = data_set.sample_with_replacement(&mut rng, 5);
        assert_eq!(sample.sample_size(), 5);
        assert_eq!(sample.labels(), data_set.labels());
    }

    #[test]
    fn test_bootstrap_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let bootstrap_iter = data_set.bootstrap_iter(&mut rng, 5, 10);
        assert_eq!(bootstrap_iter.len(), 10);
        assert_eq!(bootstrap_iter.size_hint(), (10, Some(10)));
        bootstrap_iter.for_each(|sample| {
            assert_eq!(sample.sample_size(), 5);
            assert_eq!(sample.labels(), data_set.labels());
        });
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

/* Test the `ParallelDataSetSample` trait using `CategoricalDataMatrix`. */
#[cfg(test)]
mod test_parallel_data_set_sample {
    use ndarray::prelude::*;
    use rand::prelude::*;
    use rand_xoshiro::Xoshiro256StarStar;
    use rayon::prelude::*;

    use crate::prelude::*;

    #[test]
    fn test_par_bootstrap_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let bootstrap_iter = data_set.par_bootstrap_iter(&mut rng, 5, 10);
        assert_eq!(bootstrap_iter.len(), 10);
        assert_eq!(bootstrap_iter.opt_len(), Some(10));
        bootstrap_iter.for_each(|sample| {
            assert_eq!(sample.sample_size(), 5);
            assert_eq!(sample.labels(), data_set.labels());
        });
    }
}

/// Data set split trait.
pub trait DataSetSplit: DataSet {
    /// K-fold iterator type.
    type KFoldIter<'a>: Iterator<Item = Self> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Leave-one-out iterator type.
    type LeaveOneOutIter<'a>: Iterator<Item = Self> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Leave-p-out iterator type.
    type LeavePOutIter<'a>: Iterator<Item = Self> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Split the data set into training and test sets with a given test percentage.
    ///
    /// # Panics
    ///
    /// Panics if `test_percentage` is not in the range `[0, 1]`.
    ///
    /// # Note
    ///
    /// The data set is shuffled before splitting.
    ///
    fn train_test_split<R: Rng>(&self, rng: &mut R, test_percentage: f64) -> (Self, Self);

    /// Split the data set into k-folds.
    ///
    /// # Panics
    ///
    /// Panics if `k` is greater than the total number of samples in the data set.
    ///
    /// # Note
    ///
    /// The data set is shuffled before splitting.
    /// The folds are not guaranteed to be of equal size, e.g. if `k` is not a divisor of
    /// the total number of samples, then there will be one fold with less than `k` samples.
    ///
    fn k_fold_iter<'a, R: Rng>(&'a self, rng: &mut R, k: usize) -> Self::KFoldIter<'a>;

    /// Split the data set into leave-one-out folds.
    ///
    /// # Note
    ///
    /// The data set is shuffled before splitting.
    ///
    fn leave_one_out_iter<'a, R: Rng>(&'a self, rng: &mut R) -> Self::LeaveOneOutIter<'a>;

    /// Split the data set into leave-p-out folds.
    ///
    /// # Panics
    ///
    /// Panics if `p` is greater than the total number of samples in the data set.
    ///
    /// # Note
    ///
    /// The data set is shuffled before splitting.
    ///
    fn leave_p_out_iter<'a, R: Rng>(&'a self, rng: &mut R, p: usize) -> Self::LeavePOutIter<'a>;
}

/// K-fold split iterator.
pub struct KFoldIterator<'a, D> {
    data_set: &'a D,
    indices: VecDeque<Vec<usize>>,
}

impl<'a, D> KFoldIterator<'a, D>
where
    D: DataSet,
{
    /// Construct a new k-fold iterator.
    #[inline]
    pub fn new<R: Rng>(data_set: &'a D, rng: &mut R, k: usize) -> Self {
        // Get sample size.
        let n = data_set.sample_size();

        // Check that `k` is not greater than the total number of samples.
        assert!(k <= n, "k is greater than the total number of samples.");

        // Allocate split indices.
        let mut indices = (0..n).collect_vec();
        // Shuffle split indices.
        indices.shuffle(rng);
        // Compute chunk size.
        let chunk_size = n / k + ((n % k) > 0) as usize;
        // Split indices in `n` chunks.
        let indices = indices.chunks(chunk_size).map(|i| i.to_vec()).collect();

        Self { data_set, indices }
    }
}

impl<'a, D, T> Iterator for KFoldIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.indices.is_empty() {
            // ... return `None`.
            return None;
        }

        // Pop the next fold indices.
        let indices = self.indices.pop_front().unwrap();
        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((indices.len(), self.data_set.data().ncols()));
        // For each fold index ...
        for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indices.len(), Some(self.indices.len()))
    }
}

impl<'a, D, T> ExactSizeIterator for KFoldIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }
}

impl<'a, D, T> FusedIterator for KFoldIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
}

/// Leave-one-out split iterator.
pub struct LeaveOneOutIterator<'a, D> {
    data_set: &'a D,
    indices: Vec<usize>,
    skip: usize,
}

impl<'a, D> LeaveOneOutIterator<'a, D>
where
    D: DataSet,
{
    /// Construct a new leave-one-out iterator.
    #[inline]
    pub fn new<R: Rng>(data_set: &'a D, rng: &mut R) -> Self {
        // Get sample size.
        let n = data_set.sample_size();
        // Allocate split indices.
        let mut indices = (0..n).collect_vec();
        // Shuffle split indices.
        indices.shuffle(rng);
        // Initialize the skip counter.
        let skip = 0;

        Self {
            data_set,
            indices,
            skip,
        }
    }
}

impl<'a, D, T> Iterator for LeaveOneOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.skip >= self.indices.len() {
            // ... return `None`.
            return None;
        }

        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((
            self.data_set.sample_size() - 1,
            self.data_set.data().ncols(),
        ));
        // Align rows and indices.
        let indices = self.indices.iter().enumerate();
        // Filter out the skip index.
        let indices = indices.filter_map(|(i, j)| (self.skip != i).then_some(j));
        // For each fold index ...
        for (mut row, &i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }
        // Increment the skip counter.
        self.skip += 1;

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indices.len(), Some(self.indices.len()))
    }
}

impl<'a, D, T> ExactSizeIterator for LeaveOneOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }
}

impl<'a, D, T> FusedIterator for LeaveOneOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
}

/// Leave-p-out split iterator.
pub struct LeavePOutIterator<'a, D> {
    data_set: &'a D,
    indices: Vec<Vec<usize>>,
    skip: usize,
}

impl<'a, D> LeavePOutIterator<'a, D>
where
    D: DataSet,
{
    /// Construct a new leave-p-out iterator.
    #[inline]
    pub fn new<R: Rng>(data_set: &'a D, rng: &mut R, p: usize) -> Self {
        // Get sample size.
        let n = data_set.sample_size();

        // Check that `p` is not greater than the total number of samples.
        assert!(p <= n, "p is greater than the total number of samples.");

        // Allocate split indices.
        let mut indices = (0..n).collect_vec();
        // Shuffle split indices.
        indices.shuffle(rng);
        // Compute chunk size.
        let chunk_size = n / p + ((n % p) > 0) as usize;
        // Split indices in `n` chunks.
        let indices = indices.chunks(chunk_size).map(|i| i.to_vec()).collect();
        // Initialize the skip counter.
        let skip = 0;

        Self {
            data_set,
            indices,
            skip,
        }
    }
}

impl<'a, D, T> Iterator for LeavePOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.skip >= self.indices.len() {
            // ... return `None`.
            return None;
        }

        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((
            self.data_set.sample_size() - self.indices[self.skip].len(),
            self.data_set.data().ncols(),
        ));
        // Align rows and indices.
        let indices = self.indices.iter().enumerate();
        // Filter out the skip index.
        let indices = indices.filter_map(|(i, j)| (self.skip != i).then_some(j));
        // For each fold index ...
        for (mut row, &i) in data.rows_mut().into_iter().zip(indices.flatten()) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }
        // Increment the skip counter.
        self.skip += 1;

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indices.len(), Some(self.indices.len()))
    }
}

impl<'a, D, T> ExactSizeIterator for LeavePOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }
}

impl<'a, D, T> FusedIterator for LeavePOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
}

impl<D, T> DataSetSplit for D
where
    D: DataSet<Data = Array2<T>>,
    T: Clone + Zero,
{
    type KFoldIter<'a> = KFoldIterator<'a, D> where D: 'a;

    type LeaveOneOutIter<'a> = LeaveOneOutIterator<'a, D> where D: 'a;

    type LeavePOutIter<'a> = LeavePOutIterator<'a, D> where D: 'a;

    fn train_test_split<R: Rng>(&self, rng: &mut R, test_percentage: f64) -> (Self, Self) {
        // Check that the test percentage is in the range `[0, 1]`.
        assert!(
            (0.0..=1.0).contains(&test_percentage),
            "Test percentage is not in the range [0, 1]."
        );

        // Get sample size.
        let n = self.sample_size();
        // Compute the number of test samples.
        let test_size = (n as f64 * test_percentage).round() as usize;
        // Compute the number of training samples.
        let train_size = n - test_size;

        // Allocate memory for the training data.
        let mut train_data = Self::Data::zeros((train_size, self.data().ncols()));
        // Allocate memory for the test data.
        let mut test_data = Self::Data::zeros((test_size, self.data().ncols()));

        // Initialize the training and test indices.
        let mut indices = (0..n).collect_vec();
        // Shuffle the indices.
        indices.shuffle(rng);
        // Split the indices into training and test indices.
        let (train_indices, test_indices) = indices.split_at(train_size);

        // For each training index ...
        for (mut row, &i) in train_data.rows_mut().into_iter().zip(train_indices) {
            // ... assign the training sample.
            row.assign(&self.data().row(i));
        }

        // For each test index ...
        for (mut row, &i) in test_data.rows_mut().into_iter().zip(test_indices) {
            // ... assign the test sample.
            row.assign(&self.data().row(i));
        }

        (
            Self::with_data_labels(train_data, self.labels().clone()),
            Self::with_data_labels(test_data, self.labels().clone()),
        )
    }

    #[inline]
    fn k_fold_iter<'a, R: Rng>(&'a self, rng: &mut R, k: usize) -> Self::KFoldIter<'a> {
        Self::KFoldIter::new(self, rng, k)
    }

    #[inline]
    fn leave_one_out_iter<'a, R: Rng>(&'a self, rng: &mut R) -> Self::LeaveOneOutIter<'a> {
        Self::LeaveOneOutIter::new(self, rng)
    }

    #[inline]
    fn leave_p_out_iter<'a, R: Rng>(&'a self, rng: &mut R, p: usize) -> Self::LeavePOutIter<'a> {
        Self::LeavePOutIter::new(self, rng, p)
    }
}

/* Test the `DataSetSplit` trait using `CategoricalDataMatrix`. */
#[cfg(test)]
mod test_data_set_split {
    use ndarray::prelude::*;
    use rand::prelude::*;
    use rand_xoshiro::Xoshiro256StarStar;

    use crate::prelude::*;

    #[test]
    #[should_panic]
    fn test_train_test_split_panic() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        data_set.train_test_split(&mut rng, 1.1);
    }

    #[test]
    fn test_train_test_split() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let (train_set, test_set) = data_set.train_test_split(&mut rng, 0.2);
        assert_eq!(train_set.sample_size(), 8);
        assert_eq!(test_set.sample_size(), 2);
        assert_eq!(train_set.labels(), data_set.labels());
        assert_eq!(test_set.labels(), data_set.labels());
    }

    #[test]
    #[should_panic]
    fn test_k_fold_iter_panic() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        data_set.k_fold_iter(&mut rng, 11);
    }

    #[test]
    fn test_k_fold_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let k_fold_iter = data_set.k_fold_iter(&mut rng, 5);
        assert_eq!(k_fold_iter.len(), 5);
        assert_eq!(k_fold_iter.size_hint(), (5, Some(5)));
        k_fold_iter.for_each(|fold| {
            assert_eq!(fold.sample_size(), 2);
            assert_eq!(fold.labels(), data_set.labels());
        });
    }

    #[test]
    fn test_leave_one_out_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let leave_one_out_iter = data_set.leave_one_out_iter(&mut rng);
        assert_eq!(leave_one_out_iter.len(), 10);
        assert_eq!(leave_one_out_iter.size_hint(), (10, Some(10)));
        leave_one_out_iter.for_each(|fold| {
            assert_eq!(fold.sample_size(), 9);
            assert_eq!(fold.labels(), data_set.labels());
        });
    }

    #[test]
    #[should_panic]
    fn test_leave_p_out_iter_panic() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        data_set.leave_p_out_iter(&mut rng, 11);
    }

    #[test]
    fn test_leave_p_out_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let leave_p_out_iter = data_set.leave_p_out_iter(&mut rng, 5);
        assert_eq!(leave_p_out_iter.len(), 5);
        assert_eq!(leave_p_out_iter.size_hint(), (5, Some(5)));
        leave_p_out_iter.for_each(|fold| {
            assert_eq!(fold.sample_size(), 8);
            assert_eq!(fold.labels(), data_set.labels());
        });
    }
}

/// Parallel data set split trait.
pub trait ParallelDataSetSplit: DataSet + Send {
    /// Parallel k-fold iterator type.
    type ParallelKFoldIter<'a>: ParallelIterator<Item = Self>
    where
        Self: 'a + Send;

    /// Parallel leave-one-out iterator type.
    type ParallelLeaveOneOutIter<'a>: ParallelIterator<Item = Self>
    where
        Self: 'a + Send;

    /// Parallel leave-p-out iterator type.
    type ParallelLeavePOutIter<'a>: ParallelIterator<Item = Self>
    where
        Self: 'a + Send;

    /// Split the data set into k-folds in parallel.
    ///
    /// # Panics
    ///
    /// Panics if `k` is greater than the total number of samples in the data set.
    ///
    /// # Note
    ///
    /// The data set is shuffled before splitting.
    /// The folds are not guaranteed to be of equal size, e.g. if `k` is not a divisor of
    /// the total number of samples, then there will be one fold with less than `k` samples.
    ///
    fn par_k_fold_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
        k: usize,
    ) -> Self::ParallelKFoldIter<'a>;

    /// Split the data set into leave-one-out folds in parallel.
    ///
    /// # Note
    ///
    /// The data set is shuffled before splitting.
    ///
    fn par_leave_one_out_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
    ) -> Self::ParallelLeaveOneOutIter<'a>;

    /// Split the data set into leave-p-out folds in parallel.
    ///
    /// # Panics
    ///
    /// Panics if `p` is greater than the total number of samples in the data set.
    ///
    /// # Note
    ///
    /// The data set is shuffled before splitting.
    ///
    fn par_leave_p_out_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
        p: usize,
    ) -> Self::ParallelLeavePOutIter<'a>;
}

/// Parallel k-fold split iterator.
pub struct ParallelKFoldIterator<'a, D> {
    data_set: &'a D,
    indices: VecDeque<Vec<usize>>,
}

impl<'a, D> ParallelKFoldIterator<'a, D>
where
    D: DataSet + Send,
{
    /// Construct a new parallel k-fold iterator.
    #[inline]
    pub fn new<R: Rng>(data_set: &'a D, rng: &mut R, k: usize) -> Self {
        // Get sample size.
        let n = data_set.sample_size();

        // Check that `k` is not greater than the total number of samples.
        assert!(k <= n, "k is greater than the total number of samples.");

        // Allocate split indices.
        let mut indices = (0..n).collect_vec();
        // Shuffle split indices.
        indices.shuffle(rng);
        // Compute chunk size.
        let chunk_size = n / k + ((n % k) > 0) as usize;
        // Split indices in `n` chunks.
        let indices = indices.chunks(chunk_size).map(|i| i.to_vec()).collect();

        Self { data_set, indices }
    }
}

impl<'a, D, T> ParallelIterator for ParallelKFoldIterator<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
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
        Some(self.indices.len())
    }
}

struct ParallelKFoldSplitProducer<'a, D> {
    data_set: &'a D,
    indices: VecDeque<Vec<usize>>,
}

impl<'a, D> From<ParallelKFoldIterator<'a, D>> for ParallelKFoldSplitProducer<'a, D> {
    #[inline]
    fn from(producer: ParallelKFoldIterator<'a, D>) -> Self {
        Self {
            data_set: producer.data_set,
            indices: producer.indices,
        }
    }
}

impl<'a, D, T> Iterator for ParallelKFoldSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.indices.is_empty() {
            // ... return `None`.
            return None;
        }

        // Pop the next fold indices.
        let indices = self.indices.pop_front().unwrap();
        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((indices.len(), self.data_set.data().ncols()));
        // For each fold index ...
        for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indices.len(), Some(self.indices.len()))
    }
}

impl<'a, D, T> ExactSizeIterator for ParallelKFoldSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }
}

impl<'a, D, T> DoubleEndedIterator for ParallelKFoldSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.indices.is_empty() {
            // ... return `None`.
            return None;
        }

        // Pop the next fold indices.
        let indices = self.indices.pop_back().unwrap();
        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((indices.len(), self.data_set.data().ncols()));
        // For each fold index ...
        for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }
}

impl<'a, D, T> Producer for ParallelKFoldSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type Item = D;

    type IntoIter = Self;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        // Split the indices.
        let mut a = self.indices;
        let b = a.split_off(index);
        a.shrink_to_fit();

        // Construct the producers.
        (Self { indices: a, ..self }, Self { indices: b, ..self })
    }
}

impl<'a, D, T> IndexedParallelIterator for ParallelKFoldIterator<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(ParallelKFoldSplitProducer::from(self))
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        // For each fold ...
        let producer = self.indices.into_par_iter().map(move |indices| {
            // Allocate memory for the fold data.
            let mut data = D::Data::zeros((indices.len(), self.data_set.data().ncols()));
            // For each fold index ...
            for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
                // ... assign the fold.
                row.assign(&self.data_set.data().row(i));
            }

            D::with_data_labels(data, self.data_set.labels().clone())
        });

        bridge(producer, consumer)
    }

    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }
}

/// Parallel leave-one-out split iterator.
pub struct ParallelLeaveOneOutIterator<'a, D> {
    data_set: &'a D,
    indices: Arc<Vec<usize>>,
    skip: usize,
    skip_max: usize,
}

impl<'a, D> ParallelLeaveOneOutIterator<'a, D>
where
    D: DataSet + Send,
{
    /// Construct a new parallel leave-one-out iterator.
    #[inline]
    pub fn new<R: Rng>(data_set: &'a D, rng: &mut R) -> Self {
        // Get sample size.
        let n = data_set.sample_size();
        // Allocate split indices.
        let mut indices = (0..n).collect_vec();
        // Shuffle split indices.
        indices.shuffle(rng);
        // Convert the indices to an Arc.
        let indices = Arc::new(indices);
        // Initialize the skip counter.
        let skip = 0;
        // Initialize the skip maximum.
        let skip_max = n;

        Self {
            data_set,
            indices,
            skip,
            skip_max,
        }
    }
}

impl<'a, D, T> ParallelIterator for ParallelLeaveOneOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
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
        Some(self.indices.len())
    }
}

struct ParallelLeaveOneOutSplitProducer<'a, D> {
    data_set: &'a D,
    indices: Arc<Vec<usize>>,
    skip: usize,
    skip_max: usize,
}

impl<'a, D> From<ParallelLeaveOneOutIterator<'a, D>> for ParallelLeaveOneOutSplitProducer<'a, D> {
    #[inline]
    fn from(producer: ParallelLeaveOneOutIterator<'a, D>) -> Self {
        Self {
            data_set: producer.data_set,
            indices: producer.indices,
            skip: producer.skip,
            skip_max: producer.skip_max,
        }
    }
}

impl<'a, D, T> Iterator for ParallelLeaveOneOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.skip >= self.skip_max {
            // ... return `None`.
            return None;
        }

        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((
            self.data_set.sample_size() - 1,
            self.data_set.data().ncols(),
        ));
        // Align rows and indices.
        let indices = self.indices.iter().enumerate();
        // Filter out the skip index.
        let indices = indices.filter_map(|(i, j)| (self.skip != i).then_some(j));
        // For each fold index ...
        for (mut row, &i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }
        // Increment the skip counter.
        self.skip += 1;

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Compute the remaining number of folds.
        let remaining = self.skip_max - self.skip;

        (remaining, Some(remaining))
    }
}

impl<'a, D, T> Producer for ParallelLeaveOneOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type Item = D;

    type IntoIter = Self;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        // Construct the producers.
        (
            Self {
                data_set: self.data_set,
                indices: self.indices.clone(),
                skip: self.skip,
                skip_max: index,
            },
            Self {
                data_set: self.data_set,
                indices: self.indices.clone(),
                skip: index,
                skip_max: self.skip_max,
            },
        )
    }
}

impl<'a, D, T> IndexedParallelIterator for ParallelLeaveOneOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(ParallelLeaveOneOutSplitProducer::from(self))
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        // For each fold ...
        let producer = (self.skip..self.skip_max).into_par_iter().map(|skip| {
            // Allocate memory for the fold data.
            let mut data = D::Data::zeros((
                self.data_set.sample_size() - 1,
                self.data_set.data().ncols(),
            ));
            // Align rows and indices.
            let indices = self.indices.iter().enumerate();
            // Filter out the skip index.
            let indices = indices.filter_map(|(i, j)| (skip != i).then_some(j));
            // For each fold index ...
            for (mut row, &i) in data.rows_mut().into_iter().zip(indices) {
                // ... assign the fold.
                row.assign(&self.data_set.data().row(i));
            }

            D::with_data_labels(data, self.data_set.labels().clone())
        });

        bridge(producer, consumer)
    }

    #[inline]
    fn len(&self) -> usize {
        self.skip_max - self.skip
    }
}

impl<'a, D, T> ExactSizeIterator for ParallelLeaveOneOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn len(&self) -> usize {
        self.skip_max - self.skip
    }
}

impl<'a, D, T> DoubleEndedIterator for ParallelLeaveOneOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.skip >= self.skip_max {
            // ... return `None`.
            return None;
        }

        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((
            self.data_set.sample_size() - 1,
            self.data_set.data().ncols(),
        ));
        // Align rows and indices.
        let indices = self.indices.iter().enumerate();
        // Filter out the skip index.
        let indices = indices.filter_map(|(i, j)| ((self.skip_max - 1) != i).then_some(j));
        // For each fold index ...
        for (mut row, &i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }
        // Decrement the skip_max counter.
        self.skip_max -= 1;

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }
}

/// Parallel leave-p-out split iterator.
pub struct ParallelLeavePOutIterator<'a, D> {
    data_set: &'a D,
    indices: Arc<Vec<Vec<usize>>>,
    skip: usize,
    skip_max: usize,
}

impl<'a, D> ParallelLeavePOutIterator<'a, D>
where
    D: DataSet + Send,
{
    /// Construct a new parallel leave-p-out iterator.
    #[inline]
    pub fn new<R: Rng>(data_set: &'a D, rng: &mut R, p: usize) -> Self {
        // Get sample size.
        let n = data_set.sample_size();

        // Check that `p` is not greater than the total number of samples.
        assert!(p <= n, "p is greater than the total number of samples.");

        // Allocate split indices.
        let mut indices = (0..n).collect_vec();
        // Shuffle split indices.
        indices.shuffle(rng);
        // Compute chunk size.
        let chunk_size = n / p + ((n % p) > 0) as usize;
        // Split indices in `n` chunks.
        let indices = indices.chunks(chunk_size).map(|i| i.to_vec()).collect();
        // Convert the indices to an Arc.
        let indices = Arc::new(indices);
        // Initialize the skip counter.
        let skip = 0;
        // Initialize the skip maximum.
        let skip_max = p;

        Self {
            data_set,
            indices,
            skip,
            skip_max,
        }
    }
}

impl<'a, D, T> ParallelIterator for ParallelLeavePOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
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
        Some(self.skip_max - self.skip)
    }
}

struct ParallelLeavePOutSplitProducer<'a, D> {
    data_set: &'a D,
    indices: Arc<Vec<Vec<usize>>>,
    skip: usize,
    skip_max: usize,
}

impl<'a, D> From<ParallelLeavePOutIterator<'a, D>> for ParallelLeavePOutSplitProducer<'a, D> {
    #[inline]
    fn from(producer: ParallelLeavePOutIterator<'a, D>) -> Self {
        Self {
            data_set: producer.data_set,
            indices: producer.indices,
            skip: producer.skip,
            skip_max: producer.skip_max,
        }
    }
}

impl<'a, D, T> Iterator for ParallelLeavePOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.skip >= self.skip_max {
            // ... return `None`.
            return None;
        }

        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((
            self.data_set.sample_size() - self.indices[self.skip].len(),
            self.data_set.data().ncols(),
        ));
        // Align rows and indices.
        let indices = self.indices.iter().enumerate();
        // Filter out the skip index.
        let indices = indices.filter_map(|(i, j)| (self.skip != i).then_some(j));
        // For each fold index ...
        for (mut row, &i) in data.rows_mut().into_iter().zip(indices.flatten()) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }
        // Increment the skip counter.
        self.skip += 1;

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Compute the remaining number of folds.
        let remaining = self.skip_max - self.skip;

        (remaining, Some(remaining))
    }
}

impl<'a, D, T> Producer for ParallelLeavePOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type Item = D;

    type IntoIter = Self;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        // Construct the producers.
        (
            Self {
                data_set: self.data_set,
                indices: self.indices.clone(),
                skip: self.skip,
                skip_max: index,
            },
            Self {
                data_set: self.data_set,
                indices: self.indices.clone(),
                skip: index,
                skip_max: self.skip_max,
            },
        )
    }
}

impl<'a, D, T> IndexedParallelIterator for ParallelLeavePOutIterator<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(ParallelLeavePOutSplitProducer::from(self))
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        // For each fold ...
        let producer = (self.skip..self.skip_max).into_par_iter().map(|skip| {
            // Allocate memory for the fold data.
            let mut data = D::Data::zeros((
                self.data_set.sample_size() - self.indices[skip].len(),
                self.data_set.data().ncols(),
            ));
            // Align rows and indices.
            let indices = self.indices.iter().enumerate();
            // Filter out the skip index.
            let indices = indices.filter_map(|(i, j)| (skip != i).then_some(j));
            // For each fold index ...
            for (mut row, &i) in data.rows_mut().into_iter().zip(indices.flatten()) {
                // ... assign the fold.
                row.assign(&self.data_set.data().row(i));
            }

            D::with_data_labels(data, self.data_set.labels().clone())
        });

        bridge(producer, consumer)
    }

    #[inline]
    fn len(&self) -> usize {
        self.skip_max - self.skip
    }
}

impl<'a, D, T> ExactSizeIterator for ParallelLeavePOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn len(&self) -> usize {
        self.skip_max - self.skip
    }
}

impl<'a, D, T> DoubleEndedIterator for ParallelLeavePOutSplitProducer<'a, D>
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // If the remaining number of folds is zero ...
        if self.skip >= self.skip_max {
            // ... return `None`.
            return None;
        }

        // Allocate memory for the fold data.
        let mut data = D::Data::zeros((
            self.data_set.sample_size() - self.indices[self.skip].len(),
            self.data_set.data().ncols(),
        ));
        // Align rows and indices.
        let indices = self.indices.iter().enumerate();
        // Filter out the skip index.
        let indices = indices.filter_map(|(i, j)| ((self.skip_max - 1) != i).then_some(j));
        // For each fold index ...
        for (mut row, &i) in data.rows_mut().into_iter().zip(indices.flatten()) {
            // ... assign the fold.
            row.assign(&self.data_set.data().row(i));
        }
        // Decrement the skip_max counter.
        self.skip_max -= 1;

        Some(D::with_data_labels(data, self.data_set.labels().clone()))
    }
}

impl<D, T> ParallelDataSetSplit for D
where
    D: DataSet<Data = Array2<T>> + Send,
    T: Clone + Zero,
{
    type ParallelKFoldIter<'a> = ParallelKFoldIterator<'a, D>
    where
        D: 'a + Send;

    type ParallelLeaveOneOutIter<'a> = ParallelLeaveOneOutIterator<'a, D>
    where
        D: 'a + Send;

    type ParallelLeavePOutIter<'a> = ParallelLeavePOutIterator<'a, D>
    where
        D: 'a + Send;

    #[inline]
    fn par_k_fold_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
        k: usize,
    ) -> Self::ParallelKFoldIter<'a> {
        Self::ParallelKFoldIter::new(self, rng, k)
    }

    #[inline]
    fn par_leave_one_out_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
    ) -> Self::ParallelLeaveOneOutIter<'a> {
        Self::ParallelLeaveOneOutIter::new(self, rng)
    }

    #[inline]
    fn par_leave_p_out_iter<'a, R: Rng + SeedableRng + Send>(
        &'a self,
        rng: &mut R,
        p: usize,
    ) -> Self::ParallelLeavePOutIter<'a> {
        Self::ParallelLeavePOutIter::new(self, rng, p)
    }
}

/* Test the `ParallelDataSetSplit` trait using `CategoricalDataMatrix`. */
#[cfg(test)]
mod test_parallel_data_set_split {
    use ndarray::prelude::*;
    use rand::prelude::*;
    use rand_xoshiro::Xoshiro256StarStar;
    use rayon::prelude::*;

    use crate::prelude::*;

    #[test]
    #[should_panic]
    fn test_par_k_fold_iter_panic() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        data_set.par_k_fold_iter(&mut rng, 11);
    }

    #[test]
    fn test_par_k_fold_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let k_fold_iter = data_set.par_k_fold_iter(&mut rng, 5);
        assert_eq!(k_fold_iter.len(), 5);
        assert_eq!(k_fold_iter.opt_len(), Some(5));
        k_fold_iter.for_each(|fold| {
            assert_eq!(fold.sample_size(), 2);
            assert_eq!(fold.labels(), data_set.labels());
        });
    }

    #[test]
    fn test_par_leave_one_out_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let leave_one_out_iter = data_set.par_leave_one_out_iter(&mut rng);
        assert_eq!(leave_one_out_iter.len(), 10);
        assert_eq!(leave_one_out_iter.opt_len(), Some(10));
        leave_one_out_iter.for_each(|fold| {
            assert_eq!(fold.sample_size(), 9);
            assert_eq!(fold.labels(), data_set.labels());
        });
    }

    #[test]
    #[should_panic]
    fn test_par_leave_p_out_iter_panic() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        data_set.par_leave_p_out_iter(&mut rng, 11);
    }

    #[test]
    fn test_par_leave_p_out_iter() {
        let data = Array2::zeros((10, 2));
        let labels = [("X", ["a", "b", "c"]), ("Y", ["a", "b", "c"])]
            .into_iter()
            .map(|(l, s)| (l.into(), s.iter().map(|&s| s.into()).collect()))
            .collect();
        let data_set = CategoricalDataMatrix::with_data_labels(data, labels);
        let mut rng = Xoshiro256StarStar::seed_from_u64(42);
        let leave_p_out_iter = data_set.par_leave_p_out_iter(&mut rng, 5);
        assert_eq!(leave_p_out_iter.len(), 5);
        assert_eq!(leave_p_out_iter.opt_len(), Some(5));
        leave_p_out_iter.for_each(|fold| {
            assert_eq!(fold.sample_size(), 8);
            assert_eq!(fold.labels(), data_set.labels());
        });
    }
}
