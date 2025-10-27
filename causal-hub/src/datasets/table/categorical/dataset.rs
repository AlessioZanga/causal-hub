use std::{
    fmt::Display,
    io::{Read, Write},
};

use csv::{ReaderBuilder, WriterBuilder};
use itertools::Itertools;
use log::debug;
use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    io::CsvIO,
    models::Labelled,
    types::{Labels, Set, States},
};

/// A type alias for a categorical variable.
pub type CatType = u8;
/// A type alias for a categorical sample.
pub type CatSample = Array1<CatType>;

/// A struct representing a categorical dataset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatTable {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    values: Array2<CatType>,
}

impl Labelled for CatTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
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
    /// * If the number of variable states is higher than `CatType::MAX`.
    /// * If the number of variables is different from the number of values columns.
    /// * If the variables values are not smaller than the number of states.
    ///
    /// # Returns
    ///
    /// A new categorical dataset instance.
    ///
    pub fn new(mut states: States, mut values: Array2<CatType>) -> Self {
        // Log the creation of the categorical dataset.
        debug!(
            "Creating a new categorical dataset with {} variables and {} samples.",
            states.len(),
            values.nrows()
        );

        // Check if the number of states is less than `CatType::MAX`.
        states.iter().for_each(|(label, state)| {
            assert!(
                state.len() <= CatType::MAX as usize,
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
                    x < states[i].len() as CatType,
                    "Values of variable '{label}' must be less than the number of states: \n\
                    \t expected: values[.., '{label}'] < |states['{label}']| , \n\
                    \t found:    values[.., '{label}'] == {x} and |states['{label}']| == {} .",
                    states[i].len(),
                    label = states.get_index(i).unwrap().0,
                );
            });

        // Check that the labels are sorted.
        if !states.keys().is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<usize> = (0..states.len()).collect();
            // Sort the indices by labels.
            indices.sort_by_key(|&i| states.get_index(i).unwrap().0);
            // Sort the states.
            states.sort_keys();
            // Allocate new values.
            let mut new_values = values.clone();
            // Sort the new values according to the sorted indices.
            indices.into_iter().enumerate().for_each(|(i, j)| {
                new_values.column_mut(i).assign(&values.column(j));
            });
            // Update values.
            values = new_values;
        }

        // For each variable ...
        for (mut col, states) in values.columns_mut().into_iter().zip(states.values_mut()) {
            // ... check if the states are sorted.
            if !states.is_sorted() {
                // Clone the states.
                let mut new_states = states.clone();
                // Sort the states.
                new_states.sort();
                // Map values to sorted states.
                col.iter_mut().for_each(|value| {
                    *value = new_states
                        .get_index_of(&states[*value as usize])
                        .expect("Failed to get new state index.")
                        as CatType;
                });
                // Update the states.
                *states = new_states;
            }
        }

        // Get the labels of the variables.
        let labels = states.keys().cloned().collect();
        // Get the shape of the states.
        let shape = states.values().map(Set::len).collect();

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
    type Values = Array2<CatType>;

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
    fn from_csv_reader<R: Read>(reader: R) -> Self {
        // Create a CSV reader from the string.
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        // Assert that the reader has headers.
        assert!(reader.has_headers(), "Reader must have headers.");

        // Read the headers.
        let labels: Labels = reader
            .headers()
            .expect("Failed to read the headers.")
            .into_iter()
            .map(|x| x.to_owned())
            .collect();

        // Get the states of the variables.
        let mut states: States = labels
            .iter()
            .map(|x| (x.clone(), Default::default()))
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
                        x as CatType
                    })
                    .collect();
                // Collect the values.
                row
            })
            .collect();

        // Get the number of rows and columns.
        let ncols = labels.len();
        let nrows = values.len() / ncols;
        // Reshape the values to the correct shape.
        let values = values
            .into_shape_with_order((nrows, ncols))
            .expect("Failed to rearrange values to the correct shape.");

        // Construct the dataset.
        Self::new(states, values)
    }

    fn to_csv_writer<W: Write>(&self, writer: W) {
        // Create the CSV writer.
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        // Write the headers.
        writer
            .write_record(self.labels.iter())
            .expect("Failed to write CSV headers.");

        // Write the records.
        self.values.rows().into_iter().for_each(|row| {
            // Map the row values to states.
            let record = row
                .iter()
                .zip(self.states().values())
                .map(|(&x, states)| &states[x as usize]);
            // Write the record.
            writer
                .write_record(record)
                .expect("Failed to write CSV record.");
        });
    }
}
