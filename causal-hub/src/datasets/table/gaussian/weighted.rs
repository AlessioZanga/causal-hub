use ndarray::prelude::*;

use crate::{
    datasets::{Dataset, GaussSample, GaussTable},
    models::Labelled,
    types::Labels,
};

/// A type alias for a Gaussian weighted sample.
pub type GaussWtdSample = (GaussSample, f64);

/// A multivariate Gaussian weighted dataset.
#[derive(Clone, Debug)]
pub struct GaussWtdTable {
    dataset: GaussTable,
    weights: Array1<f64>,
}

impl Labelled for GaussWtdTable {
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

impl GaussWtdTable {
    /// Creates a new Gaussian weighted dataset.
    ///
    /// # Arguments
    ///
    /// * `dataset` - The Gaussian dataset.
    /// * `weights` - The weights of the samples.
    ///
    /// # Panics
    ///
    /// * Panics if the number of weights is not equal to the number of samples.
    /// * Panics if any weight is not in the range [0, 1].
    ///
    /// # Returns
    ///
    /// A new Gaussian weighted dataset instance.
    ///
    pub fn new(dataset: GaussTable, weights: Array1<f64>) -> Self {
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

    /// Returns the weights of the samples in the Gaussian distribution.
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

impl Dataset for GaussWtdTable {
    type Values = GaussTable;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.dataset
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.weights.sum()
    }
}
