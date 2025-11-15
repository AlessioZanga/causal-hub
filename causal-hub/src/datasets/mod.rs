mod table;
pub use table::*;

mod trajectory;
use itertools::Either;
pub use trajectory::*;

use crate::types::{Map, Set};

/// A trait for dataset.
pub trait Dataset {
    /// The type of the values.
    type Values;

    /// The values of the dataset.
    ///
    /// # Returns
    ///
    /// A reference to the values.
    ///
    fn values(&self) -> &Self::Values;

    /// The sample size.
    ///
    /// # Notes
    ///
    /// If the dataset is weighted, this should return the sum of the weights.
    ///
    /// # Returns
    ///
    /// The number of samples in the dataset.
    ///
    fn sample_size(&self) -> f64;
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
    /// * `r` - An optional missing mechanism specification.
    ///
    /// # Returns
    ///
    /// Either a complete or weighted dataset.
    ///
    fn apply_missing_method(
        &self,
        m: &MissingMethod,
        x: Option<&Set<usize>>,
        r: Option<&Map<usize, Set<usize>>>,
    ) -> Either<Self::Complete, Self::Weighted>;

    /// Perform list-wise (LW) deletion to handle missing data.
    ///
    /// # Returns
    ///
    /// A complete dataset obtained via LW deletion.
    ///
    fn lw_deletion(&self) -> Self::Complete;

    /// Perform pair-wise (PW) deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for PW deletion.
    ///
    /// # Returns
    ///
    /// A complete dataset restricted to the specified columns via PW deletion.
    ///
    fn pw_deletion(&self, x: &Set<usize>) -> Self::Complete;

    /// Perform inverse probability weighting (IPW) deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for IPW deletion.
    /// * `r` - The missing data indicators.
    ///
    /// # Returns
    ///
    /// A weighted dataset restricted to the specified columns via IPW deletion.
    ///
    fn ipw_deletion(&self, x: &Set<usize>, r: &Map<usize, Set<usize>>) -> Self::Weighted;

    /// Perform augmented inverse probability weighting (AIPW) deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for AIPW deletion.
    /// * `r` - The missing data indicators.
    ///
    /// # Returns
    ///
    /// A weighted dataset restricted to the specified columns via AIPW deletion.
    ///
    fn aipw_deletion(&self, x: &Set<usize>, r: &Map<usize, Set<usize>>) -> Self::Weighted;
}
