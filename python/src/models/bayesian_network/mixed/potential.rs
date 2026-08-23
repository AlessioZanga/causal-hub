use std::sync::{Arc, RwLock};

use backend::{
    datasets::{CatEv, GaussEv},
    io::JsonIO,
    models::{CatPhi, GaussPhi, Labelled, MixedPhi, Phi},
};
use pyo3::{exceptions::PyTypeError, prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::{PyCatEv, PyGaussEv},
    error::to_pyerr,
    impl_from_into_lock, indices_from,
    models::{PyCatCPD, PyCatPhi, PyGaussCPD, PyGaussPhi},
};

/// A unified potential for mixed Bayesian networks.
///
#[gen_stub_pyclass]
#[pyclass(name = "MixedPhi", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyMixedPhi {
    inner: Arc<RwLock<MixedPhi>>,
}

impl_from_into_lock!(PyMixedPhi, MixedPhi);

impl PartialEq for PyMixedPhi {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMixedPhi {
    /// Returns true if the potential is categorical.
    ///
    pub fn is_categorical(&self) -> bool {
        matches!(*self.lock(), MixedPhi::Categorical(_))
    }

    /// Returns true if the potential is Gaussian.
    ///
    pub fn is_gaussian(&self) -> bool {
        matches!(*self.lock(), MixedPhi::Gaussian(_))
    }

    /// Returns the inner CatPhi if the potential is categorical.
    ///
    pub fn as_catphi(&self) -> PyResult<Option<PyCatPhi>> {
        match &*self.lock() {
            MixedPhi::Categorical(potential) => Ok(Some(potential.clone().into())),
            _ => Ok(None),
        }
    }

    /// Returns the inner GaussPhi if the potential is Gaussian.
    ///
    pub fn as_gaussphi(&self) -> PyResult<Option<PyGaussPhi>> {
        match &*self.lock() {
            MixedPhi::Gaussian(potential) => Ok(Some(potential.clone().into())),
            _ => Ok(None),
        }
    }

    /// Returns the labels of the potential.
    ///
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.lock().parameters_size())
    }

    /// Conditions the potential on observed evidence.
    ///
    pub fn condition(&self, evidence: &Bound<'_, PyAny>) -> PyResult<Self> {
        let lock = self.lock();
        match &*lock {
            MixedPhi::Categorical(potential) => {
                let support = potential.support().clone();
                let ev: CatEv = PyCatEv::from_any(evidence, &support)?.into();
                potential
                    .condition(&ev)
                    .map(MixedPhi::Categorical)
                    .map(Into::into)
                    .map_err(to_pyerr)
            }
            MixedPhi::Gaussian(potential) => {
                let labels = potential.labels().clone();
                let ev: GaussEv = PyGaussEv::from_any(evidence, &labels)?.into();
                potential
                    .condition(&ev)
                    .map(MixedPhi::Gaussian)
                    .map(Into::into)
                    .map_err(to_pyerr)
            }
            _ => Err(PyTypeError::new_err(
                "Unexpected MixedPhi potential variant.",
            )),
        }
    }

    /// Marginalizes the potential over a set of variables.
    ///
    pub fn marginalize(&self, x: &Bound<'_, PyAny>) -> PyResult<Self> {
        let lock = self.lock();
        let x = indices_from!(x, &*lock)?;
        Phi::marginalize(&*lock, &x)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Normalizes the potential.
    ///
    pub fn normalize(&self) -> PyResult<Self> {
        Phi::normalize(&*self.lock())
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Creates a MixedPhi from a CatCPD.
    ///
    #[classmethod]
    pub fn from_cat_cpd(_cls: &Bound<'_, PyType>, cpd: &PyCatCPD) -> PyResult<Self> {
        CatPhi::from_cpd(cpd.lock().clone())
            .map(MixedPhi::Categorical)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Creates a MixedPhi from a GaussCPD.
    ///
    #[classmethod]
    pub fn from_gauss_cpd(_cls: &Bound<'_, PyType>, cpd: &PyGaussCPD) -> PyResult<Self> {
        GaussPhi::from_cpd(cpd.lock().clone())
            .map(MixedPhi::Gaussian)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Converts to a CatCPD if the potential is categorical.
    ///
    pub fn into_cat_cpd(&self, x: &Bound<'_, PyAny>, z: &Bound<'_, PyAny>) -> PyResult<PyCatCPD> {
        let lock = self.lock();
        let x = indices_from!(x, &*lock)?;
        let z = indices_from!(z, &*lock)?;
        let inner = lock.clone();
        match inner {
            MixedPhi::Categorical(potential) => {
                potential.into_cpd(&x, &z).map(Into::into).map_err(to_pyerr)
            }
            _ => Err(PyTypeError::new_err(
                "The MixedPhi potential is not categorical.",
            )),
        }
    }

    /// Converts to a GaussCPD if the potential is Gaussian.
    ///
    pub fn into_gauss_cpd(
        &self,
        x: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
    ) -> PyResult<PyGaussCPD> {
        let lock = self.lock();
        let x = indices_from!(x, &*lock)?;
        let z = indices_from!(z, &*lock)?;
        let inner = lock.clone();
        match inner {
            MixedPhi::Gaussian(potential) => {
                potential.into_cpd(&x, &z).map(Into::into).map_err(to_pyerr)
            }
            _ => Err(PyTypeError::new_err(
                "The MixedPhi potential is not Gaussian.",
            )),
        }
    }

    /// Returns the string representation of the MixedPhi.
    ///
    pub fn __repr__(&self) -> PyResult<String> {
        let lock = self.lock();
        let variant = match &*lock {
            MixedPhi::Categorical(_) => "Categorical",
            MixedPhi::Gaussian(_) => "Gaussian",
            _ => "Unknown",
        };
        Ok(format!(
            "MixedPhi({}, labels={:?})",
            variant,
            lock.labels().iter().cloned().collect::<Vec<_>>()
        ))
    }

    /// Multiplies two potentials.
    ///
    pub fn __mul__(&self, other: &Self) -> PyResult<Self> {
        let product = &*self.lock() * &*other.lock();
        Ok(product.into())
    }

    /// In-place multiplication of two potentials.
    ///
    pub fn __imul__(&mut self, other: &Self) -> PyResult<()> {
        let mut lock = self.lock_mut();
        let other = other.lock().clone();
        *lock *= &other;
        Ok(())
    }

    /// Divides two potentials.
    ///
    pub fn __truediv__(&self, other: &Self) -> PyResult<Self> {
        let quotient = &*self.lock() / &*other.lock();
        Ok(quotient.into())
    }

    /// In-place division of two potentials.
    ///
    pub fn __idiv__(&mut self, other: &Self) -> PyResult<()> {
        let mut lock = self.lock_mut();
        let other = other.lock().clone();
        *lock /= &other;
        Ok(())
    }

    /// Read instance from a JSON string.
    ///
    #[classmethod]
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        MixedPhi::from_json_string(json)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON string.
    ///
    pub fn to_json_string(&self) -> PyResult<String> {
        self.lock().to_json_string().map_err(to_pyerr)
    }

    /// Read instance from a JSON file.
    ///
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        MixedPhi::from_json_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON file.
    ///
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path).map_err(to_pyerr)
    }
}
