use std::collections::BTreeMap;

use causal_hub_rust::distributions::{CPD, CatCIM};
use numpy::{PyArray3, prelude::*};
use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

use crate::impl_deref_from_into;

/// A struct representing a categorical conditional intensity matrix (CIM).
#[gen_stub_pyclass]
#[pyclass(name = "CatCIM")]
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fn label(&self) -> PyResult<&str> {
        Ok(self.inner.label().as_ref())
    }

    /// Returns the states of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The states of the conditioned variable.
    ///
    pub fn states(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.states().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the cardinality of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioned variable.
    ///
    pub fn cardinality(&self) -> PyResult<usize> {
        Ok(self.inner.cardinality())
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

    /// Returns the cardinality of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioning variables.
    ///
    pub fn conditioning_cardinality(&self) -> PyResult<Vec<usize>> {
        Ok(self.inner.conditioning_cardinality().to_vec())
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

    /// Returns the sample size of the dataset used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample size of the dataset used to fit the distribution.
    ///
    pub fn sample_size(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.sample_size())
    }

    /// Returns the sample log-likelihood of the dataset given the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample log-likelihood of the dataset given the distribution.
    ///
    pub fn sample_log_likelihood(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.sample_log_likelihood())
    }
}
