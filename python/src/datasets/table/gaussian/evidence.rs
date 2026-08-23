use std::sync::{Arc, RwLock};

use backend::{
    datasets::{GaussEv, GaussEvT},
    models::Labelled,
    types::Labels,
};
use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyAnyMethods, PyDict, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::{
    error::{Error, to_pyerr},
    impl_from_into_lock,
};

/// A Gaussian evidence.
///
#[gen_stub_pyclass]
#[pyclass(name = "GaussEv", module = "causal_hub.datasets", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyGaussEv {
    inner: Arc<RwLock<GaussEv>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyGaussEv, GaussEv);

impl PyGaussEv {
    /// Constructs a gaussian evidence from a generic Python object.
    ///
    /// Accepted inputs are:
    /// - `GaussEv`
    /// - `dict[str, float]`
    ///
    pub fn from_any(evidence: &Bound<'_, PyAny>, with_labels: &Labels) -> PyResult<Self> {
        if let Ok(evidence) = evidence.extract::<PyGaussEv>() {
            Ok(evidence)
        } else if let Ok(evidence) = evidence.cast::<PyDict>() {
            let evidences: Vec<_> = evidence
                .items()
                .into_iter()
                .map(|key_value| {
                    let (key, value) =
                        key_value.extract::<(Bound<'_, PyAny>, Bound<'_, PyAny>)>()?;
                    let label = key.extract::<String>()?;
                    let value = value.extract::<f64>()?;

                    let event = with_labels
                        .get_index_of(&label)
                        .ok_or_else(|| Error::new_err(format!("Variable '{}' not found", label)))?;

                    Ok(GaussEvT::CertainPositive { event, value })
                })
                .collect::<PyResult<_>>()?;

            GaussEv::new(with_labels.clone(), evidences)
                .map(Into::into)
                .map_err(to_pyerr)
        } else {
            Err(PyErr::new::<PyTypeError, _>(
                "Expected evidence to be a GaussEv or a dict[str, float].",
            ))
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussEv {
    /// Returns the labels of the gaussian evidence.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels of the gaussian evidence.
    ///
    #[inline]
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Constructs a new gaussian evidence from a dictionary.
    ///
    /// Parameters
    /// ----------
    /// evidence: dict[str, float]
    ///     A dictionary mapping variable labels to observed values.
    /// with_labels: Iterable[str] | None
    ///     Optional full labels ordering. If not provided, labels are inferred from `evidence` keys.
    ///
    /// Returns
    /// -------
    /// GaussEv
    ///     A new gaussian evidence instance.
    ///
    #[classmethod]
    #[pyo3(signature = (
        evidence,
        with_labels = None
    ))]
    pub fn from_dict(
        _cls: &Bound<'_, PyType>,
        evidence: &Bound<'_, PyDict>,
        with_labels: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let labels: Labels = match with_labels {
            Some(labels) => labels
                .try_iter()?
                .map(|x| x?.extract::<String>())
                .collect::<PyResult<_>>()?,
            None => evidence
                .keys()
                .iter()
                .map(|x| x.extract::<String>())
                .collect::<PyResult<_>>()?,
        };

        Self::from_any(evidence.as_any(), &labels)
    }
}
