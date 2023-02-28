use std::{collections::BTreeSet, fmt::Debug};

use polars::prelude::*;
use serde::{Deserialize, Serialize};

/// Data set trait.
pub trait DataSet:
    Clone + Debug + From<DataFrame> + Sync + Serialize + for<'a> Deserialize<'a>
{
    /// Data set underlying data structure.
    type Data;

    /// Get the set of variables labels.
    fn labels(&self) -> &BTreeSet<String>;

    /// Get reference to underlying values.
    fn values(&self) -> &Self::Data;
}
