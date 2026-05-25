use std::{
    borrow::Cow,
    fmt::Display,
    io::{Read, Write},
    sync::Arc,
};

use csv::{ReaderBuilder, WriterBuilder};
use itertools::Itertools;
use log::debug;
use ndarray::prelude::*;

use crate::{
    datasets::{CatEv, CatEvT, Dataset},
    io::CsvIO,
    models::{CatSupport, Labelled},
    types::{Error, Labels, Result, Set},
};

/// A type alias for a categorical variable.
pub type CatType = u8;
/// A type alias for a categorical sample.
pub type CatSample = Array1<CatType>;

/// A struct representing a categorical dataset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatTable {
    labels: Labels,
    support: CatSupport,
    shape: Array1<usize>,
    values: Array2<CatType>,
}

/// Concrete iterator over categorical table evidences.
pub struct CatTableEvidenceIter<'a> {
    rows: ndarray::iter::LanesIter<'a, CatType, Ix1>,
    support: &'a CatSupport,
}

impl<'a> Iterator for CatTableEvidenceIter<'a> {
    type Item = Result<CatEv>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.rows.next()?;

        let evidences = row
            .iter()
            .enumerate()
            .map(|(event, &state)| CatEvT::CertainPositive {
                event,
                state: state as usize,
            });

