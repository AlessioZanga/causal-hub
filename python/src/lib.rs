pub mod distributions;
pub mod graphs;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod causal_hub {
    use super::*;

    #[pymodule]
    mod distributions {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Set the module members.
            m.add_class::<crate::distributions::PyCategoricalCPD>()?;
            // Import the submodules.
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.distributions", m)
            })
        }
    }

    #[pymodule]
    mod graphs {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Set the module members.
            m.add_class::<crate::graphs::PyDiGraph>()?;
            // Import the submodules.
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.graphs", m)
            })
        }
    }
}
