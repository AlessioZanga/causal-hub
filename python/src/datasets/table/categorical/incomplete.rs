use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatIncTable, CatType, Dataset, IncDataset},
    models::Labelled,
    types::States,
};
use numpy::{PyArray1, PyArray2, PyArrayMethods, ToPyArray, ndarray::prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::{datasets::PyMissingTable, error::to_pyerr, impl_from_into_lock};

/// A categorical incomplete tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "CatIncTable", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatIncTable {
    inner: Arc<RwLock<CatIncTable>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatIncTable, CatIncTable);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatIncTable {
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
        self.lock()
            .states()
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

    /// The missing information table.
    ///
    /// Returns
    /// -------
    /// MissingTable
    ///     A missing information table instance.
    ///
    pub fn missing(&self) -> PyResult<PyMissingTable> {
        Ok(self.lock().missing().clone().into())
    }

    /// Constructs a new categorical incomplete tabular dataset from a Pandas DataFrame.
    ///
    /// Parameters
    /// ----------
    ///
    /// df: pandas.DataFrame
    ///     A Pandas DataFrame containing categorical columns with missing values.
    ///
    /// Returns
    /// -------
    /// CatIncTable
    ///     A new categorical incomplete tabular dataset instance.
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
        // Get categories.
        let mut states = States::default();
        for label in &labels {
            // Get the categories of the column.
            let categories = df.get_item(label)?.getattr("cat")?.getattr("categories")?;
            let categories: Vec<String> = categories.call_method0("to_list")?.extract()?;
            // Add the categories to the states.
            states.insert(label.clone(), categories.into_iter().collect());
        }

        // Get values.
        let numpy = py.import("numpy")?;
        let values = Array2::zeros((df.getattr("shape")?.get_item(0)?.extract()?, labels.len()));
        let mut values = values;
        for (i, label) in labels.iter().enumerate() {
            let column = df.get_item(label)?;
            let codes = column.getattr("cat")?.getattr("codes")?;
            let codes = numpy.call_method1("asarray", (codes, "int64"))?;
            let codes: Bound<'_, PyArray1<i64>> = codes.extract()?;
            let codes = codes.readonly();
            values.column_mut(i).assign(
                &codes
                    .as_array()
                    .mapv(|x| if x < 0 { 255 } else { x as CatType }),
            );
        }

        // Construct the categorical incomplete tabular dataset.
        Ok(CatIncTable::new(states, values).map_err(to_pyerr)?.into())
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
        let states = inner.states();
        let values = inner.values();

        // Construct the DataFrame from a dictionary to avoid chained assignment warnings.
        let dict = PyDict::new(py);
        for (i, label) in labels.iter().enumerate() {
            // Get the categories and codes for the variable.
            let categories: Vec<String> = states[label].iter().cloned().collect();
            let codes: Vec<i16> = values
                .column(i)
                .iter()
                .map(|&x| if x == 255 { -1 } else { x as i16 })
                .collect();
            // Construct the categorical series.
            let series = pd
                .getattr("Categorical")?
                .call_method1("from_codes", (codes.into_pyobject(py)?, categories))?;
            // Add the series to the dictionary.
            dict.set_item(label, series)?;
        }

        // Construct the DataFrame.
        let kwargs = PyDict::new(py);
        kwargs.set_item("columns", labels)?;
        let df = pd.call_method("DataFrame", (dict,), Some(&kwargs))?;

        Ok(df.into())
    }
}
