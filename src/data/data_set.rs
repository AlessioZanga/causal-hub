use std::{
    collections::{btree_set, BTreeMap, BTreeSet},
    fmt::Debug,
    iter::{FusedIterator, Map},
    ops::Deref,
};

use is_sorted::IsSorted;
use itertools::Itertools;
use ndarray::prelude::*;
use polars::prelude::*;
use rand::{distributions::Uniform, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::types::{FxIndexMap, FxIndexSet};

/// Data set trait.
pub trait DataSet:
    Clone + Debug + From<DataFrame> + Into<DataFrame> + Sync + Serialize + for<'a> Deserialize<'a>
{
    /// Data set underlying data structure.
    type Data;

    /// Labels iterator type.
    type LabelsIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Get reference to underlying data.
    fn data(&self) -> &Self::Data;

    /// Get the set of variables labels.
    fn labels(&self) -> Self::LabelsIter<'_>;

    /// Get sample size.
    fn sample_size(&self) -> usize;

    /// Draw `n` samples without replacement.
    ///
    /// # Panics
    ///
    /// Panics if `n` is higher than the total number of samples in the data set.
    ///
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self;

    /// Draw `n` samples with replacement.
    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self;
}

/* Implement DiscreteDataSet */

/// Data matrix for discrete data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteDataSet {
    data: Array2<u8>,
    cardinality: Vec<u8>,
    states: FxIndexMap<String, FxIndexSet<String>>,
}

impl DiscreteDataSet {
    /// Construct a new discrete data matrix given data and states.
    pub fn new<V, I, J>(data: Array2<u8>, states: I) -> Self
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
        assert_eq!(data.ncols(), states.len());
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

    /// Gets the vector of variables cardinalities.
    #[inline]
    pub fn cardinality(&self) -> &Vec<u8> {
        &self.cardinality
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

        // Update data encoding w.r.t. new states.
        states.into_iter().for_each(|(k, v)| {
            // Get columns to be updated.
            let (i, _, s_v) = self.states.get_full(&k).unwrap();
            // Align data encodings.
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
}

impl From<DataFrame> for DiscreteDataSet {
    fn from(df: DataFrame) -> Self {
        // Check for missing data.
        assert!(
            !df.iter().any(|s| s.is_null().any()),
            concat!(
                "DataSet must contain no missing values.",
                "Refer to `DiscreteDataSetWithMissing` to handle missing values properly."
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
        let mut data = df
            .to_ndarray::<UInt32Type>()
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

impl From<DiscreteDataSet> for DataFrame {
    fn from(data_set: DiscreteDataSet) -> Self {
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

impl DataSet for DiscreteDataSet {
    type Data = Array2<u8>;

    type LabelsIter<'a> =
        Map<indexmap::map::Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    #[inline]
    fn data(&self) -> &Self::Data {
        &self.data
    }

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.states.keys().map(|x| x.as_str())
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.data.nrows()
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Check if there are enough samples.
        assert!(
            self.data.nrows() >= n,
            "Sample size is higher than the total number of samples in the data set."
        );

        // Allocate the new data set.
        let mut data = Array2::zeros((n, self.data.ncols()));
        // Define the new rows index.
        let mut idx = (0..self.data.nrows()).collect_vec();
        // Shuffle the rows index.
        idx.shuffle(rng);
        // Fill new dataset.
        idx.into_iter()
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| data.row_mut(i).assign(&self.data.row(j)));

        Self {
            states: self.states.clone(),
            cardinality: self.cardinality.clone(),
            data,
        }
    }

    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Allocate the new data set.
        let mut data = Array2::zeros((n, self.data.ncols()));
        // Define the new rows index.
        let idx = Uniform::new(0, self.data.nrows());
        // Fill new dataset.
        rng.sample_iter(idx)
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| data.row_mut(i).assign(&self.data.row(j)));

        Self {
            data,
            cardinality: self.cardinality.clone(),
            states: self.states.clone(),
        }
    }
}

/* Implement ContinuousDataSet */

/// Data matrix for continuous data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContinuousDataSet {
    data: Array2<f64>,
    labels: BTreeSet<String>,
}

impl From<DataFrame> for ContinuousDataSet {
    fn from(df: DataFrame) -> Self {
        // Check for missing data.
        assert!(
            !df.iter().any(|s| s.is_null().any()),
            concat!(
                "DataSet must contain no missing data. ",
                "Refer to `ContinuousDataSetWithMissing` to handle missing data properly."
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
        let data = df
            .to_ndarray::<Float64Type>()
            .expect("Fail to cast to ndarray matrix");

        // Get variables as set of strings.
        let labels = df.get_column_names_owned().into_iter().map_into().collect();

        Self { data, labels }
    }
}

impl From<ContinuousDataSet> for DataFrame {
    fn from(data_set: ContinuousDataSet) -> Self {
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

impl DataSet for ContinuousDataSet {
    type Data = Array2<f64>;

    type LabelsIter<'a> = Map<btree_set::Iter<'a, String>, fn(&'a String) -> &'a str>;

    #[inline]
    fn data(&self) -> &Self::Data {
        &self.data
    }

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.data.nrows()
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Check if there are enough samples.
        assert!(
            self.data.nrows() >= n,
            "Sample size is higher than the total number of samples in the data set."
        );

        // Allocate the new data set.
        let mut data = Array2::zeros((n, self.data.ncols()));
        // Define the new rows index.
        let mut idx = (0..self.data.nrows()).collect_vec();
        // Shuffle the rows index.
        idx.shuffle(rng);
        // Fill new dataset.
        idx.into_iter()
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| data.row_mut(i).assign(&self.data.row(j)));

        Self {
            data,
            labels: self.labels.clone(),
        }
    }

    fn sample_with_replacement<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self {
        // Allocate the new data set.
        let mut data = Array2::zeros((n, self.data.ncols()));
        // Define the new rows index.
        let idx = Uniform::new(0, self.data.nrows());
        // Fill new dataset.
        rng.sample_iter(idx)
            // Take only n samples.
            .take(n)
            .enumerate()
            .for_each(|(i, j)| data.row_mut(i).assign(&self.data.row(j)));

        Self {
            data,
            labels: self.labels.clone(),
        }
    }
}
