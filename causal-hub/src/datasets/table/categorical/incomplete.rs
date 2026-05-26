use std::{
    borrow::Cow,
    io::{Read, Write},
    sync::Arc,
};

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    datasets::{
        CatEv, CatEvT, CatTable, CatType, CatWtdTable, Dataset, IncDataset, MissingMechanism,
        MissingTable,
    },
    estimators::{BE, CPDEstimator},
    io::CsvIO,
    models::{CPD, CatSupport, Labelled},
    set, support,
    types::{Error, Labels, Result, Set},
};

/// A struct representing an incomplete categorical dataset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatIncTable {
    labels: Labels,
    support: CatSupport,
    shape: Array1<usize>,
    values: Array2<CatType>,
    missing: MissingTable,
}

/// Concrete iterator over incomplete categorical table evidences.
pub struct CatIncTableEvidenceIter<'a> {
    rows: ndarray::iter::LanesIter<'a, CatType, Ix1>,
    support: &'a CatSupport,
    missing: CatType,
}

impl<'a> Iterator for CatIncTableEvidenceIter<'a> {
    type Item = Result<CatEv>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.rows.next()?;

        let evidences = row.iter().enumerate().filter_map(|(event, &state)| {
            (state != self.missing).then_some(CatEvT::CertainPositive {
                event,
                state: state as usize,
            })
        });

        Some(CatEv::new(self.support.clone(), evidences))
    }
}

