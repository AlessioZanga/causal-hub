use csv::ReaderBuilder;
use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    io::CsvIO,
    models::Labelled,
    types::{Error, Labels},
};

/// A type alias for a gaussian variable.
pub type GaussType = f64;
/// A type alias for a gaussian sample.
pub type GaussSample = Array1<GaussType>;

/// A struct representing a gaussian dataset.
#[derive(Clone, Debug)]
pub struct GaussTable {
    labels: Labels,
    values: Array2<GaussType>,
}

impl Labelled for GaussTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl GaussTable {
    /// Creates a new gaussian dataset.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the variables.
    /// * `values` - The values of the variables.
    ///
    /// # Panics
    ///
    /// * Panics if the number of columns in `values` does not match the number of `labels`.
    ///
    /// # Results
    ///
    /// A new gaussian dataset instance.
    ///
    pub fn new(mut labels: Labels, mut values: Array2<GaussType>) -> Result<Self, Error> {
        // Check that the number of labels matches the number of columns in values.
        if labels.len() != values.ncols() {
            return Err(Error::ColumnsLabelsMismatch {
                columns: values.ncols(),
                labels: labels.len(),
            });
        }
        // Check values are finite.
        if !values.iter().all(|&x| x.is_finite()) {
            return Err(Error::NonFiniteValues);
        }

        // Sort labels and values accordingly.
        if !labels.is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<_> = (0..labels.len()).collect();
            // Sort the indices by labels.
            indices.sort_by_key(|&i| &labels[i]);
            // Sort the labels.
            labels.sort();
            // Allocate new values.
            let mut new_values = values.clone();
            // Sort the new values according to the sorted indices.
            indices.into_iter().enumerate().for_each(|(i, j)| {
                new_values.column_mut(i).assign(&values.column(j));
            });
            // Update values.
            values = new_values;
        }

        Ok(Self { labels, values })
    }
}

impl Dataset for GaussTable {
    type Values = Array2<GaussType>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.values.nrows() as f64
    }
}

impl CsvIO for GaussTable {
    fn from_csv(csv: &str) -> Result<Self, Error> {
        // Create a CSV reader from the string.
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(csv.as_bytes());

        // Assert that the reader has headers.
        assert!(reader.has_headers(), "Reader must have headers.");

        // Read the headers.
        let labels: Labels = reader
            .headers()
            .expect("Failed to read the headers.")
            .into_iter()
            .map(|x| x.to_owned())
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
                        // Cast the value.
                        x.parse::<GaussType>().unwrap()
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
        Self::new(labels, values)
    }

    fn to_csv(&self) -> String {
        todo!() // FIXME:
    }
}