        Some(CatEv::new(self.support.clone(), evidences))
    }
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
    /// * `support` - The variables support.
    /// * `values` - The values of the variables.
    ///
    /// # Notes
    ///
    /// * Labels and support will be sorted in alphabetical order.
    ///
    /// # Errors
    ///
    /// * If the number of variable support is higher than `CatType::MAX`.
    /// * If the number of variables is different from the number of values columns.
    /// * If the variables values are not smaller than the number of support.
    ///
    /// # Panics
    ///
    /// * If the variable labels are not unique.
    /// * If the variable support are not unique.
    ///
    /// # Returns
    ///
    /// A new categorical dataset instance.
    ///
    pub fn new(mut support: CatSupport, mut values: Array2<CatType>) -> Result<Self> {
        // Log the creation of the categorical dataset.
        debug!(
            "Creating a new categorical dataset with {} variables and {} samples.",
            support.len(),
            values.nrows()
        );

        // Check if the number of support is less than `CatType::MAX`.
        support.iter().try_for_each(|(label, state)| {
            if state.len() > CatType::MAX as usize {
                return Err(Error::InvalidParameter(
                    &format!("support[{label}]"),
                    &format!("should have less than 256 support, found {}", state.len()),
                ));
            }
            Ok(())
        })?;
        // Check if the number of variables is equal to the number of columns.
        if support.len() != values.ncols() {
            return Err(Error::IncompatibleShape(
                &format!("|support| = {}", support.len()),
                &format!("|cols| = {}", values.ncols()),
            ));
        }
        // Check if the maximum value of the values is less than the number of support.
        values
            .fold_axis(Axis(0), 0, |&a, &b| if a > b { a } else { b })
            .into_iter()
            .enumerate()
            .try_for_each(|(i, x)| {
                let (label, support) = support
                    .get_index(i)
                    .ok_or_else(|| Error::IndexOutOfBounds(i))?;

                if x >= support.len() as CatType {
                    return Err(Error::InvalidParameter(
                        &format!("values[.., '{label}']"),
                        &format!(
                            "must be less than the number of support ({}), found {x}",
                            support.len()
                        ),
                    ));
                }
                Ok(())
            })?;

        // Check that the labels are sorted.
        if !support.keys().is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<usize> = (0..support.len()).collect();
            // Sort the indices by labels.
            let keys: Vec<_> = support.keys().collect();
            indices.sort_by_key(|&i| keys[i]);
            // Sort the support.
            support.sort_keys();
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
        values
            .columns_mut()
            .into_iter()
            .zip(support.values_mut())
            .try_for_each(|(mut col, support)| -> Result<_> {
                // ... check if the support are sorted.
                if !support.is_sorted() {
                    // Clone the support.
                    let mut new_states = support.clone();
                    // Sort the support.
                    new_states.sort();
                    // Map values to sorted support.
                    col.iter_mut().try_for_each(|value| -> Result<_> {
                        // Get the state.
                        let state = &support[*value as usize];
                        // Map the value to the sorted support.
                        *value = new_states
                            .get_index_of(state)
                            .ok_or_else(|| Error::MissingState(state))?
                            as CatType;
                        Ok(())
                    })?;
                    // Update the support.
                    *support = new_states;
                }
                Ok(())
            })?;

        // Get the labels of the variables.
        let labels = support.keys().cloned().collect();
        // Get the shape of the support.
        let shape = support.values().map(Set::len).collect();

        Ok(Self {
            labels,
            support,
            shape,
            values,
        })
    }

    /// Returns the support of the variables in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the vector of support.
    ///
    #[inline]
    pub const fn support(&self) -> &CatSupport {
        &self.support
    }

    /// Returns the shape of the set of support in the categorical distribution.
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
        // Get the maximum length of the labels and support.
        let n = self
            .labels()
            .iter()
            .chain(self.support().values().flatten())
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
                .map(|(i, &x)| &self.support()[i][x as usize])
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
    type Support = CatSupport;
    type Evidence = CatEv;
    type EvidenceIter<'a> = CatTableEvidenceIter<'a>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.support)
    }

    fn evidence_iter(&self) -> Self::EvidenceIter<'_> {
        CatTableEvidenceIter {
            rows: self.values.rows().into_iter(),
            support: &self.support,
        }
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.values.nrows() as f64
    }

    fn select(&self, x: &Set<usize>) -> Result<Self> {
        // Check that the indices are valid.
        x.iter().try_for_each(|&i| {
            if i >= self.values.ncols() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;

        // Select the support.
        let support: CatSupport = x
            .iter()
            .map(|&i| {
                self.support
                    .get_index(i)
                    .map(|(label, support)| (label.clone(), support.clone()))
                    .ok_or_else(|| Error::IndexOutOfBounds(i))
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
        Self::new(support, values)
    }
}

impl CsvIO for CatTable {
    fn from_csv_reader<R: Read>(reader: R) -> Result<Self> {
        // Create a CSV reader from the string.
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        // Check if the reader has headers.
        if !reader.has_headers() {
            return Err(Error::MissingHeader());
        }

        // Read the headers.
        let labels: Labels = reader
            .headers()?
            .into_iter()
            .map(|x| x.to_owned())
            .collect();

        // Get the support of the variables.
        let mut support: CatSupport = labels
            .iter()
            .map(|x| (x.clone(), Default::default()))
            .collect();

        // Read the records.
        let values: Vec<CatType> = reader.into_records().enumerate().try_fold(
            Vec::new(),
            |mut values, (i, row)| -> Result<_> {
                // Get the record row.
                let row = row.map_err(|e| Error::Csv(Arc::new(e)))?;
                // Zip the row with the support.
                for (j, (x, support)) in row.into_iter().zip(support.values_mut()).enumerate() {
                    // Check if the value is empty.
                    if x.is_empty() {
                        return Err(Error::MissingValue(i + 1, j + 1));
                    }
                    // Insert the value into the support, if not present.
                    let (idx, _) = support.insert_full(x.to_owned());
                    // Collect the value.
                    values.push(idx as CatType);
                }

                Ok(values)
            },
        )?;

        // Convert the values to an array.
        let values = Array1::from_vec(values);

        // Get the number of rows and columns.
        let ncols = labels.len();
        let nrows = values.len() / ncols;
        // Reshape the values to the correct shape.
        let values = values.into_shape_with_order((nrows, ncols))?;

        // Construct the dataset.
        Self::new(support, values)
    }

    fn to_csv_writer<W: Write>(&self, writer: W) -> Result<()> {
        // Create the CSV writer.
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        // Write the headers.
        writer.write_record(self.labels.iter())?;

        // Write the records.
        for row in self.values.rows() {
            // Map the row values to support.
            let record = row
                .iter()
                .zip(self.support().values())
                .map(|(&x, support)| &support[x as usize]);
            // Write the record.
            writer.write_record(record)?;
        }

        Ok(())
    }
}
