use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{CatTable, CatType, Dataset},
    models::Labelled,
    types::{Labels, States},
};

/// A multivariate trajectory.
#[derive(Clone, Debug)]
pub struct CatTrj {
    events: CatTable,
    times: Array1<f64>,
}

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
    pub fn new(states: States, mut events: Array2<CatType>, mut times: Array1<f64>) -> Self {
        // Assert the number of rows in values and times are equal.
        assert_eq!(
            events.nrows(),
            times.len(),
            "Trajectory events and times must have the same length."
        );
        // Assert times must be positive and finite.
        times.iter().for_each(|&t| {
            assert!(
                t.is_finite() && t >= 0.,
                "Trajectory times must be finite and positive: \n\
                \t expected: time >= 0 , \n\
                \t found:    time == {t} ."
            );
        });

        // Sort values by times.
        let mut sorted_idx: Vec<_> = (0..events.nrows()).collect();
        sorted_idx.sort_by(|&a, &b| {
            times[a]
                .partial_cmp(&times[b])
                // Due to previous assertions, this should never fail.
                .unwrap_or_else(|| unreachable!())
        });

        // Check if the times are already sorted.
        if !sorted_idx.iter().is_sorted() {
            // Sort times.
            let mut new_times = times.clone();
            new_times
                .iter_mut()
                .enumerate()
                .for_each(|(i, new_time)| *new_time = times[sorted_idx[i]]);
            // Update the times with the sorted values.
            times = new_times;

            // Sort events by time.
            let mut new_events = events.clone();
            // Sort the events by the sorted indices.
            new_events
                .rows_mut()
                .into_iter()
                .enumerate()
                .for_each(|(i, mut new_events_row)| {
                    new_events_row.assign(&events.row(sorted_idx[i]));
                });
            // Update the events with the sorted values.
            events = new_events;
        }

        // Assert no duplicate times.
        {
            // Count the number of unique times.
            let count = times.iter().dedup().count();
            // Get the length of the times array.
            let length = times.len();
            // Assert the number of unique times is equal to the length of the times array.
            assert_eq!(
                count, length,
                "Trajectory times must be unique: \n\
                \t expected: {count} deduplicated time-points, \n\
                \t found:    {length} non-deduplicated time-points, \n\
                \t for:      {times}."
            );
        }

        // Assert at max one state change per transition.
        events
            .rows()
            .into_iter()
            .zip(&times)
            .tuple_windows()
            .for_each(|((e_i, t_i), (e_j, t_j))| {
                // Count the number of state changes.
                let count = e_i.iter().zip(e_j).filter(|(a, b)| a != b).count();
                // Assert there is one and only one state change.
                assert!(
                    count <= 1,
                    "Trajectory events must contain at max one change per transition: \n\
                    \t expected: count <= 1 state change, \n\
                    \t found:    count == {count} state changes, \n\
                    \t for:      {e_i} event with time {t_i}, \n\
                    \t and:      {e_j} event with time {t_j}."
                );
            });

        // Create a new categorical dataset instance.
        let events = CatTable::new(states, events);

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
    pub const fn states(&self) -> &States {
        self.events.states()
    }

    /// Returns the shape of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the shape of the trajectory.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        self.events.shape()
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

impl Labelled for CatTrj {
    #[inline]
    fn labels(&self) -> &Labels {
        self.events.labels()
    }
}

impl Dataset for CatTrj {
    type Values = Array2<CatType>;

    #[inline]
    fn values(&self) -> &Self::Values {
        self.events.values()
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.events.values().nrows() as f64
    }
}

/// A collection of multivariate trajectories.
#[derive(Clone, Debug)]
pub struct CatTrjs {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    values: Vec<CatTrj>,
}

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
    /// * The trajectories have different shape.
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
        // Assert every trajectory has the same shape.
        assert!(
            values
                .windows(2)
                .all(|trjs| trjs[0].shape().eq(trjs[1].shape())),
            "All trajectories must have the same shape."
        );

        // Get the labels, states and shape from the first trajectory.
        let (labels, states, shape) = match values.first() {
            None => (Labels::default(), States::default(), Array1::default((0,))),
            Some(x) => (x.labels().clone(), x.states().clone(), x.shape().clone()),
        };

        Self {
            labels,
            states,
            shape,
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
    pub fn states(&self) -> &States {
        &self.states
    }

    /// Returns the shape of the trajectories.
    ///
    /// # Returns
    ///
    /// A reference to the shape of the trajectories.
    ///
    #[inline]
    pub fn shape(&self) -> &Array1<usize> {
        &self.shape
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

impl Labelled for CatTrjs {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl Dataset for CatTrjs {
    type Values = Vec<CatTrj>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.values.iter().map(Dataset::sample_size).sum()
    }
}
