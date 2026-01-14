use std::{
    collections::BTreeSet,
    sync::{Arc, RwLock},
};

use backend::{datasets::MissingTable, models::Labelled};
use numpy::{PyArray1, PyArray2, PyArrayMethods, ToPyArray};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A struct for missing information in a tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "MissingTable", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyMissingTable {
    inner: Arc<RwLock<MissingTable>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyMissingTable, MissingTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyMissingTable {
    /// Constructs a new missing information table.
    ///
    /// Parameters
    /// ----------
    /// labels: list[str]
    ///     A list of strings containing the labels of the variables.
    /// mask: numpy.ndarray
    ///     A boolean NumPy array containing the missing information mask.
    ///
    /// Returns
    /// -------
    /// MissingTable
    ///     A new missing information table instance.
    ///
    #[new]
    pub fn new(labels: Vec<String>, mask: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Extract the mask.
        let mask: Bound<'_, PyArray2<bool>> = mask.extract()?;
        let mask = mask.readonly();
        let mask = mask.as_array().to_owned();
        // Construct the labels.
        let labels = labels.into_iter().collect();
        Ok(MissingTable::new(labels, mask).into())
    }

    /// The labels of the dataset.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of strings containing the labels of the dataset.
    ///
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// The fully observed variable sets.
    ///
    /// Returns
    /// -------
    /// set[int]
    ///     The set of fully observed variables.
    ///
    pub fn fully_observed(&self) -> PyResult<BTreeSet<usize>> {
        Ok(self.lock().fully_observed().iter().cloned().collect())
    }

    /// The partially observed variable sets.
    ///
    /// Returns
    /// -------
    /// set[int]
    ///     The set of partially observed variables.
    ///
    pub fn partially_observed(&self) -> PyResult<BTreeSet<usize>> {
        Ok(self.lock().partially_observed().iter().cloned().collect())
    }

    /// The missing mask of the dataset.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D NumPy array containing the missing mask of the dataset.
    ///
    pub fn missing_mask<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<u8>>> {
        Ok(self.lock().missing_mask().mapv(|x| x as u8).to_pyarray(py))
    }

    /// The missing mask by columns.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 1D NumPy array containing the missing mask by columns.
    ///
    pub fn missing_mask_by_cols<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray1<u8>>> {
        Ok(self
            .lock()
            .missing_mask_by_cols()
            .mapv(|x| x as u8)
            .to_pyarray(py))
    }

    /// The missing mask by rows.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 1D NumPy array containing the missing mask by rows.
    ///
    pub fn missing_mask_by_rows<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray1<u8>>> {
        Ok(self
            .lock()
            .missing_mask_by_rows()
            .mapv(|x| x as u8)
            .to_pyarray(py))
    }

    /// The missing count of the dataset.
    ///
    /// Returns
    /// -------
    /// int
    ///     The number of missing values in the dataset.
    ///
    pub fn missing_count(&self) -> PyResult<usize> {
        Ok(self.lock().missing_count())
    }

    /// The missing count by columns.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 1D NumPy array containing the missing count by columns.
    ///
    pub fn missing_count_by_cols<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyArray1<u64>>> {
        Ok(self
            .lock()
            .missing_count_by_cols()
            .mapv(|x| x as u64)
            .to_pyarray(py))
    }

    /// The missing count by rows.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 1D NumPy array containing the missing count by rows.
    ///
    pub fn missing_count_by_rows<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyArray1<u64>>> {
        Ok(self
            .lock()
            .missing_count_by_rows()
            .mapv(|x| x as u64)
            .to_pyarray(py))
    }

    /// The missing rate of the dataset.
    ///
    /// Returns
    /// -------
    /// float
    ///     The missing rate of the dataset.
    ///
    pub fn missing_rate(&self) -> PyResult<f64> {
        Ok(self.lock().missing_rate())
    }

    /// The missing rate by columns.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 1D NumPy array containing the missing rate by columns.
    ///
    pub fn missing_rate_by_cols<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyArray1<f64>>> {
        Ok(self.lock().missing_rate_by_cols().to_pyarray(py))
    }

    /// The missing rate by rows.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 1D NumPy array containing the missing rate by rows.
    ///
    pub fn missing_rate_by_rows<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyArray1<f64>>> {
        Ok(self.lock().missing_rate_by_rows().to_pyarray(py))
    }

    /// The missing correlation of the dataset.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D NumPy array containing the missing correlation of the dataset.
    ///
    pub fn missing_correlation<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self.lock().missing_correlation().to_pyarray(py))
    }

    /// The missing covariance of the dataset.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D NumPy array containing the missing covariance of the dataset.
    ///
    pub fn missing_covariance<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self.lock().missing_covariance().to_pyarray(py))
    }

    /// The complete columns count.
    ///
    /// Returns
    /// -------
    /// int
    ///     The number of complete columns in the dataset.
    ///
    pub fn complete_cols_count(&self) -> PyResult<usize> {
        Ok(self.lock().complete_cols_count())
    }

    /// The complete rows count.
    ///
    /// Returns
    /// -------
    /// int
    ///     The number of complete rows in the dataset.
    ///
    pub fn complete_rows_count(&self) -> PyResult<usize> {
        Ok(self.lock().complete_rows_count())
    }
}
