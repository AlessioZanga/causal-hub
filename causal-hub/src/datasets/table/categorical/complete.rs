use std::{
    fmt::Display,
    io::{Read, Write},
    sync::Arc,
};

use csv::{ReaderBuilder, WriterBuilder};
use itertools::Itertools;
use log::debug;
use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    io::CsvIO,
    models::Labelled,
    types::{Error, Labels, Result, Set, States},
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
    pub fn new(mut states: States, mut values: Array2<CatType>) -> Result<Self> {
        // Log the creation of the categorical dataset.
        debug!(
            "Creating a new categorical dataset with {} variables and {} samples.",
            states.len(),
            values.nrows()
        );

        // Check if the number of states is less than `CatType::MAX`.
        for (label, state) in &states {
            if state.len() > CatType::MAX as usize {
                return Err(Error::InvalidParameter(
                    format!("states[{label}]"),
                    format!("should have less than 256 states, found {}", state.len()),
                ));
            }
        }
        // Check if the number of variables is equal to the number of columns.
        if states.len() != values.ncols() {
            return Err(Error::IncompatibleShape(
                format!("|states| = {}", states.len()),
                format!("|cols| = {}", values.ncols()),
            ));
        }
        // Check if the maximum value of the values is less than the number of states.
        for (i, x) in values
            .fold_axis(Axis(0), 0, |&a, &b| if a > b { a } else { b })
            .into_iter()
            .enumerate()
        {
            let (label, states) = states.get_index(i).ok_or(Error::VertexOutOfBounds(i))?;

            if x >= states.len() as CatType {
                return Err(Error::InvalidParameter(
                    format!("values[.., '{label}']"),
                    format!(
                        "must be less than the number of states ({}), found {x}",
                        states.len()
                    ),
                ));
            }
        }

        // Check that the labels are sorted.
        if !states.keys().is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<usize> = (0..states.len()).collect();
            // Sort the indices by labels.
            let keys: Vec<_> = states.keys().collect();
            indices.sort_by_key(|&i| keys[i]);
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
                for value in col.iter_mut() {
                    *value = new_states
                        .get_index_of(&states[*value as usize])
                        .ok_or_else(|| Error::MissingState(states[*value as usize].clone()))?
                        as CatType;
                }
                // Update the states.
                *states = new_states;
            }
        }

        // Get the labels of the variables.
        let labels = states.keys().cloned().collect();
        // Get the shape of the states.
        let shape = states.values().map(Set::len).collect();

        Ok(Self {
            labels,
            states,
            shape,
            values,
        })
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

    fn select(&self, x: &Set<usize>) -> Result<Self> {
        // Check that the indices are valid.
        for &i in x {
            if i >= self.values.ncols() {
                return Err(Error::VertexOutOfBounds(i));
            }
        }

        // Select the states.
        let states: States = x
            .iter()
            .map(|&i| {
                self.states
                    .get_index(i)
                    .map(|(label, states)| (label.clone(), states.clone()))
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
        Self::new(states, values)
    }
}

impl CsvIO for CatTable {
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

        // Get the states of the variables.
        let mut states: States = labels
            .iter()
            .map(|x| (x.clone(), Default::default()))
            .collect();

        // Read the records.
        let values: Vec<CatType> =
            reader
                .into_records()
                .enumerate()
                .try_fold(Vec::new(), |mut values, (i, row)| {
                    // Get the record row.
                    let row = row.map_err(|e| Error::Csv(Arc::new(e)))?;
                    // Zip the row with the states.
                    for (j, (x, states)) in row.into_iter().zip(states.values_mut()).enumerate() {
                        // Check if the value is empty.
                        if x.is_empty() {
                            return Err(Error::MissingValue(i + 1, j + 1));
                        }
                        // Insert the value into the states, if not present.
                        let (idx, _) = states.insert_full(x.to_owned());
                        // Collect the value.
                        values.push(idx as CatType);
                    }

                    Ok::<_, Error>(values)
                })?;

        // Convert the values to an array.
        let values = Array1::from_vec(values);

        // Get the number of rows and columns.
        let ncols = labels.len();
        let nrows = values.len() / ncols;
        // Reshape the values to the correct shape.
        let values = values.into_shape_with_order((nrows, ncols))?;

        // Construct the dataset.
        Self::new(states, values)
    }

    fn to_csv_writer<W: Write>(&self, writer: W) -> Result<()> {
        // Create the CSV writer.
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        // Write the headers.
        writer.write_record(self.labels.iter())?;

        // Write the records.
        for row in self.values.rows() {
            // Map the row values to states.
            let record = row
                .iter()
                .zip(self.states().values())
                .map(|(&x, states)| &states[x as usize]);
            // Write the record.
            writer.write_record(record)?;
        }

        Ok(())
    }
}
