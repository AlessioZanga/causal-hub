use std::collections::BTreeMap;

use causal_hub::{
    datasets::{CatTrj, CatTrjs, Dataset},
    types::{FxIndexMap, FxIndexSet},
};
use numpy::{PyArray1, ndarray::prelude::*, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};

use crate::impl_deref_from_into;

#[pyclass(name = "CatTrj")]
#[derive(Clone, Debug)]
pub struct PyCatTrj {
    inner: CatTrj,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatTrj, CatTrj);

#[pymethods]
impl PyCatTrj {
    /// Constructs a new categorical trajectory from a Pandas DataFrame.
    ///
    /// # Arguments
    ///
    /// * `df` - A Pandas DataFrame containing the trajectory data.
    /// * `with_states` - An optional dictionary of states.
    ///
    /// # Notes
    ///
    /// * The data frame must contain a column named "time" that represents the time of each event.
    /// * Every other column in the data frame must represent a categorical variable.
    ///
    /// # Returns
    ///
    /// A new categorical trajectory instance.
    ///
    #[new]
    #[pyo3(signature = (df, with_states = None))]
    pub fn new(
        py: Python<'_>,
        df: &Bound<'_, PyAny>,
        with_states: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
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

        // Extract the time column from the data frame.
        let time = df.get_item("time")?;

        // Check that the dtype of the time column is a float.
        let dtype = time
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
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

        // Check that the data frame is not empty.
        assert!(!columns.is_empty(), "The data frame is empty.");

        // Construct the states.
        let states: FxIndexMap<_, _> = with_states
            // If `with_states` is provided, convert it to a FxIndexMap.
            .map(|states| {
                // Iterate over the items.
                states
                    .items()
                    .into_iter()
                    .map(|key_value| {
                        // Cast the key_value to a tuple.
                        let (key, value) = key_value
                            .extract::<(Bound<'_, PyAny>, Bound<'_, PyAny>)>()
                            .unwrap();
                        // Convert the key to a String.
                        let key = key.extract::<String>().unwrap();
                        // Convert the value to a Vec<String>.
                        let value: FxIndexSet<_> = value
                            .try_iter()?
                            .map(|x| x?.extract::<String>())
                            .collect::<PyResult<_>>()?;
                        // Return the key and value.
                        Ok((key, value))
                    })
                    .collect::<PyResult<_>>()
            })
            // Otherwise, infer the states from the columns.
            .unwrap_or_else(|| {
                // Convert the columns categories to states.
                columns
                    .into_iter()
                    // Return the column name and the set of unique values.
                    .map(|name| {
                        // Extract the column from the data frame.
                        let column = df.get_item(&name)?;

                        // Check that the dtype of the column is a string.
                        let dtype = column
                            .getattr("dtype")?
                            .getattr("name")?
                            .extract::<String>()?;
                        assert_eq!(
                            dtype, "category",
                            "Expected a category column, but '{dtype}' found.",
                        );

                        // Invoke the 'cat' accessory method.
                        let states = column.getattr("cat")?.getattr("categories")?;
                        // Iterate over the states and convert them to a Vec<String>.
                        let states: FxIndexSet<String> = states
                            .try_iter()?
                            .map(|x| x?.extract::<String>())
                            .collect::<PyResult<_>>()?;

                        Ok((name, states))
                    })
                    .collect::<PyResult<_>>()
            })
            .unwrap();

        // Initialize the categorical variables values.
        let mut values = Array2::from_elem(shape, 0u8);
        // Extract the categorical variables values.
        values.columns_mut().into_iter().zip(&states).try_for_each(
            |(mut value, (name, states))| {
                // Extract the column from the data frame.
                let column = df.get_item(name)?;
                // Invoke the to_numpy method on the column.
                let column = column.getattr("to_numpy")?.call0()?;
                // Extract the column as a PyArray1<PyObject>.
                let column = column.downcast::<PyArray1<PyObject>>()?.to_owned_array();
                // Map the PyObject to String and convert it to u8
                let column = column.map(|x| {
                    // Get the value.
                    let x = x.extract::<String>(py).unwrap();
                    // Map the value to u8.
                    states.get_index_of(&x).unwrap() as u8
                });
                // Extract the column from the data frame.
                value.assign(&column);

                Ok::<_, PyErr>(())
            },
        )?;

        // Construct the categorical trajectory.
        let inner = CatTrj::new(states, values, time);

        Ok(Self { inner })
    }

    /// Returns the labels of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the labels of the categorical trajectory.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the states of the categorical trajectory.
    ///
    pub fn states<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<&'a str, Bound<'a, PyTuple>>> {
        Ok(self
            .inner
            .states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.as_ref();
                let states = states.iter().map(String::as_str);
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }
}

#[pyclass(name = "CatTrjs")]
#[derive(Clone, Debug)]
pub struct PyCatTrjs {
    inner: CatTrjs,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatTrjs, CatTrjs);

#[pymethods]
impl PyCatTrjs {
    /// Constructs a new categorical trajectories from an iterable of Pandas DataFrames.
    ///
    /// # Arguments
    ///
    /// * `dfs` - An iterable of Pandas DataFrames containing the trajectory data.
    /// * `with_states` - An optional dictionary of states.
    ///
    /// # Notes
    ///
    /// * Each data frame must contain a column named "time" that represents the time of each event.
    /// * Every other column in the data frame must represent a categorical variable.
    ///
    /// # Returns
    ///
    /// A new categorical trajectories instance.
    ///
    #[new]
    #[pyo3(signature = (dfs, with_states = None))]
    pub fn new(
        py: Python<'_>,
        dfs: &Bound<'_, PyAny>,
        with_states: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Convert the iterable to a Vec<PyAny>.
        let dfs: Vec<PyCatTrj> = dfs
            .try_iter()?
            .map(|df| PyCatTrj::new(py, &df.unwrap(), with_states))
            .collect::<PyResult<_>>()?;
        // Convert the Vec<PyCatTrj> to Vec<CatTrj>.
        let dfs: Vec<_> = dfs.into_iter().map(Into::into).collect();
        // Create a new CatTrjs with the given parameters.
        let inner = CatTrjs::new(dfs);

        Ok(Self { inner })
    }

    /// Returns the labels of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the labels of the categorical trajectory.
    ///
    #[inline]
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the states of the categorical trajectory.
    ///
    pub fn states<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<&'a str, Bound<'a, PyTuple>>> {
        Ok(self
            .inner
            .states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.as_ref();
                let states = states.iter().map(String::as_str);
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }
}
