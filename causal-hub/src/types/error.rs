use std::{panic::Location, sync::Arc};

use thiserror::Error;

/// The error kind type for this crate.
#[derive(Error, Debug, Clone)]
pub enum ErrorKind {
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
    /// An error related to float parsing.
    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),
    /// An error related to ndarray shape operations.
    #[error(transparent)]
    NdarrayShape(#[from] ndarray::ShapeError),
    /// An error related to ndarray statistics.
    #[error(transparent)]
    NdarrayMinMax(#[from] ndarray_stats::errors::MinMaxError),
    /// An error related to random distribution uniform sampling.
    #[error(transparent)]
    RandDistrUniform(#[from] rand_distr::uniform::Error),
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
    /// An error related to statistics.
    #[error("Statistics error: {0}")]
    Stats(String),
    /// An error related to random distributions.
    #[error("Random distribution error: {0}")]
    RandDistr(String),
    /// An error related to shape.
    #[error("Shape error: {0}")]
    Shape(String),
    /// An error related to unreachable code.
    #[error("Unreachable error: {0}")]
    Unreachable(String),
    /// An error related to lock poisoning.
    #[error("Lock poisoning error: {0}")]
    Poison(String),
    /// Index is out of bounds.
    #[error("Index `{0}` is out of bounds")]
    IndexOutOfBounds(usize),
    /// Labels must be unique.
    #[error("Labels must be unique.")]
    NonUniqueLabels,
    /// An error indicating that a set cannot be empty.
    #[error("Set {0} must not be empty")]
    EmptySet(String),
    /// An error indicating that two sets must be disjoint.
    #[error("Sets {0} and {1} must be disjoint")]
    SetsNotDisjoint(String, String),
    /// An error indicating that one set must be a subset of another.
    #[error("Set {0} must be a subset of set {1}")]
    SubsetMismatch(String, String),
    /// An error indicating that the graph must be a DAG.
    #[error("Graph must be a DAG")]
    NotADag,
    /// An error indicating that a parameter is invalid.
    #[error("Invalid parameter {0}: {1}")]
    InvalidParameter(String, String),
    /// An error indicating a conflict in prior knowledge.
    #[error("Prior knowledge conflict: {0}")]
    PriorKnowledgeConflict(String),
    /// An error indicating that the labels of the graphs are incompatible.
    #[error("Labels mismatch: {0} != {1}")]
    LabelMismatch(String, String),
    /// An error indicating that sufficient statistics are missing.
    #[error("Missing sufficient statistics")]
    MissingSufficientStatistics,
    /// An error indicating that the log-likelihood is missing.
    #[error("Missing log-likelihood")]
    MissingLogLikelihood,
    /// An error indicating that a CSV file is missing headers.
    #[error("CSV file must have headers")]
    MissingHeader,
    /// An error indicating that the shape of the data is incompatible.
    #[error("Incompatible shape: {0} != {1}")]
    IncompatibleShape(String, String),
    /// An error indicating that a state is missing.
    #[error("State {0} not found")]
    MissingState(String),
    /// An error indicating that a label is missing.
    #[error("Label {0} not found")]
    MissingLabel(String),
    /// An error indicating that a value is NaN.
    #[error("Value is NaN")]
    NanValue,
    /// An error indicating that a value is missing.
    #[error("Missing value at line {0}, column {1}")]
    MissingValue(usize, usize),
    /// An error indicating that an object construction failed.
    #[error("Object construction failed: {0}")]
    ConstructionError(String),
    /// Other errors.
    #[error(transparent)]
    Other(Arc<Box<dyn std::error::Error + Send + Sync>>),
}

/// The error type for this crate.
#[derive(Debug, Clone)]
pub struct Error {
    /// The error kind.
    pub kind: ErrorKind,
    /// The location of the error.
    pub location: &'static Location<'static>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.kind, self.location)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

impl From<ErrorKind> for Error {
    #[track_caller]
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            location: Location::caller(),
        }
    }
}