impl Labelled for CatIncTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CatIncTable {
    /// Creates a new categorical incomplete tabular data instance.
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
    /// A new categorical incomplete tabular data instance.
    ///
    pub fn new(mut support: CatSupport, mut values: Array2<CatType>) -> Result<Self> {
        // Check if the number of support is less than `CatType::MAX`.
        support.iter().try_for_each(|(label, state)| {
            if state.len() > CatType::MAX as usize {
                return Err(Error::InvalidParameter(
                    label,
                    &format!("should have less than 256 support, found {}", state.len()),
                ));
            }
            Ok(())
        })?;
        // Check if the number of variables is equal to the number of columns.
        if support.len() != values.ncols() {
            return Err(Error::IncompatibleShape(
                &support.len().to_string(),
                &values.ncols().to_string(),
            ));
        }
        // Check if the maximum value of the values is less than the number of support.
        let max_values = values.fold_axis(
            Axis(0),
            0,
            // Find max while ignoring missing values.
            |&a, &b| if a > b || b == Self::MISSING { a } else { b },
        );
        max_values.into_iter().enumerate().try_for_each(|(i, x)| {
            if x >= support[i].len() as CatType {
                return Err(Error::IndexOutOfBounds(x as usize));
            }
            Ok(())
        })?;

        // Check that the labels are sorted.
        if !support.keys().is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<usize> = (0..support.len()).collect();
            // Sort the indices by labels.
            indices.sort_by(|&i, &j| {
                support
                    .get_index(i)
                    .map(|(l, _)| l)
                    .cmp(&support.get_index(j).map(|(l, _)| l))
            });
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
                        // If the value is not missing ...
                        if *value != Self::MISSING {
                            // ... map it to the new state index.
                            *value = new_states
                                .get_index_of(&support[*value as usize])
                                .ok_or_else(|| Error::MissingState(&support[*value as usize]))?
                                as CatType;
                        }
                        Ok(())
                    })?;
                    // Update the support.
                    *support = new_states;
                }
                Ok(())
            })?;

        // Get the labels of the variables.
        let labels: Labels = support.keys().cloned().collect();
        // Get the shape of the support.
        let shape = support.values().map(Set::len).collect();

        // Create the missing mask.
        let missing_mask = values.mapv(|x| x == Self::MISSING);
        // Initialize the missing table.
        let missing = MissingTable::new(labels.clone(), missing_mask)?;

        Ok(Self {
            labels,
            support,
            shape,
            values,
            missing,
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

impl Dataset for CatIncTable {
    type Values = Array2<CatType>;
    type Support = CatSupport;
    type Evidence = CatEv;
    type EvidenceIter<'a> = CatIncTableEvidenceIter<'a>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.support)
    }

    fn evidence_iter(&self) -> Self::EvidenceIter<'_> {
        CatIncTableEvidenceIter {
            rows: self.values.rows().into_iter(),
            support: &self.support,
            missing: Self::MISSING,
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

impl IncDataset for CatIncTable {
    type Missing = CatType;
    const MISSING: Self::Missing = CatType::MAX;

    type Complete = CatTable;
    type Weighted = CatWtdTable;

    #[inline]
    fn missing(&self) -> &MissingTable {
        &self.missing
    }

    fn ipw_weights(
        &self,
        d_u: &Self::Complete,
        u: &Set<usize>,
        pr: &MissingMechanism,
    ) -> Result<Array1<f64>> {
        // Get (`R_i`, `Pi_R_i`) associated to `U_i`.
        let pr_iter = u.iter().filter_map(|&ri| pr.get(&ri).map(|pri| (ri, pri)));
        // Filter out `R_i` with no parents.
        let pr_iter = pr_iter.filter(|(_, pri)| !pri.is_empty());

        // Define function to compute the weights associated to each `R_i`.
        let beta_i = |d_u: &Self::Complete, ri: usize, pri: &Set<usize>| -> Result<Array1<f64>> {
            /* Compute P(Pi_R_i | R_Pi_R_i = 0) and P(Pi_R_i | R_i = 0, R_Pi_R_i = 0) */

            // Apply pairwise deletion.
            let d_pri_rpri = self.pw_deletion(pri)?;
            let d_pri_ri_rpri = self.pw_deletion(&(&set![ri] | pri))?;
            // Map the indices w.r.t. the new dataset.
            let x_pri_rpri = d_pri_rpri.indices_from(pri, self.labels())?;
            let x_pri_ri_rpri = d_pri_ri_rpri.indices_from(pri, self.labels())?;
            // Compute the distribution.
            let p_pri_rpri = BE::new(&d_pri_rpri).fit(&x_pri_rpri, &set![])?;
            let p_pri_ri_rpri = BE::new(&d_pri_ri_rpri).fit(&x_pri_ri_rpri, &set![])?;

            // Map indices of pri w.r.t d_u.
            let x_pri_u = d_u.indices_from(pri, self.labels())?;

            // Allocate the `R_i`-specific weights.
            let mut b_pri_rpri = Array::zeros(d_u.values().nrows());
            let mut b_pri_ri_rpri = b_pri_rpri.clone();
            // Fill the `R_i`-specific weights.
            for (d_u_j, (b_pri_rpri_j, b_pri_ri_rpri_j)) in d_u
                .values()
                .rows()
                .into_iter()
                .zip(b_pri_rpri.iter_mut().zip(b_pri_ri_rpri.iter_mut()))
            {
                // Get the parents values for the j-th rows.
                let pri_j = x_pri_u.iter().map(|&j| d_u_j[j]).collect();
                // Get the parents weights associated to each row.
                *b_pri_rpri_j = p_pri_rpri.pf(&pri_j, &array![])?;
                *b_pri_ri_rpri_j = p_pri_ri_rpri.pf(&pri_j, &array![])?;
            }
            // Compute the `R_i`-specific weights.
            Ok(b_pri_rpri / b_pri_ri_rpri)
        };

        // Compute the weights associated to each `R_i`.
        let mut beta = Array::ones(d_u.values().nrows());
        for (ri, pri) in pr_iter {
            let beta_i = beta_i(d_u, ri, pri)?;
            beta *= &beta_i;
        }

        // Rescale the weights.
        if beta.sum() > 0. {
            beta *= (beta.len() as f64) / beta.sum();
        }

        Ok(beta)
    }

    fn lw_deletion(&self) -> Result<Self::Complete> {
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
        Self::Complete::new(self.support.clone(), new_values)
    }

    fn pw_deletion(&self, x: &Set<usize>) -> Result<Self::Complete> {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let s = support![];
            let v = Array::default((0, 0));
            return Self::Complete::new(s, v);
        }

        // Check that the indices are valid.
        x.iter().try_for_each(|&i| {
            if i >= self.values.ncols() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;

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

        // Select the support for the specified columns.
        let new_states = cols
            .iter()
            .map(|&j| {
                self.support
                    .get_index(j)
                    .map(|(label, state)| (label.clone(), state.clone()))
                    .ok_or_else(|| Error::IndexOutOfBounds(j))
            })
            .collect::<Result<_>>()?;

        // Return new complete dataset.
        Self::Complete::new(new_states, new_values)
    }

    fn ipw_deletion(&self, x: &Set<usize>, pr: &MissingMechanism) -> Result<Self::Weighted> {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let s = support![];
            let v = Array::default((0, 0));
            let w = Array::default(0);
            return Self::Weighted::new(Self::Complete::new(s, v)?, w);
        }

        // Check that the indices are valid.
        x.iter().try_for_each(|&i| {
            if i >= self.values.ncols() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;
        // Check that the missing mechanism indices are valid.
        pr.keys().try_for_each(|&i| {
            if i >= self.values.ncols() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;
        // Check that the missing mechanism is sorted.
        if !pr.keys().is_sorted() {
            return Err(Error::InvalidParameter(
                "missing_mechanism",
                "keys must be sorted.",
            ));
        }
        if !pr.values().all(|pri| pri.iter().is_sorted()) {
            return Err(Error::InvalidParameter(
                "missing_mechanism",
                "values must be sorted.",
            ));
        }

        // Compute U recursively from X and Pi_R following the IPW algorithm.
        let mut u = x.clone();
        let mut pru: Set<_> = x
            .iter()
            .flat_map(|&x| pr.get(&x).cloned())
            .flatten()
            .collect();
        // Compute the transitive closure of the parents.
        while !pru.is_subset(&u) {
            u.extend(pru.drain(..));
            pru.extend(u.iter().flat_map(|&u| pr.get(&u).cloned()).flatten());
        }
        // Sort U.
        u.sort();

        // Apply pairwise deletion.
        let d_u = self.pw_deletion(&u)?;
        // Compute the weights w.r.t. pairwise deleted dataset.
        let b_u = self.ipw_weights(&d_u, &u, pr)?;

        // Map the indices to the restricted dataset.
        let x = d_u.indices_from(x, self.labels())?;
        // Since U is a superset of X, restrict U to X.
        let d_x = d_u.select(&x)?;

        // Return new weighted dataset.
        Self::Weighted::new(d_x, b_u)
    }

    fn aipw_deletion(&self, x: &Set<usize>, pr: &MissingMechanism) -> Result<Self::Weighted> {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let s = support![];
            let v = Array::default((0, 0));
            let w = Array::default(0);
            return Self::Weighted::new(Self::Complete::new(s, v)?, w);
        }

        // Check that the indices are valid.
        x.iter().try_for_each(|&i| {
            if i >= self.values.ncols() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;
        // Check that the missing mechanism indices are valid.
        pr.keys().try_for_each(|&i| {
            if i >= self.values.ncols() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;
        // Check that the missing mechanism is sorted.
        if !pr.keys().is_sorted() {
            return Err(Error::InvalidParameter(
                "missing_mechanism",
                "keys must be sorted.",
            ));
        }
        if !pr.values().all(|pri| pri.iter().is_sorted()) {
            return Err(Error::InvalidParameter(
                "missing_mechanism",
                "values must be sorted.",
            ));
        }

        // Compute W recursively from X and Pi_R following the IPW algorithm.
        let mut w = x.clone();
        let prw: Set<_> = x
            .iter()
            .flat_map(|x| pr.get(x).cloned())
            .flatten()
            .collect();
        // Sort W.
        w.sort();

        // Get the set of partially observed variables.
        let v_m = self.missing().partially_observed();
        // Check if the intersection of Pi_R_W and V_M is empty.
        if (&(&prw - &w) & v_m).is_empty() {
            return self.ipw_deletion(x, pr); // ... IPW.
        };

        // Otherwise, apply pairwise deletion w.r.t. X.
        let d_x = self.pw_deletion(x)?;
        let b_x = Array::ones(d_x.values().nrows()); // ... aIPW.
        // Return new weighted dataset.
        Self::Weighted::new(d_x, b_x)
    }
}

impl CsvIO for CatIncTable {
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
        let values: Vec<CatType> =
            reader
                .into_records()
                .try_fold(Vec::new(), |mut values, row| -> Result<_> {
                    // Get the record row.
                    let row = row.map_err(|e| Error::Csv(Arc::new(e)))?;
                    // Get the record values and convert to indices.
                    values.extend(
                        row.into_iter()
                            .zip(support.values_mut())
                            .map(|(x, support)| {
                                // Check if the value is missing.
                                if x.is_empty() {
                                    Self::MISSING
                                } else {
                                    // Insert the value into the support, if not present.
                                    let (x, _) = support.insert_full(x.to_owned());
                                    // Cast the value.
                                    x as CatType
                                }
                            }),
                    );

                    Ok(values)
                })?;

        // Get the number of rows and columns.
        let ncols = labels.len();
        let nrows = values.len() / ncols;
        // Reshape the values to the correct shape.
        let values = Array1::from_vec(values).into_shape_with_order((nrows, ncols))?;

        // Construct the dataset.
        Self::new(support, values)
    }

    fn to_csv_writer<W: Write>(&self, writer: W) -> Result<()> {
        // Create the CSV writer.
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        // Write the headers.
        writer.write_record(self.labels.iter())?;

        // Create an empty string for missing values.
        let missing = String::new();

        // Write the records.
        for row in self.values.rows() {
            // Zip the row with the support.
            let record = row.iter().zip(self.support().values());
            // Map the row values to support.
            let record = record.map(|(&x, support)| {
                // Check if the value is missing.
                if x == Self::MISSING {
                    return &missing;
                }
                // Return the state label.
                &support[x as usize]
            });
            // Write the record.
            writer.write_record(record)?;
        }

        Ok(())
    }
}
