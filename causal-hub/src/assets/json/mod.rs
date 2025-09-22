use dry::macro_for;
use log::debug;
use paste::paste;

use crate::{
    io::JsonIO,
    models::{CatCTBN, GaussBN},
};

macro_for!(
    $bn in [
        arth150, ecoli70, magic_irri, magic_niab
    ] {
    paste! {
        #[doc = "Load the `" $bn:upper "` Gaussian BN from the assets."]
        pub fn [<load_ $bn>]() -> GaussBN {
            // Log the loading of the BN.
            debug!("Loading the '{}' BN from assets.", stringify!($bn));
            // Read the JSON file and return the BN.
            GaussBN::from_json(include_str!(concat!(stringify!($bn), ".json")))
        }
    }
});

macro_for!(
    $ctbn in [
        eating
    ] {
    paste! {
        #[doc = "Load the `" $ctbn:upper "` categorical CTBN from the assets."]
        pub fn [<load_ $ctbn>]() -> CatCTBN {
            // Log the loading of the CTBN.
            debug!("Loading the '{}' CTBN from assets.", stringify!($ctbn));
            // Read the JSON file and return the CTBN.
            CatCTBN::from_json(include_str!(concat!(stringify!($ctbn), ".json")))
        }
    }
});
