use std::{
    collections::{btree_set, BTreeMap, BTreeSet},
    iter::Map,
    ops::Deref,
};

use is_sorted::IsSorted;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

use super::DataSet;
use crate::types::{FxIndexMap, FxIndexSet};

/* Implement CategoricalDataMatrix */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalDataMatrix {
    data: Array2<u8>,
    cardinality: Vec<u8>,
    states: FxIndexMap<String, FxIndexSet<String>>,
}

impl CategoricalDataMatrix {
    #[inline]
    pub fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    pub fn with_states<I, J, K, V>(mut self, states: I) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // Accumulate states.
        let states: FxIndexMap<String, FxIndexSet<String>> = states
            .into_iter()
            .map(|(k, v)| (k.into(), v.into_iter().map_into().sorted().collect()))
            .sorted_by(|(a, _), (b, _)| a.cmp(b))
            .collect();

        // Assert new states are superset of the existing ones.
        assert!(states.iter().all(|(k, v)| self.states[k].is_subset(v)));

        // Update values encoding w.r.t. new states.
        states.into_iter().for_each(|(k, v)| {
            // Get columns to be updated.
            let (i, _, s_v) = self.states.get_full(&k).unwrap();
            // Align values encodings.
            self.data.column_mut(i).map_inplace(|x| {
                // Set new location w.r.t. previous state.
                *x = v.get_index_of(&s_v[*x as usize]).unwrap() as u8;
            });
            // Set new states.
            self.states[&k] = v;
        });

        // Compute cardinalities.
        self.cardinality = self
            .states
            .values()
            .map(|x| {
                x.len()
                    .try_into()
                    .expect("Max number of allowed states for each variable is u8::MAX")
            })
            .collect_vec();

        self
    }

    #[inline]
    pub fn cardinality(&self) -> &Vec<u8> {
        &self.cardinality
    }
}

