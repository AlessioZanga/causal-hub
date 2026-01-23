use itertools::{Either, Itertools};
use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;

use crate::{
    models::Labelled,
    types::{Error, Labels, Result, Set},
};

/// A struct for missing information in a tabular dataset.
#[derive(Clone, Debug)]
pub struct MissingTable {
    labels: Labels,
    fully_observed: Set<usize>,
    partially_observed: Set<usize>,
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
    complete_cols_count: usize,
    complete_rows_count: usize,
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
    pub fn new(mut labels: Labels, mut missing_mask: Array2<bool>) -> Result<Self> {
        // Check if dimensions match.
        if labels.len() != missing_mask.ncols() {
            return Err(Error::Dataset(format!(
                "Number of labels ({}) must match the number of columns in the missing mask ({}).",
                labels.len(),
                missing_mask.ncols()
            )));
        }

        // Check if labels are sorted.
        if !labels.is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<usize> = (0..labels.len()).collect();
            // Sort the indices by labels.
            indices.sort_by_key(|&i| &labels[i]);
            // Sort the labels.
            labels.sort();
            // Allocate new missing mask.
            let mut new_missing_mask = missing_mask.clone();
            // Sort the new missing mask according to the sorted indices.
            indices.into_iter().enumerate().for_each(|(i, j)| {
                new_missing_mask
                    .column_mut(i)
                    .assign(&missing_mask.column(j));
            });
            // Update missing mask.
            missing_mask = new_missing_mask;
        }

        // Compute missing counts.
        let missing_count_by_cols = missing_mask.rows().into_iter().fold(
            // Map to numeric one at a time to save memory.
            Array::zeros(missing_mask.ncols()),
            |acc, row| acc + row.mapv(|x| x as usize),
        );
        let missing_count_by_rows = missing_mask.columns().into_iter().fold(
            // Map to numeric one at a time to save memory.
            Array::zeros(missing_mask.nrows()),
            |acc, col| acc + col.mapv(|x| x as usize),
        );
        let missing_count = missing_count_by_cols.sum();

        // Compute missing mask by cols and rows.
        let missing_mask_by_cols = missing_count_by_cols.mapv(|x| x > 0);
        let missing_mask_by_rows = missing_count_by_rows.mapv(|x| x > 0);

        // Compute fully and partially observed variable sets.
        let (fully_observed, partially_observed) = missing_mask_by_cols
            .iter()
            .enumerate()
            .partition_map(|(i, &x)| {
                if !x {
                    Either::Left(i)
                } else {
                    Either::Right(i)
                }
            });

        // Compute complete counts.
        let complete_cols_count = missing_mask_by_cols.mapv(|x| (!x) as usize).sum();
        let complete_rows_count = missing_mask_by_rows.mapv(|x| (!x) as usize).sum();

        // Compute missing rates.
        let missing_rate_by_cols =
            missing_count_by_cols.mapv(|x| x as f64) / missing_mask.nrows() as f64;
        let missing_rate_by_rows =
            missing_count_by_rows.mapv(|x| x as f64) / missing_mask.ncols() as f64;
        let missing_rate = missing_count as f64 / missing_mask.len() as f64;

        // TODO: Make this optional for large datasets.
        // Map to numeric (float) mask.
        let missing_mask_numeric = missing_mask.mapv(|x| x as u8 as f64);
        // Transpose for correlation/covariance computation.
        let missing_mask_numeric = missing_mask_numeric.t();
        // Compute missing correlation.
        let missing_correlation = missing_mask_numeric
            .pearson_correlation()
            .map_err(|e| Error::Stats(e.to_string()))?;
        // Compute missing covariance.
        let missing_covariance = missing_mask_numeric
            .cov(1.)
            .map_err(|e| Error::Stats(e.to_string()))?;

        Ok(Self {
            labels,
            fully_observed,
            partially_observed,
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
            complete_cols_count,
            complete_rows_count,
        })
    }

    /// Get the set of fully observed variables.
    ///
    /// # Returns
    ///
    /// A reference to the set of fully observed variables.
    ///
    #[inline]
    pub const fn fully_observed(&self) -> &Set<usize> {
        &self.fully_observed
    }

    /// Get the set of partially observed variables.
    ///
    /// # Returns
    ///
    /// A reference to the set of partially observed variables.
    ///
    #[inline]
    pub const fn partially_observed(&self) -> &Set<usize> {
        &self.partially_observed
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

    /// Get the count of complete columns (without any missing values) in the table.
    ///
    /// # Returns
    ///
    /// The count of complete columns.
    ///
    #[inline]
    pub const fn complete_cols_count(&self) -> usize {
        self.complete_cols_count
    }

    /// Get the count of complete rows (without any missing values) in the table.
    ///
    /// # Returns
    ///
    /// The count of complete rows.
    ///
    #[inline]
    pub const fn complete_rows_count(&self) -> usize {
        self.complete_rows_count
    }
}
