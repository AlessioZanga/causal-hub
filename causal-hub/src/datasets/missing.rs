use std::{
    fmt::{Display, Formatter},
    ops::Index,
};

use itertools::{Either, Itertools};
use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;
use serde::{Deserialize, Serialize};

use crate::{
    datasets::Dataset,
    models::Labelled,
    types::{Error, Labels, Map, Result, Set},
};

/// A struct representing the missing data indicators.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct MissingMechanism {
    labels: Labels,
    pr: Map<usize, Set<usize>>,
}

impl MissingMechanism {
    /// Create a new missing mechanism.
    pub fn new(labels: Labels, mut pr: Map<usize, Set<usize>>) -> Result<Self> {
        // Check if all indices are within bounds.
        let n = labels.len();
        for (&x, ys) in &pr {
            if x >= n {
                return Err(Error::IndexOutOfBounds(x));
            }
            for &y in ys {
                if y >= n {
                    return Err(Error::IndexOutOfBounds(y));
                }
            }
        }

        // Sort the missing mechanism.
        pr.sort_keys();
        pr.iter_mut().for_each(|(_, ys)| ys.sort());

        Ok(Self { labels, pr })
    }

    /// Returns the number of missing variables.
    pub fn len(&self) -> usize {
        self.pr.len()
    }

    /// Checks if the missing mechanism is empty.
    pub fn is_empty(&self) -> bool {
        self.pr.is_empty()
    }

    /// Returns the missing variables.
    pub fn keys(&self) -> impl Iterator<Item = &usize> {
        self.pr.keys()
    }

    /// Returns the missingness parents.
    pub fn values(&self) -> impl Iterator<Item = &Set<usize>> {
        self.pr.values()
    }

    /// Checks if a variable is missing.
    pub fn contains_key(&self, x: &usize) -> bool {
        self.pr.contains_key(x)
    }

    /// Returns the missingness parents for a given variable.
    pub fn get(&self, x: &usize) -> Option<&Set<usize>> {
        self.pr.get(x)
    }

    /// Inserts a missing variable and its missingness parents.
    pub fn insert(&mut self, x: usize, mut y: Set<usize>) {
        // Sort the missingness parents.
        y.sort();
        // Insert in sorted order.
        self.pr.insert_sorted(x, y);
    }
}

impl Labelled for MissingMechanism {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl Index<usize> for MissingMechanism {
    type Output = Set<usize>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pr[&index]
    }
}

impl IntoIterator for MissingMechanism {
    type Item = (usize, Set<usize>);
    type IntoIter = indexmap::map::IntoIter<usize, Set<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pr.into_iter()
    }
}

impl<'a> IntoIterator for &'a MissingMechanism {
    type Item = (&'a usize, &'a Set<usize>);
    type IntoIter = indexmap::map::Iter<'a, usize, Set<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pr.iter()
    }
}

/// An enum representing different methods for handling missing data.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum MissingMethod {
    /// List-wise deletion missing handling method.
    LW,
    /// Pair-wise deletion missing handling method.
    PW,
    /// Inverse probability weighting missing handling method.
    IPW,
    /// Augmented inverse probability weighting missing handling method.
    AIPW,
}

/// Missing mechanism t types.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MissingType {
    /// Missing Completely At Random.
    MCAR,
    /// Missing At Random.
    MAR,
    /// Missing Not At Random.
    MNAR,
}

impl Display for MissingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MCAR => write!(f, "MCAR"),
            Self::MAR => write!(f, "MAR"),
            Self::MNAR => write!(f, "MNAR"),
        }
    }
}

/// A struct for missing information in a tabular dataset.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
            return Err(Error::IncompatibleShape(
                &format!("|labels| = {}", labels.len()),
                &format!("|cols| = {}", missing_mask.ncols()),
            ));
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
        let missing_count_by_cols = missing_mask.mapv(|x| x as usize).sum_axis(Axis(0));
        let missing_count_by_rows = missing_mask.mapv(|x| x as usize).sum_axis(Axis(1));
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
            .map_err(|evidence| Error::Stats(&evidence.to_string()))?;
        // Compute missing covariance.
        let missing_covariance = missing_mask_numeric
            .cov(1.)
            .map_err(|evidence| Error::Stats(&evidence.to_string()))?;

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

