use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatTrj, CatTrjs, CatType, Dataset},
    models::Labelled,
    types::{Set, States},
};
use numpy::{PyArray1, PyArray2, ndarray::prelude::*, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A categorical trajectory.
#[gen_stub_pyclass]
#[pyclass(name = "CatTrj", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatTrj {
    inner: Arc<RwLock<CatTrj>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatTrj, CatTrj);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTrj {
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

    /// Returns the values of the trajectory.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A reference to the values of the trajectory.
    ///
    pub fn values<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<CatType>>> {
        Ok(self.lock().values().to_pyarray(py))
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

    /// Constructs a new categorical trajectory from a Pandas DataFrame.
    ///
    /// Parameters
    /// ----------
    /// df: pandas.DataFrame
    ///     A Pandas DataFrame containing the trajectory data.
    ///     The data frame must contain a column named "time" that represents the time of each event.
    ///     Every other column in the data frame must represent a categorical variable.
    ///
    /// Returns
    /// -------
    /// CatTrj
    ///     A new categorical trajectory instance.
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
        let mut shape = df.getattr("shape")?.extract::<(usize, usize)>()?;

        // Invoke the columns method.
        let columns = df.getattr("columns")?;
        // Convert the columns to a Vec<String>.
        let mut columns: Vec<String> = columns
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Check that the data frame is not empty.
        assert!(!columns.is_empty(), "The data frame is empty.");

        // Extract the time column from the data frame.
        let time = df.get_item("time")?;

        // Get the dtype of the time column.
        let dtype = time
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        // Check that the dtype is a float64.
        assert_eq!(
            dtype, "float64",
            "Expected a float64 column, but '{dtype}' found.",
        );

        // Invoke the to_numpy method on the time column.
        let time = time.getattr("to_numpy")?.call0()?;
        // Extract the time column as a PyArray1<f64>.
        let time = time.downcast::<PyArray1<f64>>()?.to_owned_array();
        // Remove the "time" column from the columns vector.
        columns.remove(columns.iter().position(|x| x == "time").unwrap());
        // Decrement the shape of the data frame.
        shape.1 -= 1;

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

        // Construct the categorical trajectory.
        let inner = CatTrj::new(states, values, time);
        // Wrap the dataset in an Arc<RwLock>.
        let inner = Arc::new(RwLock::new(inner));

        Ok(Self { inner })
    }

    /// Converts the categorical trajectory to a Pandas DataFrame.
    ///
    /// Returns
    /// -------
    /// pandas.DataFrame
    ///     A Pandas DataFrame representation of the categorical trajectory.
    ///
    pub fn to_pandas<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        // Import the pandas module.
        let pd = py.import("pandas")?;

        // Create a dictionary to hold the data.
        let df = PyDict::new(py);

        // Get lock on the inner field.
        let lock = self.lock();

        // Add the time column.
        let time = lock.times().to_pyarray(py);
        df.set_item("time", time)?;

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

/// A collection of categorical trajectories.
#[gen_stub_pyclass]
#[pyclass(name = "CatTrjs", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatTrjs {
    inner: Arc<RwLock<CatTrjs>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatTrjs, CatTrjs);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTrjs {
    /// Returns the labels of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels of the categorical trajectory.
    ///
    #[inline]
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
    /// list[CatTrj]
    ///     A list of categorical trajectories.
    ///
    pub fn values(&self) -> PyResult<Vec<PyCatTrj>> {
        Ok(self
            .lock()
            .values()
            .iter()
            .cloned()
            .map(|trj| trj.into())
            .collect())
    }

    /// Constructs a new categorical trajectories from an iterable of Pandas DataFrames.
    ///
    /// Parameters
    /// ----------
    /// dfs: Iterable[pandas.DataFrame]
    ///     An iterable of Pandas DataFrames containing the trajectory data.
    ///     Each data frame must contain a column named "time" that represents the time of each event.
    ///     Every other column in the data frame must represent a categorical variable.
    ///
    /// Returns
    /// -------
    /// CatTrjs
    ///     A new categorical trajectories instance.
    ///
    #[classmethod]
    pub fn from_pandas(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dfs: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Convert the iterable to a Vec<PyAny>.
        let dfs: Vec<PyCatTrj> = dfs
            .try_iter()?
            .map(|df| PyCatTrj::from_pandas(_cls, py, &df.unwrap()))
            .collect::<PyResult<_>>()?;
        // Convert the Vec<PyCatTrj> to Vec<CatTrj>.
        let dfs: Vec<_> = dfs.into_iter().map(Into::into).collect();

        // Create a new CatTrjs with the given parameters.
        let inner = CatTrjs::new(dfs);
        // Wrap the dataset in an Arc<RwLock>.
        let inner = Arc::new(RwLock::new(inner));

        Ok(Self { inner })
    }

    /// Converts the categorical trajectories to a list of Pandas DataFrames.
    ///
    /// Returns
    /// -------
    /// list[pandas.DataFrame]
    ///     A list of Pandas DataFrame representations of the categorical trajectories.
    ///
    pub fn to_pandas<'a>(&self, py: Python<'a>) -> PyResult<Vec<Bound<'a, PyAny>>> {
        // Convert each trajectory to a Pandas DataFrame.
        self.lock()
            .values()
            .iter()
            .cloned()
            .map(PyCatTrj::from)
            .map(|trj| trj.to_pandas(py))
            .collect::<PyResult<_>>()
    }
}
