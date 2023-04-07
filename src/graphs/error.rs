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
    /// Multiple types of edges between nodes
    #[error("Multiple types of edges between two nodes are not allowed")]
    MultipleTypesEdges,
}
