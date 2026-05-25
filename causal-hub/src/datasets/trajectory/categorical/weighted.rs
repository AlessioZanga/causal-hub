use std::borrow::Cow;

use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::{CatTrj, CatTrjEv, CatType, Dataset},
    models::{CatSupport, Labelled},
    types::{Error, Labels, Result, Set},
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
                "weight",
                &format!("must be in the range [0, 1], but got {weight}"),
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
    ///
    /// The weight of the trajectory.
    ///
    #[inline]
    pub const fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns the support of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the support of the trajectory.
    ///
    #[inline]
    pub const fn support(&self) -> &CatSupport {
        self.trajectory.support()
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
    type Support = CatSupport;
    type Evidence = CatTrjEv;
    type EvidenceIter<'a> = <CatTrj as Dataset>::EvidenceIter<'a>;

    #[inline]
    fn values(&self) -> &Self::Values {
        self.trajectory.values()
    }

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(self.trajectory.support())
    }

    fn evidence_iter(&self) -> Self::EvidenceIter<'_> {
        self.trajectory.evidence_iter()
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
    support: CatSupport,
    shape: Array1<usize>,
    values: Vec<CatWtdTrj>,
}

/// Concrete iterator over weighted trajectories evidences.
pub struct CatWtdTrjsEvidenceIter<'a> {
    trajectories: std::slice::Iter<'a, CatWtdTrj>,
    current: Option<<CatWtdTrj as Dataset>::EvidenceIter<'a>>,
}

impl<'a> Iterator for CatWtdTrjsEvidenceIter<'a> {
    type Item = Result<CatTrjEv>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.current.as_mut()
                && let Some(item) = current.next()
            {
                return Some(item);
            }

            self.current = self.trajectories.next().map(Dataset::evidence_iter);

            self.current.as_ref()?;
        }
    }
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
    /// * The trajectories have different support.
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
            return Err(Error::IncompatibleShape("labels", "all trajectories"));
        }
        // Check if every trajectory has the same support.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].support().eq(trjs[1].support()))
        {
            return Err(Error::IncompatibleShape("support", "all trajectories"));
        }
        // Check if every trajectory has the same shape.
        if !values
            .windows(2)
            .all(|trjs| trjs[0].shape().eq(trjs[1].shape()))
        {
            return Err(Error::IncompatibleShape("shape", "all trajectories"));
        }

        // Get the labels, support and shape from the first trajectory.
        let trj = values
            .first()
            .ok_or_else(|| Error::EmptySet("trajectories"))?;
        let labels = trj.labels().clone();
        let support = trj.support().clone();
        let shape = trj.shape().clone();

        Ok(Self {
            labels,
            support,
            shape,
            values,
        })
    }

    /// Returns the support of the trajectories.
    ///
    /// # Returns
    ///
    /// A reference to the support of the trajectories.
    ///
    #[inline]
    pub fn support(&self) -> &CatSupport {
        &self.support
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
                support: Default::default(),
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
                support: Default::default(),
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
    type Support = CatSupport;
    type Evidence = CatTrjEv;
    type EvidenceIter<'a> = CatWtdTrjsEvidenceIter<'a>;

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.values
    }

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.support)
    }

    fn evidence_iter(&self) -> Self::EvidenceIter<'_> {
        CatWtdTrjsEvidenceIter {
            trajectories: self.values.iter(),
            current: None,
        }
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
