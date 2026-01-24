#![allow(missing_docs)]
#![allow(unused_doc_comments)]

use pyo3::{create_exception, exceptions::PyException, PyErr};

/// A custom exception type for the `causal-hub` package.
create_exception!(causal_hub, Error, PyException);

/// Convert a backend error into a Python exception.
#[inline]
pub fn backend_error_to_pyerr(err: backend::types::Error) -> PyErr {
    Error::new_err(err.to_string())
}

/// Convert a lock poisoning error into a Python exception.
#[inline]
pub fn poison_error_to_pyerr<T>(_err: std::sync::PoisonError<T>) -> PyErr {
    Error::new_err("Failed to acquire lock: lock poisoned")
}
