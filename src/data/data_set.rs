use std::{collections::BTreeSet, fmt::Debug, ops::Deref};

use polars::prelude::*;
use serde::{Deserialize, Serialize};

/// Data set trait.
pub trait DataSet:
    Clone
    + Debug
    + Deref<Target = Self::Data>
    + From<DataFrame>
    + Sync
    + Serialize
    + for<'a> Deserialize<'a>
{
    /// Data set underlying data structure.
    type Data;

    /// Gets the set of variables labels.
    fn labels(&self) -> &BTreeSet<String>;
}
