use pyo3::{exceptions::PyException, prelude::*};
use thiserror::Error;

/// An enum representing custom errors.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// An error originating from the backend.
    #[error(transparent)]
    FFI(#[from] backend::types::Error),
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        PyException::new_err(err.to_string())
    }
}
