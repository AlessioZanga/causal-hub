use std::collections::BTreeMap;

use backend::{
    datasets::{CatWtdTrj, CatWtdTrjs, Dataset},
    models::Labelled,
};
use numpy::{PyArray1, prelude::*};
use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::derive::*;

use crate::{datasets::PyCatTrj, impl_deref_from_into};

/// A categorical trajectory with a weight.
#[gen_stub_pyclass]
#[pyclass(name = "CatWtdTrj", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatWtdTrj {
    inner: CatWtdTrj,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatWtdTrj, CatWtdTrj);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatWtdTrj {
    /// Returns the labels of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels of the categorical trajectory.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the trajectory.
    ///
    /// Returns
    /// -------
    /// CatTrj
    ///     A reference to the trajectory.
    ///
    pub fn trajectory(&self) -> PyResult<PyCatTrj> {
        Ok(self.inner.trajectory().clone().into())
    }

    /// Returns the weight of the trajectory.
    ///
    /// Returns
    /// -------
    /// float
    ///     The weight of the trajectory.
    ///
    pub fn weight(&self) -> f64 {
        self.inner.weight()
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A reference to the states of the categorical trajectory.
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
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A reference to the times of the trajectory.
    ///
    pub fn times<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray1<f64>>> {
        Ok(self.inner.times().to_pyarray(py))
    }
}

/// A collection of categorical trajectories with weights.
#[gen_stub_pyclass]
#[pyclass(name = "CatWtdTrjs", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatWtdTrjs {
    inner: CatWtdTrjs,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatWtdTrjs, CatWtdTrjs);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatWtdTrjs {
    /// Returns the labels of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels of the categorical trajectory.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A reference to the states of the categorical trajectory.
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
    /// Returns
    /// -------
    /// list[CatWtdTrj]
    ///     A vector of categorical trajectories.
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
