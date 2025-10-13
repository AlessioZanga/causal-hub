use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatWtdTrj, CatWtdTrjs, Dataset},
    models::Labelled,
};
use numpy::{PyArray1, prelude::*};
use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::derive::*;

use crate::{datasets::PyCatTrj, impl_from_into_lock};

/// A categorical trajectory with a weight.
#[gen_stub_pyclass]
#[pyclass(name = "CatWtdTrj", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatWtdTrj {
    inner: Arc<RwLock<CatWtdTrj>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatWtdTrj, CatWtdTrj);

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
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the trajectory.
    ///
    /// Returns
    /// -------
    /// CatTrj
    ///     A reference to the trajectory.
    ///
    pub fn trajectory(&self) -> PyResult<PyCatTrj> {
        Ok(self.lock().trajectory().clone().into())
    }

    /// Returns the weight of the trajectory.
    ///
    /// Returns
    /// -------
    /// float
    ///     The weight of the trajectory.
    ///
    pub fn weight(&self) -> f64 {
        self.lock().weight()
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A reference to the states of the categorical trajectory.
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

    /// Returns the times of the trajectory.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A reference to the times of the trajectory.
    ///
    pub fn times<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray1<f64>>> {
        Ok(self.lock().times().to_pyarray(py))
    }
}

/// A collection of categorical trajectories with weights.
#[gen_stub_pyclass]
#[pyclass(name = "CatWtdTrjs", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatWtdTrjs {
    inner: Arc<RwLock<CatWtdTrjs>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatWtdTrjs, CatWtdTrjs);

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
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A reference to the states of the categorical trajectory.
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

    /// Return the trajectories.
    ///
    /// Returns
    /// -------
    /// list[CatWtdTrj]
    ///     A vector of categorical trajectories.
    ///
    pub fn values(&self) -> PyResult<Vec<PyCatWtdTrj>> {
        Ok(self
            .lock()
            .values()
            .iter()
            .cloned()
            .map(|trj| trj.into())
            .collect())
    }
}
