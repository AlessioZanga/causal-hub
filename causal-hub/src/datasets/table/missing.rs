use ndarray::prelude::*;

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

    /// Get the missing correlation matrix.
    ///
    /// # Returns
    ///
    /// A reference to the missing correlation matrix.
    ///
    #[inline]
    pub const fn missing_correlation(&self) -> &Array2<f64> {
        &self.missing_correlation
    }

    /// Get the missing covariance matrix.
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
