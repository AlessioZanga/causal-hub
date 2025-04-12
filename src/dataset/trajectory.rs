use ndarray::prelude::*;

use crate::{dataset::Dataset, types::FxIndexSet};

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
    pub fn new<I, J, K, V>(states: I, events: Array2<u8>, times: Array1<f64>) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // FIXME: Sort values by times.

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

    #[inline]
    pub const fn events(&self) -> &CategoricalDataset {
        &self.events
    }

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
        &self.events.labels()
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.events.values()
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.events.values().nrows()
    }
}
