use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatIncTable, CatType, Dataset, IncDataset},
    models::{CatSupport, Labelled},
    random::{Random, RngCatIncTable},
};
use numpy::{PyArray1, PyArray2, PyArrayMethods, ToPyArray, ndarray::prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    datasets::{PyCatTable, PyMissingMechanism, PyMissingTable},
    error::to_pyerr,
    impl_from_into_lock,
};

/// A categorical incomplete tabular dataset.
///
#[gen_stub_pyclass]
#[pyclass(name = "CatIncTable", module = "causal_hub.datasets", from_py_object)]
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

    /// Generates a random categorical incomplete tabular dataset.
    ///
    /// Parameters
    /// ----------
    /// dataset : CatTable
    ///     A categorical tabular dataset instance.
    /// missing_mechanism : MissingMechanism
    ///     A missing mechanism instance.
    /// p_min : float
    ///     The minimum probability of missingness.
    /// p_max : float
    ///     The maximum probability of missingness.
    /// seed : int, optional
    ///     The seed for the random number generator. Default is 31.
    ///
    /// Returns
    /// -------
    /// CatIncTable
    ///     A random categorical incomplete tabular dataset instance.
    ///
    #[classmethod]
    #[pyo3(signature = (
        dataset,
        missing_mechanism,
        p_min,
        p_max,
        seed = 31
    ))]
    pub fn random(
        _cls: &Bound<'_, PyType>,
        dataset: &Bound<'_, PyCatTable>,
        missing_mechanism: &Bound<'_, PyMissingMechanism>,
        p_min: f64,
        p_max: f64,
        seed: u64,
    ) -> PyResult<Self> {
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Get the inner dataset.
        let dataset = dataset.borrow();
        let dataset = dataset.lock();
        // Get the inner missing mechanism.
        let missing_mechanism = missing_mechanism.borrow();
        let missing_mechanism = missing_mechanism.lock();

        // Create a new RngCatIncTable and generate a random categorical incomplete tabular dataset.
        RngCatIncTable::new(&mut rng, &dataset, &missing_mechanism, p_min, p_max)
            .and_then(|mut x| x.random())
            .map(Into::into)
            .map_err(to_pyerr)
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
        let mut support = CatSupport::default();
        for label in &labels {
            // Get the categories of the column.
            let categories = df.get_item(label)?.getattr("cat")?.getattr("categories")?;
            let categories: Vec<String> = categories.call_method0("to_list")?.extract()?;
            // Add the categories to the support.
            support.insert(label.clone(), categories.into_iter().collect());
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
        CatIncTable::new(support, values)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Constructs a new categorical incomplete tabular dataset from a Polars DataFrame.
    ///
    /// Parameters
    /// ----------
    ///
    /// df: polars.DataFrame
    ///     A Polars DataFrame containing categorical columns with missing values.
    ///
    /// Returns
    /// -------
    /// CatIncTable
    ///     A new categorical incomplete tabular dataset instance.
    ///
    #[classmethod]
    pub fn from_polars(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        df: Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Import the polars module.
        let pl = py.import("polars")?;

        // Check that the object is a DataFrame.
        if !df.is_instance(&pl.getattr("DataFrame")?)? {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "Input must be a Polars DataFrame.",
            ));
        }

        // Get labels.
        let labels: Vec<String> = df.getattr("columns")?.extract()?;

        // Get categories.
        let mut support = CatSupport::default();
        for label in &labels {
            let column = df.call_method1("get_column", (label,))?;
            let dtype = column.getattr("dtype")?.str()?.extract::<String>()?;
            if !(dtype.contains("Categorical") || dtype.contains("Enum")) {
                return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                    "Column '{label}' must be categorical, found '{dtype}'."
                )));
            }

            let categories = column.call_method0("drop_nulls")?.call_method0("unique")?;
            let categories: Vec<String> = categories
                .try_iter()?
                .map(|x| x?.extract::<String>())
                .collect::<PyResult<_>>()?;
            support.insert(label.clone(), categories.into_iter().collect());
        }

        // Get values.
        let n_rows: usize = df.getattr("shape")?.get_item(0)?.extract()?;
        let mut values = Array2::zeros((n_rows, labels.len()));
        for (i, label) in labels.iter().enumerate() {
            let column = df.call_method1("get_column", (label,))?;
            let items: Vec<Option<String>> = column.call_method0("to_list")?.extract()?;
            let column_values: Result<Vec<CatType>, _> = items
                .into_iter()
                .map(|x| match x {
                    Some(x) => support[label]
                        .get_index_of(&x)
                        .map(|idx| idx as CatType)
                        .ok_or_else(|| {
                            pyo3::exceptions::PyValueError::new_err(format!(
                                "Unknown state '{x}' for column '{label}'."
                            ))
                        }),
                    None => Ok(255),
                })
                .collect();
            values
                .column_mut(i)
                .assign(&Array1::from_vec(column_values?));
        }

        CatIncTable::new(support, values)
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
    pub fn to_pandas(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        // Get reference to Pandas.
        let pd = py.import("pandas")?;
        // Get reference to the inner dataset.
        let inner = self.lock();
        let labels: Vec<String> = inner.labels().iter().cloned().collect();
        let states = inner.support();
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

    /// Converts the dataset to a Polars DataFrame.
    ///
    /// Returns
    /// -------
    /// polars.DataFrame
    ///     A Polars DataFrame.
    ///
    pub fn to_polars(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        // Import the polars module.
        let pl = py.import("polars")?;

        // Build dictionary of polars.Series columns.
        let data = PyDict::new(py);

        let inner = self.lock();
        let labels: Vec<String> = inner.labels().iter().cloned().collect();
        let states = inner.support();
        let values = inner.values();

        for (i, label) in labels.iter().enumerate() {
            let col: Vec<Option<String>> = values
                .column(i)
                .iter()
                .map(|&x| {
                    if x == 255 {
                        None
                    } else {
                        Some(states[label][x as usize].clone())
                    }
                })
                .collect();

            let series = pl.getattr("Series")?.call1((label.clone(), col))?;
            let series = series.call_method1("cast", (pl.getattr("Categorical")?,))?;
            data.set_item(label, series)?;
        }

        let df = pl.getattr("DataFrame")?.call1((data,))?;
        Ok(df.into())
    }
}
