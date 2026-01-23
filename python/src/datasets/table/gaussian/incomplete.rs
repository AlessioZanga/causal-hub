use std::sync::{Arc, RwLock};

use backend::{
    datasets::{Dataset, GaussIncTable, GaussType, IncDataset},
    models::Labelled,
};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, ToPyArray};
use pyo3::{
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::{datasets::PyMissingTable, error::Error, impl_from_into_lock};

/// A Gaussian incomplete tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "GaussIncTable", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyGaussIncTable {
    inner: Arc<RwLock<GaussIncTable>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyGaussIncTable, GaussIncTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussIncTable {
    /// Constructs a new Gaussian incomplete tabular dataset.
    ///
    /// Parameters
    /// ----------
    /// labels : list[str]
    ///     A list of strings containing the labels of the dataset.
    /// values : numpy.ndarray
    ///     A 2D numpy array containing the values of the dataset.
    ///
    /// Returns
    /// -------
    /// GaussIncTable
    ///     A new Gaussian incomplete tabular dataset instance.
    ///
    #[new]
    pub fn new(labels: Vec<String>, values: PyReadonlyArray2<GaussType>) -> PyResult<Self> {
        let values = values.as_array().to_owned();
        let labels = labels.into_iter().collect();
        let inner =
            GaussIncTable::new(labels, values).map_err(|e| Error::new_err(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
        })
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

    /// The values of the dataset.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D numpy array containing the values of the dataset.
    ///
    pub fn values<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<GaussType>>> {
        Ok(self.lock().values().to_pyarray(py))
    }

    /// The number of samples in the dataset.
    ///
    /// Returns
    /// -------
    /// int
    ///     To number of samples in the dataset.
    ///
    pub fn sample_size(&self) -> PyResult<usize> {
        Ok(self.lock().sample_size() as usize)
    }

    /// The missing information of the dataset.
    ///
    /// Returns
    /// -------
    /// MissingTable
    ///     The missing information of the dataset.
    ///
    pub fn missing(&self) -> PyResult<PyMissingTable> {
        Ok(self.lock().missing().clone().into())
    }

    /// Constructs a new gaussian incomplete tabular dataset from a Pandas DataFrame.
    ///
    /// Parameters
    /// ----------
    ///
    /// df: pandas.DataFrame
    ///     A Pandas DataFrame containing gaussian columns with missing values.
    ///
    /// Returns
    /// -------
    /// GaussIncTable
    ///     A new gaussian incomplete tabular dataset instance.
    ///
    #[classmethod]
    pub fn from_pandas(_cls: &Bound<'_, PyType>, df: Bound<'_, PyAny>) -> PyResult<Self> {
        // Get references to Python and Pandas.
        let py = df.py();
        let pd = py.import("pandas")?;
        // Check if the input is a Pandas DataFrame.
        if !df.is_instance(&pd.getattr("DataFrame")?)? {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "Input must be a Pandas DataFrame.",
            ));
        }

        // Get labels.
        let labels: Vec<String> = df.getattr("columns")?.call_method0("to_list")?.extract()?;

        // Get values.
        let values: Bound<'_, PyArray2<GaussType>> =
            df.call_method1("to_numpy", ("float64",))?.extract()?;
        let values = values.readonly().as_array().to_owned();
        let labels = labels.into_iter().collect();

        // Construct the gaussian incomplete tabular dataset.
        Ok(GaussIncTable::new(labels, values)
            .map_err(|e| Error::new_err(e.to_string()))?
            .into())
    }

    /// Converts the dataset to a Pandas DataFrame.
    ///
    /// Returns
    /// -------
    /// pandas.DataFrame
    ///     A Pandas DataFrame.
    ///
    pub fn to_pandas(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        // Get reference to Pandas.
        let pd = py.import("pandas")?;
        // Get reference to the inner dataset.
        let inner = self.lock();
        let labels: Vec<String> = inner.labels().iter().cloned().collect();
        // values with NANs.
        let values = inner.values().to_pyarray(py);

        // Construct the DataFrame.
        let kwargs = PyDict::new(py);
        kwargs.set_item("columns", labels)?;
        let df = pd.call_method("DataFrame", (values,), Some(&kwargs))?;

        Ok(df.into())
    }
}
