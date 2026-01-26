use std::{
    io::{Read, Write},
    sync::Arc,
};

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    io::CsvIO,
    models::Labelled,
    types::{Error, Labels, Result, Set},
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
    pub fn new(mut labels: Labels, mut values: Array2<GaussType>) -> Result<Self> {
        // Check if the number of labels matches the number of columns in values.
        if labels.len() != values.ncols() {
            return Err(Error::IncompatibleShape(
                labels.len().to_string(),
                values.ncols().to_string(),
            ));
        }

        // Sort labels and values accordingly.
        if !labels.is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<usize> = (0..labels.len()).collect();
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
        // Check values are finite.
        if !values.iter().all(|&x| x.is_finite()) {
            return Err(Error::InvalidParameter(
                "values".to_string(),
                "must be finite".to_string(),
            ));
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

    fn select(&self, x: &Set<usize>) -> Result<Self> {
        // Check that the indices are valid.
        if let Some(&i) = x.iter().find(|&&i| i >= self.values.ncols()) {
            return Err(Error::VertexOutOfBounds(i));
        }

        // Select the labels.
        let labels: Labels = x
            .iter()
            .map(|&i| {
                self.labels
                    .get_index(i)
                    .cloned()
                    .ok_or(Error::VertexOutOfBounds(i))
            })
            .collect::<Result<_>>()?;

        // Select the values.
        let mut new_values = Array2::zeros((self.values.nrows(), x.len()));
        // Copy the selected columns.
        x.iter().enumerate().for_each(|(j, &i)| {
            new_values.column_mut(j).assign(&self.values.column(i));
        });
        // Update the values.
        let values = new_values;

        // Return the new dataset.
        Self::new(labels, values)
    }
}

impl CsvIO for GaussTable {
    fn from_csv_reader<R: Read>(reader: R) -> Result<Self> {
        // Create a CSV reader from the string.
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        // Check if the reader has headers.
        if !reader.has_headers() {
            return Err(Error::MissingHeader);
        }

        // Read the headers.
        let labels: Labels = reader
            .headers()?
            .into_iter()
            .map(|x| x.to_owned())
            .collect();

        // Read the records.
        let values: Vec<GaussType> = reader
            .into_records()
            .enumerate()
            .map(|(i, row)| {
                // Get the record row.
                let row = row.map_err(|e| Error::Csv(Arc::new(e)))?;
                // Get the record values and convert to indices.
                row.into_iter()
                    .enumerate()
                    .map(|(j, x)| {
                        // Check for missing values.
                        if x.is_empty() {
                            return Err(Error::MissingValue(i + 1, j + 1));
                        }
                        // Cast the value.
                        Ok(x.parse::<GaussType>()?)
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        // Convert the values to an array.
        let values = Array1::from_vec(values);

        // Get the number of rows and columns.
        let ncols = labels.len();
        let nrows = values.len() / ncols;
        // Reshape the values to the correct shape.
        let values = values.into_shape_with_order((nrows, ncols))?;

        // Construct the dataset.
        Self::new(labels, values)
    }

    fn to_csv_writer<W: Write>(&self, writer: W) -> Result<()> {
        // Create the CSV writer.
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        // Write the headers.
        writer.write_record(self.labels.iter())?;

        // Write the records.
        self.values
            .rows()
            .into_iter()
            .try_for_each(|row| -> Result<_> {
                // Map the row values to strings.
                let record = row.iter().map(|x| x.to_string());
                // Write the record.
                writer.write_record(record)?;

                Ok(())
            })?;

        Ok(())
    }
}
