use ndarray::prelude::*;

use crate::{datasets::Dataset, models::Labelled, types::Labels};

/// A type alias for a gaussian variable.
pub type GaussType = f64;
/// A type alias for a gaussian sample.
pub type GaussSample = Array1<GaussType>;

/// A struct representing a gaussian dataset.
#[derive(Clone, Debug)]
pub struct GaussTable {
    labels: Labels,
    values: Array2<GaussType>,
}

impl Labelled for GaussTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl GaussTable {
    /// Creates a new gaussian dataset.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the variables.
    /// * `values` - The values of the variables.
    ///
    /// # Panics
    ///
    /// * Panics if the number of columns in `values` does not match the number of `labels`.
    ///
    /// # Results
    ///
    /// A new gaussian dataset instance.
    ///
    pub fn new(labels: Labels, values: Array2<GaussType>) -> Self {
        // Assert that the number of labels matches the number of columns in values.
        assert_eq!(
            labels.len(),
            values.ncols(),
            "Number of labels must match number of columns in values."
        );

        // Sort labels and values accordingly.
        if !labels.is_sorted() {
            todo!() // FIXME:
        }
        // Assert values are finite.
        assert!(
            values.iter().all(|&x| x.is_finite()),
            "Values must have finite values."
        );

        Self { labels, values }
    }
}

impl Dataset for GaussTable {
    type Values = Array2<GaussType>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.values.nrows() as f64
    }
}
