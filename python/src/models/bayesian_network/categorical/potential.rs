use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    io::JsonIO,
    models::{CatPhi, HasLabels, Phi},
};
use numpy::{PyArrayDyn, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::{
    datasets::PyCatEv, error::to_pyerr, impl_from_into_lock, indices_from,
    models::bayesian_network::categorical::PyCatCPD,
};

/// A struct representing a categorical potential.
///
#[gen_stub_pyclass]
#[pyclass(name = "CatPhi", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCatPhi {
    inner: Arc<RwLock<CatPhi>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatPhi, CatPhi);

impl PartialEq for PyCatPhi {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCatPhi {
    /// Returns the labels of the potential.
    ///
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the support of the potential.
    ///
    pub fn support<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<String, Bound<'a, PyTuple>>> {
        self.lock()
            .support()
            .iter()
            .map(|(label, states)| {
                let label = label.clone();
                let states = PyTuple::new(py, states.iter().cloned())?;
                Ok((label, states))
            })
            .collect()
    }

    /// Returns the shape of the potential.
    ///
    pub fn shape(&self) -> PyResult<Vec<usize>> {
        Ok(self.lock().shape().to_vec())
    }

    /// Returns the parameters.
    ///
    pub fn parameters<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArrayDyn<f64>>> {
        Ok(self.lock().parameters().to_pyarray(py))
    }

    /// Returns the parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.lock().parameters_size())
    }

    /// Conditions the potential on observed evidence.
    ///
    pub fn condition(&self, evidence: &Bound<'_, PyAny>) -> PyResult<Self> {
        let support = self.lock().support().clone();
        let ev: backend::datasets::CatEv = PyCatEv::from_any(evidence, &support)?.into();
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

    /// Normalizes the potential so its entries sum to one.
    ///
    pub fn normalize(&self) -> PyResult<Self> {
        Phi::normalize(&*self.lock())
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Converts a CatCPD to a CatPhi.
    ///
    #[classmethod]
    pub fn from_cpd(_cls: &Bound<'_, PyType>, cpd: &PyCatCPD) -> PyResult<Self> {
        <CatPhi as Phi>::from_cpd(cpd.lock().clone())
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Converts the potential to a CatCPD.
    ///
    pub fn into_cpd(&self, x: &Bound<'_, PyAny>, z: &Bound<'_, PyAny>) -> PyResult<PyCatCPD> {
        let lock = self.lock();
        let x = indices_from!(x, lock)?;
        let z = indices_from!(z, lock)?;
        let inner = lock.clone();
        inner.into_cpd(&x, &z).map(Into::into).map_err(to_pyerr)
    }

    /// Returns the string representation of the CatPhi.
    ///
    pub fn __repr__(&self) -> PyResult<String> {
        let lock = self.lock();
        Ok(format!(
            "CatPhi(labels={:?})",
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
        CatPhi::from_json_string(json)
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
        CatPhi::from_json_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON file.
    ///
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path).map_err(to_pyerr)
    }
}