/// A trait for incomplete datasets.
pub trait IncDataset: Dataset + Sized {
    /// The type of the missing data indicator.
    type Missing;
    /// The value of the missing data indicator.
    const MISSING: Self::Missing;

    /// The type of the complete dataset.
    type Complete;
    /// The type of the weighted dataset.
    type Weighted;

    /// Get the missing information.
    ///
    /// # Returns
    ///
    /// A reference to the missing information.
    ///
    fn missing(&self) -> &MissingTable;

    /// Apply a missing data handling method to the dataset.
    ///
    /// # Arguments
    ///
    /// * `m` - The missing data handling method to apply.
    /// * `x` - An optional set of variables to consider for missing data handling.
    /// * `pr` - An optional missing mechanism specification.
    ///
    /// # Errors
    ///
    /// * If the set of variables to consider for missing data handling is empty.
    /// * If any variable in the set is out of bounds.
    ///
    /// # Returns
    ///
    /// Either a complete or weighted dataset.
    ///
    fn apply_missing_method(
        &self,
        model: &MissingMethod,
        x: Option<&Set<usize>>,
        pr: Option<&MissingMechanism>,
    ) -> Result<Either<Self::Complete, Self::Weighted>> {
        // Get short alias for missing method.
        use MissingMethod as MM;
        // Apply the missing method with the provided arguments.
        match (model, x, pr) {
            (MM::LW, _, _) => self.lw_deletion().map(Either::Left),
            (MM::PW, Some(x), _) => self.pw_deletion(x).map(Either::Left),
            (MM::IPW, Some(x), Some(pr)) => self.ipw_deletion(x, pr).map(Either::Right),
            (MM::AIPW, Some(x), Some(pr)) => self.aipw_deletion(x, pr).map(Either::Right),
            _ => Err(Error::InvalidParameter(
                "missing_method",
                &format!(
                    "Invalid arguments for applying missing method:\n\
                    \t missing method:      '{model:?}' , \n\
                    \t selected variables:  '{x:?}' , \n\
                    \t missing mechanism:   '{pr:?}' .",
                ),
            )),
        }
    }

    /// Compute the weights to perform IPW.
    fn ipw_weights(
        &self,
        d_u: &Self::Complete,
        u: &Set<usize>,
        pr: &MissingMechanism,
    ) -> Result<Array1<f64>>;

    /// Perform list-wise (LW) deletion to handle missing data.
    ///
    /// # Errors
    ///
    /// * If the dataset is empty after LW deletion.
    ///
    /// # Returns
    ///
    /// A complete dataset obtained via LW deletion.
    ///
    fn lw_deletion(&self) -> Result<Self::Complete>;

    /// Perform pair-wise (PW) deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for PW deletion.
    ///
    /// # Errors
    ///
    /// * If the set of variables to consider for missing data handling is empty.
    /// * If any variable in the set is out of bounds.
    ///
    /// # Returns
    ///
    /// A complete dataset restricted to the specified columns via PW deletion.
    ///
    fn pw_deletion(&self, x: &Set<usize>) -> Result<Self::Complete>;

    /// Perform inverse probability weighting (IPW) deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for IPW deletion.
    /// * `pr` - The missing data indicators.
    ///
    /// # Errors
    ///
    /// * If the set of variables to consider for missing data handling is empty.
    /// * If any variable in the set is out of bounds.
    ///
    /// # Returns
    ///
    /// A weighted dataset restricted to the specified columns via IPW deletion.
    ///
    fn ipw_deletion(&self, x: &Set<usize>, pr: &MissingMechanism) -> Result<Self::Weighted>;

    /// Perform augmented inverse probability weighting (AIPW) deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for AIPW deletion.
    /// * `pr` - The missing data indicators.
    ///
    /// # Errors
    ///
    /// * If the set of variables to consider for missing data handling is empty.
    /// * If any variable in the set is out of bounds.
    ///
    /// # Returns
    ///
    /// A weighted dataset restricted to the specified columns via AIPW deletion.
    ///
    fn aipw_deletion(&self, x: &Set<usize>, pr: &MissingMechanism) -> Result<Self::Weighted>;
}
