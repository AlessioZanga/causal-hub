#![warn(missing_docs)]
//! # CausalHub
//!
//! CausalHub is a library for causal inference and causal discovery.
//! It provides tools for estimating causal effects, learning causal structures, and more.

/// Assets such as datasets, models, and other resources.
pub mod assets;
/// Dataset structures.
pub mod datasets;
/// Estimators for parameters and structures.
pub mod estimators;
/// Models.
pub mod models;
/// Utility functions.
pub mod utils;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

/// A Python module implemented in Rust.
#[pymodule]
mod causal_hub {
    use super::*;

    /// Submodule `assets`.
    #[pymodule]
    mod assets {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Initialization.
            Python::attach(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.assets", m)
            })?;

            // Categorical BNs.
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

            // Gaussian BNs.
            m.add_function(wrap_pyfunction!(crate::assets::load_arth150, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_ecoli70, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_magic_irri, m)?)?;
            m.add_function(wrap_pyfunction!(crate::assets::load_magic_niab, m)?)?;

            // Categorical CTBNs.
            m.add_function(wrap_pyfunction!(crate::assets::load_eating, m)?)?;

            Ok(())
        }
    }

    /// Submodule `datasets`.
    #[pymodule]
    mod datasets {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Initialization.
            Python::attach(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.datasets", m)
            })?;

            m.add_class::<crate::datasets::PyCatTable>()?;
            m.add_class::<crate::datasets::PyGaussTable>()?;
            m.add_class::<crate::datasets::PyCatTrj>()?;
            m.add_class::<crate::datasets::PyCatTrjs>()?;
            m.add_class::<crate::datasets::PyCatTrjEv>()?;
            m.add_class::<crate::datasets::PyCatTrjsEv>()?;
            m.add_class::<crate::datasets::PyCatWtdTrj>()?;
            m.add_class::<crate::datasets::PyCatWtdTrjs>()?;

            Ok(())
        }
    }

    /// Submodule `estimation`.
    #[pymodule]
    mod estimation {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Initialization.
            Python::attach(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.estimation", m)
            })?;

            m.add_function(wrap_pyfunction!(crate::estimators::em, m)?)?;
            m.add_function(wrap_pyfunction!(crate::estimators::sem, m)?)?;
            m.add_class::<crate::estimators::PyPK>()?;

            Ok(())
        }
    }

    /// Submodule `models`.
    #[pymodule]
    mod models {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Initialization.
            Python::attach(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("causal_hub.models", m)
            })?;

            m.add_class::<crate::models::PyCatBN>()?;
            m.add_class::<crate::models::PyGaussBN>()?;
            m.add_class::<crate::models::PyCatCTBN>()?;
            m.add_class::<crate::models::PyCatCPD>()?;
            m.add_class::<crate::models::PyGaussCPD>()?;
            m.add_class::<crate::models::PyCatCIM>()?;
            m.add_class::<crate::models::PyDiGraph>()?;

            Ok(())
        }
    }

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Initialize the logger.
        pyo3_log::init();

        Ok(())
    }
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
