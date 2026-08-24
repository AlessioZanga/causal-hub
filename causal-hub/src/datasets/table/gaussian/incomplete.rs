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
        Dataset, GaussEv, GaussEvT, GaussTable, GaussType, GaussWtdTable, IncDataset,
        MissingMechanism, MissingTable,
    },
    estimators::{BE, CPDEstimator},
    io::CsvIO,
    labels,
    models::{CPD, GaussSupport, HasLabels},
    set,
    types::{Error, Labels, Result, Set},
};

/// A struct representing an incomplete gaussian dataset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GaussIncTable {
    labels: Labels,
    values: Array2<GaussType>,
    missing: MissingTable,
}

/// Concrete iterator over incomplete Gaussian table evidences.
pub struct GaussIncTableEvidenceIter<'a> {
    rows: ndarray::iter::LanesIter<'a, GaussType, Ix1>,
    labels: &'a Labels,
}

impl<'a> Iterator for GaussIncTableEvidenceIter<'a> {
    type Item = Result<GaussEv>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.rows.next()?;

        let evidences = row.iter().enumerate().filter_map(|(event, &value)| {
            (!value.is_nan()).then_some(GaussEvT::CertainPositive { event, value })
        });

        Some(GaussEv::new(self.labels.clone(), evidences))
    }
}

impl HasLabels for GaussIncTable {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl GaussIncTable {
    /// Creates a new gaussian incomplete tabular data instance.
    pub fn new(mut labels: Labels, mut values: Array2<GaussType>) -> Result<Self> {
        // Check if the number of variables is equal to the number of columns.
        if labels.len() != values.ncols() {
            return Err(Error::IncompatibleShape(
                &labels.len().to_string(),
                &values.ncols().to_string(),
            ));
        }

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
        let missing = MissingTable::new(labels.clone(), missing_mask)?;

        Ok(Self {
            labels,
            values,
            missing,
        })
    }
}

impl Dataset for GaussIncTable {
    type Values = Array2<GaussType>;
    type Support = GaussSupport;
    type Evidence = GaussEv;
    type EvidenceIter<'a> = GaussIncTableEvidenceIter<'a>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Owned(
            self.labels
                .iter()
                .map(|l| (l.clone(), (f64::NEG_INFINITY, f64::INFINITY)))
                .collect(),
        )
    }

    fn evidence_iter(&self) -> Self::EvidenceIter<'_> {
        GaussIncTableEvidenceIter {
            rows: self.values.rows().into_iter(),
            labels: &self.labels,
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

        // Select the labels.
        let labels: Labels = x
            .iter()
            .map(|&i| {
                self.labels
                    .get_index(i)
                    .cloned()
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
            .filter(|row| row.iter().all(|&x| !x.is_nan()));

        // Filter valid rows.
        new_values
            .rows_mut()
            .into_iter()
            .zip(rows)
            .for_each(|(mut new_row, row)| new_row.assign(&row));

        // Return the complete dataset.
        Self::Complete::new(self.labels.clone(), new_values)
    }

    fn pw_deletion(&self, x: &Set<usize>) -> Result<Self::Complete> {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let stats = labels![];
            let v = Array::default((0, 0));
            return GaussTable::new(stats, v);
        }

        // Check that the indices are valid.
        x.iter().try_for_each(|&i| {
            if i >= self.values.ncols() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;

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
            .map(|&j| {
                self.labels
                    .get_index(j)
                    .cloned()
                    .ok_or_else(|| Error::IndexOutOfBounds(j))
            })
            .collect::<Result<_>>()?;

        // Return the complete dataset.
        Self::Complete::new(new_labels, new_values)
    }

    fn ipw_deletion(&self, x: &Set<usize>, pr: &MissingMechanism) -> Result<Self::Weighted> {
        // If no columns are specified, return an empty dataset.
        if x.is_empty() {
            let stats = labels![];
            let v = Array::default((0, 0));
            let w = Array::default(0);
            return Self::Weighted::new(Self::Complete::new(stats, v)?, w);
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
            let l = labels![];
            let v = Array::default((0, 0));
            let w = Array::default(0);
            return Self::Weighted::new(Self::Complete::new(l, v)?, w);
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

impl CsvIO for GaussIncTable {
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

        // Read the records.
        let values: Vec<GaussType> =
            reader
                .into_records()
                .try_fold(Vec::new(), |mut values, row| -> Result<_> {
                    // Get the record row.
                    let row = row.map_err(|evidence| Error::Csv(Arc::new(evidence)))?;
                    // Extend the values.
                    values.extend(
                        row.iter()
                            .map(|x| x.parse::<GaussType>().unwrap_or(Self::MISSING)),
                    );
                    Ok(values)
                })?;

        // Convert values to an array.
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
