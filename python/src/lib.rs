pub mod assets;
pub mod distributions;
pub mod graphs;
pub mod models;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod causal_hub {
    use super::*;

    #[pymodule]
    mod assets {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Set the module members.
            m.add_function(wrap_pyfunction!(crate::assets::load_alarm, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_andes, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_asia, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_barley, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_cancer, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_child, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_diabetes, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_earthquake, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_hailfinder, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_hepar2, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_insurance, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_link, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_mildew, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_munin1, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_pathfinder, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_pigs, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_sachs, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_survey, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_water, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_win95pts, m)?)?;

            // Import the submodules.
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.assets", m)
            })
        }
    }

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

    #[pymodule]
    mod models {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Set the module members.
            m.add_class::<crate::models::PyCategoricalBN>()?;

            // Import the submodules.
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.models", m)
            })
        }
    }
}
