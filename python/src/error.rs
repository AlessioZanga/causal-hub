#![allow(missing_docs)]
#![allow(unused_doc_comments)]

use pyo3::{PyErr, create_exception, exceptions::PyException};

/// A custom exception type for the `causal-hub` package.
create_exception!(causal_hub, Error, PyException);

/// Convert a backend error into a Python exception.
#[inline]
pub fn to_pyerr(e: backend::types::Error) -> PyErr {
    Error::new_err(e.to_string())
}