impl From<std::io::Error> for Error {
    #[track_caller]
    fn from(err: std::io::Error) -> Self {
        ErrorKind::Io(Arc::new(err)).into()
    }
}

impl From<csv::Error> for Error {
    #[track_caller]
    fn from(err: csv::Error) -> Self {
        ErrorKind::Csv(Arc::new(err)).into()
    }
}

impl From<serde_json::Error> for Error {
    #[track_caller]
    fn from(err: serde_json::Error) -> Self {
        ErrorKind::Json(Arc::new(err)).into()
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    #[track_caller]
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ErrorKind::Other(Arc::new(err)).into()
    }
}

impl From<std::string::FromUtf8Error> for Error {
    #[track_caller]
    fn from(err: std::string::FromUtf8Error) -> Self {
        ErrorKind::Utf8(err).into()
    }
}

impl From<std::num::ParseFloatError> for Error {
    #[track_caller]
    fn from(err: std::num::ParseFloatError) -> Self {
        ErrorKind::ParseFloat(err).into()
    }
}

impl From<ndarray::ShapeError> for Error {
    #[track_caller]
    fn from(err: ndarray::ShapeError) -> Self {
        ErrorKind::NdarrayShape(err).into()
    }
}

impl From<ndarray_stats::errors::MinMaxError> for Error {
    #[track_caller]
    fn from(err: ndarray_stats::errors::MinMaxError) -> Self {
        ErrorKind::NdarrayMinMax(err).into()
    }
}

impl From<rand_distr::uniform::Error> for Error {
    #[track_caller]
    fn from(err: rand_distr::uniform::Error) -> Self {
        ErrorKind::RandDistrUniform(err).into()
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

/// Helper to construct error with location.
#[track_caller]
pub fn err<T>(kind: ErrorKind) -> Result<T> {
    Err(Error::from(kind))
}

// Backward compatibility constructors.
impl Error {
    /// An error related to linear algebra operations.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Linalg(s: &str) -> Self {
        ErrorKind::Linalg(s.to_string()).into()
    }

    /// An error related to probability calculations.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Probability(s: &str) -> Self {
        ErrorKind::Probability(s.to_string()).into()
    }

