use std::io::{Read, Write};

use csv::{ReaderBuilder, WriterBuilder};
use itertools::Either;
use ndarray::prelude::*;

use crate::{
    datasets::{
        Dataset, GaussTable, GaussType, GaussWtdTable, IncDataset, MissingMethod as MM,
        MissingTable,
    },
    io::CsvIO,
    labels,
    models::Labelled,
    types::{Labels, Map, Result, Set},
};

/// A struct representing an incomplete gaussian dataset.
#[derive(Clone, Debug)]
pub struct GaussIncTable {
    labels: Labels,
    values: Array2<GaussType>,
    missing: MissingTable,
}

impl Labelled for GaussIncTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl GaussIncTable {
    /// Creates a new gaussian incomplete tabular data instance.
    pub fn new(mut labels: Labels, mut values: Array2<GaussType>) -> Self {
        // Check if the number of variables is equal to the number of columns.
        assert_eq!(
            labels.len(),
            values.ncols(),
            "Number of variables must be equal to the number of columns: \n\
            \t expected:    |labels| == |values.columns()| , \n\
            \t found:       |labels| == {} and |values.columns()| == {} .",
            labels.len(),
            values.ncols()
        );

        // Check that the labels are sorted.
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

        // Create the missing mask.
        let missing_mask = values.mapv(|x| x.is_nan());
        // Initialize the missing table.
        let missing = MissingTable::new(labels.clone(), missing_mask);

        Self {
            labels,
            values,
            missing,
        }
    }

    /// List-wise deletion missing handler.
    fn lw_deletion(&self) -> GaussTable {
        // Allocate new values.
        let mut new_values = Array::zeros((
            self.missing.complete_rows_count(), //
            self.values.ncols(),
        ));

        // Get complete rows.
        let rows = self
            .values
            .rows()
            .into_iter()
            .filter(|row| row.iter().all(|&x| !x.is_nan()));

        // Filter valid rows.
        new_values
            .rows_mut()
            .into_iter()
            .zip(rows)
            .for_each(|(mut new_row, row)| new_row.assign(&row));

        // Return the complete dataset.
        GaussTable::new(self.labels.clone(), new_values)
    }

    /// Pair-wise deletion missing handler.
    fn pw_deletion(&self, x: &Set<usize>) -> GaussTable {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let s = labels![];
            let v = Array::default((0, 0));
            return GaussTable::new(s, v);
        }

        // Assert that the indices are valid.
        x.iter().for_each(|&i| {
            assert!(
                i < self.values.ncols(),
                "Index out of bounds in PW deletion: \n\
                \t expected:    index < |values.columns()| , \n\
                \t found:       index == {} and |values.columns()| == {} .",
                i,
                self.values.ncols()
            );
        });

        // Clone the indices.
        let mut cols: Vec<usize> = x.iter().cloned().collect();
        // Sort the indices.
        cols.sort();

        // Get the indices of complete rows for the specified columns.
        let rows: Vec<_> = self
            .values
            .rows()
            .into_iter()
            .enumerate()
            .filter_map(|(i, row)| {
                // Check if all specified columns are not missing.
                if cols.iter().all(|&j| !row[j].is_nan()) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        // Collect the values for the specified rows and columns.
        let new_values = Array::from_shape_fn(
            (rows.len(), cols.len()), //
            |(i, j)| self.values[[rows[i], cols[j]]],
        );

        // Select the labels for the specified columns.
        let new_labels = cols
            .iter()
            .map(|&j| self.labels.get_index(j).unwrap())
            .cloned()
            .collect();

        // Return the complete dataset.
        GaussTable::new(new_labels, new_values)
    }
}

impl Dataset for GaussIncTable {
    type Values = Array2<GaussType>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.values.nrows() as f64
    }

    fn select(&self, x: &Set<usize>) -> Self {
        // Assert that the indices are valid.
        x.iter().for_each(|&i| {
            assert!(
                i < self.values.ncols(),
                "Index out of bounds in variables selection: \n\
                \t expected:    index < |columns| , \n\
                \t found:       index == {} and |columns| == {} .",
                i,
                self.values.ncols()
            );
        });

        // Select the labels.
        let labels: Labels = x
            .iter()
            .map(|&i| self.labels.get_index(i).unwrap())
            .cloned()
            .collect();

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

impl IncDataset for GaussIncTable {
    type Missing = GaussType;
    const MISSING: Self::Missing = GaussType::NAN;

    type Complete = GaussTable;
    type Weighted = GaussWtdTable;

    #[inline]
    fn missing(&self) -> &MissingTable {
        &self.missing
    }

    fn apply_missing_method(
        &self,
        m: &MM,
        x: Option<&Set<usize>>,
        _pr: Option<&Map<usize, Set<usize>>>,
    ) -> Either<Self::Complete, Self::Weighted> {
        // Apply the missing method with the provided arguments.
        match (m, x) {
            (MM::LW, _) => Either::Left(self.lw_deletion()),
            (MM::PW, Some(x)) => Either::Left(self.pw_deletion(x)),
            _ => panic!(
                "Invalid arguments for applying missing method:\n
                \t missing method:      '{m:?}' , \n\
                \t selected variables:  '{x:?}' ."
            ),
        }
    }

    fn lw_deletion(&self) -> Self::Complete {
        self.lw_deletion()
    }

    fn pw_deletion(&self, x: &Set<usize>) -> Self::Complete {
        self.pw_deletion(x)
    }

    fn ipw_deletion(&self, _x: &Set<usize>, _pr: &Map<usize, Set<usize>>) -> Self::Weighted {
        unimplemented!("IPW deletion not implemented for Gaussian data yet.")
    }

    fn aipw_deletion(&self, _x: &Set<usize>, _pr: &Map<usize, Set<usize>>) -> Self::Weighted {
        unimplemented!("AIPW deletion not implemented for Gaussian data yet.")
    }
}

impl CsvIO for GaussIncTable {
    fn from_csv_reader<R: Read>(reader: R) -> Result<Self> {
        // Create a CSV reader from the string.
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        // Assert that the reader has headers.
        assert!(reader.has_headers(), "Reader must have headers.");

        // Read the headers.
        let labels: Labels = reader
            .headers()?
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
                    .map(|(_, x)| {
                        // Cast the value.
                        x.parse::<GaussType>().unwrap_or(Self::MISSING)
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
        let values = values.into_shape_with_order((nrows, ncols))?;

        // Construct the dataset.
        Ok(Self::new(labels, values))
    }

    fn to_csv_writer<W: Write>(&self, writer: W) -> Result<()> {
        // Create the CSV writer.
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        // Write the headers.
        writer.write_record(self.labels.iter())?;

        // Write the records.
        for row in self.values.rows() {
            // Map the row values to strings.
            let record = row.iter().map(|&x| {
                if x.is_nan() {
                    "".to_string()
                } else {
                    x.to_string()
                }
            });
            // Write the record.
            writer.write_record(record)?;
        }

        Ok(())
    }
}
