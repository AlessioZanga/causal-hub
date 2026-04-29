use std::sync::{Arc, RwLock};

use backend::{
    datasets::{Dataset, GaussIncTable, GaussType, IncDataset},
    models::Labelled,
    random::{Random, RngGaussIncTable},
};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, ToPyArray, ndarray::prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    datasets::{PyGaussTable, PyMissingMechanism, PyMissingTable},
    error::to_pyerr,
    impl_from_into_lock,
};

/// A Gaussian incomplete tabular dataset.
#[gen_stub_pyclass]
#[pyclass(name = "GaussIncTable", module = "causal_hub.datasets", from_py_object)]
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
        GaussIncTable::new(labels, values)
            .map(Into::into)
            .map_err(to_pyerr)
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

    /// Generates a random gaussian incomplete tabular dataset.
    ///
    /// Parameters
    /// ----------
    /// dataset : GaussTable
    ///     A gaussian tabular dataset instance.
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
    /// GaussIncTable
    ///     A random gaussian incomplete tabular dataset instance.
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
        dataset: &Bound<'_, PyGaussTable>,
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

        // Create a new RngGaussIncTable and generate a random gaussian incomplete tabular dataset.
        RngGaussIncTable::new(&mut rng, &dataset, &missing_mechanism, p_min, p_max)
            .and_then(|mut x| x.random())
            .map(Into::into)
            .map_err(to_pyerr)
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
        GaussIncTable::new(labels, values)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Constructs a new gaussian incomplete tabular dataset from a Polars DataFrame.
    ///
    /// Parameters
    /// ----------
    ///
    /// df: polars.DataFrame
    ///     A Polars DataFrame containing gaussian columns with missing values.
    ///
    /// Returns
    /// -------
    /// GaussIncTable
    ///     A new gaussian incomplete tabular dataset instance.
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

        // Get values.
        let n_rows: usize = df.getattr("shape")?.get_item(0)?.extract()?;
        let mut values = Array2::zeros((n_rows, labels.len()));
        for (i, label) in labels.iter().enumerate() {
            let column = df.call_method1("get_column", (label,))?;
            let dtype = column.getattr("dtype")?.str()?.extract::<String>()?;
            if !dtype.contains("Float64") {
                return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                    "Column '{label}' must be Float64, found '{dtype}'."
                )));
            }

            let items: Vec<Option<GaussType>> = column.call_method0("to_list")?.extract()?;
            let items: Vec<GaussType> = items
                .into_iter()
                .map(|x| x.unwrap_or(GaussType::NAN))
                .collect();
            values.column_mut(i).assign(&Array1::from_vec(items));
        }

        let labels = labels.into_iter().collect();
        GaussIncTable::new(labels, values)
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
        // values with NANs.
        let values = inner.values().to_pyarray(py);

        // Construct the DataFrame.
        let kwargs = PyDict::new(py);
        kwargs.set_item("columns", labels)?;
        let df = pd.call_method("DataFrame", (values,), Some(&kwargs))?;

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
        let values = inner.values();

        for (i, label) in labels.iter().enumerate() {
            let col: Vec<GaussType> = values.column(i).iter().copied().collect();
            let series = pl.getattr("Series")?.call1((label.clone(), col))?;
            data.set_item(label, series)?;
        }

        let df = pl.getattr("DataFrame")?.call1((data,))?;
        Ok(df.into())
    }
}
