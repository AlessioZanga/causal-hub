use std::borrow::Cow;

use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    datasets::{CatEv, CatSample, CatTable, Dataset},
    models::{CatSupport, HasLabels},
    types::{Error, Labels, Result, Set},
};

/// A type alias for a categorical weighted sample.
pub type CatWtdSample = (CatSample, f64);

/// A multivariate categorical weighted dataset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatWtdTable {
    dataset: CatTable,
    weights: Array1<f64>,
}

impl HasLabels for CatWtdTable {
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
    /// # Errors
    ///
    /// * If the number of weights is different from the number of samples.
    /// * If any weight is not finite.
    ///
    /// # Returns
    ///
    /// A new categorical weighted dataset instance.
    ///
    pub fn new(dataset: CatTable, weights: Array1<f64>) -> Result<Self> {
        // Check if the number of weights is equal to the number of samples.
        if dataset.values().nrows() != weights.len() {
            return Err(Error::InvalidParameter(
                "weights",
                "must have the same length as the dataset",
            ));
        }
        // Check if any weight is finite.
        if !weights.iter().all(|&w| w.is_finite()) {
            return Err(Error::InvalidParameter("weights", "must be finite"));
        }

        Ok(Self { dataset, weights })
    }

    /// Returns the support of the variables in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the vector of support.
    ///
    #[inline]
    pub const fn support(&self) -> &CatSupport {
        self.dataset.support()
    }

    /// Returns the shape of the set of support in the categorical distribution.
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
    type Support = CatSupport;
    type Evidence = CatEv;
    type EvidenceIter<'a> = <CatTable as Dataset>::EvidenceIter<'a>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.dataset
    }

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(self.dataset.support())
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

impl From<CatTable> for CatWtdTable {
    #[inline]
    fn from(dataset: CatTable) -> Self {
        let weights = Array::ones(dataset.values().nrows());
        Self { dataset, weights }
    }
}
