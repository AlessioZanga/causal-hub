use std::fmt::Display;

use approx::{AbsDiffEq, RelativeEq, relative_eq};
use itertools::Itertools;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::CPD;
use crate::{
    types::{EPSILON, FxIndexMap, FxIndexSet},
    utils::{RMI, collect_states},
};

/// A struct representing a categorical distribution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalConditionalProbabilityDistribution {
    // Labels of the conditioned variable.
    label: String,
    states: FxIndexSet<String>,
    cardinality: usize,
    // Labels of the conditioning variables.
    conditioning_labels: FxIndexSet<String>,
    conditioning_states: FxIndexMap<String, FxIndexSet<String>>,
    conditioning_cardinality: Array1<usize>,
    // Ravel multi index.
    ravel_multi_index: RMI,
    // Parameters.
    parameters: Array2<f64>,
    parameters_size: usize,
    // Fitted statistics.
    sample_size: Option<f64>,
    sample_log_likelihood: Option<f64>,
}

/// A type alias for a categorical conditional probability distribution.
pub type CatCPD = CategoricalConditionalProbabilityDistribution;

impl CatCPD {
    /// Creates a new categorical conditional probability distribution.
    ///
    /// # Arguments
    ///
    /// * `states` - The variable label and states.
    /// * `conditioning_states` - The conditioning variables labels and states.
    /// * `parameters` - The probabilities of the states.
    ///
    /// # Panics
    ///
    /// * If the variable labels are not unique.
    /// * If the variable states are not unique.
    /// * If the number of states of the first variable does not match the number of columns.
    /// * If the product of the number of states of the remaining variables does not match the number of rows.
    /// * If the probabilities do not sum to one by row, unless empty.
    ///
    /// # Returns
    ///
    /// A new `CatCPD` instance.
    ///
    pub fn new<I, J, K, L, M, N, O>(
        state: (L, I),
        conditioning_states: J,
        parameters: Array2<f64>,
    ) -> Self
    where
        I: IntoIterator<Item = M>,
        J: IntoIterator<Item = (N, K)>,
        K: IntoIterator<Item = O>,
        L: AsRef<str>,
        M: AsRef<str>,
        N: AsRef<str>,
        O: AsRef<str>,
    {
        // Unpack label and states.
        let (label, states) = state;
        // Convert variable label to a string.
        let label = label.as_ref().to_owned();

        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the variable.
        let mut states: FxIndexSet<_> = states
            .into_iter()
            .inspect(|_| n += 1)
            .map(|state| state.as_ref().to_owned())
            .collect();

        // Assert unique labels.
        assert_eq!(states.len(), n, "Variables states must be unique.");

        // Get the states cardinality.
        let cardinality = states.len();

        // Get the states of the conditioning variables.
        let mut conditioning_states = collect_states(conditioning_states);
        // Get the labels of the variables.
        let mut conditioning_labels: FxIndexSet<_> = conditioning_states.keys().cloned().collect();
        // Get the cardinality of the set of states.
        let conditioning_cardinality: Array1<_> =
            conditioning_states.values().map(|i| i.len()).collect();

        // Check that label is not in the conditioning labels.
        assert!(
            !conditioning_states.contains_key(&label),
            "Conditioned variable cannot be a conditioning variable."
        );
        // Check if the number of states of the first variable matches the number of columns.
        assert_eq!(
            parameters.ncols(),
            states.len(),
            "Number of states must match the number of columns."
        );
        // Check if the product of the number of states of the remaining variables matches the number of rows.
        assert_eq!(
            parameters.nrows(),
            conditioning_cardinality.iter().product::<usize>(),
            "Product of the number of conditioning states must match the number of rows."
        );

        // Assert the probabilities sum to one by row.
        parameters
            .sum_axis(Axis(1))
            .iter()
            .enumerate()
            .for_each(|(i, &x)| {
                if !relative_eq!(x, 1.0, epsilon = EPSILON) {
                    panic!("Failed to sum probability to one: {}.", parameters.row(i));
                }
            });

        // Compute the parameters size.
        let parameters_size = parameters.ncols().saturating_sub(1) * parameters.nrows();

        // Sort the columns.
        let mut columns_idx: Vec<_> = (0..states.len()).collect();
        columns_idx.sort_by_key(|&i| &states[i]);
        // Sort the labels.
        states.sort();

        // Sort the rows.
        let mut row_indices: Vec<_> = (0..conditioning_cardinality.product()).collect();
        // Sort the conditioning labels.
        conditioning_labels.sort();
        conditioning_states.sort_keys();
        // Compute the new multi index.
        let row_multi_index: Vec<_> = conditioning_states
            .values()
            .multi_cartesian_product()
            .collect();
        row_indices.sort_by_key(|&i| &row_multi_index[i]);
        // Sort the conditioning states.
        conditioning_states.values_mut().for_each(|x| x.sort());
        // Update conditioning cardinality.
        let conditioning_cardinality: Array1<_> =
            conditioning_states.values().map(|i| i.len()).collect();

        // Allocate new parameters.
        let mut new_parameters = parameters.clone();
        // Sort the values by the indices of the states labels.
        new_parameters
            .columns_mut()
            .into_iter()
            .enumerate()
            .for_each(|(i, mut new_parameters_col)| {
                // Assign the sorted values to the new values array.
                new_parameters_col.assign(&parameters.column(columns_idx[i]));
            });
        // Sort the values by multi indices.
        new_parameters.rows_mut().into_iter().enumerate().for_each(
            |(i, mut new_parameters_row)| {
                // Assign the sorted values to the new values array.
                new_parameters_row.assign(&parameters.row(row_indices[i]));
            },
        );
        // Update the values with the new sorted values.
        let parameters = new_parameters;

        // Construct the ravel multi index.
        let ravel_multi_index = RMI::new(conditioning_cardinality.iter().copied());

        // Debug assert to check the sorting of the labels.
        debug_assert!(
            states.iter().is_sorted(),
            "Conditioned states must be sorted."
        );
        debug_assert!(
            conditioning_labels.iter().is_sorted(),
            "Conditioning labels must be sorted."
        );
        debug_assert!(
            conditioning_states.keys().is_sorted(),
            "Conditioning labels must be sorted."
        );
        debug_assert!(
            conditioning_states.values().all(|x| x.iter().is_sorted()),
            "Conditioning states must be sorted."
        );

        Self {
            label,
            states,
            cardinality,
            conditioning_labels,
            conditioning_states,
            conditioning_cardinality,
            ravel_multi_index,
            parameters,
            parameters_size,
            sample_size: None,
            sample_log_likelihood: None,
        }
    }

