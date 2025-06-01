use causal_hub::distributions::CatCIM;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::impl_deref_from_into;

/// A struct representing a categorical conditional intensity matrix (CIM).
#[pyclass(name = "CatCIM")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PyCatCIM {
    inner: CatCIM,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatCIM, CatCIM);

#[pymethods]
impl PyCatCIM {
    #[new]
    pub fn new() -> PyResult<Self> {
        todo!() // FIXME:
    }
}
