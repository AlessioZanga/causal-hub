use ndarray::prelude::*;
use rayon::prelude::*;

use super::CategoricalDataset;
use crate::{
    datasets::Dataset,
    types::{FxIndexMap, FxIndexSet},
};

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

/// A collection of multivariate trajectories.
#[derive(Clone, Debug)]
pub struct CategoricalTrajectories {
    trajectories: Vec<CategoricalTrajectory>,
}

/// A type alias for a collection of multivariate trajectories.
pub type CategoricalTrjs = CategoricalTrajectories;

impl CategoricalTrajectories {
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
    ///
    /// # Returns
    ///
    /// A new instance of `CategoricalTrajectories`.
    ///
    pub fn new<I>(trajectories: I) -> Self
    where
        I: IntoIterator<Item = CategoricalTrajectory>,
    {
        // Collect the trajectories into a vector.
        let trajectories: Vec<_> = trajectories.into_iter().collect();

        // Assert every trajectory has the same labels.
        assert!(
            trajectories
                .windows(2)
                .all(|trjs| trjs[0].labels().eq(trjs[1].labels())),
            "All trajectories must have the same labels."
        );
        // Assert every trajectory has the same states.
        assert!(
            trajectories
                .windows(2)
                .all(|trjs| trjs[0].states().eq(trjs[1].states())),
            "All trajectories must have the same states."
        );
        // Assert every trajectory has the same cardinality.
        assert!(
            trajectories
                .windows(2)
                .all(|trjs| trjs[0].cardinality().eq(trjs[1].cardinality())),
            "All trajectories must have the same cardinality."
        );

        Self { trajectories }
    }

    /// Returns the states of the trajectories.
    ///
    /// # Panics
    ///
    /// Panics if the dataset is empty.
    ///
    /// # Returns
    ///
    /// A reference to the states of the trajectories.
    ///
    #[inline]
    pub fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        self.trajectories
            .first()
            .expect("Dataset is empty.")
            .states()
    }

    /// Returns the cardinality of the trajectories.
    ///
    /// # Panics
    ///
    /// Panics if the dataset is empty.
    ///
    /// # Returns
    ///
    /// A reference to the cardinality of the trajectories.
    ///
    #[inline]
    pub fn cardinality(&self) -> &Array1<usize> {
        self.trajectories
            .first()
            .expect("Dataset is empty.")
            .cardinality()
    }
}

impl FromIterator<CategoricalTrj> for CategoricalTrajectories {
    #[inline]
    fn from_iter<I: IntoIterator<Item = CategoricalTrj>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl FromParallelIterator<CategoricalTrj> for CategoricalTrajectories {
    #[inline]
    fn from_par_iter<I: IntoParallelIterator<Item = CategoricalTrj>>(iter: I) -> Self {
        // TODO: Avoid collecting into a Vec, this is a workaround.
        Self::new(iter.into_par_iter().collect::<Vec<_>>())
    }
}

impl<'a> IntoIterator for &'a CategoricalTrjs {
    type IntoIter = std::slice::Iter<'a, CategoricalTrj>;
    type Item = &'a CategoricalTrj;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.trajectories.iter()
    }
}

impl<'a> IntoParallelRefIterator<'a> for CategoricalTrjs {
    type Item = &'a CategoricalTrj;
    type Iter = rayon::slice::Iter<'a, CategoricalTrj>;

    #[inline]
    fn par_iter(&'a self) -> Self::Iter {
        self.trajectories.par_iter()
    }
}

impl Dataset for CategoricalTrjs {
    type Labels = FxIndexSet<String>;
    type Values = Array2<u8>;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        self.trajectories
            .first()
            .expect("Dataset is empty.")
            .labels()
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        self.trajectories
            .first()
            .expect("Dataset is empty.")
            .values()
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.trajectories.iter().map(|x| x.sample_size()).sum()
    }
}
