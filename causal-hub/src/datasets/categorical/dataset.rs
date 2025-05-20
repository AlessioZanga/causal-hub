use std::fmt::Display;

use itertools::Itertools;
use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    types::{FxIndexMap, FxIndexSet},
    utils::sort_states,
};

/// A struct representing a categorical sample.
#[derive(Clone, Debug)]
pub struct CategoricalSample {
    values: Array2<u8>,
}

/// A type alias for a categorical sample.
pub type CatSample = CategoricalSample;

// FIXME: Implement `CatSample` methods.

/// A struct representing a categorical dataset.
#[derive(Clone, Debug)]
pub struct CategoricalDataset {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    values: Array2<u8>,
}

/// A type alias for a categorical dataset.
pub type CatData = CategoricalDataset;

impl CatData {
    /// Creates a new categorical dataset.
    ///
    /// # Arguments
    ///
    /// * `states` - The variables states.
    /// * `values` - The values of the variables.
    ///
    /// # Notes
    ///
    /// * Labels and states will be sorted in alphabetical order.
    ///
    /// # Panics
    ///
    /// * If the variable labels are not unique.
    /// * If the variable states are not unique.
    /// * If the number of variable states is higher than `u8::MAX`.
    /// * If the number of variables is different from the number of values columns.
    /// * If the variables values are not smaller than the number of states.
    ///
    /// # Returns
    ///
    /// A new `CategoricalDataset` instance.
    ///
    pub fn new<I, J, K, V>(states: I, values: Array2<u8>) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the variables.
        let states: FxIndexMap<_, _> = states
            .into_iter()
            .inspect(|_| n += 1)
            .map(|(label, states)| {
                // Convert the variable label to a string.
                let label = label.as_ref().to_owned();

                // Initialize states counter.
                let mut n = 0;
                // Convert the variable states to a set of strings.
                let states: FxIndexSet<_> = states
                    .into_iter()
                    .inspect(|_| n += 1)
                    .map(|x| x.as_ref().to_owned())
                    .collect();
                // Assert unique states.
                assert_eq!(states.len(), n, "Variables states must be unique.");

                (label, states)
            })
            .collect();

        // Assert unique labels.
        assert_eq!(states.len(), n, "Variables labels must be unique.");
        // Check if the number of variables is equal to the number of columns.
        assert_eq!(
            states.len(),
            values.ncols(),
            "Number of variables must be equal to the number of columns."
        );

        // Get the indices to sort the labels and states labels.
        let (states, indices) = sort_states(states);

        // Allocate the new values array.
        let mut new_values = values.clone();
        // Sort the values by the indices of the states labels.
        new_values
            .columns_mut()
            .into_iter()
            .enumerate()
            .for_each(|(i, mut new_values_col)| {
                // Get the indices of the states labels.
                let (label_idx, states_idx) = &indices[i];
                // Get the corresponding states labels.
                let values_col = values.column(*label_idx);
                // Sort the values by the indices of the states labels. TODO: Check `u8::MAX` limit.
                let values_col = values_col.mapv(|x| states_idx[x as usize] as u8);
                // Assign the sorted values to the new values array.
                new_values_col.assign(&values_col);
            });
        // Update the values with the new sorted values.
        let values = new_values;

        // Get the labels of the variables.
        let labels: FxIndexSet<_> = states.keys().cloned().collect();
        // Get the cardinality of the set of states.
        let cardinality: Array1<_> = states.values().map(|i| i.len()).collect();

        // Check if the number of states is less than `u8::MAX`.
        assert!(
            cardinality.iter().all(|&x| x <= u8::MAX as usize),
            "Number of states must be less than {}.",
            u8::MAX
        );
        // Check if the maximum value of the values is less than the number of states.
        assert!(
            values
                .fold_axis(Axis(0), 0, |&a, &b| a.max(b))
                .into_iter()
                .zip(&cardinality)
                .all(|(x, &y)| (x as usize) < y),
            "Variables values must be smaller than the number of states."
        );

        // Debug assert to check the sorting of the labels.
        debug_assert!(labels.iter().is_sorted(), "Labels must be sorted.");
        debug_assert!(states.keys().is_sorted(), "Labels must be sorted.");
        debug_assert!(
            states.values().all(|x| x.iter().is_sorted()),
            "States must be sorted."
        );

        Self {
            labels,
            states,
            cardinality,
            values,
        }
    }

    /// Returns the states of the variables in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the vector of states.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// Returns the cardinality of the set of states in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of cardinality.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }
}

impl Display for CatData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Get the maximum length of the labels and states.
        let n = self
            .labels()
            .iter()
            .chain(self.states().values().flatten())
            .map(|x| x.len())
            .max()
            .unwrap_or(0);

        // Write the top line.
        let hline = std::iter::repeat_n("-", (n + 3) * self.labels().len() + 1).join("");
        writeln!(f, "{hline}")?;
        // Write the header.
        let header = self
            .labels()
            .iter()
            .map(|x| format!("{x:width$}", width = n))
            .join(" | ");
        writeln!(f, "| {header} |")?;
        // Write the separator.
        let separator = (0..self.labels().len()).map(|_| "-".repeat(n)).join(" | ");
        writeln!(f, "| {separator} |")?;
        // Write the values.
        for row in self.values.rows() {
            // Get the state corresponding to the value.
            let row = row
                .iter()
                .enumerate()
                .map(|(i, &x)| &self.states()[i][x as usize])
                .map(|x| format!("{x:width$}", width = n))
                .join(" | ");
            writeln!(f, "| {row} |")?;
        }
        // Write the bottom line.
        writeln!(f, "{hline}")
    }
}

impl Dataset for CatData {
    type Labels = FxIndexSet<String>;
    type Values = Array2<u8>;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        &self.labels
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.values.nrows()
    }
}
