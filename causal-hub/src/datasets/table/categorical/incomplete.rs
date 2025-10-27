use std::io::{Read, Write};

use ndarray::prelude::*;

use crate::{
    datasets::{CatTable, CatType, CatWtdTable, Dataset, IncDataset, MissingTable},
    io::CsvIO,
    models::Labelled,
    types::{Labels, Set, States},
};

/// A struct representing an incomplete categorical dataset.
#[derive(Clone, Debug)]
pub struct CatIncTable {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    values: Array2<CatType>,
    missing: MissingTable,
}

impl Labelled for CatIncTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CatIncTable {
    /// Creates a new categorical incomplete tabular data instance.
    pub fn new(mut states: States, mut values: Array2<CatType>) -> Self {
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
            .fold_axis(
                Axis(0),
                0,
                // Find max while ignoring missing values.
                |&a, &b| if a > b || b == Self::MISSING { a } else { b },
            )
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
                    // If the value is not missing ...
                    if *value != Self::MISSING {
                        // ... map it to the new state index.
                        *value = new_states
                            .get_index_of(&states[*value as usize])
                            .expect("Failed to get new state index.")
                            as CatType;
                    }
                });
                // Update the states.
                *states = new_states;
            }
        }

        // Get the labels of the variables.
        let labels: Labels = states.keys().cloned().collect();
        // Get the shape of the states.
        let shape = states.values().map(Set::len).collect();

        // Create the missing mask.
        let missing_mask = values.mapv(|x| x == Self::MISSING);
        // Initialize the missing table.
        let missing = MissingTable::new(labels.clone(), missing_mask);

        Self {
            labels,
            states,
            shape,
            values,
            missing,
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

impl Dataset for CatIncTable {
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

impl IncDataset for CatIncTable {
    type Missing = CatType;
    const MISSING: Self::Missing = CatType::MAX;

    type Complete = CatTable;
    type Weighted = CatWtdTable;

    #[inline]
    fn missing(&self) -> &MissingTable {
        &self.missing
    }

    fn lw_deletion(&self) -> Self::Complete {
        // Allocate new values.
        let mut new_values = Array::zeros((
            self.missing.complete_rows_count(), //
            self.values.ncols(),
        ));

        // Get row-indicator pairs.
        self.values
            .rows()
            .into_iter()
            .zip(self.missing.missing_mask_by_rows())
            // Filter for complete rows only.
            .filter_map(|(row, &is_complete)| if !is_complete { Some(row) } else { None })
            // Fill new values with complete rows only.
            .zip(new_values.rows_mut())
            .for_each(|(row, mut new_row)| new_row.assign(&row));

        // Return new complete categorical table.
        Self::Complete::new(self.states.clone(), new_values)
    }

    fn pw_deletion(&self, x: &Set<usize>) -> Self::Complete {
        // Assert that the indices are valid.
        x.iter().for_each(|&i| {
            assert!(
                i < self.values.ncols(),
                "Index out of bounds in pair-wise deletion: \n\
                \t expected:    index < |values.columns()| , \n\
                \t found:       index == {} and |values.columns()| == {} .",
                i,
                self.values.ncols()
            );
        });

        // Clone the indices.
        let mut cols = x.clone();
        // Sort the indices.
        cols.sort();

        // Get the indices of complete rows for the specified columns.
        let rows: Vec<_> = self
            .missing
            .missing_mask()
            .rows()
            .into_iter()
            .enumerate()
            .filter_map(|(i, row)| {
                // Check if all specified columns are not missing.
                if !cols.iter().any(|&j| row[j]) {
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

        // Select the states for the specified columns.
        let new_states = cols
            .iter()
            .map(|&j| self.states.get_index(j).unwrap())
            .map(|(label, state)| (label.clone(), state.clone()))
            .collect();

        // Return new complete categorical table.
        Self::Complete::new(new_states, new_values)
    }

    fn ipw_deletion(&self, _x: &Set<usize>) -> Self::Weighted {
        todo!() // FIXME:
    }

    fn aipw_deletion(&self, _x: &Set<usize>) -> Self::Weighted {
        todo!() // FIXME:
    }
}

impl CsvIO for CatIncTable {
    fn from_csv_reader<R: Read>(_csv: R) -> Self {
        todo!() // FIXME:
    }

    fn to_csv_writer<W: Write>(&self, _writer: W) {
        todo!() // FIXME:
    }
}
