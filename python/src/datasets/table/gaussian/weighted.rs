use std::sync::{Arc, RwLock};

use backend::{
    datasets::{Dataset, GaussType, GaussWtdTable},
    models::Labelled,
};
use numpy::{PyArray1, PyArray2, ToPyArray};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use crate::{datasets::PyGaussTable, error::to_pyerr, impl_from_into_lock};

/// A Gaussian weighted tabular dataset.
///
#[gen_stub_pyclass]
#[pyclass(name = "GaussWtdTable", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyGaussWtdTable {
    inner: Arc<RwLock<GaussWtdTable>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyGaussWtdTable, GaussWtdTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussWtdTable {
    /// Constructs a new Gaussian weighted tabular dataset.
    ///
    /// Parameters
    /// ----------
    /// dataset : GaussTable
    ///     A Gaussian tabular dataset instance.
    /// weights : numpy.ndarray
    ///     A 1D NumPy array of non-negative finite weights, one per sample.
    ///
    /// Returns
    /// -------
    /// GaussWtdTable
    ///     A new Gaussian weighted tabular dataset instance.
    #[new]
    pub fn new(dataset: &Bound<'_, PyGaussTable>, weights: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Get the reference to the inner dataset and clone it.
        // Get the Gaussian tabular dataset.
        let dataset: PyGaussTable = dataset.extract()?;
        let dataset = dataset.lock().clone();
        // Extract the weights as a vector of floats.
        let weights = weights.extract::<Vec<f64>>()?;

        // Construct the weighted table.
        Ok(GaussWtdTable::new(dataset, weights.into())
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

    /// The values of the dataset.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D NumPy array containing the values of the dataset.
    ///
    pub fn values<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<GaussType>>> {
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
