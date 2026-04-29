use ndarray::prelude::*;

use crate::{
    datasets::{Dataset, GaussEv, GaussSample, GaussTable},
    models::Labelled,
    types::{Error, Labels, Result, Set},
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
    /// # Returns
    ///
    /// A new Gaussian weighted dataset instance.
    ///
    pub fn new(dataset: GaussTable, weights: Array1<f64>) -> Result<Self> {
        // Check if the number of weights is equal to the number of samples.
        if dataset.values().nrows() != weights.len() {
            return Err(Error::IncompatibleShape(
                &dataset.values().nrows().to_string(),
                &weights.len().to_string(),
            ));
        }
        // Check that all weights are non-negative.
        if !weights.iter().all(|&w| w >= 0.0) {
            return Err(Error::InvalidParameter("weights", "must be non-negative"));
        }

        Ok(Self { dataset, weights })
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
    type Evidence = GaussEv;
    type EvidenceIter<'a> = <GaussTable as Dataset>::EvidenceIter<'a>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.dataset
    }

    fn evidence_iter(&self) -> Self::EvidenceIter<'_> {
        self.dataset.evidence_iter()
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

impl From<GaussTable> for GaussWtdTable {
    #[inline]
    fn from(dataset: GaussTable) -> Self {
        let weights = Array::ones(dataset.values().nrows());
        Self { dataset, weights }
    }
}
