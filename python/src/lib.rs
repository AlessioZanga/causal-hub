pub mod graphs;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn causal_hub(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
