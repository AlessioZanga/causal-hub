use ndarray::prelude::*;

use crate::{
    datasets::{CatTable, CatType, Dataset, IncDataset, MissingTable},
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
    type Complete = CatTable;
    type Missing = CatType;

    const MISSING: Self::Missing = CatType::MAX;

    #[inline]
    fn missing(&self) -> &MissingTable {
        &self.missing
    }

    fn lw_deletion(&self) -> Self::Complete {
        todo!() // FIXME:
    }

    fn pw_deletion(&self, _x: &Set<usize>) -> Self {
        todo!() // FIXME:
    }

    fn ipw_deletion(&self, _x: &Set<usize>) -> Self {
        todo!() // FIXME:
    }

    fn aipw_deletion(&self, _x: &Set<usize>) -> Self {
        todo!() // FIXME:
    }

    fn into_complete(self) -> Self::Complete {
        todo!() // FIXME:
    }
}
