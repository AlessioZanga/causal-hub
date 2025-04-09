use dry::macro_for;
use paste::paste;

use crate::{io::BifReader, model::CategoricalBN};

macro_for!(
    $bn in [
        alarm,
        andes,
        asia,
        barley,
        cancer,
        child,
        diabetes,
        earthquake,
        hailfinder,
        hepar2,
        insurance,
        link,
        mildew,
        munin1,
        pathfinder,
        pigs,
        sachs,
        survey,
        water,
        win95pts
    ] {
    paste! {
        #[doc = "Load the `" $bn:upper "` BN from a BIF file."]
        pub fn [<load_ $bn>]() -> CategoricalBN {
            BifReader::read(include_str!(concat!(stringify!($bn), ".bif")))
        }
    }
});
