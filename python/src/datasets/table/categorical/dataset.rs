use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatTable, CatType, Dataset},
    models::Labelled,
    types::{Set, States},
};
use numpy::{PyArray1, PyArray2, PyArrayMethods, ToPyArray, ndarray::prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A categorical tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "CatTable", module = "causal_hub.datasets")]
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

    /// Returns the states of the dataset.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A dictionary mapping each label to a tuple of its possible states.
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

        // Convert the columns categories to states.
        let states: States = columns
            .into_iter()
            // Return the column name and the set of unique values.
            .map(|name| {
                // Extract the column from the data frame.
                let column = df.get_item(&name)?;
                // Invoke the 'cat' accessory method.
                let states = column.getattr("cat")?.getattr("categories")?;
                // Iterate over the states and convert them to a Vec<String>.
                let states: Set<String> = states
                    .try_iter()?
                    .map(|x| x?.extract::<String>())
                    .collect::<PyResult<_>>()?;

                Ok((name, states))
            })
            .collect::<PyResult<_>>()?;

        // Initialize the categorical variables values.
        let mut values = Array2::from_elem(shape, CatType::default());
        // Extract the categorical variables values.
        values.columns_mut().into_iter().zip(&states).try_for_each(
            |(mut value, (name, states))| {
                // Extract the column from the data frame.
                let column = df.get_item(name)?;
                // Invoke the to_numpy method on the column.
                let column = column.getattr("to_numpy")?.call0()?;
                // Extract the column as a PyArray1<PyObject>.
                let column = column.downcast::<PyArray1<Py<PyAny>>>()?.to_owned_array();
                // Map the PyObject to String and convert it to CatType.
                let column = column.map(|x| {
                    // Get the value.
                    let x = x.extract::<String>(py).unwrap();
                    // Map the value to CatType.
                    states.get_index_of(&x).unwrap() as CatType
                });
                // Extract the column from the data frame.
                value.assign(&column);

                Ok::<_, PyErr>(())
            },
        )?;

        // Construct the dataset.
        let inner = CatTable::new(states, values);
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
        // Get states and values.
        let states = lock.states().iter();
        let values = lock.values().columns();

        // For each column, create a Pandas Series and insert it into the dictionary.
        for ((label, states), values) in states.zip(values) {
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
}
