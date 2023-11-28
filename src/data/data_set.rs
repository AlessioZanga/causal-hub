use std::{fmt::Debug, iter::FusedIterator};

use ndarray::prelude::*;
use ndarray_rand::rand_distr::num_traits::Zero;
use polars::prelude::*;
use rand::{distributions::Uniform, prelude::*, seq::index};
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
    /// Draw `n` samples without replacement.
    ///
    /// # Panics
    ///
    /// Panics if `n` is greater than the total number of samples in the data set.
    ///
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self;

    /// Draw `n` samples with replacement.
    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self;
}

impl<D, A> DataSetSample for D
where
    D: DataSet<Data = Array2<A>>,
    A: Clone + Zero,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Check that the sample size is not greater than the total number of samples.
        assert!(
            n <= self.sample_size(),
            "Sample size is greater than the total number of samples."
        );

        // Allocate memory for the samples.
        let mut data = Array2::<A>::zeros((n, self.data().ncols()));

        // Initialize the sample indices.
        let indices = index::sample(rng, self.sample_size(), n);

        // For each sample index ...
        for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the sample.
            row.assign(&self.data().row(i));
        }

        Self::with_data_labels(data, self.labels().clone())
    }

    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Allocate memory for the samples.
        let mut data = Array2::<A>::zeros((n, self.data().ncols()));

        // Initialize the sample indices range.
        let indices = rng.sample_iter(Uniform::new(0, self.sample_size()));

        // For each sample ...
        for (mut row, i) in data.rows_mut().into_iter().zip(indices) {
            // ... assign the sample.
            row.assign(&self.data().row(i));
        }

        Self::with_data_labels(data, self.labels().clone())
    }
}
