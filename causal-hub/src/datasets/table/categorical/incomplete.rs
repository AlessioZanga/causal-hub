use std::io::{Read, Write};

use csv::{ReaderBuilder, WriterBuilder};
use itertools::Either;
use ndarray::{Zip, prelude::*};

use crate::{
    datasets::{
        CatTable, CatType, CatWtdTable, Dataset, IncDataset, MissingMethod as MM, MissingTable,
    },
    estimators::{BE, CPDEstimator},
    io::CsvIO,
    models::{CPD, Labelled},
    set, states,
    types::{Labels, Map, Set, States},
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

        // Select the states.
        let states: States = x
            .iter()
            .map(|&i| self.states.get_index(i).unwrap())
            .map(|(label, states)| (label.clone(), states.clone()))
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
        Self::new(states, values)
    }
}

impl CatIncTable {
    /// Compute the weights to perform IPW.
    fn ipw_weights(
        &self,
        d_u: &<Self as IncDataset>::Complete,
        u: &Set<usize>,
        pr: &Map<usize, Set<usize>>,
    ) -> Array1<f64> {
        // Get (`R_i`, `Pi_R_i`) associated to `U_i`.
        let pr = u.iter().map(|&ri| (ri, &pr[ri]));
        // Filter out `R_i` with no parents.
        let pr = pr.filter(|(_, pri)| !pri.is_empty());

        // Define function to compute the weights associated to each `R_i`.
        let beta_i = |d_u: &CatTable, ri: usize, pri: &Set<usize>| -> Array1<f64> {
            /* Compute P(Pi_R_i | R_Pi_R_i = 0) and P(Pi_R_i | R_i = 0, R_Pi_R_i = 0) */

            // Apply pairwise deletion.
            let d_pri_rpri = self.pw_deletion(pri);
            let d_pri_ri_rpri = self.pw_deletion(&(&set![ri] | pri));
            // Map the indices w.r.t. the new dataset.
            let x_pri_rpri = d_pri_rpri.indices_from(pri, self.labels());
            let x_pri_ri_rpri = d_pri_ri_rpri.indices_from(pri, self.labels());
            // Compute the distribution.
            let p_pri_rpri = BE::new(&d_pri_rpri).fit(&x_pri_rpri, &set![]);
            let p_pri_ri_rpri = BE::new(&d_pri_ri_rpri).fit(&x_pri_ri_rpri, &set![]);

            /* Compute the weights. */

            // Allocate the `R_i`-specific weights.
            let mut b_pri_rpri = Array::zeros(d_u.values().nrows());
            let mut b_pri_ri_rpri = b_pri_rpri.clone();
            // Fill the `R_i`-specific weights.
            Zip::from(d_u.values().rows())
                .and(b_pri_rpri.view_mut())
                .and(b_pri_ri_rpri.view_mut())
                .for_each(|d_u_j, b_pri_rpri_j, b_pri_ri_rpri_j| {
                    // Get the parents values for the j-th rows.
                    let pri_j = pri.iter().map(|&j| d_u_j[j]).collect();
                    // Get the parents weights associated to each row.
                    *b_pri_rpri_j = p_pri_rpri.pf(&pri_j, &array![]);
                    *b_pri_ri_rpri_j = p_pri_ri_rpri.pf(&pri_j, &array![]);
                });
            // Compute the `R_i`-specific weights.
            b_pri_rpri / b_pri_ri_rpri
        };

        // Compute the weights associated to each `R_i`.
        let pr = pr.map(|(ri, pri)| beta_i(d_u, ri, pri));
        // Compute the product of the weights associated to each `R_i`.
        let mut beta = pr.fold(
            // Fold the weights.
            Array::ones(d_u.values().nrows()),
            |mut beta, beta_i| {
                beta *= &beta_i;
                beta
            },
        );

        // Rescale the weights.
        beta *= (beta.len() as f64) / beta.sum();

        beta
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

    fn apply_missing_method(
        &self,
        m: &MM,
        x: Option<&Set<usize>>,
        pr: Option<&Map<usize, Set<usize>>>,
    ) -> Either<Self::Complete, Self::Weighted> {
        // Apply the missing method with the provided arguments.
        match (m, x, pr) {
            (MM::LW, _, _) => Either::Left(self.lw_deletion()),
            (MM::PW, Some(x), _) => Either::Left(self.pw_deletion(x)),
            (MM::IPW, Some(x), Some(pr)) => Either::Right(self.ipw_deletion(x, pr)),
            (MM::AIPW, Some(x), Some(pr)) => Either::Right(self.aipw_deletion(x, pr)),
            _ => panic!(
                "Invalid arguments for applying missing method:\n
                \t missing method:      '{m:?}' , \n\
                \t selected variables:  '{x:?}' , \n\
                \t missing mechanism:   '{pr:?}' ."
            ),
        }
    }

    fn lw_deletion(&self) -> Self::Complete {
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
            .zip(self.missing.missing_mask_by_rows())
            // Filter for complete rows only.
            .filter_map(|(row, &is_complete)| if !is_complete { Some(row) } else { None });

        // Fill new values with complete rows only.
        rows.zip(new_values.rows_mut())
            .for_each(|(row, mut new_row)| new_row.assign(&row));

        // Return new complete dataset.
        Self::Complete::new(self.states.clone(), new_values)
    }

    fn pw_deletion(&self, x: &Set<usize>) -> Self::Complete {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let s = states![];
            let v = Array::default((0, 0));
            return Self::Complete::new(s, v);
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

        // Return new complete dataset.
        Self::Complete::new(new_states, new_values)
    }

    fn ipw_deletion(&self, x: &Set<usize>, pr: &Map<usize, Set<usize>>) -> Self::Weighted {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let s = states![];
            let v = Array::default((0, 0));
            let w = Array::default(0);
            return Self::Weighted::new(Self::Complete::new(s, v), w);
        }

        // Assert that the indices are valid.
        x.iter().for_each(|&i| {
            assert!(
                i < self.values.ncols(),
                "Index out of bounds in IPW deletion: \n\
                \t expected:    index < |columns| , \n\
                \t found:       index == {} and |columns| == {} .",
                i,
                self.values.ncols()
            );
        });
        // Assert that the number of columns in the missing mechanism is valid.
        assert_eq!(
            pr.len(),
            self.values.ncols(),
            "Number of columns in the missing mechanism must be equal to the number of columns: \n\
            \t expected:    |missing_mechanism.keys()| == |columns| , \n\
            \t found:       |missing_mechanism.keys()| == {} and |columns| == {} .",
            pr.len(),
            self.values.ncols()
        );
        // Assert that the missing mechanism indices are valid.
        pr.keys().for_each(|&i| {
            assert!(
                i < self.values.ncols(),
                "Index out of bounds in IPW deletion missing mechanism: \n\
                \t expected:    index < |columns| , \n\
                \t found:       index == {} and |columns| == {} .",
                i,
                self.values.ncols()
            );
        });
        // Assert that the missing mechanism is sorted.
        assert!(
            pr.keys().is_sorted(),
            "Missing mechanism keys must be sorted."
        );
        assert!(
            pr.values().all(|pri| pri.iter().is_sorted()),
            "Missing mechanism values must be sorted."
        );

        // Compute U recursively from X and Pi_R following the IPW algorithm.
        let mut u = x.clone();
        let mut pru: Set<_> = x.iter().flat_map(|&x| &pr[x]).copied().collect();
        // Compute the transitive closure of the parents.
        while !pru.is_subset(&u) {
            u.extend(pru.drain(..));
            pru.extend(u.iter().flat_map(|&u| &pr[u]).copied());
        }
        // Sort U.
        u.sort();

        // Apply pairwise deletion.
        let d_u = self.pw_deletion(&u);
        // Compute the weights w.r.t. pairwise deleted dataset.
        let b_u = self.ipw_weights(&d_u, &u, pr);

        // Map the indices to the restricted dataset.
        let x = d_u.indices_from(x, self.labels());
        // Since U is a superset of X, restrict U to X.
        let d_x = d_u.select(&x);

        // Return new weighted dataset.
        Self::Weighted::new(d_x, b_u)
    }

