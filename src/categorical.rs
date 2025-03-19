use approx::relative_eq;
use ndarray::{Array2, Axis};

use crate::utils::{FxIndexMap, FxIndexSet};

/// A struct representing a categorical distribution.
///
pub struct Categorical {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    probabilities: Array2<f64>,
}

impl Categorical {
    /// Creates a new (conditional) categorical distribution.
    ///
    /// # Arguments
    ///
    /// * `variables` - The variables and their states. Must be unique.
    /// * `probabilities` - The probabilities of the states.
    ///
    /// # Notes
    ///
    /// The first variable is the one conditioned on as P(X | Z).
    ///
    /// # Returns
    ///
    /// A new `Categorical` instance.
    ///
    pub fn new(variables: &[(&str, Vec<&str>)], probabilities: Array2<f64>) -> Self {
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
        // Check variables labels are unique.
        assert_eq!(
            states.len(),
            variables.len(),
            "Variable labels must be unique."
        );
        // Check variables states are unique.
        assert!(
            states
                .values()
                .map(|i| i.len())
                .eq(variables.iter().map(|(_, i)| i.len())),
            "Variable states must be unique."
        );

        // Check if the number of states of the first variable matches the number of columns.
        assert_eq!(
            probabilities.ncols(),
            states.get_index(0).map(|(_, i)| i.len()).unwrap_or(0),
            "Number of states of the first variable does not match the number of columns."
        );
        // Check if the product of the number of states of the remaining variables matches the number of rows.
        assert_eq!(
            probabilities.nrows(),
            states.iter().skip(1).map(|(_, i)| i.len()).product(),
            "Product of the number of states of the remaining variables does not match the number of rows."
        );
        // Assert the probabilities sum to one by row, unless empty.
        assert!(
            probabilities.is_empty()
                || probabilities
                    .sum_axis(Axis(1))
                    .iter()
                    .all(|&i| relative_eq!(i, 1.0)),
            "Probabilities must sum to one by row."
        );

        Self {
            labels,
            states,
            probabilities,
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

    /// Returns the probabilities of the states in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of probabilities.
    ///
    pub fn probabilities(&self) -> &Array2<f64> {
        &self.probabilities
    }
}
