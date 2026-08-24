use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatEv, CatEvT},
    models::{CatSupport, HasLabels},
    types::Set,
};
use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::{
    error::{Error, to_pyerr},
    impl_from_into_lock,
};

/// A categorical evidence.
///
#[gen_stub_pyclass]
#[pyclass(name = "CatEv", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCatEv {
    inner: Arc<RwLock<CatEv>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatEv, CatEv);

impl PyCatEv {
    /// Constructs a categorical evidence from a generic Python object.
    ///
    /// Accepted inputs are:
    /// - `CatEv`
    /// - `dict[str, str]`
    ///
    pub fn from_any(evidence: &Bound<'_, PyAny>, with_states: &CatSupport) -> PyResult<Self> {
        if let Ok(evidence) = evidence.extract::<PyCatEv>() {
            Ok(evidence)
        } else if let Ok(evidence) = evidence.cast::<PyDict>() {
            let evidences: Vec<_> = evidence
                .items()
                .into_iter()
                .map(|key_value| {
                    let (key, value) =
                        key_value.extract::<(Bound<'_, PyAny>, Bound<'_, PyAny>)>()?;
                    let label = key.extract::<String>()?;
                    let state = value.extract::<String>()?;

                    let event = with_states
                        .get_index_of(&label)
                        .ok_or_else(|| Error::new_err(format!("Variable '{}' not found", label)))?;
                    let state = with_states[event].get_index_of(&state).ok_or_else(|| {
                        Error::new_err(format!(
                            "State '{}' not found for variable '{}'",
                            state, label
                        ))
                    })?;

                    Ok(CatEvT::CertainPositive { event, state })
                })
                .collect::<PyResult<_>>()?;

            CatEv::new(with_states.clone(), evidences)
                .map(Into::into)
                .map_err(to_pyerr)
        } else {
            Err(PyErr::new::<PyTypeError, _>(
                "Expected evidence to be a CatEv or a dict[str, str].",
            ))
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCatEv {
    /// Returns the labels of the categorical evidence.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels of the categorical evidence.
    ///
    #[inline]
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the support of the categorical evidence.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A reference to the support of the categorical evidence.
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

    /// Constructs a new categorical evidence from a dictionary.
    ///
    /// Parameters
    /// ----------
    /// evidence: dict[str, str]
    ///     A dictionary mapping variable labels to observed states.
    /// with_states: dict[str, Iterable[str]]
    ///     A dictionary mapping each variable to all its possible states.
    ///
    /// Returns
    /// -------
    /// CatEv
    ///     A new categorical evidence instance.
    ///
    #[classmethod]
    pub fn from_dict(
        _cls: &Bound<'_, PyType>,
        evidence: &Bound<'_, PyDict>,
        with_states: &Bound<'_, PyDict>,
    ) -> PyResult<Self> {
        // Convert the support dictionary.
        let mut support: CatSupport = with_states
            .items()
            .into_iter()
            .map(|key_value| {
                let (key, value) = key_value.extract::<(Bound<'_, PyAny>, Bound<'_, PyAny>)>()?;
                let key = key.extract::<String>()?;
                let value: Set<_> = value
                    .try_iter()?
                    .map(|x| x?.extract::<String>())
                    .collect::<PyResult<_>>()?;

                Ok((key, value))
            })
            .collect::<PyResult<_>>()?;

        // Sort support.
        support.sort_keys();
        support.values_mut().for_each(Set::sort);

        Self::from_any(evidence.as_any(), &support)
    }
}