    fn aipw_deletion(&self, x: &Set<usize>, pr: &Map<usize, Set<usize>>) -> Self::Weighted {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let s = states![];
            let v = Array::default((0, 0));
            let w = Array::default(0);
            return Self::Weighted::new(Self::Complete::new(s, v), w);
        }

        // Assert that the indices are valid.
        x.iter().for_each(|&i| {
            assert!(
                i < self.values.ncols(),
                "Index out of bounds in IPW deletion: \n\
                \t expected:    index < |columns| , \n\
                \t found:       index == {} and |columns| == {} .",
                i,
                self.values.ncols()
            );
        });
        // Assert that the number of columns in the missing mechanism is valid.
        assert_eq!(
            pr.len(),
            self.values.ncols(),
            "Number of columns in the missing mechanism must be equal to the number of columns: \n\
            \t expected:    |missing_mechanism.keys()| == |columns| , \n\
            \t found:       |missing_mechanism.keys()| == {} and |columns| == {} .",
            pr.len(),
            self.values.ncols()
        );
        // Assert that the missing mechanism indices are valid.
        pr.keys().for_each(|&i| {
            assert!(
                i < self.values.ncols(),
                "Index out of bounds in IPW deletion missing mechanism: \n\
                \t expected:    index < |columns| , \n\
                \t found:       index == {} and |columns| == {} .",
                i,
                self.values.ncols()
            );
        });
        // Assert that the missing mechanism is sorted.
        assert!(
            pr.keys().is_sorted(),
            "Missing mechanism keys must be sorted."
        );
        assert!(
            pr.values().all(|pri| pri.iter().is_sorted()),
            "Missing mechanism values must be sorted."
        );

        // Compute W recursively from X and Pi_R following the IPW algorithm.
        let mut w = x.clone();
        let prw: Set<_> = x.iter().flat_map(|&x| &pr[x]).copied().collect();
        // Sort W.
        w.sort();

        // Get the set of partially observed variables.
        let v_m = self.missing().partially_observed();
        // Check if the intersection of Pi_R_W and V_M is empty.
        if (&(&prw - &w) & v_m).is_empty() {
            return self.ipw_deletion(x, pr); // ... IPW.
        };

        // Otherwise, apply pairwise deletion w.r.t. X.
        let d_x = self.pw_deletion(x);
        let b_x = Array::ones(d_x.values().nrows()); // ... aIPW.
        // Return new weighted dataset.
        Self::Weighted::new(d_x, b_x)
    }
}

impl CsvIO for CatIncTable {
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
                    .zip(states.values_mut())
                    .map(|(x, states)| {
                        // Check if the value is missing.
                        if x.is_empty() {
                            return Self::MISSING;
                        }
                        // Insert the value into the states, if not present.
                        let (x, _) = states.insert_full(x.to_owned());
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

        // Create an empty string for missing values.
        let missing = String::new();

        // Write the records.
        self.values.rows().into_iter().for_each(|row| {
            // Zip the row with the states.
            let record = row.iter().zip(self.states().values());
            // Map the row values to states.
            let record = record.map(|(&x, states)| {
                // Check if the value is missing.
                if x == Self::MISSING {
                    return &missing;
                }
                // Return the state label.
                &states[x as usize]
            });
            // Write the record.
            writer
                .write_record(record)
                .expect("Failed to write CSV record.");
        });
    }
}
