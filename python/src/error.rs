#![allow(missing_docs)]
#![allow(unused_doc_comments)]

use pyo3::{create_exception, exceptions::PyException};

/// A custom exception type for the `causal-hub` package.
create_exception!(causal_hub, Error, PyException);
