use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    io::JsonIO,
    models::{CPD, GaussCPD, Labelled},
    random::{Random, RngGaussCPD},
    types::Labels,
};
use numpy::prelude::*;
use pyo3::{
    prelude::*,
    types::{PyDict, PyType},
};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{error::to_pyerr, impl_from_into_lock};

/// A struct representing a Gaussian conditional probability distribution.
///
#[gen_stub_pyclass]
#[pyclass(name = "GaussCPD", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyGaussCPD {
    inner: Arc<RwLock<GaussCPD>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyGaussCPD, GaussCPD);

impl PartialEq for PyGaussCPD {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussCPD {
    /// Returns the label of the conditioned variable.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the label.
    ///
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the labels of the conditioned variables.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the conditioning labels.
    ///
    pub fn conditioning_labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().conditioning_labels().iter().cloned().collect())
    }

    /// Returns the support of each variable.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[float, float]]
    ///     A map from variable names to (min, max) ranges.
    ///
    pub fn support(&self) -> PyResult<BTreeMap<String, (f64, f64)>> {
        let lock = self.lock();
        Ok(CPD::support(&*lock)
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect())
    }

    /// Returns the support of the conditioning variables.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[float, float]]
    ///     A map from conditioning variable names to (min, max) ranges.
    ///
    pub fn conditioning_support(&self) -> PyResult<BTreeMap<String, (f64, f64)>> {
        let lock = self.lock();
        Ok(CPD::conditioning_support(&*lock)
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect())
    }

    /// Returns the parameters.
    ///
    /// Returns
    /// -------
    /// dict[str, ...]
    ///     A reference to the parameters.
    ///
    pub fn parameters<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        // Allocate the dictionary.
        let dict = PyDict::new(py);
        // Get a lock on the inner field.
        let lock = self.lock();
        // Get the parameters.
        let parameters = lock.parameters();
        // Add the coefficients matrix.
        dict.set_item("coefficients", parameters.coefficients().to_pyarray(py))?;
        // Add the intercept vector.
        dict.set_item("intercept", parameters.intercept().to_pyarray(py))?;
        // Add the covariance matrix.
        dict.set_item("covariance", parameters.covariance().to_pyarray(py))?;
        // Return the dictionary.
        Ok(dict)
    }

    /// Returns the parameters size.
    ///
    /// Returns
    /// -------
    /// int
    ///     The parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.lock().parameters_size())
    }

    /// Returns the fitted statistics used to fit the distribution, if any.
    ///
    /// Returns
    /// -------
    /// dict[str, ...] | None
    ///     A dictionary containing the fitted statistics used to fit the distribution, if any.
    ///
    pub fn fitted_statistics<'a>(&self, py: Python<'a>) -> PyResult<Option<Bound<'a, PyDict>>> {
        self.lock()
            .fitted_statistics()
            .map(|s| {
                // Allocate the dictionary.
                let dict = PyDict::new(py);
                // Add the response mean vector.
                dict.set_item(
                    "fitted_response_mean",
                    s.fitted_response_mean().to_pyarray(py),
                )?;
                // Add the design mean vector.
                dict.set_item("fitted_design_mean", s.fitted_design_mean().to_pyarray(py))?;
                // Add the response covariance matrix.
                dict.set_item(
                    "fitted_response_covariance",
                    s.fitted_response_covariance().to_pyarray(py),
                )?;
                // Add the cross covariance matrix.
                dict.set_item(
                    "fitted_cross_covariance",
                    s.fitted_cross_covariance().to_pyarray(py),
                )?;
                // Add the design covariance matrix.
                dict.set_item(
                    "fitted_design_covariance",
                    s.fitted_design_covariance().to_pyarray(py),
                )?;
                // Add the sample size.
                dict.set_item("fitted_size", s.fitted_size())?;
                // Return the dictionary.
                Ok(dict)
            })
            .transpose()
    }

    /// Returns the log-likelihood given the distribution, if any.
    ///
    /// Returns
    /// -------
    /// float | None
    ///     The log-likelihood given the distribution, if any.
    ///
    pub fn fitted_log_likelihood(&self) -> PyResult<Option<f64>> {
        Ok(self.lock().fitted_log_likelihood())
    }

    /// Generates a random Gaussian conditional probability distribution.
    ///
    /// Parameters
    /// ----------
    /// labels: Iterable[str]
    ///     The labels of the target variables.
    /// conditioning_labels: Iterable[str]
    ///     The labels of the conditioning variables.
    /// s_a: float, default=1.0
    ///     The standard deviation of the regression coefficients.
    /// s_b: float, default=1.0
    ///     The standard deviation of the intercept.
    /// e: float, default=1e-6
    ///     A small positive constant for covariance regularization.
    /// seed: int, default=31
    ///     The seed for the random number generator.
    ///
    /// Returns
    /// -------
    /// GaussCPD
    ///     A random Gaussian conditional probability distribution.
    ///
    #[classmethod]
    #[pyo3(signature = (
        labels,
        conditioning_labels,
        s_a = 1.0,
        s_b = 1.0,
        e = 1e-6,
        seed = 31
    ))]
    pub fn random(
        _cls: &Bound<'_, PyType>,
        labels: &Bound<'_, PyAny>,
        conditioning_labels: &Bound<'_, PyAny>,
        s_a: f64,
        s_b: f64,
        e: f64,
        seed: u64,
    ) -> PyResult<Self> {
        // Convert the PyAny to a Labels.
        let labels: Labels = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Convert the PyAny to a Labels.
        let conditioning_labels: Labels = conditioning_labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;

        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Create a new RngGaussCPD and generate a random CPD.
        RngGaussCPD::new(&mut rng, &labels, &conditioning_labels, s_a, s_b, e)
            .and_then(|mut x| x.random())
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Read instance from a JSON string.
    ///
    /// Parameters
    /// ----------
    /// json: str
    ///     The JSON string to read from.
    ///
    /// Returns
    /// -------
    /// GaussCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        GaussCPD::from_json_string(json)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON string.
    ///
    /// Returns
    /// -------
    /// str
    ///     A JSON string representation of the instance.
    ///
    pub fn to_json_string(&self) -> PyResult<String> {
        self.lock().to_json_string().map_err(to_pyerr)
    }

    /// Read instance from a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to read from.
    ///
    /// Returns
    /// -------
    /// GaussCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        GaussCPD::from_json_file(path)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// Write instance to a JSON file.
    ///
    /// Parameters
    /// ----------
    /// path: str
    ///     The path to the JSON file to write to.
    ///
    pub fn to_json_file(&self, path: &str) -> PyResult<()> {
        self.lock().to_json_file(path).map_err(to_pyerr)
    }
}
