use std::sync::Arc;

use thiserror::Error;

/// The error type for this crate.
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// An error related to I/O operations.
    #[error(transparent)]
    Io(Arc<std::io::Error>),
    /// An error related to CSV parsing.
    #[error(transparent)]
    Csv(Arc<csv::Error>),
    /// An error related to JSON parsing.
    #[error(transparent)]
    Json(Arc<serde_json::Error>),
    /// An error related to UTF-8 conversion.
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    /// An error related to ndarray shape operations.
    #[error(transparent)]
    NdarrayShape(#[from] ndarray::ShapeError),
    /// An error related to ndarray statistics.
    #[error(transparent)]
    NdarrayMinMax(#[from] ndarray_stats::errors::MinMaxError),
    /// An error related to linear algebra operations.
    #[error("Linear Algebra error: {0}")]
    Linalg(String),
    /// An error related to probability calculations.
    #[error("Probability error: {0}")]
    Probability(String),
    /// An error related to parsing.
    #[error("Parsing error: {0}")]
    Parsing(String),
    /// An error related to missing data.
    #[error("Missing data error: {0}")]
    MissingData(String),
    /// An error related to datasets.
    #[error("Dataset error: {0}")]
    Dataset(String),
    /// An error related to models.
    #[error("Model error: {0}")]
    Model(String),
    /// An error related to statistics.
    #[error("Statistics error: {0}")]
    Stats(String),
    /// An error related to random distributions.
    #[error("Random distribution error: {0}")]
    RandDistr(String),
    /// An error related to illegal arguments.
    #[error("Illegal argument error: {0}")]
    IllegalArgument(String),
    /// An error related to shape.
    #[error("Shape error: {0}")]
    Shape(String),
    /// An error related to unreachable code.
    #[error("Unreachable error: {0}")]
    Unreachable(String),
    /// An error related to lock poisoning.
    #[error("Lock poisoning error: {0}")]
    Poison(String),
    /// Other errors.
    #[error(transparent)]
    Other(Arc<Box<dyn std::error::Error + Send + Sync>>),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(Arc::new(err))
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Self::Csv(Arc::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(Arc::new(err))
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Other(Arc::new(err))
    }
}

/// A specialized [`Result`] type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
