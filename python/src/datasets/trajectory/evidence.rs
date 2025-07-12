use core::panic;
use std::collections::BTreeMap;

use causal_hub_rust::{
    datasets::{CatTrjEv, CatTrjEvT, CatTrjsEv, Dataset},
    types::{FxIndexSet, States},
};
use numpy::{PyArray1, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

use crate::impl_deref_from_into;

#[gen_stub_pyclass]
#[pyclass(name = "CatTrjEv")]
#[derive(Clone, Debug)]
pub struct PyCatTrjEv {
    inner: CatTrjEv,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatTrjEv, CatTrjEv);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTrjEv {
    /// Constructs a new categorical trajectory evidence from a Pandas DataFrame.
    ///
    /// # Arguments
    ///
    /// * `df` - A Pandas DataFrame containing the trajectory evidence data.
    /// * `with_states` - An optional dictionary of states.
    ///
    /// # Notes
    ///
    /// * The data frame must contain the following columns:
    /// - `event`: The event type (string).
    /// - `state`: The state of the event (string).
    /// - `start_time`: The start time of the event (float64).
    /// - `end_time`: The end time of the event (float64).
    ///
    /// # Returns
    ///
    /// A new categorical trajectory evidence instance.
    ///
    #[new]
    #[pyo3(signature = (df, with_states = None))]
    pub fn new(
        py: Python<'_>,
        df: &Bound<'_, PyAny>,
        with_states: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Short the evidence.
        use CatTrjEvT as E;

        // Import the pandas module.
        let pandas = py.import("pandas")?;

        // Assert that the object is a DataFrame.
        assert!(
            df.is_instance(&pandas.getattr("DataFrame")?)?,
            "Expected a Pandas DataFrame, but '{}' found.",
            df.get_type().name()?
        );

        // Get 'event', 'state', 'start_time', and 'end_time' columns.
        let (event, state, start_time, end_time) = (
            df.get_item("event")?,
            df.get_item("state")?,
            df.get_item("start_time")?,
            df.get_item("end_time")?,
        );

        // Check the dtypes of the columns.
        let event_dtype = event
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        assert!(
            event_dtype == "object",
            "Expected a string column, but '{event_dtype}' found."
        );
        let state_dtype = state
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        assert!(
            state_dtype == "object",
            "Expected a string column, but '{state_dtype}' found."
        );
        let start_time_dtype = start_time
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        assert!(
            start_time_dtype == "float64",
            "Expected a float64 column, but '{start_time_dtype}' found."
        );
        let end_time_dtype = end_time
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        assert!(
            end_time_dtype == "float64",
            "Expected a float64 column, but '{end_time_dtype}' found."
        );

        // Collect the iterators into vectors.
        let (event, state, start_time, end_time) = (
            event
                .try_iter()?
                .map(|x| x?.extract::<String>())
                .collect::<PyResult<Vec<_>>>()?,
            state
                .try_iter()?
                .map(|x| x?.extract::<String>())
                .collect::<PyResult<Vec<_>>>()?,
            start_time
                .getattr("to_numpy")?
                .call0()?
                .downcast::<PyArray1<f64>>()?
                .to_owned_array(),
            end_time
                .getattr("to_numpy")?
                .call0()?
                .downcast::<PyArray1<f64>>()?
                .to_owned_array(),
        );

        // Construct the states.
        let states: States = with_states
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
                todo!() // FIXME:
            })
            .unwrap();

        // Zip the iterators together.
        let evidence = event
            .into_iter()
            .zip(state)
            .zip(start_time)
            .zip(end_time)
            // Flatten the nested tuples.
            .map(|(((event, state), start_time), end_time)| {
                // Convert the event and state.
                let event = states
                    .get_index_of(&event)
                    .unwrap_or_else(|| panic!("Event '{event}' not found in states."));
                let state = states[event]
                    .get_index_of(&state)
                    .unwrap_or_else(|| panic!("State '{state}' not found for event '{event}'."));
                // Construct the evidence.
                Ok(E::CertainPositiveInterval {
                    event,
                    state,
                    start_time,
                    end_time,
                })
            })
            .collect::<PyResult<Vec<_>>>()?;

        // Construct the evidence.
        let inner = CatTrjEv::new(states, evidence);
        // Return the evidence.
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

#[gen_stub_pyclass]
#[pyclass(name = "CatTrjsEv")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PyCatTrjsEv {
    inner: CatTrjsEv,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatTrjsEv, CatTrjsEv);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTrjsEv {
    /// Constructs a new categorical trajectory evidence from an iterable of Pandas DataFrames.
    ///
    /// # Arguments
    ///
    /// * `dfs` - An iterable of Pandas DataFrames containing the trajectory evidence data.
    /// * `with_states` - An optional dictionary of states.
    ///
    /// # Notes
    ///
    /// * The data frames must contain the following columns:
    /// - `event`: The event type (string).
    /// - `state`: The state of the event (string).
    /// - `start_time`: The start time of the event (float64).
    /// - `end_time`: The end time of the event (float64).
    ///
    /// # Returns
    ///
    /// A new categorical trajectory evidence instance.
    ///
    #[new]
    #[pyo3(signature = (dfs, with_states = None))]
    pub fn new(
        py: Python<'_>,
        dfs: &Bound<'_, PyAny>,
        with_states: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Convert the iterable to a Vec<PyAny>.
        let dfs: Vec<PyCatTrjEv> = dfs
            .try_iter()?
            .map(|df| PyCatTrjEv::new(py, &df.unwrap(), with_states))
            .collect::<PyResult<_>>()?;
        // Convert the Vec<PyCatTrjEv> to Vec<CatTrjEv>.
        let dfs: Vec<_> = dfs.into_iter().map(Into::into).collect();
        // Create a new CatTrjsEv with the given parameters.
        let inner = CatTrjsEv::new(dfs);

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
