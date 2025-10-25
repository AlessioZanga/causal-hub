use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;

use crate::{models::Labelled, types::Labels};

/// A struct for missing information in a tabular dataset.
#[derive(Clone, Debug)]
pub struct MissingTable {
    labels: Labels,
    missing_mask: Array2<bool>,
    missing_mask_by_cols: Array1<bool>,
    missing_mask_by_rows: Array1<bool>,
    missing_count: usize,
    missing_count_by_cols: Array1<usize>,
    missing_count_by_rows: Array1<usize>,
    missing_rate: f64,
    missing_rate_by_cols: Array1<f64>,
    missing_rate_by_rows: Array1<f64>,
    missing_correlation: Array2<f64>,
    missing_covariance: Array2<f64>,
}

impl Labelled for MissingTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl MissingTable {
    /// Create a new missing information table from the given labels and missing mask.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the dataset.
    /// * `missing_mask` - A boolean matrix indicating missing values.
    ///
    /// # Returns
    ///
    /// A new missing information instance.
    ///
    pub fn new(labels: Labels, missing_mask: Array2<bool>) -> Self {
        // FIXME: Assertions.
        // FIXME: Check for sorted labels.

        // Map to numeric (integer) mask.
        let missing_mask_numeric = missing_mask.mapv(|x| x as usize);

        // Compute missing counts.
        let missing_count_by_cols = missing_mask_numeric.sum_axis(Axis(0));
        let missing_count_by_rows = missing_mask_numeric.sum_axis(Axis(1));
        let missing_count = missing_count_by_cols.sum();

        // Compute missing mask by cols and rows.
        let missing_mask_by_cols = missing_count_by_cols.mapv(|x| x > 0);
        let missing_mask_by_rows = missing_count_by_rows.mapv(|x| x > 0);

        // Compute missing rates.
        let missing_rate_by_cols =
            missing_count_by_cols.mapv(|x| x as f64) / missing_mask.nrows() as f64;
        let missing_rate_by_rows =
            missing_count_by_rows.mapv(|x| x as f64) / missing_mask.ncols() as f64;
        let missing_rate = missing_count as f64 / missing_mask.len() as f64;

        // Map to numeric (float) mask.
        let missing_mask_numeric = missing_mask_numeric.mapv(|x| x as f64);

        // Transpose for correlation/covariance computation.
        let missing_mask_numeric = missing_mask_numeric.t();

        // Compute missing correlation.
        let missing_correlation = missing_mask_numeric
            .pearson_correlation()
            .expect("Failed to compute missing correlation.");
        // Compute missing covariance.
        let missing_covariance = missing_mask_numeric
            .cov(1.)
            .expect("Failed to compute missing covariance.");

        Self {
            labels,
            missing_mask,
            missing_mask_by_cols,
            missing_mask_by_rows,
            missing_count,
            missing_count_by_cols,
            missing_count_by_rows,
            missing_rate,
            missing_rate_by_cols,
            missing_rate_by_rows,
            missing_correlation,
            missing_covariance,
        }
    }

    /// Get the missing mask indicating the presence of missing values in the table.
    ///
    /// # Returns
    ///
    /// A reference to the missing mask.
    ///
    #[inline]
    pub const fn missing_mask(&self) -> &Array2<bool> {
        &self.missing_mask
    }

    /// Get the missing mask indicating the presence of missing values in each column.
    ///
    /// # Returns
    ///
    /// A reference to the missing mask by columns.
    ///
    #[inline]
    pub const fn missing_mask_by_cols(&self) -> &Array1<bool> {
        &self.missing_mask_by_cols
    }

    /// Get the missing mask indicating the presence of missing values in each row.
    ///
    /// # Returns
    ///
    /// A reference to the missing mask by rows.
    ///
    #[inline]
    pub const fn missing_mask_by_rows(&self) -> &Array1<bool> {
        &self.missing_mask_by_rows
    }

    /// Get the total count of missing values in the table.
    ///
    /// # Returns
    ///
    /// The count of missing values.
    ///
    #[inline]
    pub const fn missing_count(&self) -> usize {
        self.missing_count
    }

    /// Get the count of missing values in each column.
    ///
    /// # Returns
    ///
    /// A reference to the missing count by columns.
    ///
    #[inline]
    pub const fn missing_count_by_cols(&self) -> &Array1<usize> {
        &self.missing_count_by_cols
    }

    /// Get the count of missing values in each row.
    ///
    /// # Returns
    ///
    /// A reference to the missing count by rows.
    ///
    #[inline]
    pub const fn missing_count_by_rows(&self) -> &Array1<usize> {
        &self.missing_count_by_rows
    }

    /// Get the overall missing rate in the table.
    ///
    /// # Returns
    ///
    /// The percentage of missing values.
    ///
    #[inline]
    pub const fn missing_rate(&self) -> f64 {
        self.missing_rate
    }

    /// Get the missing rate in each column.
    ///
    /// # Returns
    ///
    /// A reference to the missing percentage by columns.
    ///
    #[inline]
    pub const fn missing_rate_by_cols(&self) -> &Array1<f64> {
        &self.missing_rate_by_cols
    }

    /// Get the missing rate in each row.
    ///
    /// # Returns
    ///
    /// A reference to the missing percentage by rows.
    ///
    #[inline]
    pub const fn missing_rate_by_rows(&self) -> &Array1<f64> {
        &self.missing_rate_by_rows
    }

    /// Get the missing (Pearson) correlation matrix.
    ///
    /// # Returns
    ///
    /// A reference to the missing correlation matrix.
    ///
    #[inline]
    pub const fn missing_correlation(&self) -> &Array2<f64> {
        &self.missing_correlation
    }

    /// Get the missing (unbiased) covariance matrix.
    ///
    /// # Returns
    ///
    /// A reference to the missing covariance matrix.
    ///
    #[inline]
    pub const fn missing_covariance(&self) -> &Array2<f64> {
        &self.missing_covariance
    }
}
