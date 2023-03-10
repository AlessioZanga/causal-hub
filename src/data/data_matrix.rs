use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};

use is_sorted::IsSorted;
use itertools::Itertools;
use ndarray::prelude::*;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::FxIndexMap;

use super::DataSet;

/* Implement DiscreteDataMatrix */

/// Data matrix for discrete data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteDataMatrix {
    labels: BTreeSet<String>,
    states: FxIndexMap<String, Vec<String>>,
    cardinality: Vec<usize>,
    values: Array2<usize>,
}

impl DiscreteDataMatrix {
    /// Construct a new discrete data matrix given data encoding, labels and states.
    pub fn new<V, I, J, K>(labels: I, states: J, values: Array2<usize>) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, K)>,
        K: IntoIterator<Item = V>,
    {
        // Construct the labels set.
        let labels: BTreeSet<String> = labels.into_iter().map(|x| x.into()).collect();
        // Check labels consistency.
        assert_eq!(values.ncols(), labels.len());
        // Construct the states map.
        let states: FxIndexMap<String, Vec<String>> = states
            .into_iter()
            .map(|(x, ys)| (x.into(), ys.into_iter().map(|y| y.into()).collect()))
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            .collect();
        // Check states consistency.
        assert!(labels.iter().eq(states.keys()));
        // Compute cardinalities from states.
        let cardinality = labels.iter().map(|l| states[l].len()).collect();
        // Check cardinalities.
        assert_eq!(
            values
                .fold_axis(Axis(1), 0, |&acc, &x| usize::max(acc, x))
                .into_iter()
                .collect_vec(),
            cardinality
        );

        Self {
            labels,
            states,
            cardinality,
            values,
        }
    }
}

impl From<DataFrame> for DiscreteDataMatrix {
    fn from(df: DataFrame) -> Self {
        // Check for missing values.
        assert!(
            !df.iter().any(|s| s.is_null().any()),
            concat!(
                "DataSet must contain no missing values.",
                "Refer to `DiscreteDataMatrixWithMissing` to handle missing values properly."
            )
        );

        // Check for wrong data type.
        assert!(
            df.iter().all(|s| !s.dtype().is_float()),
            "DataSet must contain only discrete types"
        );

        // Cast to discrete datatype.
        let df = df.iter().map(|s| {
            s.cast(&DataType::Utf8)
                .expect("Failed to cast to intermediate UTF-8 datatype")
                .cast(&DataType::Categorical(None))
                .expect("Failed to cast to discrete datatype")
        });

        // Sort columns by name.
        let df: DataFrame = df.sorted_by(|a, b| a.name().cmp(b.name())).collect();

        // Get underlying data matrix.
        let mut values = df
            .to_ndarray::<UInt32Type>()
            .expect("Fail to cast to ndarray matrix")
            .mapv(|x| x as usize);

        // Get variables as set of strings.
        let labels: BTreeSet<_> = df.get_column_names_owned().into_iter().collect();

        // Get variables states.
        let states: FxIndexMap<_, _> = df
            // Iterate over the columns.
            .iter()
            // Get index-to-label mapping.
            .map(|s| {
                (
                    s.name().to_owned(),
                    s.categorical()
                        .expect("Failed to access discrete representation")
                        .get_rev_map()
                        .deref(),
                )
            })
            // Get states.
            .map(|(label, states)| match states {
                RevMapping::Global(map, states, _) => {
                    // Reorder to vector of states.
                    let map: BTreeMap<_, _> = map
                        .into_iter()
                        .map(|(&i, &j)| (i as usize, j as usize))
                        .collect();
                    let states = map
                        .into_values()
                        .map(|i| states.get(i).unwrap().into())
                        .collect_vec();

                    (label, states)
                }
                RevMapping::Local(states) => {
                    // Cast to vector of states.
                    let states = states.values_iter().map(|s| s.into()).collect_vec();

                    (label, states)
                }
            })
            // Get series index.
            .enumerate()
            // Check that states are sorted.
            .map(|(i, (label, mut states))| {
                // Check if states are ordered.
                if !states.iter().is_sorted() {
                    // If not, build a map of the sorted indices.
                    let mut indices = (0..states.len()).collect_vec();
                    indices.sort_by_key(|&i| &states[i]);
                    // Sort the data.
                    values.column_mut(i).mapv_inplace(|x| indices[x]);
                    // Sort the labels.
                    states.sort();
                }

                (label, states)
            })
            // Sort by variables labels.
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            // Collect variables states.
            .collect();

        // Compute cardinalities from states.
        let cardinality = labels.iter().map(|l| states[l].len()).collect();

        Self {
            labels,
            states,
            cardinality,
            values,
        }
    }
}

impl DataSet for DiscreteDataMatrix {
    type Data = Array2<usize>;

    #[inline]
    fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }

    /// Get reference to underlying values.
    #[inline]
    fn values(&self) -> &Self::Data {
        &self.values
    }
}

impl DiscreteDataMatrix {
    /// Gets the map of variables to their states.
    #[inline]
    pub fn states(&self) -> &FxIndexMap<String, Vec<String>> {
        &self.states
    }

    /// Gets the vector of variables cardinalities.
    #[inline]
    pub fn cardinality(&self) -> &Vec<usize> {
        &self.cardinality
    }
}

/* Implement ContinuousDataMatrix */

/// Data matrix for continuous data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContinuousDataMatrix {
    values: Array2<f64>,
    labels: BTreeSet<String>,
}

impl From<DataFrame> for ContinuousDataMatrix {
    fn from(df: DataFrame) -> Self {
        // Check for missing values.
        assert!(
            !df.iter().any(|s| s.is_null().any()),
            concat!(
                "DataSet must contain no missing values. ",
                "Refer to `ContinuousDataMatrixWithMissing` to handle missing values properly."
            )
        );

        // Check for wrong data type.
        assert!(
            df.iter().all(|s| s.dtype().is_float()),
            "DataSet must contain only float types"
        );

        // Sort columns by name.
        let df: DataFrame = df
            .iter()
            .sorted_by(|a, b| a.name().cmp(b.name()))
            .cloned()
            .collect();

        // Get underlying data matrix.
        let values = df
            .to_ndarray::<Float64Type>()
            .expect("Fail to cast to ndarray matrix");

        // Get variables as set of strings.
        let labels = df.get_column_names_owned().into_iter().collect();

        Self { labels, values }
    }
}

impl DataSet for ContinuousDataMatrix {
    type Data = Array2<f64>;

    #[inline]
    fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }

    #[inline]
    fn values(&self) -> &Self::Data {
        &self.values
    }
}
