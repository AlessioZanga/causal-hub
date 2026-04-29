use dry::macro_for;
use paste::paste;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use crate::models::{PyCatBN, PyCatCTBN, PyGaussBN};

macro_for!(
    $bn in [
        alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
        hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
        pigs, sachs, survey, water, win95pts
    ] {
    paste! {
        #[doc = "Load the `" $bn:upper "` categorical BN from the assets."]
        #[gen_stub_pyfunction(module = "causal_hub.assets")]
        #[pyfunction]
        pub fn [<load_ $bn>]() -> PyResult<PyCatBN> {
            backend::assets::[<load_ $bn>]()
                .map_err(crate::error::to_pyerr)
                .map(Into::into)
        }
    }
});

macro_for!(
    $bn in [
        arth150, ecoli70, magic_irri, magic_niab
    ] {
    paste! {
        #[doc = "Load the `" $bn:upper "` Gaussian BN from the assets."]
        #[gen_stub_pyfunction(module = "causal_hub.assets")]
        #[pyfunction]
        pub fn [<load_ $bn>]() -> PyResult<PyGaussBN> {
            backend::assets::[<load_ $bn>]()
                .map_err(crate::error::to_pyerr)
                .map(Into::into)
        }
    }
});

/// Load the `EATING` categorical CTBN from the assets.
#[gen_stub_pyfunction(module = "causal_hub.assets")]
#[pyfunction]
pub fn load_eating() -> PyResult<PyCatCTBN> {
    backend::assets::load_eating()
        .map_err(crate::error::to_pyerr)
        .map(Into::into)
}
