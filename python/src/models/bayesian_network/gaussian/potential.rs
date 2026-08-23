use std::sync::{Arc, RwLock};

use backend::{
    io::JsonIO,
    models::{GaussPhi, Labelled, Phi},
};
use numpy::{PyArray1, PyArray2, prelude::*};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::PyGaussEv, error::to_pyerr, impl_from_into_lock, indices_from,
    models::bayesian_network::gaussian::PyGaussCPD,
};

/// A struct representing a Gaussian potential (information form).
///
#[gen_stub_pyclass]
#[pyclass(name = "GaussPhi", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyGaussPhi {
    inner: Arc<RwLock<GaussPhi>>,
}

impl_from_into_lock!(PyGaussPhi, GaussPhi);

impl PartialEq for PyGaussPhi {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussPhi {
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
        let labels = self.lock().labels().clone();
        let ev: backend::datasets::GaussEv = PyGaussEv::from_any(evidence, &labels)?.into();
        Phi::condition(&*self.lock(), &ev)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Marginalizes the potential over a set of variables.
    ///
    pub fn marginalize(&self, x: &Bound<'_, PyAny>) -> PyResult<Self> {
        let lock = self.lock();
        let x = indices_from!(x, lock)?;
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

    /// Returns the precision matrix of the potential.
    ///
    pub fn precision_matrix<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self.lock().parameters().precision_matrix().to_pyarray(py))
    }

    /// Returns the information vector of the potential.
    ///
    pub fn information_vector<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray1<f64>>> {
        Ok(self.lock().parameters().information_vector().to_pyarray(py))
    }

    /// Returns the log normalization constant of the potential.
    ///
    pub fn log_normalization_constant(&self) -> PyResult<f64> {
        Ok(self.lock().parameters().log_normalization_constant())
    }

    /// Converts a GaussCPD to a GaussPhi.
    ///
    #[classmethod]
    pub fn from_cpd(_cls: &Bound<'_, PyType>, cpd: &PyGaussCPD) -> PyResult<Self> {
        <GaussPhi as Phi>::from_cpd(cpd.lock().clone())
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Converts the potential to a GaussCPD.
    ///
    pub fn into_cpd(&self, x: &Bound<'_, PyAny>, z: &Bound<'_, PyAny>) -> PyResult<PyGaussCPD> {
        let lock = self.lock();
        let x = indices_from!(x, lock)?;
        let z = indices_from!(z, lock)?;
        let inner = lock.clone();
        inner.into_cpd(&x, &z).map(Into::into).map_err(to_pyerr)
    }

    /// Returns the string representation of the GaussPhi.
    ///
    pub fn __repr__(&self) -> PyResult<String> {
        let lock = self.lock();
        Ok(format!(
            "GaussPhi(labels={:?})",
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
        GaussPhi::from_json_string(json)
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
        GaussPhi::from_json_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON file.
    ///
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path).map_err(to_pyerr)
    }
}
