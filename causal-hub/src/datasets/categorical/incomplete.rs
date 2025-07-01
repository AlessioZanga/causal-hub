use ndarray::prelude::*;

use super::CatSample;
use crate::types::{Labels, States};

/// A struct representing a categorical incomplete sample.
#[derive(Clone, Debug)]
pub struct CategoricalIncompleteSample {
    sample: CatSample,
}

/// A type alias for the categorical incomplete sample.
pub type CatIncSample = CategoricalIncompleteSample;

/// A struct representing a categorical incomplete dataset.
#[derive(Clone, Debug)]
pub struct CategoricalIncompleteDataset {
    labels: Labels,
    states: States,
    cardinality: Array1<usize>,
    values: Array2<u8>,
}

/// A type alias for the categorical incomplete dataset.
pub type CatIncData = CategoricalIncompleteDataset;