    /// Returns the states of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The states of the conditioned variable.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexSet<String> {
        &self.states
    }

    /// Returns the cardinality of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioned variable.
    ///
    #[inline]
    pub const fn cardinality(&self) -> usize {
        self.cardinality
    }

    /// Returns the states of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The states of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.conditioning_states
    }

    /// Returns the cardinality of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_cardinality(&self) -> &Array1<usize> {
        &self.conditioning_cardinality
    }

    /// Returns the ravel multi index of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The ravel multi index of the conditioning variables.
    ///
    #[inline]
    pub const fn ravel_multi_index(&self) -> &RMI {
        &self.ravel_multi_index
    }

    /// Returns the sample size of the dataset used to fit the distribution, if any.
    ///
    /// # Note
    ///
    /// The sample size could be non-integer if the distribution was fitted using a weighted dataset.
    ///
    /// # Returns
    ///
    /// The sample size of the dataset used to fit the distribution.
    ///
    #[inline]
    pub const fn sample_size(&self) -> Option<f64> {
        self.sample_size
    }

    /// Returns the sample log-likelihood of the dataset given the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample log-likelihood of the dataset given the distribution.
    ///
    #[inline]
    pub const fn sample_log_likelihood(&self) -> Option<f64> {
        self.sample_log_likelihood
    }

    /// Creates a new categorical conditional probability distribution.
    ///
    /// # Arguments
    ///
    /// * `states` - The variables states.
    /// * `parameters` - The probabilities of the states.
    /// * `sample_size` - The sample size of the dataset used to fit the distribution, if any.
    /// * `sample_log_likelihood` - The sample log-likelihood of the dataset given the distribution, if any.
    ///
    /// # Panics
    ///
    /// See `new` method for panics.
    ///
    /// # Returns
    ///
    /// A new `CatCPD` instance.
    ///
    pub fn with_sample_size<I, J, K, L, M, N, O>(
        state: (L, I),
        conditioning_states: J,
        parameters: Array2<f64>,
        sample_size: Option<f64>,
        sample_log_likelihood: Option<f64>,
    ) -> Self
    where
        I: IntoIterator<Item = M>,
        J: IntoIterator<Item = (N, K)>,
        K: IntoIterator<Item = O>,
        L: AsRef<str>,
        M: AsRef<str>,
        N: AsRef<str>,
        O: AsRef<str>,
    {
        // Assert the sample size is finite and non-negative.
        sample_size.inspect(|&x| {
            assert!(
                x.is_finite() && x >= 0.,
                "Sample size must be finite and non-negative: \n\
                \t expected: sample_size >= 0, \n\
                \t found:    sample_size == {x} ."
            )
        });
        // Assert the sample log-likelihood is finite and non-positive.
        sample_log_likelihood.inspect(|&x| {
            assert!(
                x.is_finite() && x <= 0.,
                "Sample log-likelihood must be finite and non-positive: \n\
                \t expected: sample_ll <= 0 , \n\
                \t found:    sample_ll == {x} ."
            )
        });

        // Construct the categorical CPD.
        let mut cpd = Self::new(state, conditioning_states, parameters);

        // Set the sample size and log-likelihood.
        cpd.sample_size = sample_size;
        cpd.sample_log_likelihood = sample_log_likelihood;

        cpd
    }
}

