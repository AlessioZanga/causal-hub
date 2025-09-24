use ndarray::prelude::*;

use crate::{
    datasets::{CatSample, CatTable, Dataset},
    models::Labelled,
    types::{Labels, States},
};

/// A type alias for a categorical weighted sample.
pub type CatWtdSample = (CatSample, f64);

/// A multivariate categorical weighted dataset.
#[derive(Clone, Debug)]
pub struct CatWtdTable {
    dataset: CatTable,
    weights: Array1<f64>,
}

impl Labelled for CatWtdTable {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

impl CatWtdTable {
    /// Creates a new categorical weighted dataset.
    ///
    /// # Arguments
    ///
    /// * `dataset` - The categorical dataset.
    /// * `weights` - The weights of the samples.
    ///
    /// # Panics
    ///
    /// * Panics if the number of weights is not equal to the number of samples.
    /// * Panics if any weight is not in the range [0, 1].
    ///
    /// # Returns
    ///
    /// A new categorical weighted dataset instance.
    ///
    pub fn new(dataset: CatTable, weights: Array1<f64>) -> Self {
        assert_eq!(
            dataset.values().nrows(),
            weights.len(),
            "The number of weights must be equal to the number of samples."
        );
        assert!(
            weights.iter().all(|&w| (0.0..=1.0).contains(&w)),
            "All weights must be in the range [0, 1]."
        );

        Self { dataset, weights }
    }

    /// Returns the states of the variables in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the vector of states.
    ///
    #[inline]
    pub const fn states(&self) -> &States {
        self.dataset.states()
    }

    /// Returns the shape of the set of states in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of shape.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        self.dataset.shape()
    }

    /// Returns the weights of the samples in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of weights.
    ///
    #[inline]
    pub const fn weights(&self) -> &Array1<f64> {
        &self.weights
    }
}

impl Dataset for CatWtdTable {
    type Values = CatTable;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.dataset
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.weights.sum()
    }
}
