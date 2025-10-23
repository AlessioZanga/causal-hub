use std::sync::{Arc, RwLock};

use backend::{
    io::JsonIO,
    models::{CPD, GaussCPD, Labelled},
};
use numpy::prelude::*;
use pyo3::{
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A struct representing a Gaussian conditional probability distribution.
#[gen_stub_pyclass]
#[pyclass(name = "GaussCPD", module = "causal_hub.models", eq)]
#[derive(Clone, Debug)]
pub struct PyGaussCPD {
    inner: Arc<RwLock<GaussCPD>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyGaussCPD, GaussCPD);

impl PartialEq for PyGaussCPD {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussCPD {
    /// Returns the label of the conditioned variable.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the label.
    ///
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the labels of the conditioned variables.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the conditioning labels.
    ///
    pub fn conditioning_labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().conditioning_labels().iter().cloned().collect())
    }

    /// Returns the parameters.
    ///
    /// Returns
    /// -------
    /// dict[str, ...]
    ///     A reference to the parameters.
    ///
    pub fn parameters<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        // Allocate the dictionary.
        let dict = PyDict::new(py);
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the parameters.
        let parameters = lock.parameters();
        // Add the coefficients matrix.
        dict.set_item("coefficients", parameters.coefficients().to_pyarray(py))
            .expect("Failed to set coefficients.");
        // Add the intercept vector.
        dict.set_item("intercept", parameters.intercept().to_pyarray(py))
            .expect("Failed to set intercept.");
        // Add the covariance matrix.
        dict.set_item("covariance", parameters.covariance().to_pyarray(py))
            .expect("Failed to set covariance.");
        // Return the dictionary.
        Ok(dict)
    }

    /// Returns the parameters size.
    ///
    /// Returns
    /// -------
    /// int
    ///     The parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.lock().parameters_size())
    }

    /// Returns the sample statistics used to fit the distribution, if any.
    ///
    /// Returns
    /// -------
    /// dict[str, ...] | None
    ///     A dictionary containing the sample statistics used to fit the distribution, if any.
    ///
    pub fn sample_statistics<'a>(&self, py: Python<'a>) -> PyResult<Option<Bound<'a, PyDict>>> {
        Ok(self.lock().sample_statistics().map(|s| {
            // Allocate the dictionary.
            let dict = PyDict::new(py);
            // Add the response mean vector.
            dict.set_item(
                "sample_response_mean",
                s.sample_response_mean().to_pyarray(py),
            )
            .expect("Failed to set sample response mean.");
            // Add the design mean vector.
            dict.set_item("sample_design_mean", s.sample_design_mean().to_pyarray(py))
                .expect("Failed to set sample design mean.");
            // Add the response covariance matrix.
            dict.set_item(
                "sample_response_covariance",
                s.sample_response_covariance().to_pyarray(py),
            )
            .expect("Failed to set sample response covariance.");
            // Add the cross covariance matrix.
            dict.set_item(
                "sample_cross_covariance",
                s.sample_cross_covariance().to_pyarray(py),
            )
            .expect("Failed to set sample cross covariance.");
            // Add the design covariance matrix.
            dict.set_item(
                "sample_design_covariance",
                s.sample_design_covariance().to_pyarray(py),
            )
            .expect("Failed to set sample design covariance.");
            // Add the sample size.
            dict.set_item("sample_size", s.sample_size())
                .expect("Failed to set sample size.");
            // Return the dictionary.
            dict
        }))
    }

    /// Returns the sample log-likelihood given the distribution, if any.
    ///
    /// Returns
    /// -------
    /// float | None
    ///     The sample log-likelihood given the distribution, if any.
    ///
    pub fn sample_log_likelihood(&self) -> PyResult<Option<f64>> {
        Ok(self.lock().sample_log_likelihood())
    }

    /// Read instance from a JSON string.
    ///
    /// Parameters
    /// ----------
    /// json: str
    ///     The JSON string to read from.
    ///
    /// Returns
    /// -------
    /// GaussCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(GaussCPD::from_json(json))),
        })
    }

    /// Write instance to a JSON string.
    ///
    /// Returns
    /// -------
    /// str
    ///     A JSON string representation of the instance.
    ///
    pub fn to_json(&self) -> PyResult<String> {
        Ok(self.lock().to_json())
    }

    /// Read instance from a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to read from.
    ///
    /// Returns
    /// -------
    /// GaussCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn read_json(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(GaussCPD::read_json(path))),
        })
    }

    /// Write instance to a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to write to.
    ///
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.lock().write_json(path);
        Ok(())
    }
}
