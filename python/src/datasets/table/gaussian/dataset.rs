use std::sync::{Arc, RwLock};

use backend::{
    datasets::{Dataset, GaussTable, GaussType},
    models::Labelled,
    types::Labels,
};
use numpy::{PyArray1, PyArray2, PyArrayMethods, ToPyArray, ndarray::prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A Gaussian tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "GaussTable", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyGaussTable {
    inner: Arc<RwLock<GaussTable>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyGaussTable, GaussTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussTable {
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
    ///     A 2D NumPy array containing the values of the dataset.
    ///
    pub fn values<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<GaussType>>> {
        Ok(self.lock().values().to_pyarray(py))
    }

    /// The sample size.
    ///
    /// Returns
    /// -------
    /// float
    ///     The number of samples in the dataset.
    ///     If the dataset is weighted, this returns the sum of the weights.
    ///
    pub fn sample_size(&self) -> PyResult<f64> {
        Ok(self.lock().sample_size())
    }

    /// Constructs a new Gaussian tabular dataset from a Pandas DataFrame.
    ///
    /// Parameters
    /// ----------
    /// df: pandas.DataFrame
    ///     A Pandas DataFrame containing only float64 columns.
    ///
    /// Returns
    /// -------
    /// GaussTable
    ///     A new Gaussian tabular dataset instance.
    ///
    #[classmethod]
    pub fn from_pandas(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        df: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Import the pandas module.
        let pd = py.import("pandas")?;

        // Assert that the object is a DataFrame.
        assert!(
            df.is_instance(&pd.getattr("DataFrame")?)?,
            "Expected a Pandas DataFrame, but '{}' found.",
            df.get_type().name()?
        );

        // Get the shape of the data frame.
        let shape = df.getattr("shape")?.extract::<(usize, usize)>()?;

        // Invoke the columns method.
        let columns = df.getattr("columns")?;
        // Convert the columns to a Vec<String>.
        let columns: Labels = columns
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Check that the data frame is not empty.
        assert!(!columns.is_empty(), "The data frame is empty.");

        // Check that the dtype of the column is a f64.
        for name in &columns {
            // Extract the column from the data frame.
            let column = df.get_item(name)?;
            // Get the dtype of the column.
            let dtype = column
                .getattr("dtype")?
                .getattr("name")?
                .extract::<String>()?;
            // Check that the dtype is a float64.
            assert_eq!(
                dtype, "float64",
                "Expected a float64 column, but '{dtype}' found."
            );
        }

        // Initialize the variables values.
        let mut values = Array2::from_elem(shape, GaussType::default());
        // Extract the variables values.
        values
            .columns_mut()
            .into_iter()
            .zip(&columns)
            .try_for_each(|(mut value, name)| {
                // Extract the column from the data frame.
                let column = df.get_item(name)?;
                // Invoke the to_numpy method on the column.
                let column = column.getattr("to_numpy")?.call0()?;
                // Extract the column as a PyArray1<f64>.
                let column = column.downcast::<PyArray1<f64>>()?.to_owned_array();
                // Extract the column from the data frame.
                value.assign(&column);

                Ok::<_, PyErr>(())
            })?;

        // Create the Gaussian tabular dataset.
        let inner = GaussTable::new(columns, values);
        // Wrap the dataset in an Arc<RwLock>.
        let inner = Arc::new(RwLock::new(inner));

        Ok(Self { inner })
    }

    /// Converts the dataset to a Pandas DataFrame.
    ///
    /// Returns
    /// -------
    /// pandas.DataFrame
    ///     A Pandas DataFrame.
    ///
    pub fn to_pandas<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        // Import the pandas module.
        let pd = py.import("pandas")?;

        // Create a dictionary to hold the data.
        let df = PyDict::new(py);

        // Get lock on the inner field.
        let lock = self.lock();
        // Get labels and values.
        let labels = lock.labels().iter();
        let values = lock.values().columns();

        // For each column, create a Pandas Series and insert it into the dictionary.
        for (label, values) in labels.zip(values) {
            // Construct a Series from the values.
            let series = pd.getattr("Series")?.call1((values.to_pyarray(py),))?;
            // Insert the column into the dictionary.
            df.set_item(label, series)?;
        }

        // Construct the DataFrame.
        pd.getattr("DataFrame")?.call1((df,))
    }
}
