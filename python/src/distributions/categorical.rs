use std::collections::BTreeMap;

use causal_hub::distributions::{CPD, CategoricalCPD};
use numpy::{PyArray2, prelude::*};
use pyo3::{prelude::*, types::PyTuple};

/// A struct representing a categorical conditional probability distribution (CPD).
#[pyclass(name = "CategoricalCPD")]
#[derive(Clone, Debug)]
pub struct PyCategoricalCPD {
    inner: CategoricalCPD,
}

impl From<CategoricalCPD> for PyCategoricalCPD {
    fn from(inner: CategoricalCPD) -> Self {
        Self { inner }
    }
}

impl From<PyCategoricalCPD> for CategoricalCPD {
    fn from(outer: PyCategoricalCPD) -> Self {
        outer.inner
    }
}

#[pymethods]
impl PyCategoricalCPD {
    #[new]
    fn new(
        state: Bound<'_, PyTuple>,
        conditioning_states: Bound<'_, PyAny>,
        parameters: Bound<'_, PyArray2<f64>>,
    ) -> PyResult<Self> {
        // Convert the PyTuple to a (String, Vec<String>).
        let state = state.extract::<(String, Vec<String>)>()?;
        // Convert the PyIterator to a Vec<(String, Vec<String>)>.
        let conditioning_states: Vec<(String, Vec<String>)> = conditioning_states
            .try_iter()?
            .map(|x| x?.extract::<(String, Vec<String>)>())
            .collect::<PyResult<_>>()?;
        // Convert the PyArray2<f64> to a Array2<f64>.
        let parameters = parameters.to_owned_array();
        // Create a new CategoricalCPD with the given parameters.
        Ok(Self {
            inner: CategoricalCPD::new(state, conditioning_states, parameters),
        })
    }

    /// Returns the label of the conditioned variable.
    ///
    /// # Returns
    ///
    /// A reference to the label.
    ///
    fn label(&self) -> PyResult<&str> {
        Ok(self.inner.label().as_ref())
    }

    /// Returns the states of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The states of the conditioned variable.
    ///
    fn states(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.states().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the cardinality of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioned variable.
    ///
    fn cardinality(&self) -> PyResult<usize> {
        Ok(self.inner.cardinality())
    }

    /// Returns the labels of the conditioned variables.
    ///
    /// # Returns
    ///
    /// A reference to the conditioning labels.
    ///
    fn conditioning_labels(&self) -> PyResult<Vec<&str>> {
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
    fn conditioning_states<'a>(
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
    fn conditioning_cardinality(&self) -> PyResult<Vec<usize>> {
        Ok(self.inner.conditioning_cardinality().to_vec())
    }

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    fn parameters<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self.inner.parameters().to_pyarray(py))
    }

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.inner.parameters_size())
    }

    /// Returns the sample size of the dataset used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample size of the dataset used to fit the distribution.
    ///
    fn sample_size(&self) -> PyResult<Option<usize>> {
        Ok(self.inner.sample_size())
    }

    /// Returns the sample log-likelihood of the dataset given the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample log-likelihood of the dataset given the distribution.
    ///
    fn sample_log_likelihood(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.sample_log_likelihood())
    }

    /// Returns the string representation of the CategoricalCPD.
    fn __repr__(&self) -> PyResult<String> {
        // Get the string representation of the CategoricalCPD.
        Ok(self.inner.to_string())
    }
}
