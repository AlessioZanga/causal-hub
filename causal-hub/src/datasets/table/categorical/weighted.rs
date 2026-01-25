use ndarray::prelude::*;

use crate::{
    datasets::{CatSample, CatTable, Dataset},
    models::Labelled,
    types::{Error, Labels, Result, Set, States},
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
    /// # Returns
    ///
    /// A new categorical weighted dataset instance.
    ///
    pub fn new(dataset: CatTable, weights: Array1<f64>) -> Result<Self> {
        // Check if the number of weights is equal to the number of samples.
        if dataset.values().nrows() != weights.len() {
            return Err(Error::InvalidParameter(
                "weights".into(),
                "must have the same length as the dataset".into(),
            ));
        }
        // Check if any weight is finite.
        if !weights.iter().all(|&w| w.is_finite()) {
            return Err(Error::InvalidParameter(
                "weights".into(),
                "must be finite".into(),
            ));
        }

        Ok(Self { dataset, weights })
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

    fn select(&self, x: &Set<usize>) -> Result<Self> {
        // Select the dataset.
        let dataset = self.dataset.select(x)?;
        // Select the weights.
        let weights = self.weights.clone();
        // Return the new weighted dataset.
        Self::new(dataset, weights)
    }
}
