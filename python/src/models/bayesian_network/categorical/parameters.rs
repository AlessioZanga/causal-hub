use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    io::JsonIO,
    models::{CPD, CatCPD, Labelled},
};
use numpy::{PyArray2, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A struct representing a categorical conditional probability distribution.
#[gen_stub_pyclass]
#[pyclass(name = "CatCPD", module = "causal_hub.models", eq)]
#[derive(Clone, Debug)]
pub struct PyCatCPD {
    inner: Arc<RwLock<CatCPD>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatCPD, CatCPD);

impl PartialEq for PyCatCPD {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCatCPD {
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

    /// Returns the states of the conditioned variable.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     The states of the conditioned variable.
    ///
    pub fn states<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<String, Bound<'a, PyTuple>>> {
        Ok(self
            .lock()
            .states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.clone();
                let states = states.iter().cloned();
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }

    /// Returns the shape of the conditioned variable.
    ///
    /// Returns
    /// -------
    /// list[int]
    ///     The shape of the conditioned variable.
    ///
    pub fn shape(&self) -> PyResult<Vec<usize>> {
        Ok(self.lock().shape().to_vec())
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

    /// Returns the states of the conditioning variables.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     The states of the conditioning variables.
    ///
    pub fn conditioning_states<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<BTreeMap<String, Bound<'a, PyTuple>>> {
        Ok(self
            .lock()
            .conditioning_states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.clone();
                let states = states.iter().cloned();
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }

    /// Returns the shape of the conditioning variables.
    ///
    /// Returns
    /// -------
    /// list[int]
    ///     The shape of the conditioning variables.
    ///
    pub fn conditioning_shape(&self) -> PyResult<Vec<usize>> {
        Ok(self.lock().conditioning_shape().to_vec())
    }

    /// Returns the parameters.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A reference to the parameters.
    ///
    pub fn parameters<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self.lock().parameters().to_pyarray(py))
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
            // Add the conditional counts.
            dict.set_item(
                "sample_conditional_counts",
                s.sample_conditional_counts().to_pyarray(py),
            )
            .expect("Failed to set sample conditional counts.");
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

    /// Returns the string representation of the CatCPD.
    pub fn __repr__(&self) -> PyResult<String> {
        // Get the string representation of the CatCPD.
        Ok(self.lock().to_string())
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
    /// CatCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(CatCPD::from_json(json))),
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
    /// CatCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn read_json(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RwLock::new(CatCPD::read_json(path))),
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
