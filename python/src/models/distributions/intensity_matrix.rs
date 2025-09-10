use std::collections::BTreeMap;

use causal_hub_rust::{
    io::JsonIO,
    models::{CPD, CatCIM},
};
use numpy::{PyArray3, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_deref_from_into;

/// A struct representing a categorical conditional intensity matrix (CIM).
#[gen_stub_pyclass]
#[pyclass(name = "CatCIM", eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyCatCIM {
    inner: CatCIM,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatCIM, CatCIM);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatCIM {
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
    pub fn parameters<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray3<f64>>> {
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

    /// Returns the sample conditional counts used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample conditional counts used to fit the distribution, if any.
    ///
    pub fn sample_conditional_counts<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<Option<Bound<'a, PyArray3<f64>>>> {
        Ok(self
            .inner
            .sample_conditional_counts()
            .map(|counts| counts.to_pyarray(py)))
    }

    /// Returns the sample conditional times used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample conditional times used to fit the distribution, if any.
    ///
    pub fn sample_conditional_times<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<Option<Bound<'a, PyArray3<f64>>>> {
        Ok(self
            .inner
            .sample_conditional_times()
            .map(|times| times.to_pyarray(py)))
    }

    /// Returns the sample size used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample size used to fit the distribution.
    ///
    pub fn sample_size(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.sample_size())
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
            inner: CatCIM::from_json(json),
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
            inner: CatCIM::read_json(path),
        })
    }

    /// Write class to a JSON file.
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        Ok(self.inner.write_json(path))
    }
}
