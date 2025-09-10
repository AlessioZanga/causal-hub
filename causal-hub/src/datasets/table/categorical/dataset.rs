use std::fmt::Display;

use csv::ReaderBuilder;
use itertools::Itertools;
use log::debug;
use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    io::CsvIO,
    types::{Labels, States},
    utils::{collect_states, sort_states},
};

/// A type alias for a categorical sample.
pub type CatSample = Array1<u8>;

/// A struct representing a categorical dataset.
#[derive(Clone, Debug)]
pub struct CatTable {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    values: Array2<u8>,
}

impl CatTable {
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
    /// A new categorical dataset instance.
    ///
    pub fn new<I, J, K, V>(states: I, mut values: Array2<u8>) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        // Collect the states into a map.
        let mut states = collect_states(states);
        // Get the labels of the variables.
        let mut labels: Labels = states.keys().cloned().collect();
        // Get the shape of the states.
        let mut shape = Array::from_iter(states.values().map(|x| x.len()));

        // Log the creation of the categorical dataset.
        debug!(
            "Creating a new categorical dataset with {} variables and {} samples.",
            states.len(),
            values.nrows()
        );

        // Check if the number of states is less than `u8::MAX`.
        states.iter().for_each(|(label, state)| {
            assert!(
                state.len() < u8::MAX as usize,
                "Variable '{label}' should have less than 256 states: \n\
                \t expected:    |states| <  256 , \n\
                \t found:       |states| == {} .",
                state.len()
            );
        });
        // Check if the number of variables is equal to the number of columns.
        assert_eq!(
            states.len(),
            values.ncols(),
            "Number of variables must be equal to the number of columns: \n\
            \t expected:    |states| == |values.columns()| , \n\
            \t found:       |states| == {} and |values.columns()| == {} .",
            states.len(),
            values.ncols()
        );
        // Check if the maximum value of the values is less than the number of states.
        values
            .fold_axis(Axis(0), 0, |&a, &b| if a > b { a } else { b })
            .into_iter()
            .enumerate()
            .for_each(|(i, x)| {
                assert!(
                    x < states[i].len() as u8,
                    "Values of variable '{label}' must be less than the number of states: \n\
                    \t expected: values[.., '{label}'] < |states['{label}']| , \n\
                    \t found:    values[.., '{label}'] == {x} and |states['{label}']| == {} .",
                    states[i].len(),
                    label = labels[i],
                );
            });

        // Check if the values are already sorted.
        if !states.keys().is_sorted() || !states.values().all(|x| x.iter().is_sorted()) {
            // Sort the states and labels.
            let (new_states, sorted_idx) = sort_states(states);
            // Update the states with the sorted states.
            states = new_states;
            labels = states.keys().cloned().collect();
            shape = Array::from_iter(states.values().map(|x| x.len()));
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
        // Debug assert shape must match the number of states.
        debug_assert!(
            shape
                .iter()
                .zip(states.values())
                .all(|(&a, b)| a == b.len()),
            "Shape must match the number of states values."
        );

        Self {
            labels,
            states,
            shape,
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
    pub const fn states(&self) -> &States {
        &self.states
    }

    /// Returns the shape of the set of states in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of shape.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        &self.shape
    }
}

impl Display for CatTable {
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
        let header = self.labels().iter().map(|x| format!("{x:n$}")).join(" | ");
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
                .map(|x| format!("{x:n$}"))
                .join(" | ");
            writeln!(f, "| {row} |")?;
        }
        // Write the bottom line.
        writeln!(f, "{hline}")
    }
}

impl Dataset for CatTable {
    type Values = Array2<u8>;

    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.values.nrows() as f64
    }
}

impl CsvIO for CatTable {
    fn from_csv(csv: &str) -> Self {
        // Create a CSV reader from the string.
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(csv.as_bytes());

        // Assert that the reader has headers.
        assert!(reader.has_headers(), "Reader must have headers.");

        // Initialize the counter.
        let mut n = 0;
        // Read the headers.
        let labels: Labels = reader
            .headers()
            .expect("Failed to read the headers.")
            .into_iter()
            .inspect(|_| n += 1)
            .map(|x| x.to_owned())
            .collect();
        // Assert unique labels.
        assert_eq!(labels.len(), n, "Header labels must be unique.");

        // Get the states of the variables.
        let mut states: States = labels
            .into_iter()
            .map(|x| (x, Default::default()))
            .collect();

        // Read the records.
        let values: Array1<_> = reader
            .into_records()
            .enumerate()
            .flat_map(|(i, row)| {
                // Get the record row.
                let row = row.unwrap_or_else(|_| panic!("Malformed record on line {}.", i + 1));
                // Get the record values and convert to indices.
                let row: Vec<_> = row
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| {
                        // Assert no missing values.
                        assert!(!x.is_empty(), "Missing value on line {}.", i + 1);
                        // Insert the value into the states, if not present.
                        let (x, _) = states[i].insert_full(x.to_owned());
                        // Cast the value.
                        x as u8
                    })
                    .collect();
                // Collect the values.
                row
            })
            .collect();

        // Get the number of rows and columns.
        let ncols = states.len();
        let nrows = values.len() / ncols;
        // Reshape the values to the correct shape.
        let values = values
            .into_shape_with_order((nrows, ncols))
            .expect("Failed to rearrange values to the correct shape.");

        // Construct the categorical dataset.
        Self::new(states, values)
    }

    fn to_csv(&self) -> String {
        todo!() // FIXME:
    }

    fn read_csv(path: &str) -> Self {
        // TODO: Reading the entire file to a string is not efficient.
        Self::from_csv(&std::fs::read_to_string(path).expect("Failed to read CSV file."))
    }

    fn write_csv(&self, path: &str) {
        std::fs::write(path, self.to_csv()).expect("Failed to write CSV file.");
    }
}
