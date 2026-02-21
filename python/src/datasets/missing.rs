use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{MissingMechanism, MissingMethod, MissingType as MissingType_},
    models::Labelled,
    random::{Random, RngMissingMechanism},
};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{error::to_pyerr, impl_from_into_lock, models::PyDiGraph};

/// Missing mechanism types.
#[gen_stub_pyclass_enum]
#[pyclass(name = "MissingType", module = "causal_hub.datasets")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PyMissingType {
    /// Missing Completely At Random.
    MCAR,
    /// Missing At Random.
    MAR,
    /// Missing Not At Random.
    MNAR,
}

impl From<MissingType_> for PyMissingType {
    fn from(value: MissingType_) -> Self {
        match value {
            MissingType_::MCAR => Self::MCAR,
            MissingType_::MAR => Self::MAR,
            MissingType_::MNAR => Self::MNAR,
        }
    }
}

impl From<PyMissingType> for MissingType_ {
    fn from(value: PyMissingType) -> Self {
        match value {
            PyMissingType::MCAR => Self::MCAR,
            PyMissingType::MAR => Self::MAR,
            PyMissingType::MNAR => Self::MNAR,
        }
    }
}

/// Missing data handling method.
#[gen_stub_pyclass_enum]
#[pyclass(name = "MissingMethod", module = "causal_hub.datasets")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PyMissingMethod {
    /// List-wise deletion.
    LW,
    /// Pair-wise deletion.
    PW,
    /// Inverse probability weighting.
    IPW,
    /// Augmented inverse probability weighting.
    AIPW,
}

impl From<MissingMethod> for PyMissingMethod {
    fn from(value: MissingMethod) -> Self {
        match value {
            MissingMethod::LW => Self::LW,
            MissingMethod::PW => Self::PW,
            MissingMethod::IPW => Self::IPW,
            MissingMethod::AIPW => Self::AIPW,
            _ => panic!("Unsupported missing method"),
        }
    }
}

impl From<PyMissingMethod> for MissingMethod {
    fn from(value: PyMissingMethod) -> Self {
        match value {
            PyMissingMethod::LW => Self::LW,
            PyMissingMethod::PW => Self::PW,
            PyMissingMethod::IPW => Self::IPW,
            PyMissingMethod::AIPW => Self::AIPW,
        }
    }
}

/// A struct representing the missing data indicators.
#[gen_stub_pyclass]
#[pyclass(name = "MissingMechanism", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyMissingMechanism {
    inner: Arc<RwLock<MissingMechanism>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyMissingMechanism, MissingMechanism);

#[gen_stub_pymethods]
#[pymethods]
impl PyMissingMechanism {
    /// Create a new missing mechanism.
    ///
    /// Parameters
    /// ----------
    /// labels: list[str]
    ///     A list of strings containing the labels of the variables.
    /// pr: dict[int, set[int]]
    ///     A dictionary mapping missing variable indices to sets of indices that cause missingness.
    ///
    /// Returns
    /// -------
    /// MissingMechanism
    ///     A new missing mechanism instance.
    ///
    #[new]
    pub fn new(labels: Vec<String>, pr: BTreeMap<usize, BTreeSet<usize>>) -> PyResult<Self> {
        // Construct the labels.
        let labels = labels.into_iter().collect();
        // Construct the missing mechanism.
        let pr = pr
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        // Create the missing mechanism.
        MissingMechanism::new(labels, pr)
            .map(Into::into)
            .map_err(to_pyerr)
    }

    /// The labels of the variables.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A list of strings containing the labels of the variables.
    ///
    pub fn labels(&self) -> Vec<String> {
        self.lock().labels().iter().cloned().collect()
    }

    /// Returns the number of missing variables.
    ///
    /// Returns
    /// -------
    /// int
    ///     The number of missing variables.
    ///
    pub fn __len__(&self) -> usize {
        self.lock().len()
    }

    /// Checks if the missing mechanism is empty.
    ///
    /// Returns
    /// -------
    /// bool
    ///     True if the missing mechanism is empty, False otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        self.lock().is_empty()
    }

    /// Returns the missing variables.
    ///
    /// Returns
    /// -------
    /// list[int]
    ///     A list of indices of the missing variables.
    ///
    pub fn keys(&self) -> Vec<usize> {
        self.lock().keys().cloned().collect()
    }

    /// Returns the causes of missingness for each missing variable.
    ///
    /// Returns
    /// -------
    /// list[set[int]]
    ///     A list of sets of indices that cause missingness for each missing variable.
    ///
    pub fn values(&self) -> Vec<BTreeSet<usize>> {
        self.lock()
            .values()
            .map(|v| v.iter().cloned().collect())
            .collect()
    }

    /// Checks if a variable is missing.
    ///
    /// Parameters
    /// ----------
    /// x: int
    ///     The index of the variable to check.
    ///
    /// Returns
    /// -------
    /// bool
    ///     True if the variable is missing, False otherwise.
    ///
    pub fn contains_key(&self, x: usize) -> bool {
        self.lock().contains_key(&x)
    }

    /// Returns the causes of missingness for a given variable.
    ///
    /// Parameters
    /// ----------
    /// x: int
    ///     The index of the variable to get the causes of missingness for.
    ///
    /// Returns
    /// -------
    /// set[int] | None
    ///     A set of indices that cause missingness for the variable, or None if the variable is not missing.
    ///
    pub fn get(&self, x: usize) -> Option<BTreeSet<usize>> {
        self.lock().get(&x).map(|v| v.iter().cloned().collect())
    }

    /// Inserts a missing variable and its causes.
    ///
    /// Parameters
    /// ----------
    /// x: int
    ///     The index of the missing variable.
    /// y: set[int]
    ///     A set of indices that cause missingness for the variable.
    ///
    pub fn insert(&mut self, x: usize, y: BTreeSet<usize>) {
        self.lock_mut().insert(x, y.into_iter().collect());
    }

    /// Generates a random missing mechanism.
    ///
    /// Parameters
    /// ----------
    /// graph: DiGraph
    ///     The graph on which to generate the missingness mechanism.
    /// missing: MissingType
    ///     The type of missingness mechanism to generate.
    /// p: float
    ///     The ratio of missing variables.
    /// seed: int, default=31
    ///     The seed for the random number generator.
    ///
    /// Returns
    /// -------
    /// MissingMechanism
    ///     A random missing mechanism.
    ///
    #[classmethod]
    #[pyo3(signature = (
        graph,
        missing,
        p,
        seed = 31
    ))]
    pub fn random(
        _cls: &Bound<'_, PyType>,
        graph: &Bound<'_, PyDiGraph>,
        missing: PyMissingType,
        p: f64,
        seed: u64,
    ) -> PyResult<Self> {
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

        // Get the inner graph.
        let graph = graph.borrow();
        let graph = graph.lock();
        // Convert the missing type.
        let missing = missing.into();

        // Create a new RngMissingMechanism and generate a random missing mechanism.
        RngMissingMechanism::new(&mut rng, &graph, missing, p)
            .and_then(|mut x| x.random())
            .map(Into::into)
            .map_err(to_pyerr)
    }
}
