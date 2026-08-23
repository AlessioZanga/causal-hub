use std::sync::{Arc, RwLock};

use backend::models::{MixedIncTable, MixedTable};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::{PyCatTable, PyGaussTable},
    impl_from_into_lock,
};

/// A unified complete dataset type for mixed Bayesian networks.
///
#[gen_stub_pyclass]
#[pyclass(name = "MixedTable", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyMixedTable {
    inner: Arc<RwLock<MixedTable>>,
}

impl_from_into_lock!(PyMixedTable, MixedTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyMixedTable {
    /// Returns true if the dataset is categorical.
    ///
    pub fn is_categorical(&self) -> bool {
        matches!(*self.lock(), MixedTable::Categorical(_))
    }

    /// Returns true if the dataset is gaussian.
    ///
    pub fn is_gaussian(&self) -> bool {
        matches!(*self.lock(), MixedTable::Gaussian(_))
    }

    /// Returns the inner CatTable if the dataset is categorical.
    ///
    pub fn as_cattable(&self) -> Option<PyCatTable> {
        match &*self.lock() {
            MixedTable::Categorical(t) => Some(t.clone().into()),
            _ => None,
        }
    }

    /// Returns the inner GaussTable if the dataset is gaussian.
    ///
    pub fn as_gausstable(&self) -> Option<PyGaussTable> {
        match &*self.lock() {
            MixedTable::Gaussian(t) => Some(t.clone().into()),
            _ => None,
        }
    }

    /// Creates a MixedTable from a CatTable.
    ///
    #[classmethod]
    pub fn from_cattable(_cls: &Bound<'_, PyType>, table: &PyCatTable) -> Self {
        MixedTable::Categorical(table.lock().clone()).into()
    }

    /// Creates a MixedTable from a GaussTable.
    ///
    #[classmethod]
    pub fn from_gausstable(_cls: &Bound<'_, PyType>, table: &PyGaussTable) -> Self {
        MixedTable::Gaussian(table.lock().clone()).into()
    }

    /// Returns the string representation of the MixedTable.
    ///
    pub fn __repr__(&self) -> String {
        let variant = match &*self.lock() {
            MixedTable::Categorical(_) => "Categorical",
            MixedTable::Gaussian(_) => "Gaussian",
            _ => "Unknown",
        };
        format!("MixedTable({})", variant)
    }
}

/// A unified incomplete dataset type for mixed Bayesian networks.
///
#[gen_stub_pyclass]
#[pyclass(name = "MixedIncTable", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyMixedIncTable {
    inner: Arc<RwLock<MixedIncTable>>,
}

impl_from_into_lock!(PyMixedIncTable, MixedIncTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyMixedIncTable {
    /// Returns true if the dataset is categorical.
    ///
    pub fn is_categorical(&self) -> bool {
        matches!(*self.lock(), MixedIncTable::Categorical(_))
    }

    /// Returns true if the dataset is gaussian.
    ///
    pub fn is_gaussian(&self) -> bool {
        matches!(*self.lock(), MixedIncTable::Gaussian(_))
    }

    /// Returns the string representation of the MixedIncTable.
    ///
    pub fn __repr__(&self) -> String {
        let variant = match &*self.lock() {
            MixedIncTable::Categorical(_) => "Categorical",
            MixedIncTable::Gaussian(_) => "Gaussian",
            _ => "Unknown",
        };
        format!("MixedIncTable({})", variant)
    }
}
