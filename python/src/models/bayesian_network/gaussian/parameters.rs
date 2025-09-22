use backend::{
    io::JsonIO,
    models::{CPD, GaussCPD, Labelled},
};
use numpy::{PyArray2, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_deref_from_into;

/// A struct representing a Gaussian conditional probability distribution.
#[gen_stub_pyclass]
#[pyclass(name = "GaussCPD", eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyGaussCPD {
    inner: GaussCPD,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyGaussCPD, GaussCPD);

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussCPD {
    /// Returns the label of the conditioned variable.
    ///
    /// # Returns
    ///
    /// A reference to the label.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the labels of the conditioned variables.
    ///
    /// # Returns
    ///
    /// A reference to the conditioning labels.
    ///
    pub fn conditioning_labels(&self) -> PyResult<Vec<&str>> {
        Ok(self
            .inner
            .conditioning_labels()
            .iter()
            .map(AsRef::as_ref)
            .collect())
    }

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    pub fn parameters<'a>(&'a self, py: Python<'a>) -> PyResult<()> {
        todo!() // FIXME:
    }

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        todo!() // FIXME:
    }

    /// Returns the sample statistics used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// A dictionary containing the sample statistics used to fit the distribution, if any.
    ///
    pub fn sample_statistics<'a>(&self, py: Python<'a>) -> PyResult<Option<Bound<'a, PyDict>>> {
        Ok(self.inner.sample_statistics().map(|sample_statistics| {
            // Allocate the dictionary.
            let stats = PyDict::new(py);
            
            todo!(); // FIXME:

            // Return the dictionary.
            stats
        }))
    }

    /// Returns the sample log-likelihood given the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample log-likelihood given the distribution.
    ///
    pub fn sample_log_likelihood(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.sample_log_likelihood())
    }

    /// Read class from a JSON string.
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: GaussCPD::from_json(json),
        })
    }

    /// Write class to a JSON string.
    pub fn to_json(&self) -> PyResult<String> {
        Ok(self.inner.to_json())
    }

    /// Read class from a JSON file.
    #[classmethod]
    pub fn read_json(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: GaussCPD::read_json(path),
        })
    }

    /// Write class to a JSON file.
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.inner.write_json(path);
        Ok(())
    }
}
