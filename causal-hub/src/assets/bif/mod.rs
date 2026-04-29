use dry::macro_for;
use log::debug;
use paste::paste;

use crate::{io::BifIO, models::CatBN, types::Result};

macro_for!(
    $bn in [
        alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
        hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
        pigs, sachs, survey, water, win95pts
    ] {
    paste! {
        #[doc = "Load the `" $bn:upper "` categorical BN from the assets."]
        pub fn [<load_ $bn>]() -> Result<CatBN> {
            // Log the loading of the BN.
            debug!("Loading the '{}' BN from assets.", stringify!($bn));
            // Read the BIF file and return the BN.
            CatBN::from_bif_string(include_str!(concat!(stringify!($bn), ".bif")))
        }
    }
});
