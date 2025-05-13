use ndarray::prelude::*;
use rayon::prelude::*;

use super::{CatTrj, Dataset};
use crate::types::{FxIndexMap, FxIndexSet};

/// A multivariate weighted trajectory.
#[derive(Clone, Debug)]
pub struct CategoricalWeightedTrajectory {
    trajectory: CatTrj,
    weight: f64,
}

/// A type alias for a weighted trajectories.
pub type CatWtdTrj = CategoricalWeightedTrajectory;

impl From<(CatTrj, f64)> for CatWtdTrj {
    fn from((trajectory, weight): (CatTrj, f64)) -> Self {
        Self::new(trajectory, weight)
    }
}

impl From<CatWtdTrj> for (CatTrj, f64) {
    fn from(other: CatWtdTrj) -> Self {
        (other.trajectory, other.weight)
    }
}

impl CatWtdTrj {
    /// Creates a new categorical weighted trajectory.
    ///
    /// # Arguments
    ///
    /// * `trajectory` - The trajectory.
    /// * `weight` - The weight of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if the weight is not in the range [0, 1].
    ///
    /// # Returns
    ///
    /// A new categorical weighted trajectory.
    ///
    pub fn new(trajectory: CatTrj, weight: f64) -> Self {
        // Assert that the weight is in the range [0, 1].
        assert!(
            (0.0..=1.0).contains(&weight),
            "Weight must be in the range [0, 1]"
        );

        Self { trajectory, weight }
    }

    /// Returns the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the trajectory.
    ///
    #[inline]
    pub const fn trajectory(&self) -> &CatTrj {
        &self.trajectory
    }

    /// Returns the weight of the trajectory.
    ///
    /// # Returns
    ///
    /// The weight of the trajectory.
    ///
    #[inline]
    pub const fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns the states of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the states of the trajectory.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        self.trajectory.states()
    }

    /// Returns the cardinality of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the cardinality of the trajectory.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        self.trajectory.cardinality()
    }

    /// Returns the events of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the events of the trajectory.
    ///
    #[inline]
    pub fn events(&self) -> &Array2<u8> {
        self.trajectory.events()
    }

    /// Returns the times of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the times of the trajectory.
    ///
    #[inline]
    pub const fn times(&self) -> &Array1<f64> {
        &self.trajectory.times()
    }
}

impl Dataset for CatWtdTrj {
    type Labels = FxIndexSet<String>;
    type Values = Array2<u8>;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        self.trajectory.labels()
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        self.trajectory.values()
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.trajectory.values().nrows()
    }
}

/// A collection of multivariate trajectories.
#[derive(Clone, Debug)]
pub struct CategoricalWeightedTrajectories {
    trajectories: Vec<CatWtdTrj>,
}

/// A type alias for a collection of multivariate trajectories.
pub type CatWtdTrjs = CategoricalWeightedTrajectories;

impl CatWtdTrjs {
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
        I: IntoIterator<Item = CatWtdTrj>,
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

impl FromIterator<CatWtdTrj> for CatWtdTrjs {
    #[inline]
    fn from_iter<I: IntoIterator<Item = CatWtdTrj>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl FromParallelIterator<CatWtdTrj> for CatWtdTrjs {
    #[inline]
    fn from_par_iter<I: IntoParallelIterator<Item = CatWtdTrj>>(iter: I) -> Self {
        // TODO: Avoid collecting into a Vec, this is a workaround.
        Self::new(iter.into_par_iter().collect::<Vec<_>>())
    }
}

impl<'a> IntoIterator for &'a CatWtdTrjs {
    type IntoIter = std::slice::Iter<'a, CatWtdTrj>;
    type Item = &'a CatWtdTrj;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.trajectories.iter()
    }
}

impl<'a> IntoParallelRefIterator<'a> for CatWtdTrjs {
    type Item = &'a CatWtdTrj;
    type Iter = rayon::slice::Iter<'a, CatWtdTrj>;

    #[inline]
    fn par_iter(&'a self) -> Self::Iter {
        self.trajectories.par_iter()
    }
}

impl Dataset for CatWtdTrjs {
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