    /// An error related to parsing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Parsing(s: &str) -> Self {
        ErrorKind::Parsing(s.to_string()).into()
    }

    /// An error related to missing data.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn MissingData(s: &str) -> Self {
        ErrorKind::MissingData(s.to_string()).into()
    }

    /// An error related to statistics.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Stats(s: &str) -> Self {
        ErrorKind::Stats(s.to_string()).into()
    }

    /// An error related to random distributions.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn RandDistr(s: &str) -> Self {
        ErrorKind::RandDistr(s.to_string()).into()
    }

    /// An error related to shape.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Shape(s: &str) -> Self {
        ErrorKind::Shape(s.to_string()).into()
    }

    /// An error related to unreachable code.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Unreachable(s: &str) -> Self {
        ErrorKind::Unreachable(s.to_string()).into()
    }

    /// An error related to lock poisoning.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Poison(s: &str) -> Self {
        ErrorKind::Poison(s.to_string()).into()
    }

    /// Index is out of bounds.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn IndexOutOfBounds(u: usize) -> Self {
        ErrorKind::IndexOutOfBounds(u).into()
    }

    /// Labels must be unique.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn NonUniqueLabels() -> Self {
        ErrorKind::NonUniqueLabels.into()
    }

    /// An error indicating that a set cannot be empty.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn EmptySet(s: &str) -> Self {
        ErrorKind::EmptySet(s.to_string()).into()
    }

    /// An error indicating that two sets must be disjoint.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn SetsNotDisjoint(s1: &str, s2: &str) -> Self {
        ErrorKind::SetsNotDisjoint(s1.to_string(), s2.to_string()).into()
    }

    /// An error indicating that one set must be a subset of another.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn SubsetMismatch(s1: &str, s2: &str) -> Self {
        ErrorKind::SubsetMismatch(s1.to_string(), s2.to_string()).into()
    }

    /// An error indicating that the graph must be a DAG.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn NotADag() -> Self {
        ErrorKind::NotADag.into()
    }

    /// An error indicating that a parameter is invalid.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn InvalidParameter(s1: &str, s2: &str) -> Self {
        ErrorKind::InvalidParameter(s1.to_string(), s2.to_string()).into()
    }

    /// An error indicating a conflict in prior knowledge.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn PriorKnowledgeConflict(s: &str) -> Self {
        ErrorKind::PriorKnowledgeConflict(s.to_string()).into()
    }

    /// An error indicating that the labels of the graphs are incompatible.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn LabelMismatch(s1: &str, s2: &str) -> Self {
        ErrorKind::LabelMismatch(s1.to_string(), s2.to_string()).into()
    }

    /// An error indicating that sufficient statistics are missing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn MissingSufficientStatistics() -> Self {
        ErrorKind::MissingSufficientStatistics.into()
    }

    /// An error indicating that the log-likelihood is missing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn MissingLogLikelihood() -> Self {
        ErrorKind::MissingLogLikelihood.into()
    }

    /// An error indicating that a CSV file is missing headers.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn MissingHeader() -> Self {
        ErrorKind::MissingHeader.into()
    }

    /// An error indicating that the shape of the data is incompatible.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn IncompatibleShape(s1: &str, s2: &str) -> Self {
        ErrorKind::IncompatibleShape(s1.to_string(), s2.to_string()).into()
    }

    /// An error indicating that a state is missing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn MissingState(s: &str) -> Self {
        ErrorKind::MissingState(s.to_string()).into()
    }

    /// An error indicating that a label is missing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn MissingLabel(s: &str) -> Self {
        ErrorKind::MissingLabel(s.to_string()).into()
    }

    /// An error indicating that a value is NaN.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn NanValue() -> Self {
        ErrorKind::NanValue.into()
    }

    /// An error indicating that a value is missing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn MissingValue(u1: usize, u2: usize) -> Self {
        ErrorKind::MissingValue(u1, u2).into()
    }

    /// An error indicating that an object construction failed.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn ConstructionError(s: &str) -> Self {
        ErrorKind::ConstructionError(s.to_string()).into()
    }
}

// Additional compatibility constructors for transparent variants
impl Error {
    /// An error related to I/O operations.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Io(err: Arc<std::io::Error>) -> Self {
        ErrorKind::Io(err).into()
    }

    /// An error related to CSV parsing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Csv(err: Arc<csv::Error>) -> Self {
        ErrorKind::Csv(err).into()
    }

    /// An error related to JSON parsing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Json(err: Arc<serde_json::Error>) -> Self {
        ErrorKind::Json(err).into()
    }

    /// An error related to UTF-8 conversion.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Utf8(err: std::string::FromUtf8Error) -> Self {
        ErrorKind::Utf8(err).into()
    }

    /// An error related to float parsing.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn ParseFloat(err: std::num::ParseFloatError) -> Self {
        ErrorKind::ParseFloat(err).into()
    }

    /// An error related to ndarray shape operations.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn NdarrayShape(err: ndarray::ShapeError) -> Self {
        ErrorKind::NdarrayShape(err).into()
    }

    /// An error related to ndarray statistics.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn NdarrayMinMax(err: ndarray_stats::errors::MinMaxError) -> Self {
        ErrorKind::NdarrayMinMax(err).into()
    }

    /// An error related to random distribution uniform sampling.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn RandDistrUniform(err: rand_distr::uniform::Error) -> Self {
        ErrorKind::RandDistrUniform(err).into()
    }

    /// Other errors.
    #[allow(non_snake_case)]
    #[track_caller]
    pub fn Other(err: Arc<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        ErrorKind::Other(err).into()
    }
}
