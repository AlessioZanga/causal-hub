use ndarray::prelude::*;

use crate::{
    datasets::Dataset,
    types::{FxIndexMap, FxIndexSet},
};

use super::CategoricalDataset;

/// A multivariate trajectory.
#[derive(Clone, Debug)]
pub struct CategoricalTrajectory {
    events: CategoricalDataset,
    times: Array1<f64>,
}

/// A type alias for a multivariate trajectory.
pub type CategoricalTrj = CategoricalTrajectory;

impl CategoricalTrj {
    /// Constructs a new trajectory instance.
    ///
    /// # Arguments
    ///
    /// * `states` - An iterator of tuples containing the state labels and their corresponding values.
    /// * `events` - A 2D array of events.
    /// * `times` - A 1D array of times.
    ///
    /// # Returns
    ///
    /// A new instance of `CategoricalTrj`.
    ///
    pub fn new<I, J, K, V>(states: I, events: Array2<u8>, times: Array1<f64>) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // Sort values by times.
        let mut indices: Vec<_> = (0..events.nrows()).collect();
        indices.sort_by(|&a, &b| times[a].partial_cmp(&times[b]).unwrap());
        // Clone the events and times arrays to avoid borrowing issues.
        let mut new_events = events.clone();
        let mut new_times = times.clone();
        // Sort the events and times arrays by the sorted indices.
        new_events
            .rows_mut()
            .into_iter()
            .zip(new_times.iter_mut())
            .enumerate()
            .for_each(|(i, (mut new_events_row, new_time))| {
                new_events_row.assign(&events.row(indices[i]));
                *new_time = times[indices[i]];
            });
        // Update the events and times with the sorted values.
        let events = new_events;
        let times = new_times;

        // Create a new categorical dataset instance.
        let events = CategoricalDataset::new(states, events);

        // Assert the number of rows in values and times are equal.
        assert_eq!(
            events.values().nrows(),
            times.len(),
            "The number of events and times must be equal."
        );

        // Debug assert times are sorted.
        debug_assert!(times.iter().is_sorted(), "Times must be sorted.");

        // Return a new trajectory instance.
        Self { events, times }
    }

    /// Returns the states of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the states of the trajectory.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        self.events.states()
    }

    /// Returns the cardinality of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the cardinality of the trajectory.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        self.events.cardinality()
    }

    /// Returns the events of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the events of the trajectory.
    ///
    #[inline]
    pub fn events(&self) -> &Array2<u8> {
        self.events.values()
    }

    /// Returns the times of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the times of the trajectory.
    ///
    #[inline]
    pub const fn times(&self) -> &Array1<f64> {
        &self.times
    }
}

impl Dataset for CategoricalTrj {
    type Labels = FxIndexSet<String>;
    type Values = Array2<u8>;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        self.events.labels()
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        self.events.values()
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.events.values().nrows()
    }
}
