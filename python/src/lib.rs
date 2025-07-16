#![warn(missing_docs)]
//! # CausalHub
//!
//! CausalHub is a library for causal inference and causal discovery.
//! It provides tools for estimating causal effects, learning causal structures, and more.

/// Assets such as datasets, models, and other resources.
pub mod assets;
/// Dataset structures.
pub mod datasets;
/// Probability distributions.
pub mod distributions;
/// Estimators for parameters and structures.
pub mod estimators;
/// Graph structures and algorithms.
pub mod graphs;
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

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Initialize the logger.
        pyo3_log::init();

        // `assets` module

        // BNs
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

        // CTBNs
        m.add_function(wrap_pyfunction!(crate::assets::load_eating, m)?)?;

        // `datasets` module
        m.add_class::<crate::datasets::PyCatTrj>()?;
        m.add_class::<crate::datasets::PyCatTrjs>()?;
        m.add_class::<crate::datasets::PyCatTrjEv>()?;
        m.add_class::<crate::datasets::PyCatTrjsEv>()?;
        m.add_class::<crate::datasets::PyCatWtdTrj>()?;
        m.add_class::<crate::datasets::PyCatWtdTrjs>()?;

        // `distributions` module
        m.add_class::<crate::distributions::PyCatCPD>()?;
        m.add_class::<crate::distributions::PyCatCIM>()?;

        // `estimators` module
        m.add_function(wrap_pyfunction!(crate::estimators::em, m)?)?;
        m.add_function(wrap_pyfunction!(crate::estimators::sem, m)?)?;
        m.add_class::<crate::estimators::PyPK>()?;

        // `graphs` module
        m.add_class::<crate::graphs::PyDiGraph>()?;

        // `models` module
        m.add_class::<crate::models::PyCatBN>()?;
        m.add_class::<crate::models::PyCatCTBN>()?;

        Ok(())
    }
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
