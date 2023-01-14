use std::{collections::BTreeSet, fmt::Debug, ops::Deref};

use polars::prelude::*;

/// Data set trait.
pub trait DataSet: Clone + Debug + Deref<Target = Self::Data> + From<DataFrame> {
    /// Data set underlying data structure.
    type Data;

    /// Gets the set of variables labels.
    fn labels(&self) -> &BTreeSet<String>;
}
