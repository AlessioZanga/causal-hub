use dry::macro_for;
use paste::paste;
use pyo3::prelude::*;

use crate::models::PyCategoricalBN;

macro_for!(
    $bn in [
        alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
        hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
        pigs, sachs, survey, water, win95pts
    ] {
    paste! {
        #[doc = "Load the `" $bn:upper "` BN from the assets."]
        #[pyfunction]
        pub fn [<load_ $bn>]() -> PyCategoricalBN {
            causal_hub::assets::[<load_ $bn>]().into()
        }
    }
});
