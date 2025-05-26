use std::fmt::Display;

use itertools::Itertools;
use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    types::{FxIndexMap, FxIndexSet},
    utils::{collect_states, sort_states},
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
    pub fn new<I, J, K, V>(states: I, mut values: Array2<u8>) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        // Collect the states into a map.
        let states = collect_states(states);
        // Get the indices to sort the labels and states labels.
        let (states, sorted_idx) = sort_states(states);
        // Get the labels of the variables.
        let labels: FxIndexSet<_> = states.keys().cloned().collect();
        // Get the cardinality of the states.
        let cardinality = Array::from_iter(states.values().map(|x| x.len()));

        // Check if the number of states is less than `u8::MAX`.
        states.iter().for_each(|(label, state)| {
            assert!(
                state.len() < u8::MAX as usize,
                "Variable '{label}' should have less than 256 states.",
            );
        });
        // Check if the number of variables is equal to the number of columns.
        assert_eq!(
            states.len(),
            values.ncols(),
            "Number of variables must be equal to the number of columns."
        );
        // Check if the maximum value of the values is less than the number of states.
        values
            .fold_axis(Axis(0), 0, |&a, &b| if a > b { a } else { b })
            .into_iter()
            .enumerate()
            .for_each(|(i, x)| {
                assert!(
                    x < states[i].len() as u8,
                    "Values of variable '{}' must be less than the number of states.",
                    labels[i]
                );
            });

        // Check if the values are already sorted.
        if !sorted_idx.iter().map(|(x, _)| x).is_sorted()
            || !sorted_idx.iter().all(|(_, y)| y.iter().is_sorted())
        {
            // Allocate the new values array.
            let mut new_values = values.clone();
            // Sort the values by the indices of the states labels.
            new_values
                .columns_mut()
                .into_iter()
                .enumerate()
                .for_each(|(i, mut new_values_col)| {
                    // Get the indices of the states labels.
                    let (label_idx, states_idx) = &sorted_idx[i];
                    // Get the corresponding states labels.
                    let values_col = values.column(*label_idx);
                    // Sort the values by the indices of the states labels.
                    let values_col = values_col.mapv(|x| states_idx[x as usize] as u8);
                    // Assign the sorted values to the new values array.
                    new_values_col.assign(&values_col);
                });
            // Update the values with the new sorted values.
            values = new_values;
        }

        // Debug assert labels are unique.
        debug_assert_eq!(
            labels.iter().unique().count(),
            labels.len(),
            "Labels must be unique."
        );
        // Debug assert labels are sorted.
        debug_assert!(labels.iter().is_sorted(), "Labels must be sorted.");
        // Debug assert states keys are unique.
        debug_assert_eq!(
            states.keys().unique().count(),
            states.len(),
            "States keys must be unique."
        );
        // Debug assert states keys are sorted.
        debug_assert!(states.keys().is_sorted(), "States keys must be sorted.");
        // Debug assert states values are unique.
        debug_assert_eq!(
            states
                .values()
                .map(|x| x.iter().unique().count())
                .sum::<usize>(),
            states.values().map(|x| x.len()).sum::<usize>(),
            "States values must be unique."
        );
        // Debug assert states values are sorted.
        debug_assert!(
            states.values().all(|x| x.iter().is_sorted()),
            "States values must be sorted."
        );
        // Debug assert labels and states keys are the same.
        debug_assert!(
            labels.iter().eq(states.keys()),
            "Labels and states keys must be the same."
        );
        // Debug assert cardinality must match the number of states.
        debug_assert!(
            cardinality
                .iter()
                .zip(states.values())
                .all(|(&a, b)| a == b.len()),
            "Cardinality must match the number of states values."
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
