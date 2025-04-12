use std::io::Read;

use csv::Reader;
use ndarray::prelude::*;

use crate::{
    datasets::CategoricalDataset,
    types::{FxIndexMap, FxIndexSet},
};

/// A trait for reading CSV files.
pub trait FromCsvReader {
    /// Reads a CSV file.
    ///
    /// # Arguments
    ///
    /// * `reader` - A CSV reader from the `csv` crate.
    ///
    /// # Returns
    ///
    /// A new instance of the implementing type.
    ///
    /// # Notes
    ///
    /// CSV reader should trim input.
    ///
    fn from_csv_reader<R: Read>(reader: Reader<R>) -> Self;
}

impl FromCsvReader for CategoricalDataset {
    /// Reads a CSV file and returns a new `CategoricalDataset` instance.
    ///
    /// # Arguments
    ///
    /// * `reader` - A CSV reader from the `csv` crate.
    ///
    /// # Panics
    ///
    /// * If the CSV reader does not have headers.
    /// * If the CSV reader fails to read the headers.
    /// * If the CSV reader fails to read a line.
    /// * If the CSV reader returns a missing value.
    ///
    /// # Returns
    ///
    /// A new `CategoricalDataset` instance.
    ///
    fn from_csv_reader<R: Read>(mut reader: Reader<R>) -> Self {
        // Assert that the reader has headers.
        assert!(reader.has_headers(), "Reader must have headers.");

        // Initialize the counter.
        let mut n = 0;
        // Read the headers.
        let labels: FxIndexSet<_> = reader
            .headers()
            .expect("Failed to read the headers.")
            .into_iter()
            .inspect(|_| n += 1)
            .map(|x| x.to_owned())
            .collect();
        // Assert unique labels.
        assert_eq!(labels.len(), n, "Header labels must be unique.");

        // Get the states of the variables.
        let mut states: FxIndexMap<_, FxIndexSet<String>> = labels
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
        CategoricalDataset::new(states, values)
    }
}
