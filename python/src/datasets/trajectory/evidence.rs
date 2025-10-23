use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use backend::{
    datasets::{CatTrjEv, CatTrjEvT, CatTrjsEv},
    models::Labelled,
    types::{Set, States},
};
use numpy::{PyArray1, prelude::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;

use crate::impl_from_into_lock;

/// A categorical trajectory evidence.
#[gen_stub_pyclass]
#[pyclass(name = "CatTrjEv", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatTrjEv {
    inner: Arc<RwLock<CatTrjEv>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatTrjEv, CatTrjEv);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTrjEv {
    /// Returns the labels of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels of the categorical trajectory.
    ///
    #[inline]
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A reference to the states of the categorical trajectory.
    ///
    pub fn states<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<String, Bound<'a, PyTuple>>> {
        Ok(self
            .lock()
            .states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.clone();
                let states = states.iter().cloned();
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }

    /// Constructs a new categorical trajectory evidence from a Pandas DataFrame.
    ///
    /// Parameters
    /// ----------
    /// df: pandas.DataFrame
    ///     A Pandas DataFrame containing the trajectory evidence data.
    ///     The data frame must contain the following columns:
    ///
    ///         - `event`: The event type (str),
    ///         - `state`: The state of the event (str),
    ///         - `start_time`: The start time of the event (float64),
    ///         - `end_time`: The end time of the event (float64).
    ///
    /// with_states: dict[str, Iterable[str]] | None
    ///     An optional dictionary mapping event labels to their possible states.
    ///     If not provided, the states will be inferred from the data frame.
    ///
    /// Returns
    /// -------
    /// CatTrjEv
    ///     A new categorical trajectory evidence instance.
    ///
    #[classmethod]
    #[pyo3(signature = (df, with_states = None))]
    pub fn from_pandas(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        df: &Bound<'_, PyAny>,
        with_states: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Short the evidence.
        use CatTrjEvT as E;

        // Import the pandas module.
        let pd = py.import("pandas")?;

        // Assert that the object is a DataFrame.
        assert!(
            df.is_instance(&pd.getattr("DataFrame")?)?,
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
        assert_eq!(
            event_dtype, "object",
            "Expected a string column, but '{event_dtype}' found."
        );
        let state_dtype = state
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        assert_eq!(
            state_dtype, "object",
            "Expected a string column, but '{state_dtype}' found."
        );
        let start_time_dtype = start_time
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        assert_eq!(
            start_time_dtype, "float64",
            "Expected a float64 column, but '{start_time_dtype}' found."
        );
        let end_time_dtype = end_time
            .getattr("dtype")?
            .getattr("name")?
            .extract::<String>()?;
        assert_eq!(
            end_time_dtype, "float64",
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
        let mut states = with_states
            .and_then(|states| {
                // Convert the PyDict to a States.
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
                        let value: Set<_> = value
                            .try_iter()?
                            .map(|x| x?.extract::<String>())
                            .collect::<PyResult<_>>()?;
                        // Return the key and value.
                        Ok((key, value))
                    })
                    .collect::<PyResult<_>>()
                    .ok()
            })
            .unwrap_or_else(|| {
                // Infer the states from the columns.
                event
                    .iter()
                    .zip(&state)
                    .fold(States::default(), |mut acc, (event, state)| {
                        // Get the entry in the states map.
                        let entry = acc.entry(event.clone()).or_default();
                        // Insert the state in the states map.
                        entry.insert(state.clone());
                        // Return the states map.
                        acc
                    })
            });

        // Sort the states.
        states.sort_keys();
        states.values_mut().for_each(Set::sort);

        // Zip the iterators together.
        let evidence: Vec<_> = event
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
            .collect::<PyResult<_>>()?;

        // Construct the evidence.
        let inner = CatTrjEv::new(states, evidence);
        // Wrap the dataset in an Arc<RwLock>.
        let inner = Arc::new(RwLock::new(inner));

        Ok(Self { inner })
    }
}

/// A collection of categorical trajectory evidences.
#[gen_stub_pyclass]
#[pyclass(name = "CatTrjsEv", module = "causal_hub.datasets")]
#[derive(Clone, Debug)]
pub struct PyCatTrjsEv {
    inner: Arc<RwLock<CatTrjsEv>>,
}

// Implement `Deref`, `From` and locks traits.
impl_from_into_lock!(PyCatTrjsEv, CatTrjsEv);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatTrjsEv {
    /// Returns the labels of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// list[str]
    ///     A reference to the labels of the categorical trajectory.
    ///
    #[inline]
    pub fn labels(&self) -> PyResult<Vec<String>> {
        Ok(self.lock().labels().iter().cloned().collect())
    }

    /// Returns the states of the categorical trajectory.
    ///
    /// Returns
    /// -------
    /// dict[str, tuple[str, ...]]
    ///     A reference to the states of the categorical trajectory.
    ///
    pub fn states<'a>(&'a self, py: Python<'a>) -> PyResult<BTreeMap<String, Bound<'a, PyTuple>>> {
        Ok(self
            .lock()
            .states()
            .iter()
            .map(|(label, states)| {
                // Get reference to the label and states.
                let label = label.clone();
                let states = states.iter().cloned();
                // Convert the states to a PyTuple.
                let states = PyTuple::new(py, states).unwrap();
                // Return a tuple of the label and states.
                (label, states)
            })
            .collect())
    }

    /// Constructs a new categorical trajectory evidence from an iterable of Pandas DataFrames.
    ///
    /// Parameters
    /// ----------
    /// dfs: Iterable[pandas.DataFrame]
    ///     An iterable of Pandas DataFrames containing the trajectory evidence data.
    ///     The data frames must contain the following columns:
    ///
    ///         - `event`: The event type (str),
    ///         - `state`: The state of the event (str),
    ///         - `start_time`: The start time of the event (float64),
    ///         - `end_time`: The end time of the event (float64).
    ///
    /// with_states: dict[str, Iterable[str]] | None
    ///     An optional dictionary mapping event labels to their possible states.
    ///     If not provided, the states will be inferred from the data frame.
    ///
    /// Returns
    /// -------
    /// CatTrjsEv
    ///     A new categorical trajectory evidence instance.
    ///
    #[classmethod]
    #[pyo3(signature = (dfs, with_states = None))]
    pub fn from_pandas(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dfs: &Bound<'_, PyAny>,
        with_states: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Convert the iterable to a Vec<PyAny>.
        let dfs: Vec<PyCatTrjEv> = dfs
            .try_iter()?
            .map(|df| PyCatTrjEv::from_pandas(_cls, py, &df.unwrap(), with_states))
            .collect::<PyResult<_>>()?;
        // Convert the Vec<PyCatTrjEv> to Vec<CatTrjEv>.
        let dfs: Vec<_> = dfs.into_iter().map(Into::into).collect();

        // Create a new CatTrjsEv with the given parameters.
        let inner = CatTrjsEv::new(dfs);
        // Wrap the dataset in an Arc<RwLock>.
        let inner = Arc::new(RwLock::new(inner));

        Ok(Self { inner })
    }
}
