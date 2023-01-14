use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::Deref,
};

use is_sorted::IsSorted;
use itertools::Itertools;
use ndarray::prelude::*;
use polars::prelude::*;

/* Implement DiscreteDataMatrix */

/// Data matrix for discrete data.
#[derive(Clone, Debug)]
pub struct DiscreteDataMatrix {
    data: Array2<usize>,
    labels: BTreeSet<String>,
    levels: HashMap<String, Vec<String>>,
    cardinality: Array1<usize>,
}

impl Deref for DiscreteDataMatrix {
    type Target = Array2<usize>;

    fn deref(&self) -> &Self::Target {
        &self.data
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

        // Cast to categorical datatype.
        let df = df.iter().map(|s| {
            s.cast(&DataType::Utf8)
                .expect("Failed to cast to intermediate UTF-8 datatype")
                .cast(&DataType::Categorical(None))
                .expect("Failed to cast to categorical datatype")
        });

        // Sort columns by name.
        let df: DataFrame = df.sorted_by(|a, b| a.name().cmp(b.name())).collect();

        // Get underlying data matrix.
        let mut data = df
            .to_ndarray::<UInt32Type>()
            .expect("Fail to cast to ndarray matrix")
            .mapv(|x| x as usize);

        // Get variables as set of strings.
        let labels: BTreeSet<String> = df.get_column_names_owned().into_iter().collect();

        // Get variables levels.
        let levels: HashMap<String, Vec<String>> = df
            // Iterate over the columns.
            .iter()
            // Get index-to-label mapping.
            .map(|s| {
                (
                    s.name().into(),
                    s.categorical()
                        .expect("Failed to access categorical representation")
                        .get_rev_map()
                        .deref(),
                )
            })
            // Get levels.
            .map(|(label, levels)| match levels {
                RevMapping::Global(map, levels, _) => {
                    // Reorder to vector of levels.
                    let map: BTreeMap<_, _> = map
                        .into_iter()
                        .map(|(&i, &j)| (i as usize, j as usize))
                        .collect();
                    let levels: Vec<_> = map
                        .into_values()
                        .map(|i| levels.get(i).unwrap().into())
                        .collect();

                    (label, levels)
                }
                RevMapping::Local(levels) => {
                    // Cast to vector of levels.
                    let levels: Vec<_> = levels.values_iter().map(|s| s.into()).collect();

                    (label, levels)
                }
            })
            // Get series index.
            .enumerate()
            // Check that levels are sorted.
            .map(|(i, (label, mut levels))| {
                // Check if levels are ordered.
                if !levels.iter().is_sorted() {
                    // If not, build a map of the sorted indices.
                    let mut indices: Vec<_> = (0..levels.len()).collect();
                    indices.sort_by_key(|&i| &levels[i]);
                    // Sort the data.
                    data.column_mut(i).mapv_inplace(|x| indices[x]);
                    // Sort the labels.
                    levels.sort();
                }

                (label, levels)
            })
            // Collect variables levels.
            .collect();

        // Compute cardinalities from levels.
        let cardinality = labels.iter().map(|l| levels[l].len()).collect();

        Self {
            data,
            labels,
            levels,
            cardinality,
        }
    }
}

impl DiscreteDataMatrix {
    /// Gets the set of variables labels.
    pub fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }

    /// Gets the map of variables to their levels.
    pub fn levels(&self) -> &HashMap<String, Vec<String>> {
        &self.levels
    }

    /// Gets the vector of variables cardinalities.
    pub fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }
}

/* Implement ContinuousDataMatrix */

/// Data matrix for continuous data.
#[derive(Clone, Debug)]
pub struct ContinuousDataMatrix {
    data: Array2<f64>,
    labels: BTreeSet<String>,
}

impl Deref for ContinuousDataMatrix {
    type Target = Array2<f64>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
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
        let labels = df.get_column_names_owned().into_iter().collect();

        Self { data, labels }
    }
}

impl ContinuousDataMatrix {
    /// Gets the set of variables labels.
    pub fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }
}