impl From<DataFrame> for CategoricalDataMatrix {
    fn from(data_frame: DataFrame) -> Self {
        // Check for missing values.
        assert!(
            !data_frame.iter().any(|s| s.is_null().any()),
            concat!(
                "DataSet must contain no missing values.",
                "Refer to `CategoricalDataMatrixWithMissing` to handle missing values properly."
            )
        );

        // Check for wrong data type.
        assert!(
            data_frame.iter().all(|s| !s.dtype().is_float()),
            "DataSet must contain only categorical types"
        );

        // Cast to categorical datatype.
        let data_frame = data_frame.iter().map(|s| {
            s.cast(&DataType::Utf8)
                .expect("Failed to cast to intermediate UTF-8 datatype")
                .cast(&DataType::Categorical(None))
                .expect("Failed to cast to categorical datatype")
        });

        // Sort columns by name.
        let data_frame: DataFrame = data_frame
            .sorted_by(|a, b| a.name().cmp(b.name()))
            .collect();

        // Get underlying data matrix.
        let mut data = data_frame
            .to_ndarray::<UInt32Type>(IndexOrder::C)
            .expect("Fail to cast to ndarray matrix")
            .mapv(|x| x as u8);

        // Get variables states.
        let states: FxIndexMap<_, _> = data_frame
            // Iterate over the columns.
            .iter()
            // Get index-to-label mapping.
            .map(|s| {
                (
                    s.name().to_owned(),
                    s.categorical()
                        .expect("Failed to access categorical representation")
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
                    let states: FxIndexSet<String> = map
                        .into_values()
                        .map(|i| states.get(i).unwrap().into())
                        .collect();

                    (label, states)
                }
                RevMapping::Local(states, _) => {
                    // Cast to vector of states.
                    let states = states.values_iter().map_into().collect();

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
                    data.column_mut(i)
                        .mapv_inplace(|x| indices[x as usize] as u8);
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
        let cardinality = states
            .values()
            .map(|s| {
                s.len()
                    .try_into()
                    .expect("Max number of allowed states for each variable is u8::MAX")
            })
            .collect_vec();

        Self {
            data,
            cardinality,
            states,
        }
    }
}

impl From<CategoricalDataMatrix> for DataFrame {
    fn from(data_set: CategoricalDataMatrix) -> Self {
        // Map columns to series.
        let series = data_set
            .states
            .into_iter()
            .zip(data_set.data.columns())
            .map(|((name, states), column)| {
                Series::new(
                    &name,
                    column
                        .into_iter()
                        .map(|&x| states[x as usize].to_string())
                        .collect_vec(),
                )
            })
            .collect_vec();

        DataFrame::new(series).unwrap()
    }
}

impl DataSet for CategoricalDataMatrix {
    type Data = Array2<u8>;

    type Labels = FxIndexMap<String, FxIndexSet<String>>;

    type LabelsIter<'a> =
        Map<indexmap::map::Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    #[inline]
    fn data(&self) -> &Self::Data {
        &self.data
    }

    #[inline]
    fn labels(&self) -> &Self::Labels {
        &self.states
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.states.keys().map(|x| x.as_str())
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.data.nrows()
    }

    fn with_data_labels(data: Self::Data, states: Self::Labels) -> Self {
        // Check labels are sorted.
        assert!(states.keys().is_sorted(), "Labels must be sorted");
        // Check states are sorted.
        assert!(
            states.values().all(|s| s.iter().is_sorted()),
            "States must be sorted"
        );
        // Check labels consistency.
        assert_eq!(data.ncols(), states.len(), "Inconsistent number of labels");
        // Check data max values match states cardinality.
        assert!(
            data.columns()
                .into_iter()
                .zip(states.values())
                .all(|(column, states)| (*column.max().unwrap() as usize) < states.len()),
            "Inconsistent data and states"
        );

        // Compute cardinalities from states.
        let cardinality = states
            .values()
            .map(|s| {
                s.len()
                    .try_into()
                    .expect("Max number of allowed states for each variable is u8::MAX")
            })
            .collect_vec();

        Self {
            data,
            cardinality,
            states,
        }
    }
}

/* Implement GaussianDataMatrix */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GaussianDataMatrix {
    data: Array2<f64>,
    labels: BTreeSet<String>,
}

impl From<DataFrame> for GaussianDataMatrix {
    fn from(data_frame: DataFrame) -> Self {
        // Check for missing values.
        assert!(
            !data_frame.iter().any(|s| s.is_null().any()),
            concat!(
                "DataSet must contain no missing values. ",
                "Refer to `GaussianDataMatrixWithMissing` to handle missing values properly."
            )
        );

        // Check for wrong data type.
        assert!(
            data_frame.iter().all(|s| s.dtype().is_float()),
            "DataSet must contain only float types"
        );

        // Sort columns by name.
        let data_frame: DataFrame = data_frame
            .iter()
            .sorted_by(|a, b| a.name().cmp(b.name()))
            .cloned()
            .collect();

        // Get underlying data matrix.
        let data = data_frame
            .to_ndarray::<Float64Type>(IndexOrder::C)
            .expect("Fail to cast to ndarray matrix");

        // Get variables as set of strings.
        let labels = data_frame
            .get_column_names_owned()
            .into_iter()
            .map_into()
            .collect();

        Self { data, labels }
    }
}

impl From<GaussianDataMatrix> for DataFrame {
    fn from(data_set: GaussianDataMatrix) -> Self {
        // Map columns to series.
        let series = data_set
            .labels
            .into_iter()
            .zip(data_set.data.columns())
            .map(|(name, column)| Series::new(&name, column.to_vec()))
            .collect_vec();

        DataFrame::new(series).unwrap()
    }
}

impl DataSet for GaussianDataMatrix {
    type Data = Array2<f64>;

    type Labels = BTreeSet<String>;

    type LabelsIter<'a> = Map<btree_set::Iter<'a, String>, fn(&'a String) -> &'a str>;

    #[inline]
    fn data(&self) -> &Self::Data {
        &self.data
    }

    #[inline]
    fn labels(&self) -> &Self::Labels {
        &self.labels
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.data.nrows()
    }

    fn with_data_labels(data: Self::Data, labels: Self::Labels) -> Self {
        // Check labels are sorted.
        assert!(labels.iter().is_sorted(), "Labels must be sorted");

        Self { data, labels }
    }
}

/* Implement ZeroInflatedNegativeBinomialDataMatrix */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZeroInflatedNegativeBinomialDataMatrix {
    data: Array2<f64>,
    labels: BTreeSet<String>,
}

pub type ZINBDataMatrix = ZeroInflatedNegativeBinomialDataMatrix;

impl From<DataFrame> for ZINBDataMatrix {
    fn from(data_frame: DataFrame) -> Self {
        // Check for missing values.
        assert!(
            !data_frame.iter().any(|s| s.is_null().any()),
            concat!(
                "DataSet must contain no missing values. ",
                "Refer to `ZeroInflatedNegativeBinomialDataMatrixWithMissing` to handle missing values properly."
            )
        );

        // Check for wrong data type.
        assert!(
            data_frame.iter().all(|s| s.dtype().is_float()),
            "DataSet must contain only float types"
        );

        // Sort columns by name.
        let data_frame: DataFrame = data_frame
            .iter()
            .sorted_by(|a, b| a.name().cmp(b.name()))
            .cloned()
            .collect();

        // Get underlying data matrix.
        let data = data_frame
            .to_ndarray::<Float64Type>(IndexOrder::C)
            .expect("Fail to cast to ndarray matrix");

        // Get variables as set of strings.
        let labels = data_frame
            .get_column_names_owned()
            .into_iter()
            .map_into()
            .collect();

        Self { data, labels }
    }
}

impl From<ZINBDataMatrix> for DataFrame {
    fn from(data_set: ZINBDataMatrix) -> Self {
        // Map columns to series.
        let series = data_set
            .labels
            .into_iter()
            .zip(data_set.data.columns())
            .map(|(name, column)| Series::new(&name, column.to_vec()))
            .collect_vec();

        DataFrame::new(series).unwrap()
    }
}

impl DataSet for ZINBDataMatrix {
    type Data = Array2<f64>;

    type Labels = BTreeSet<String>;

    type LabelsIter<'a> = Map<btree_set::Iter<'a, String>, fn(&'a String) -> &'a str>;

    #[inline]
    fn data(&self) -> &Self::Data {
        &self.data
    }

    #[inline]
    fn labels(&self) -> &Self::Labels {
        &self.labels
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.data.nrows()
    }

    fn with_data_labels(data: Self::Data, labels: Self::Labels) -> Self {
        // Check labels are sorted.
        assert!(labels.iter().is_sorted(), "Labels must be sorted");

        Self { data, labels }
    }
}
