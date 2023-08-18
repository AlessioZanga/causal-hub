use std::{
    collections::{btree_set, BTreeMap, BTreeSet},
    iter::Map,
    ops::Deref,
};

use is_sorted::IsSorted;
use itertools::Itertools;
use ndarray::prelude::*;
use polars::prelude::*;
use rand::{distributions::Uniform, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use super::DataSet;
use crate::types::{FxIndexMap, FxIndexSet};

/* Implement DiscreteDataMatrix */

/// Data matrix for discrete data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteDataMatrix {
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Vec<u8>,
    values: Array2<u8>,
    labels: BTreeSet<String>,
}

impl DiscreteDataMatrix {
    /// Construct a new discrete data matrix given data and states.
    pub fn new<V, I, J>(states: I, values: Array2<u8>) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = (V, J)>,
        J: IntoIterator<Item = V>,
    {
        // Construct the states map.
        let states: FxIndexMap<String, FxIndexSet<String>> = states
            .into_iter()
            .map(|(x, ys)| (x.into(), ys.into_iter().map_into().collect()))
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            .collect();
        // Check labels consistency.
        assert_eq!(values.ncols(), states.len());
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
            states,
            cardinality,
            values,
            labels: Default::default(),
        }
    }

    /// Gets the map of variables to their states.
    #[inline]
    pub fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// Set states of the discrete data matrix.
    ///
    /// # Panics
    ///
    /// Panics if provided states are not a superset of the existing ones.
    ///
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
            self.values.column_mut(i).map_inplace(|x| {
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

    /// Gets the vector of variables cardinalities.
    #[inline]
    pub fn cardinality(&self) -> &Vec<u8> {
        &self.cardinality
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
            .to_ndarray::<UInt32Type>(IndexOrder::C)
            .expect("Fail to cast to ndarray matrix")
            .mapv(|x| x as u8);

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
                    let states: FxIndexSet<String> = map
                        .into_values()
                        .map(|i| states.get(i).unwrap().into())
                        .collect();

                    (label, states)
                }
                RevMapping::Local(states) => {
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
                    values
                        .column_mut(i)
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

        // Get variables as set of strings.
        let labels = df.get_column_names_owned().into_iter().map_into().collect();

        Self {
            states,
            cardinality,
            values,
            labels,
        }
    }
}

impl From<DiscreteDataMatrix> for DataFrame {
    fn from(data: DiscreteDataMatrix) -> Self {
        // Map columns to series.
        let series = data
            .states
            .into_iter()
            .zip(data.values.columns())
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

impl DataSet for DiscreteDataMatrix {
    type Data = Array2<u8>;

    type LabelsIter<'a> = Map<btree_set::Iter<'a, String>, fn(&'a String) -> &'a str>;

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }

    #[inline]
    fn values(&self) -> &Self::Data {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.values.nrows()
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Check if there are enough samples.
        assert!(
            self.values.nrows() >= n,
            "Sample size is higher than the total number of samples in the data set."
        );

        // Allocate the new data set.
        let mut values = Array2::zeros((n, self.values.ncols()));
        // Define the new rows index.
        let mut idx = (0..self.values.nrows()).collect_vec();
        // Shuffle the rows index.
        idx.shuffle(rng);
        // Fill new dataset.
        idx.into_iter()
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| values.row_mut(i).assign(&self.values.row(j)));

        Self {
            states: self.states.clone(),
            cardinality: self.cardinality.clone(),
            values,
            labels: self.labels.clone(),
        }
    }

    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Allocate the new data set.
        let mut values = Array2::zeros((n, self.values.ncols()));
        // Define the new rows index.
        let idx = Uniform::new(0, self.values.nrows());
        // Fill new dataset.
        rng.sample_iter(idx)
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| values.row_mut(i).assign(&self.values.row(j)));

        Self {
            states: self.states.clone(),
            cardinality: self.cardinality.clone(),
            values,
            labels: self.labels.clone(),
        }
    }
}

/* Implement ContinuousDataMatrix */

/// Data matrix for continuous data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContinuousDataMatrix {
    labels: BTreeSet<String>,
    values: Array2<f64>,
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
            .to_ndarray::<Float64Type>(IndexOrder::C)
            .expect("Fail to cast to ndarray matrix");

        // Get variables as set of strings.
        let labels = df.get_column_names_owned().into_iter().map_into().collect();

        Self { labels, values }
    }
}

impl From<ContinuousDataMatrix> for DataFrame {
    fn from(data: ContinuousDataMatrix) -> Self {
        // Map columns to series.
        let series = data
            .labels
            .into_iter()
            .zip(data.values.columns())
            .map(|(name, column)| Series::new(&name, column.to_vec()))
            .collect_vec();

        DataFrame::new(series).unwrap()
    }
}

impl DataSet for ContinuousDataMatrix {
    type Data = Array2<f64>;

    type LabelsIter<'a> = Map<btree_set::Iter<'a, String>, fn(&'a String) -> &'a str>;

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }

    #[inline]
    fn values(&self) -> &Self::Data {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.values.nrows()
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Check if there are enough samples.
        assert!(
            self.values.nrows() >= n,
            "Sample size is higher than the total number of samples in the data set."
        );

        // Allocate the new data set.
        let mut values = Array2::zeros((n, self.values.ncols()));
        // Define the new rows index.
        let mut idx = (0..self.values.nrows()).collect_vec();
        // Shuffle the rows index.
        idx.shuffle(rng);
        // Fill new dataset.
        idx.into_iter()
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| values.row_mut(i).assign(&self.values.row(j)));

        Self {
            labels: self.labels.clone(),
            values,
        }
    }

    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Allocate the new data set.
        let mut values = Array2::zeros((n, self.values.ncols()));
        // Define the new rows index.
        let idx = Uniform::new(0, self.values.nrows());
        // Fill new dataset.
        rng.sample_iter(idx)
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| values.row_mut(i).assign(&self.values.row(j)));

        Self {
            labels: self.labels.clone(),
            values,
        }
    }
}
