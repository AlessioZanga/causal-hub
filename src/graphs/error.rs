use thiserror::Error;

/// Graph error enumerator.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorGraph {
    /// Inconsistent matrix error variant.
    #[error("Matrix must be consistent with inputs")]
    InconsistentMatrix,
    /// Non-square matrix error variant.
    #[error("Matrix must be square")]
    NonSquareMatrix,
    /// Non-symmetric matrix error variant.
    #[error("Matrix must be symmetric")]
    NonSymmetricMatrix,
}
