use std::{fmt::Debug, iter::FusedIterator};

use polars::prelude::*;
use rand::Rng;
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

    /// Draw `n` samples without replacement.
    ///
    /// # Panics
    ///
    /// Panics if `n` is higher than the total number of samples in the data set.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, sample_size: usize) -> Self;

    /// Draw `n` samples with replacement.
    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, sample_size: usize) -> Self;
}
