use std::collections::BTreeMap;

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

use crate::impl_deref_from_into;

/// A struct representing a categorical conditional probability distribution.
#[gen_stub_pyclass]
#[pyclass(name = "CatCPD", module = "causal_hub.models", eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyCatCPD {
    inner: CatCPD,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatCPD, CatCPD);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatCPD {
    /// Returns the label of the conditioned variable.
    ///
    /// # Returns
    ///
    /// A reference to the label.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the states of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The states of the conditioned variable.
    ///
    pub fn states<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<&'a str, Bound<'a, PyTuple>>> {
        Ok(self
            .inner
            .states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.as_ref();
                let states = states.iter().map(String::as_str);
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }

    /// Returns the shape of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The shape of the conditioned variable.
    ///
    pub fn shape(&self) -> PyResult<Vec<usize>> {
        Ok(self.inner.shape().to_vec())
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

    /// Returns the states of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The states of the conditioning variables.
    ///
    pub fn conditioning_states<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<BTreeMap<&'a str, Bound<'a, PyTuple>>> {
        Ok(self
            .inner
            .conditioning_states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.as_ref();
                let states = states.iter().map(String::as_str);
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }

    /// Returns the shape of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The shape of the conditioning variables.
    ///
    pub fn conditioning_shape(&self) -> PyResult<Vec<usize>> {
        Ok(self.inner.conditioning_shape().to_vec())
    }

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    pub fn parameters<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self.inner.parameters().to_pyarray(py))
    }

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.inner.parameters_size())
    }

    /// Returns the sample statistics used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// A dictionary containing the sample statistics used to fit the distribution, if any.
    ///
    pub fn sample_statistics<'a>(&self, py: Python<'a>) -> PyResult<Option<Bound<'a, PyDict>>> {
        Ok(self.inner.sample_statistics().map(|s| {
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
    /// # Returns
    ///
    /// The sample log-likelihood given the distribution.
    ///
    pub fn sample_log_likelihood(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.sample_log_likelihood())
    }

    /// Returns the string representation of the CatCPD.
    pub fn __repr__(&self) -> PyResult<String> {
        // Get the string representation of the CatCPD.
        Ok(self.inner.to_string())
    }

    /// Read class from a JSON string.
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: CatCPD::from_json(json),
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
            inner: CatCPD::read_json(path),
        })
    }

    /// Write class to a JSON file.
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.inner.write_json(path);
        Ok(())
    }
}
