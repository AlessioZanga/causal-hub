use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{CatTrj, CatType, Dataset},
    models::Labelled,
    types::{Error, Labels, Result, Set, States},
};

/// A multivariate weighted trajectory.
#[derive(Clone, Debug)]
pub struct CatWtdTrj {
    trajectory: CatTrj,
    weight: f64,
}

impl TryFrom<(CatTrj, f64)> for CatWtdTrj {
    type Error = Error;

    fn try_from((trajectory, weight): (CatTrj, f64)) -> Result<Self> {
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
    pub fn new(trajectory: CatTrj, weight: f64) -> Result<Self> {
        // Check that the weight is in the range [0, 1].
        if !(0.0..=1.0).contains(&weight) {
            return Err(Error::InvalidParameter(
                "weight".to_string(),
                format!("must be in the range [0, 1], but got {weight}"),
            ));
        }

        Ok(Self { trajectory, weight })
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
    pub const fn states(&self) -> &States {
        self.trajectory.states()
    }

    /// Returns the shape of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the shape of the trajectory.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        self.trajectory.shape()
    }

    /// Returns the times of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the times of the trajectory.
    ///
    #[inline]
    pub const fn times(&self) -> &Array1<f64> {
        self.trajectory.times()
    }
}

impl Labelled for CatWtdTrj {
    #[inline]
    fn labels(&self) -> &Labels {
        self.trajectory.labels()
    }
}

impl Dataset for CatWtdTrj {
    type Values = Array2<CatType>;

    #[inline]
    fn values(&self) -> &Self::Values {
        self.trajectory.values()
    }

    #[inline]
    fn sample_size(&self) -> f64 {
        self.weight * (self.trajectory.values().nrows() as f64)
    }

    fn select(&self, x: &Set<usize>) -> Result<Self> {
        // Select the dataset.
        let trajectory = self.trajectory.select(x)?;
        // Select the weights.
        let weight = self.weight;
        // Return the new weighted dataset.
        Self::new(trajectory, weight)
    }
}

/// A collection of weighted trajectories.
#[derive(Clone, Debug)]
pub struct CatWtdTrjs {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    values: Vec<CatWtdTrj>,
}

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
    /// * The trajectories have different shape.
    /// * The trajectories are empty.
    ///
    /// # Returns
    ///
    /// A new instance of `CategoricalTrajectories`.
    ///
    pub fn new<I>(values: I) -> Result<Self>
    where
        I: IntoIterator<Item = CatWtdTrj>,
    {
        // Collect the trajectories into a vector.
        let values: Vec<_> = values.into_iter().collect();

        // Check if every trajectory has the same labels.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].labels().eq(trjs[1].labels()))
        {
            return Err(Error::IncompatibleShape(
                "labels".into(),
                "all trajectories".into(),
            ));
        }
        // Check if every trajectory has the same states.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].states().eq(trjs[1].states()))
        {
            return Err(Error::IncompatibleShape(
                "states".into(),
                "all trajectories".into(),
            ));
        }
        // Check if every trajectory has the same shape.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].shape().eq(trjs[1].shape()))
        {
            return Err(Error::IncompatibleShape(
                "shape".into(),
                "all trajectories".into(),
            ));
        }

        // Get the labels, states and shape from the first trajectory.
        let trj = values
            .first()
            .ok_or_else(|| Error::EmptySet("trajectories".into()))?;
        let labels = trj.labels().clone();
        let states = trj.states().clone();
        let shape = trj.shape().clone();

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

impl FromIterator<CatWtdTrj> for CatWtdTrjs {
    #[inline]
    fn from_iter<I: IntoIterator<Item = CatWtdTrj>>(iter: I) -> Self {
        Self::new(iter).unwrap_or_else(|e| {
            // Log the error since we can't propagate it through the trait.
            log::error!("Failed to create CatWtdTrjs from iterator: {}", e);
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

impl FromParallelIterator<CatWtdTrj> for CatWtdTrjs {
    #[inline]
    fn from_par_iter<I: IntoParallelIterator<Item = CatWtdTrj>>(iter: I) -> Self {
        let collected = iter.into_par_iter().collect::<Vec<_>>();
        Self::new(collected).unwrap_or_else(|e| {
            // Log the error since we can't propagate it through the trait.
            log::error!("Failed to create CatWtdTrjs from parallel iterator: {}", e);
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

impl<'a> IntoIterator for &'a CatWtdTrjs {
    type IntoIter = std::slice::Iter<'a, CatWtdTrj>;
    type Item = &'a CatWtdTrj;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a> IntoParallelRefIterator<'a> for CatWtdTrjs {
    type Item = &'a CatWtdTrj;
    type Iter = rayon::slice::Iter<'a, CatWtdTrj>;

    #[inline]
    fn par_iter(&'a self) -> Self::Iter {
        self.values.par_iter()
    }
}

impl Labelled for CatWtdTrjs {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl Dataset for CatWtdTrjs {
    type Values = Vec<CatWtdTrj>;

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
