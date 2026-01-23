use pyo3::{create_exception, exceptions::PyException};

create_exception!(causal_hub, Error, PyException);
