use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{CatData, Dataset},
    types::{FxIndexMap, FxIndexSet},
};

/// A multivariate trajectory.
#[derive(Clone, Debug)]
pub struct CategoricalTrajectory {
    events: CatData,
    times: Array1<f64>,
}

/// A type alias for a multivariate trajectory.
pub type CatTrj = CategoricalTrajectory;

impl CatTrj {
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
    /// A new instance of `CatTrj`.
    ///
    pub fn new<I, J, K, V>(states: I, events: Array2<u8>, times: Array1<f64>) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        // Assert times must be positive and finite.
        assert!(
            times.iter().all(|&t| t.is_finite() && t >= 0.0),
            "Times must be positive and finite."
        );

        // Sort values by times.
        let mut indices: Vec<_> = (0..events.nrows()).collect();
        indices.sort_by(|&a, &b| {
            times[a]
                .partial_cmp(&times[b])
                // Due to previous assertions, this should never fail.
                .unwrap_or_else(|| unreachable!())
        });
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
        let events = CatData::new(states, events);

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

impl Dataset for CatTrj {
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

/// A collection of multivariate trajectories.
#[derive(Clone, Debug)]
pub struct CategoricalTrajectories {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    values: Vec<CatTrj>,
}

/// A type alias for a collection of multivariate trajectories.
pub type CatTrjs = CategoricalTrajectories;

impl CatTrjs {
    /// Constructs a new collection of trajectories.
    ///
    /// # Arguments
    ///
    /// * `trajectories` - An iterator of `CategoricalTrajectory` instances.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * The trajectories have different labels.
    /// * The trajectories have different states.
    /// * The trajectories have different cardinality.
    /// * The trajectories are empty.
    ///
    /// # Returns
    ///
    /// A new instance of `CategoricalTrajectories`.
    ///
    pub fn new<I>(values: I) -> Self
    where
        I: IntoIterator<Item = CatTrj>,
    {
        // Collect the trajectories into a vector.
        let values: Vec<_> = values.into_iter().collect();

        // Assert every trajectory has the same labels.
        assert!(
            values
                .windows(2)
                .all(|trjs| trjs[0].labels().eq(trjs[1].labels())),
            "All trajectories must have the same labels."
        );
        // Assert every trajectory has the same states.
        assert!(
            values
                .windows(2)
                .all(|trjs| trjs[0].states().eq(trjs[1].states())),
            "All trajectories must have the same states."
        );
        // Assert every trajectory has the same cardinality.
        assert!(
            values
                .windows(2)
                .all(|trjs| trjs[0].cardinality().eq(trjs[1].cardinality())),
            "All trajectories must have the same cardinality."
        );

        // Get the labels, states and cardinality from the first trajectory.
        let trj = values.first().expect("No trajectory in the dataset.");
        let labels = trj.labels().clone();
        let states = trj.states().clone();
        let cardinality = trj.cardinality().clone();

        Self {
            labels,
            states,
            cardinality,
            values,
        }
    }

    /// Returns the states of the trajectories.
    ///
    /// # Returns
    ///
    /// A reference to the states of the trajectories.
    ///
    #[inline]
    pub fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// Returns the cardinality of the trajectories.
    ///
    /// # Returns
    ///
    /// A reference to the cardinality of the trajectories.
    ///
    #[inline]
    pub fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }
}

impl FromIterator<CatTrj> for CatTrjs {
    #[inline]
    fn from_iter<I: IntoIterator<Item = CatTrj>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl FromParallelIterator<CatTrj> for CatTrjs {
    #[inline]
    fn from_par_iter<I: IntoParallelIterator<Item = CatTrj>>(iter: I) -> Self {
        Self::new(iter.into_par_iter().collect::<Vec<_>>())
    }
}

impl<'a> IntoIterator for &'a CatTrjs {
    type IntoIter = std::slice::Iter<'a, CatTrj>;
    type Item = &'a CatTrj;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a> IntoParallelRefIterator<'a> for CatTrjs {
    type Item = &'a CatTrj;
    type Iter = rayon::slice::Iter<'a, CatTrj>;

    #[inline]
    fn par_iter(&'a self) -> Self::Iter {
        self.values.par_iter()
    }
}

impl Dataset for CatTrjs {
    type Labels = FxIndexSet<String>;
    type Values = Vec<CatTrj>;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        &self.labels
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.values.iter().map(|x| x.sample_size()).sum()
    }
}