impl Display for CatCPD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Determine the maximum width for formatting based on the labels and states.
        let n = std::iter::once(self.label())
            .chain(self.states())
            .chain(self.conditioning_labels())
            .chain(self.conditioning_states().values().flatten())
            .map(|x| x.len())
            .max()
            .unwrap_or(0)
            .max(8);
        // Get the number of variables to condition on.
        let z = self.conditioning_cardinality().len();
        // Get the number of states for the first variable.
        let s = self.cardinality();

        // Create a horizontal line for table formatting.
        let hline = "-".repeat((n + 3) * (z + s) + 1);
        writeln!(f, "{hline}")?;

        // Create the header row for the table.
        let header = std::iter::repeat_n("", z) // Empty columns for the conditioning variables.
            .chain([self.label().as_str()]) // Label for the first variable.
            .chain(std::iter::repeat_n("", s.saturating_sub(1))) // Empty columns for remaining states.
            .map(|x| format!("{x:width$}", width = n)) // Format each column with fixed width.
            .join(" | ");
        writeln!(f, "| {header} |")?;

        // Create a separator row for the table.
        let separator = std::iter::repeat_n("-".repeat(n), z + s).join(" | ");
        writeln!(f, "| {separator} |")?;

        // Create the second header row with labels and states.
        let header = self
            .conditioning_labels()
            .iter()
            .chain(self.states()) // Include states of the first variable.
            .map(|x| format!("{x:width$}", width = n)) // Format each column with fixed width.
            .join(" | ");
        writeln!(f, "| {header} |")?;
        writeln!(f, "| {separator} |")?;

        // Iterate over the Cartesian product of states and parameter rows.
        for (states, values) in self
            .conditioning_states()
            .values()
            .multi_cartesian_product()
            .zip(self.parameters().rows())
        {
            // Format the states for the current row.
            let states = states.iter().map(|x| format!("{x:width$}", width = n));
            // Format the parameter values for the current row.
            let values = values.iter().map(|x| format!("{:width$.6}", x, width = n));
            // Join the states and values for the current row.
            let states_values = states.chain(values).join(" | ");
            writeln!(f, "| {states_values} |")?;
        }

        // Write the closing horizontal line for the table.
        writeln!(f, "{hline}")?;

        Ok(())
    }
}

impl PartialEq for CatCPD {
    fn eq(&self, other: &Self) -> bool {
        // Check for equality, excluding the sample values.
        self.label.eq(&other.label)
            && self.states.eq(&other.states)
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
            && self.ravel_multi_index.eq(&other.ravel_multi_index)
            && self.parameters.eq(&other.parameters)
    }
}

impl AbsDiffEq for CatCPD {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        // Check for equality, excluding the sample values.
        self.label.eq(&other.label)
            && self.states.eq(&other.states)
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
            && self.ravel_multi_index.eq(&other.ravel_multi_index)
            && self.parameters.abs_diff_eq(&other.parameters, epsilon)
    }
}

impl RelativeEq for CatCPD {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        // Check for equality, excluding the sample values.
        self.label.eq(&other.label)
            && self.states.eq(&other.states)
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
            && self.ravel_multi_index.eq(&other.ravel_multi_index)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl CPD for CatCPD {
    type Label = String;
    type ConditioningLabels = FxIndexSet<String>;
    type Parameters = Array2<f64>;
    type SS = (Array2<f64>, Array1<f64>, f64);

    #[inline]
    fn label(&self) -> &Self::Label {
        &self.label
    }

    #[inline]
    fn conditioning_labels(&self) -> &Self::ConditioningLabels {
        &self.conditioning_labels
    }

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    #[inline]
    fn parameters_size(&self) -> usize {
        self.parameters_size
    }
}
