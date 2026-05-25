use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatTable, CatType, Dataset},
    models::Labelled,
    types::{Set, Support},
};
use numpy::{PyArray1, PyArray2, PyArrayMethods, ToPyArray, ndarray::prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::{
    error::{Error, to_pyerr},
    impl_from_into_lock,
};

/// A categorical tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "CatTable", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCatTable {
    inner: Arc<RwLock<CatTable>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatTable, CatTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTable {
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

    /// Returns the support of the dataset.
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
                // Get reference to the label and states.
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

    /// Constructs a new categorical tabular dataset from a Pandas DataFrame.
    ///
    /// Parameters
    /// ----------
    ///
    /// df: pandas.DataFrame
    ///     A Pandas DataFrame containing only categorical columns.
    ///
    /// Returns
    /// -------
    /// CatTable
    ///     A new categorical tabular dataset instance.
    ///
    #[classmethod]
    pub fn from_pandas(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        df: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Import the pandas module.
        let pd = py.import("pandas")?;

        // Check that the object is a DataFrame.
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
        let columns: Vec<String> = columns
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Check that the data frame is not empty.
        assert!(!columns.is_empty(), "The data frame is empty.");

        // Check that the dtype of the column is a string.
        for name in &columns {
            // Extract the column from the data frame.
            let column = df.get_item(name)?;
            // Get the dtype of the column.
            let dtype = column
                .getattr("dtype")?
                .getattr("name")?
                .extract::<String>()?;
            // Check that the dtype is a category.
            assert_eq!(
                dtype, "category",
                "Expected a category column, but '{dtype}' found."
            );
        }

        // Convert the columns categories to support.
        let support: Support = columns
            .into_iter()
            // Return the column name and the set of unique values.
            .map(|name| {
                // Extract the column from the data frame.
                let column = df.get_item(&name)?;
                // Invoke the 'cat' accessory method.
                let categories = column.getattr("cat")?.getattr("categories")?;
                // Iterate over the categories and convert them to a Vec<String>.
                let categories: Set<String> = categories
                    .try_iter()?
                    .map(|x| x?.extract::<String>())
                    .collect::<PyResult<_>>()?;

                Ok((name, categories))
            })
            .collect::<PyResult<_>>()?;

        // Initialize the categorical variables values.
        let mut values = Array2::from_elem(shape, CatType::default());
        // Extract the categorical variables values.
        values
            .columns_mut()
            .into_iter()
            .zip(&support)
            .try_for_each(|(mut value, (name, states))| {
                // Extract the column from the data frame.
                let column = df.get_item(name)?;
                // Invoke the to_numpy method on the column.
                let column = column.getattr("to_numpy")?.call0()?;
                // Extract the column as a PyArray1<PyObject>.
                let column = column.cast::<PyArray1<Py<PyAny>>>()?.to_owned_array();
                // Map the PyObject to String and convert it to CatType.
                let column: Result<Vec<_>, _> = column
                    .iter()
                    .map(|x| {
                        // Get the value.
                        let x = x.extract::<String>(py)?;
                        // Map the value to CatType.
                        states
                            .get_index_of(&x)
                            .ok_or_else(|| Error::new_err(format!("Unknown state: {}", x)))
                            .map(|idx| idx as CatType)
                    })
                    .collect();
                let column = Array1::from_vec(column?);
                // Extract the column from the data frame.
                value.assign(&column);

                Ok::<_, PyErr>(())
            })?;

        // Construct the dataset.
        CatTable::new(support, values)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Constructs a new categorical tabular dataset from a Polars DataFrame.
    ///
    /// Parameters
    /// ----------
    ///
    /// df: polars.DataFrame
    ///     A Polars DataFrame containing only categorical columns.
    ///
    /// Returns
    /// -------
    /// CatTable
    ///     A new categorical tabular dataset instance.
    ///
    #[classmethod]
    pub fn from_polars(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        df: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Import the polars module.
        let pl = py.import("polars")?;

        // Check that the object is a DataFrame.
        assert!(
            df.is_instance(&pl.getattr("DataFrame")?)?,
            "Expected a Polars DataFrame, but '{}' found.",
            df.get_type().name()?
        );

        // Get the shape of the data frame.
        let shape = df.getattr("shape")?.extract::<(usize, usize)>()?;

        // Get columns.
        let columns: Vec<String> = df.getattr("columns")?.extract()?;

        // Check that the data frame is not empty.
        assert!(!columns.is_empty(), "The data frame is empty.");

        // Check that all columns are categorical-like.
        for name in &columns {
            let column = df.call_method1("get_column", (name,))?;
            let dtype = column.getattr("dtype")?.str()?.extract::<String>()?;
            assert!(
                dtype.contains("Categorical") || dtype.contains("Enum"),
                "Expected a categorical column, but '{dtype}' found."
            );
        }

        // Extract support.
        let support: Support = columns
            .into_iter()
            .map(|name| {
                let column = df.call_method1("get_column", (&name,))?;
                let categories = column.getattr("cat")?.call_method0("get_categories")?;
                let categories: Set<String> = categories
                    .try_iter()?
                    .map(|x| x?.extract::<String>())
                    .collect::<PyResult<_>>()?;
                Ok((name, categories))
            })
            .collect::<PyResult<_>>()?;

        // Extract values.
        let mut values = Array2::from_elem(shape, CatType::default());
        values
            .columns_mut()
            .into_iter()
            .zip(&support)
            .try_for_each(|(mut value, (name, states))| {
                let column = df.call_method1("get_column", (name,))?;
                let column: Vec<Option<String>> = column.call_method0("to_list")?.extract()?;
                let column: Result<Vec<CatType>, _> = column
                    .into_iter()
                    .map(|x| {
                        let x = x.ok_or_else(|| {
                            Error::new_err(format!("Null value found in column '{name}'"))
                        })?;
                        states
                            .get_index_of(&x)
                            .ok_or_else(|| Error::new_err(format!("Unknown state: {}", x)))
                            .map(|idx| idx as CatType)
                    })
                    .collect();
                value.assign(&Array1::from_vec(column?));

                Ok::<_, PyErr>(())
            })?;

        CatTable::new(support, values)
            .map(Into::into)
            .map_err(to_pyerr)
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
        // Get support and values.
        let support = lock.support().iter();
        let values = lock.values().columns();

        // For each column, create a Pandas Series and insert it into the dictionary.
        for ((label, states), values) in support.zip(values) {
            // Map the values to the corresponding states.
            let values: Vec<_> = values.iter().map(|&x| &states[x as usize]).collect();
            // Set the categorical states.
            let kwargs = PyDict::new(py);
            let categories: Vec<_> = states.iter().collect();
            kwargs.set_item("categories", categories)?;
            // Construct a Categorical.
            let categorical = pd.getattr("Categorical")?.call((values,), Some(&kwargs))?;
            // Construct a Series from a raw Categorical.
            let series = pd.getattr("Series")?.call1((categorical,))?;
            // Insert the column into the dictionary.
            df.set_item(label, series)?;
        }

        // Construct the DataFrame.
        pd.getattr("DataFrame")?.call1((df,))
    }

    /// Converts the dataset to a Polars DataFrame.
    ///
    /// Returns
    /// -------
    /// polars.DataFrame
    ///     A Polars DataFrame.
    ///
    pub fn to_polars<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        // Import the polars module.
        let pl = py.import("polars")?;

        // Create a dictionary to hold the data.
        let df = PyDict::new(py);

        // Get lock on the inner field.
        let lock = self.lock();
        let support = lock.support().iter();
        let values = lock.values().columns();

        for ((label, states), values) in support.zip(values) {
            let values: Vec<String> = values.iter().map(|&x| states[x as usize].clone()).collect();
            let series = pl.getattr("Series")?.call1((label.clone(), values))?;
            let series = series.call_method1("cast", (pl.getattr("Categorical")?,))?;
            df.set_item(label, series)?;
        }

        pl.getattr("DataFrame")?.call1((df,))
    }
}
