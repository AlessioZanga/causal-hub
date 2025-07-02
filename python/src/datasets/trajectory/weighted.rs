use std::collections::BTreeMap;

use causal_hub::datasets::{CatWtdTrj, CatWtdTrjs, Dataset};
use numpy::{PyArray1, prelude::*};
use pyo3::{prelude::*, types::PyTuple};

use crate::{datasets::PyCatTrj, impl_deref_from_into};

#[pyclass(name = "CatWtdTrj")]
#[derive(Clone, Debug)]
pub struct PyCatWtdTrj {
    inner: CatWtdTrj,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatWtdTrj, CatWtdTrj);

#[pymethods]
impl PyCatWtdTrj {
    /// Returns the labels of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the labels of the categorical trajectory.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the trajectory.
    ///
    pub fn trajectory(&self) -> PyResult<PyCatTrj> {
        Ok(self.inner.trajectory().clone().into())
    }

    /// Returns the weight of the trajectory.
    ///
    /// # Returns
    ///
    /// The weight of the trajectory.
    ///
    pub fn weight(&self) -> f64 {
        self.inner.weight()
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the states of the categorical trajectory.
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

    /// Returns the times of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the times of the trajectory.
    ///
    pub fn times<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.inner.times().to_pyarray(py))
    }
}

#[pyclass(name = "CatWtdTrjs")]
#[derive(Clone, Debug)]
pub struct PyCatWtdTrjs {
    inner: CatWtdTrjs,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatWtdTrjs, CatWtdTrjs);

#[pymethods]
impl PyCatWtdTrjs {
    /// Returns the labels of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the labels of the categorical trajectory.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the states of the categorical trajectory.
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

    /// Return the trajectories.
    ///
    /// # Returns
    ///
    /// A vector of categorical trajectories.
    ///
    pub fn values(&self) -> PyResult<Vec<PyCatWtdTrj>> {
        Ok(self
            .inner
            .values()
            .iter()
            .cloned()
            .map(|trj| trj.into())
            .collect())
    }
}
