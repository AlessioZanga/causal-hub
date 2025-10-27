mod table;
pub use table::*;

mod trajectory;
use either::Either;
pub use trajectory::*;

use crate::types::Set;

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
#[derive(Clone, Debug)]
pub enum MissingMethod {
    /// List-wise deletion missing handling method.
    LW,
    /// Pair-wise deletion missing handling method with specified columns.
    PW(Set<usize>),
    /// Inverse probability weighting missing handling method with specified columns.
    IPW(Set<usize>),
    /// Augmented inverse probability weighting missing handling method with specified columns.
    AIPW(Set<usize>),
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

    /// Apply the specified missing data handling method.
    ///
    /// # Arguments
    ///
    /// * `missing_method` - The missing data handling method to apply.
    ///
    /// # Returns
    ///
    /// Either a complete dataset or a weighted dataset, depending on the method applied.
    ///
    fn apply(&self, missing_method: &MissingMethod) -> Either<Self::Complete, Self::Weighted> {
        match missing_method {
            MissingMethod::LW => Either::Left(self.lw_deletion()),
            MissingMethod::PW(x) => Either::Left(self.pw_deletion(x)),
            MissingMethod::IPW(x) => Either::Right(self.ipw_deletion(x)),
            MissingMethod::AIPW(x) => Either::Right(self.aipw_deletion(x)),
        }
    }

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
    ///
    /// # Returns
    ///
    /// A weighted dataset restricted to the specified columns via IPW deletion.
    ///
    fn ipw_deletion(&self, x: &Set<usize>) -> Self::Weighted;

    /// Perform augmented inverse probability weighting (AIPW) deletion to handle missing data for the specified columns.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of column indices for AIPW deletion.
    ///
    /// # Returns
    ///
    /// A weighted dataset restricted to the specified columns via AIPW deletion.
    ///
    fn aipw_deletion(&self, x: &Set<usize>) -> Self::Weighted;
}
