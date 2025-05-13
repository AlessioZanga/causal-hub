use ndarray::prelude::*;

use super::CatSample;
use crate::types::{FxIndexMap, FxIndexSet};

/// A struct representing a categorical weighted sample.
#[derive(Clone, Debug)]
pub struct CategoricalWeightedSample {
    sample: CatSample,
    weight: f64,
}

/// A type alias for the categorical weighted sample.
pub type CatWtdSample = CategoricalWeightedSample;

// TODO: Implement `CatWtdSample` methods.

/// A struct representing a dataset of categorical weighted samples.
#[derive(Clone, Debug)]
pub struct CategoricalWeightedDataset {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    values: Vec<CatWtdSample>,
}

/// A type alias for the categorical weighted dataset.
pub type CatWtdData = CategoricalWeightedDataset;

// TODO: Implement `CatWtdData` methods.
