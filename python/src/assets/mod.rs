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
        pub fn [<load_ $bn>]() -> PyCatBN {
            backend::assets::[<load_ $bn>]().into()
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
        pub fn [<load_ $bn>]() -> PyGaussBN {
            backend::assets::[<load_ $bn>]().into()
        }
    }
});

/// Load the `EATING` categorical CTBN from the assets.
#[gen_stub_pyfunction(module = "causal_hub.assets")]
#[pyfunction]
pub fn load_eating() -> PyCatCTBN {
    backend::assets::load_eating().into()
}
