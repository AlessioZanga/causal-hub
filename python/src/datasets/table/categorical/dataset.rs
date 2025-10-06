use backend::{
    datasets::{CatTable, CatType, Dataset},
    models::Labelled,
};
use numpy::{PyArray2, ToPyArray};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use crate::impl_deref_from_into;

/// A categorical tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "CatTable", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatTable {
    inner: CatTable,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatTable, CatTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTable {
    /// Constructs a new categorical tabular dataset from a Pandas DataFrame.
    ///
    /// # Arguments
    ///
    /// * `df` - A Pandas DataFrame.
    ///
    /// # Returns
    ///
    /// A new categorical tabular dataset instance.
    ///
    #[new]
    pub fn new(py: Python<'_>, df: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Import the pandas module.
        let pandas = py.import("pandas")?;

        // Assert that the object is a DataFrame.
        assert!(
            df.is_instance(&pandas.getattr("DataFrame")?)?,
            "Expected a Pandas DataFrame, but '{}' found.",
            df.get_type().name()?
        );

        // Get the shape of the data frame.
        let mut shape = df.getattr("shape")?.extract::<(usize, usize)>()?;

        // Invoke the columns method.
        let columns = df.getattr("columns")?;
        // Convert the columns to a Vec<String>.
        let mut columns: Vec<String> = columns
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        todo!() // FIXME:
    }

    /// The labels of the dataset.
    ///
    /// # Returns
    ///
    /// A list of strings containing the labels of the dataset.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(String::as_str).collect())
    }

    /// The values of the dataset.
    ///
    /// # Returns
    ///
    /// A 2D NumPy array containing the values of the dataset.
    ///
    pub fn values<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<CatType>>> {
        Ok(self.inner.values().to_pyarray(py))
    }

    /// The sample size.
    ///
    /// # Notes
    ///
    /// If the dataset is weighted, this returns the sum of the weights.
    ///
    /// # Returns
    ///
    /// The number of samples in the dataset.
    ///
    pub fn sample_size(&self) -> PyResult<f64> {
        Ok(self.inner.sample_size())
    }
}
