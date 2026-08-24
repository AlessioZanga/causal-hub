use std::sync::{Arc, RwLock};

use backend::models::{CPD, HasLabels, MixedCPD};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;

use crate::{
    impl_from_into_lock,
    models::{PyCatCPD, PyGaussCPD},
};

/// A unified CPD type for mixed Bayesian networks.
///
#[gen_stub_pyclass]
#[pyclass(name = "MixedCPD", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyMixedCPD {
    inner: Arc<RwLock<MixedCPD>>,
}

impl_from_into_lock!(PyMixedCPD, MixedCPD);

impl PartialEq for PyMixedCPD {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMixedCPD {
    /// Returns true if the CPD is categorical.
    ///
    pub fn is_categorical(&self) -> bool {
        matches!(*self.lock(), MixedCPD::Categorical(_))
    }

    /// Returns true if the CPD is gaussian.
    ///
    pub fn is_gaussian(&self) -> bool {
        matches!(*self.lock(), MixedCPD::Gaussian(_))
    }

    /// Returns the inner CatCPD if the CPD is categorical.
    ///
    pub fn as_catcpd(&self) -> Option<PyCatCPD> {
        match &*self.lock() {
            MixedCPD::Categorical(cpd) => Some(cpd.clone().into()),
            _ => None,
        }
    }

    /// Returns the inner GaussCPD if the CPD is gaussian.
    ///
    pub fn as_gausscpd(&self) -> Option<PyGaussCPD> {
        match &*self.lock() {
            MixedCPD::Gaussian(cpd) => Some(cpd.clone().into()),
            _ => None,
        }
    }

    /// Creates a MixedCPD from a CatCPD.
    ///
    #[classmethod]
    pub fn from_catcpd(_cls: &Bound<'_, PyType>, cpd: &PyCatCPD) -> Self {
        MixedCPD::Categorical(cpd.lock().clone()).into()
    }

    /// Creates a MixedCPD from a GaussCPD.
    ///
    #[classmethod]
    pub fn from_gausscpd(_cls: &Bound<'_, PyType>, cpd: &PyGaussCPD) -> Self {
        MixedCPD::Gaussian(cpd.lock().clone()).into()
    }

    /// Returns the labels of the CPD.
    ///
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the conditioning labels of the CPD.
    ///
    pub fn conditioning_labels(&self) -> PyResult<Vec<String>> {
        Ok(CPD::conditioning_labels(&*self.lock())
            .iter()
            .cloned()
            .collect())
    }

    /// Returns the parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(CPD::parameters_size(&*self.lock()))
    }

    /// Returns the string representation of the MixedCPD.
    ///
    pub fn __repr__(&self) -> String {
        let variant = match &*self.lock() {
            MixedCPD::Categorical(_) => "Categorical",
            MixedCPD::Gaussian(_) => "Gaussian",
            _ => "Unknown",
        };
        format!("MixedCPD({})", variant)
    }
}
