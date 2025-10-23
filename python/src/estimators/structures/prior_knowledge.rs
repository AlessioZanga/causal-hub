use std::sync::{Arc, RwLock};

use backend::{estimators::PK, types::Labels};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A struct representing prior knowledge.
#[gen_stub_pyclass]
#[pyclass(name = "PK", module = "causal_hub.estimation", eq)]
#[derive(Clone, Debug)]
pub struct PyPK {
    inner: Arc<RwLock<PK>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyPK, PK);

impl PartialEq for PyPK {
    fn eq(&self, other: &Self) -> bool {
        (*self.lock()).eq(&*other.lock())
    }
}

impl Eq for PyPK {}

#[gen_stub_pymethods]
#[pymethods]
impl PyPK {
    #[new]
    fn new(
        labels: &Bound<'_, PyAny>,
        forbidden: &Bound<'_, PyAny>,
        required: &Bound<'_, PyAny>,
        temporal_order: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        // Convert Python iterators on Python strings to Rust iterators on Rust strings.
        let labels: Labels = labels
            .try_iter()?
            .map(|x| x?.extract::<String>())
            .collect::<PyResult<_>>()?;
        // Do the same for the other parameters, but map strings to indices.
        let forbidden: Vec<(usize, usize)> = forbidden
            .try_iter()?
            .map(|x| {
                // Get the strings and convert them to indices.
                x?.extract::<(String, String)>().map(|(a, b)| {
                    (
                        labels.get_index_of(&a).unwrap(),
                        labels.get_index_of(&b).unwrap(),
                    )
                })
            })
            .collect::<PyResult<_>>()?;
        let required: Vec<(usize, usize)> = required
            .try_iter()?
            .map(|x| {
                // Get the strings and convert them to indices.
                x?.extract::<(String, String)>().map(|(a, b)| {
                    (
                        labels.get_index_of(&a).unwrap(),
                        labels.get_index_of(&b).unwrap(),
                    )
                })
            })
            .collect::<PyResult<_>>()?;
        let temporal_order: Vec<Vec<usize>> = temporal_order
            .try_iter()?
            .map(|x| {
                x?.try_iter()?
                    .map(|x| {
                        // Get the string and convert it to an index.
                        x?.extract::<String>()
                            .map(|a| labels.get_index_of(&a).unwrap())
                    })
                    .collect::<PyResult<Vec<_>>>()
            })
            .collect::<PyResult<_>>()?;

        // Create the prior knowledge structure.
        Ok(PK::new(labels, forbidden, required, temporal_order).into())
    }
}
