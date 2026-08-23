use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    io::JsonIO,
    models::{CPD, CatCPD, CatSupport, Labelled},
    random::{Random, RngCatCPD},
};
use numpy::{PyArray2, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{error::to_pyerr, impl_from_into_lock};

/// A struct representing a categorical conditional probability distribution.
///
#[gen_stub_pyclass]
#[pyclass(name = "CatCPD", module = "causal_hub.models", eq, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCatCPD {
    inner: Arc<RwLock<CatCPD>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatCPD, CatCPD);

impl PartialEq for PyCatCPD {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCatCPD {
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

    /// Returns the states of the conditioned variable.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     The states of the conditioned variable.
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
            .collect()
    }

    /// Returns the shape of the conditioned variable.
    ///
    /// Returns
    /// -------
    /// list[int]
    ///     The shape of the conditioned variable.
    ///
    pub fn shape(&self) -> PyResult<Vec<usize>> {
        Ok(self.lock().shape().to_vec())
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

    /// Returns the states of the conditioning variables.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     The states of the conditioning variables.
    ///
    pub fn conditioning_support<'a>(
        &'a self,
        py: Python<'a>,
    ) -> PyResult<BTreeMap<String, Bound<'a, PyTuple>>> {
        self.lock()
            .conditioning_support()
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

    /// Returns the shape of the conditioning variables.
    ///
    /// Returns
    /// -------
    /// list[int]
    ///     The shape of the conditioning variables.
    ///
    pub fn conditioning_shape(&self) -> PyResult<Vec<usize>> {
        Ok(self.lock().conditioning_shape().to_vec())
    }

    /// Returns the parameters.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A reference to the parameters.
    ///
    pub fn parameters<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self.lock().parameters().to_pyarray(py))
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
                // Add the conditional counts.
                dict.set_item(
                    "fitted_conditional_counts",
                    s.fitted_conditional_counts().to_pyarray(py),
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

    /// Returns the string representation of the CatCPD.
    ///
    pub fn __repr__(&self) -> PyResult<String> {
        // Get the string representation of the CatCPD.
        Ok(self.lock().to_string())
    }

    /// Generates a random categorical conditional probability distribution.
    ///
    /// Parameters
    /// ----------
    /// states: dict[str, tuple[str, ...]]
    ///     The states of the variable.
    /// conditioning_support: dict[str, tuple[str, ...]]
    ///     The support of the conditioning variables.
    /// alpha: float, default=1.0
    ///     The parameter of the Dirichlet distribution.
    /// seed: int, default=31
    ///     The seed for the random number generator.
    ///
    /// Returns
    /// -------
    /// CatCPD
    ///     A random categorical conditional probability distribution.
    ///
    #[classmethod]
    #[pyo3(signature = (
        states,
        conditioning_support,
        alpha = 1.0,
        seed = 31
    ))]
    pub fn random(
        _cls: &Bound<'_, PyType>,
        states: &Bound<'_, PyDict>,
        conditioning_support: &Bound<'_, PyDict>,
        alpha: f64,
        seed: u64,
    ) -> PyResult<Self> {
        // Convert the PyDict to a CatSupport.
        let mut inner_states = CatSupport::default();
        for (label, states) in states {
            let label = label.extract::<String>()?;
            let states = states.extract::<Vec<String>>()?;
            inner_states.insert(label, states.into_iter().collect());
        }

        // Convert the PyDict to a CatSupport.
        let mut inner_conditioning_support = CatSupport::default();
        for (label, states) in conditioning_support {
            let label = label.extract::<String>()?;
            let states = states.extract::<Vec<String>>()?;
            inner_conditioning_support.insert(label, states.into_iter().collect());
        }

        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Create a new RngCatCPD and generate a random CPD.
        RngCatCPD::new(&mut rng, &inner_states, &inner_conditioning_support, alpha)
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
    /// CatCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_string(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        CatCPD::from_json_string(json)
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
    /// CatCPD
    ///     A new instance.
    ///
    #[classmethod]
    pub fn from_json_file(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        CatCPD::from_json_file(path)
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
