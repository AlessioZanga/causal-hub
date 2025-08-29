use dry::macro_for;
use log::debug;
use paste::paste;

use crate::{io::JsonIO, models::CatCTBN};

macro_for!(
    $ctbn in [
        eating
    ] {
    paste! {
        #[doc = "Load the `" $ctbn:upper "` CTBN from the assets."]
        pub fn [<load_ $ctbn>]() -> CatCTBN {
            // Log the loading of the CTBN.
            debug!("Loading the '{}' CTBN from assets.", stringify!($ctbn));
            // Read the JSON file and return the CTBN.
            CatCTBN::from_json(include_str!(concat!(stringify!($ctbn), ".json")))
        }
    }
});
