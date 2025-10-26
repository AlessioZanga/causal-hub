use ndarray::prelude::*;

use crate::{
    datasets::{CatTable, CatType, CatWtdTable, Dataset, IncDataset, MissingTable},
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
    pub fn new(_states: States, _values: Array2<CatType>) -> Self {
        todo!() // FIXME:
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
        CatTable::new(self.states.clone(), new_values)
    }

    fn pw_deletion(&self, _x: &Set<usize>) -> Self::Complete {
        todo!() // FIXME:
    }

    fn ipw_deletion(&self, _x: &Set<usize>) -> Self::Weighted {
        todo!() // FIXME:
    }

    fn aipw_deletion(&self, _x: &Set<usize>) -> Self::Weighted {
        todo!() // FIXME:
    }
}
