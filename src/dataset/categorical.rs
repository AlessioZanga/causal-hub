use std::fmt::Display;

use itertools::Itertools;
use ndarray::prelude::*;

use crate::utils::{FxIndexMap, FxIndexSet};

/// A struct representing a categorical dataset.
///
#[derive(Clone, Debug)]
pub struct CategoricalDataset {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    values: Array2<u8>,
}

impl CategoricalDataset {
    pub fn new(variables: &[(&str, Vec<&str>)], values: Array2<u8>) -> Self {
        // Get the states of the variables.
        let states: FxIndexMap<_, FxIndexSet<_>> = variables
            .iter()
            .map(|(i, j)| {
                (
                    // Convert the variable label to a string.
                    i.to_string(),
                    // Convert the variable states to a set of strings.
                    j.iter().map(|k| k.to_string()).collect(),
                )
            })
            .collect();
        // Get the labels of the variables.
        let labels: FxIndexSet<_> = states.keys().cloned().collect();
        // Get the cardinality of the set of states.
        let cardinality: Array1<_> = states.values().map(|i| i.len()).collect();
        // Check variables labels are unique.
        assert_eq!(
            states.len(),
            variables.len(),
            "Variable labels must be unique."
        );
        // Check variables states are unique.
        assert_eq!(
            cardinality,
            Array1::from_iter(variables.iter().map(|(_, i)| i.len())),
            "Variable states must be unique."
        );

        // Check if the maximum value of the values is less than the number of states.
        assert!(
            values.rows().into_iter().all(|row|
                    // For each row ...
                    row.iter().zip(&cardinality)
                    // ... check if the value is less than the number of states.
                    .all(|(x, y)| (*x as usize) < *y)),
            "Values must be less than the number of states."
        );

        Self {
            labels,
            states,
            cardinality,
            values,
        }
    }

    /// Returns the labels of the variables in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the vector of labels.
    ///
    pub fn labels(&self) -> &FxIndexSet<String> {
        &self.labels
    }

    /// Returns the states of the variables in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the vector of states.
    ///
    pub fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// Returns the cardinality of the set of states in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of cardinality.
    ///
    pub fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }

    /// Returns the values of the states in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of values.
    ///
    pub fn values(&self) -> &Array2<u8> {
        &self.values
    }
}

impl Display for CategoricalDataset {
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
        let hline = std::iter::repeat("-")
            .take((n + 3) * self.labels.len() + 1)
            .join("");
        writeln!(f, "{hline}")?;
        // Write the header.
        let mut header = self.labels.iter().map(|x| format!("{x:width$}", width = n));
        writeln!(f, "| {} |", header.join(" | "))?;
        // Write the separator.
        let separator = std::iter::repeat("-").take(n).join("");
        let mut separator = std::iter::repeat(separator).take(self.labels.len());
        writeln!(f, "| {} |", separator.join(" | "))?;
        // Write the values.
        for row in self.values.rows() {
            // Get the state corresponding to the value.
            let mut row = row
                .iter()
                .enumerate()
                .map(|(i, &x)| &self.states[i][x as usize])
                .map(|x| format!("{x:width$}", width = n));
            writeln!(f, "| {} |", row.join(" | "))?;
        }
        // Write the bottom line.
        writeln!(f, "{hline}")
    }
}
