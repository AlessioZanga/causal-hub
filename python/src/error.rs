#![allow(missing_docs)]
#![allow(unused_doc_comments)]

use pyo3::{PyErr, create_exception, exceptions::PyException};

/// A custom exception type for the `causal-hub` package.
create_exception!(causal_hub, Error, PyException);

/// Converts a backend error into the custom `causal_hub.Error` Python exception.
///
/// This function is the single entry point for mapping `backend::types::Error`
/// values across the FFI boundary: always prefer importing and calling it
/// (i.e., `use crate::error::to_pyerr;` plus `.map_err(to_pyerr)`).
///
/// Errors that originate from PyO3 itself (e.g., argument extraction errors,
/// which surface as native `TypeError`s) must not be wrapped into the custom
/// exception: convert them with `PyErr::from` (or the `?` operator) instead.
///
#[inline]
pub fn to_pyerr(e: backend::types::Error) -> PyErr {
    Error::new_err(e.to_string())
}
