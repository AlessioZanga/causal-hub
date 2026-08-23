use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatType, CatWtdTable, Dataset},
    models::Labelled,
};
use numpy::{PyArray1, PyArray2, ToPyArray};
use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::derive::*;

use crate::{datasets::PyCatTable, error::to_pyerr, impl_from_into_lock};

/// A categorical weighted tabular dataset.
///
#[gen_stub_pyclass]
#[pyclass(name = "CatWtdTable", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCatWtdTable {
    inner: Arc<RwLock<CatWtdTable>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatWtdTable, CatWtdTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatWtdTable {
    /// Constructs a new categorical weighted tabular dataset.
    ///
    /// Parameters
    /// ----------
    /// dataset : CatTable
    ///     A categorical tabular dataset instance.
    /// weights : numpy.ndarray
    ///     A 1D NumPy array of non-negative finite weights, one per sample.
    ///
    /// Returns
    /// -------
    /// CatWtdTable
    ///     A new categorical weighted tabular dataset instance.
    #[new]
    pub fn new(dataset: &Bound<'_, PyCatTable>, weights: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Get the reference to the inner dataset and clone it.
        // Get the categorical tabular dataset.
        let dataset: PyCatTable = dataset.extract()?;
        let dataset = dataset.lock().clone();
        // Extract the weights as a vector of floats.
        let weights = weights.extract::<Vec<f64>>()?;

        // Construct the weighted table.
        Ok(CatWtdTable::new(dataset, weights.into())
            .map_err(to_pyerr)?
            .into())
    }

    /// The labels of the dataset.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of strings containing the labels of the dataset.
    ///
    pub fn labels(&self) -> Vec<String> {
        self.lock().labels().iter().cloned().collect()
    }

    /// The support of the dataset.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A dictionary mapping each label to a tuple of its possible states.
    ///
    pub fn support<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<String, Bound<'a, PyTuple>>> {
        self.lock()
            .support()
            .iter()
            .map(|(label, states)| {
                // Get references to the label and states.
                let label = label.clone();
                let states = states.iter().cloned();
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states)?;
                // Return a tuple of the label and states.
                Ok((label, states))
            })
            .collect::<PyResult<_>>()
    }

    /// The values of the dataset.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D NumPy array containing the values of the dataset.
    ///
    pub fn values<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<CatType>>> {
        Ok(self.lock().values().values().to_pyarray(py))
    }

    /// The weights of the samples.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 1D NumPy array containing the weight of each sample.
    ///
    pub fn weights<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray1<f64>>> {
        Ok(self.lock().weights().to_pyarray(py))
    }

    /// The sample size.
    ///
    /// Returns
    /// -------
    /// float
    ///     The sum of the sample weights.
    ///
    pub fn sample_size(&self) -> f64 {
        self.lock().sample_size()
    }
}
