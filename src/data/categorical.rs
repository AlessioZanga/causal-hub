use std::{fmt::Display, io::Read};

use csv::Reader;
use itertools::Itertools;
use ndarray::prelude::*;

use crate::{
    io::FromCsvReader,
    types::{FxIndexMap, FxIndexSet},
};

use super::Data;

/// A struct representing a categorical data.
///
#[derive(Clone, Debug)]
pub struct CategoricalData {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    values: Array2<u8>,
}

impl CategoricalData {
    /// Creates a new categorical data.
    ///
    /// # Arguments
    ///
    /// * `variables` - The variables and their states.
    /// * `values` - The values of the variables.
    ///
    /// # Panics
    ///
    /// * If the variable labels are not unique.
    /// * If the variable states are not unique.
    /// * If the number of variables is not equal to the number of columns.
    /// * If the variables values are not smaller than the number of states.
    ///
    /// # Returns
    ///
    /// A new `CategoricalData` instance.
    ///
    pub fn new(variables: Vec<(&str, Vec<&str>)>, values: Array2<u8>) -> Self {
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

        // Check if the number of variables is equal to the number of columns.
        assert_eq!(
            cardinality.len(),
            values.ncols(),
            "Number of variables must be equal to the number of columns."
        );
        // Check if the maximum value of the values is less than the number of states.
        assert!(
            values.rows().into_iter().all(|row|
                    // For each row ...
                    row.iter().zip(&cardinality)
                    // ... check if the value is less than the number of states.
                    .all(|(x, y)| (*x as usize) < *y)),
            "Variables values must be smaller than the number of states."
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

impl Display for CategoricalData {
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

impl Data for CategoricalData {
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
}

impl FromCsvReader for CategoricalData {
    /// Reads a CSV file and returns a new `CategoricalData` instance.
    ///
    /// # Arguments
    ///
    /// * `reader` - A CSV reader from the `csv` crate.
    ///
    /// # Returns
    ///
    /// A new `CategoricalData` instance.
    ///
    fn from_csv_reader<R: Read>(mut reader: Reader<R>) -> Self {
        // Assert that the reader has headers.
        assert!(reader.has_headers(), "Failed to read the headers.");
        // Read the headers.
        let labels: FxIndexSet<_> = reader
            .headers()
            .expect("Failed to read the headers.")
            .into_iter()
            .map(|x| x.to_owned())
            .collect();

        // Read the records.
        // NOTE: One could improve this implementation
        //       by mapping the strings to u8 directly.
        let values: Array1<_> = reader
            .into_records()
            .enumerate()
            .flat_map(|(i, row)| {
                // Get the record row.
                let row = row.unwrap_or_else(|_| panic!("Failed to read line number {}.", i + 1));
                // Get the record values and convert to strings.
                let row = row.into_iter().map(|x| x.to_owned());
                // Assert no empty (missing) values.
                let row: Vec<_> = row
                    .inspect(|x| {
                        assert!(
                            !x.is_empty(),
                            "Failed to read empty value at line number {}.",
                            i + 1
                        );
                    })
                    .collect();
                // Collect the values.
                row
            })
            .collect();
        // Get the number of rows and columns.
        let ncols = labels.len();
        let nrows = values.len() / ncols;
        // Allocate the values.
        let values = values
            .into_shape_with_order((nrows, ncols))
            .expect("Failed to rearrange values to the correct shape.");

        // Get the states of the variables.
        let states: FxIndexMap<_, FxIndexSet<_>> = labels
            .iter()
            .zip(values.columns())
            .map(|(i, j)| {
                (
                    // Convert the variable label to a string.
                    i.to_owned(),
                    // Convert the variable states to a set of strings.
                    j.iter().map(|k| k.to_owned()).collect(),
                )
            })
            .collect();

        // Get the cardinality of the set of states.
        let cardinality: Array1<_> = states.values().map(|i| i.len()).collect();

        // Map the values to the states indexes.
        let values: Array2<_> = Array2::from_shape_fn(values.dim(), |(i, j)| {
            // Get the state corresponding to the value.
            states[j].get_index_of(&values[[i, j]]).unwrap() as u8
        });

        CategoricalData {
            labels,
            states,
            cardinality,
            values,
        }
    }
}
