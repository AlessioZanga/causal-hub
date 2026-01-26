use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{CatTable, CatType, Dataset},
    models::Labelled,
    types::{Error, Labels, Result, Set, States},
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
    pub fn new(
        states: States,
        mut events: Array2<CatType>,
        mut times: Array1<f64>,
    ) -> Result<Self> {
        // Assert the number of rows in values and times are equal.
        if events.nrows() != times.len() {
            return Err(Error::IncompatibleShape(
                events.nrows().to_string(),
                times.len().to_string(),
            ));
        }
        // Assert times must be positive and finite.
        times.iter().try_for_each(|&t| {
            if !t.is_finite() || t < 0. {
                return Err(Error::InvalidParameter(
                    "times".to_string(),
                    format!("value must be finite and positive, found {t}"),
                ));
            }
            Ok(())
        })?;

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
            if count != length {
                return Err(Error::InvalidParameter(
                    "times".to_string(),
                    format!("must be unique, found {} duplicates", length - count),
                ));
            }
        }

        // Assert at max one state change per transition.
        for ((e_i, _), (e_j, _)) in events.rows().into_iter().zip(&times).tuple_windows() {
            // Count the number of state changes.
            let count = e_i.iter().zip(e_j).filter(|(a, b)| a != b).count();
            // Assert there is one and only one state change.
            if count > 1 {
                return Err(Error::InvalidParameter(
                    "events".to_string(),
                    format!("must contain at max one change per transition, found {count}"),
                ));
            }
        }

        // Create a new categorical dataset instance.
        let events = CatTable::new(states, events)?;

        // Return a new trajectory instance.
        Ok(Self { events, times })
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

    fn select(&self, x: &Set<usize>) -> Result<Self> {
        // Select the dataset.
        let events = self.events.select(x)?;
        // Get states and events.
        let states = events.states().clone();
        let events = events.values().clone();
        // Select the times.
        let times = self.times.clone();
        // Return the new weighted dataset.
        Self::new(states, events, times)
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
    pub fn new<I>(values: I) -> Result<Self>
    where
        I: IntoIterator<Item = CatTrj>,
    {
        // Collect the trajectories into a vector.
        let values: Vec<_> = values.into_iter().collect();

        // Check if every trajectory has the same labels.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].labels().eq(trjs[1].labels()))
        {
            return Err(Error::ConstructionError(
                "All trajectories must have the same labels.".to_string(),
            ));
        }
        // Check if every trajectory has the same states.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].states().eq(trjs[1].states()))
        {
            return Err(Error::ConstructionError(
                "All trajectories must have the same states.".to_string(),
            ));
        }
        // Check if every trajectory has the same shape.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].shape().eq(trjs[1].shape()))
        {
            return Err(Error::ConstructionError(
                "All trajectories must have the same shape.".to_string(),
            ));
        }

        // Get the labels, states and shape from the first trajectory.
        let (labels, states, shape) = match values.first() {
            None => (Labels::default(), States::default(), Array1::default((0,))),
            Some(x) => (x.labels().clone(), x.states().clone(), x.shape().clone()),
        };

        Ok(Self {
            labels,
            states,
            shape,
            values,
        })
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
        Self::new(iter).unwrap_or_else(|e| {
            // Log the error since we can't propagate it through the trait.
            log::error!("Failed to create CatTrjs from iterator: {}", e);
            // Return a minimal valid empty instance as fallback.
            Self {
                labels: Default::default(),
                states: Default::default(),
                values: vec![],
                shape: Array1::zeros(2),
            }
        })
    }
}

impl FromParallelIterator<CatTrj> for CatTrjs {
    #[inline]
    fn from_par_iter<I: IntoParallelIterator<Item = CatTrj>>(iter: I) -> Self {
        let collected = iter.into_par_iter().collect::<Vec<_>>();
        Self::new(collected).unwrap_or_else(|e| {
            // Log the error since we can't propagate it through the trait.
            log::error!("Failed to create CatTrjs from parallel iterator: {}", e);
            // Return a minimal valid empty instance as fallback.
            Self {
                labels: Default::default(),
                states: Default::default(),
                values: vec![],
                shape: Array1::zeros(2),
            }
        })
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

    fn select(&self, x: &Set<usize>) -> Result<Self> {
        // Return the new collection of selected trajectories.
        Self::new(
            self.values
                .iter()
                .map(|trj| trj.select(x))
                .collect::<Result<Vec<_>>>()?,
        )
    }
}
