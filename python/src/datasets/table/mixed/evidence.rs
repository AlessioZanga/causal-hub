use std::sync::{Arc, RwLock};

use backend::models::MixedEv;
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::{PyCatEv, PyGaussEv},
    impl_from_into_lock,
};

/// A unified evidence type for mixed Bayesian networks.
///
#[gen_stub_pyclass]
#[pyclass(name = "MixedEv", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyMixedEv {
    inner: Arc<RwLock<MixedEv>>,
}

impl_from_into_lock!(PyMixedEv, MixedEv);

#[gen_stub_pymethods]
#[pymethods]
impl PyMixedEv {
    /// Returns true if the evidence is categorical.
    ///
    pub fn is_categorical(&self) -> bool {
        matches!(*self.lock(), MixedEv::Categorical(_))
    }

    /// Returns true if the evidence is gaussian.
    ///
    pub fn is_gaussian(&self) -> bool {
        matches!(*self.lock(), MixedEv::Gaussian(_))
    }

    /// Returns the inner CatEv if the evidence is categorical.
    ///
    pub fn as_catev(&self) -> Option<PyCatEv> {
        match &*self.lock() {
            MixedEv::Categorical(ev) => Some(ev.clone().into()),
            _ => None,
        }
    }

    /// Returns the inner GaussEv if the evidence is gaussian.
    ///
    pub fn as_gaussev(&self) -> Option<PyGaussEv> {
        match &*self.lock() {
            MixedEv::Gaussian(ev) => Some(ev.clone().into()),
            _ => None,
        }
    }

    /// Creates a MixedEv from a CatEv.
    ///
    #[classmethod]
    pub fn from_catev(_cls: &Bound<'_, PyType>, ev: &PyCatEv) -> Self {
        MixedEv::Categorical(ev.lock().clone()).into()
    }

    /// Creates a MixedEv from a GaussEv.
    ///
    #[classmethod]
    pub fn from_gaussev(_cls: &Bound<'_, PyType>, ev: &PyGaussEv) -> Self {
        MixedEv::Gaussian(ev.lock().clone()).into()
    }

    /// Returns the string representation of the MixedEv.
    ///
    pub fn __repr__(&self) -> String {
        let variant = match &*self.lock() {
            MixedEv::Categorical(_) => "Categorical",
            MixedEv::Gaussian(_) => "Gaussian",
            _ => "Unknown",
        };
        format!("MixedEv({})", variant)
    }
}
