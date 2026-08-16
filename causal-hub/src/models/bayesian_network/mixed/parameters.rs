use std::borrow::Cow;

use approx::{AbsDiffEq, RelativeEq};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    datasets::{CatSample, GaussSample},
    models::{CPD, CatCPD, CatCPDS, CatSupport, GaussCPD, GaussCPDS, GaussSupport, Labelled},
    types::{Error, Labels, Result},
};

/// Unified support metadata for mixed CPDs.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixedSupport {
    /// Categorical support (discrete states).
    Categorical(CatSupport),
    /// Gaussian support (continuous ranges).
    Gaussian(GaussSupport),
}

/// The parameters of a mixed CPD.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixedCPD {
    /// Categorical CPD.
    Categorical(CatCPD),
    /// Gaussian CPD.
    Gaussian(GaussCPD),
}

/// The sufficient statistics of a mixed CPD.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixedCPDS {
    /// Categorical sufficient statistics.
    Categorical(CatCPDS),
    /// Gaussian sufficient statistics.
    Gaussian(Box<GaussCPDS>),
}

/// A unified sample type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixedSample {
    /// Categorical sample.
    Categorical(CatSample),
    /// Gaussian sample.
    Gaussian(GaussSample),
}

impl From<CatCPD> for MixedCPD {
    #[inline]
    fn from(distribution: CatCPD) -> Self {
        Self::Categorical(distribution)
    }
}

impl From<GaussCPD> for MixedCPD {
    #[inline]
    fn from(distribution: GaussCPD) -> Self {
        Self::Gaussian(distribution)
    }
}

impl From<CatCPDS> for MixedCPDS {
    #[inline]
    fn from(stats: CatCPDS) -> Self {
        Self::Categorical(stats)
    }
}

impl From<GaussCPDS> for MixedCPDS {
    #[inline]
    fn from(stats: GaussCPDS) -> Self {
        Self::Gaussian(Box::new(stats))
    }
}

impl From<CatSample> for MixedSample {
    #[inline]
    fn from(sample: CatSample) -> Self {
        Self::Categorical(sample)
    }
}

impl From<GaussSample> for MixedSample {
    #[inline]
    fn from(sample: GaussSample) -> Self {
        Self::Gaussian(sample)
    }
}

impl Labelled for MixedCPD {
    fn labels(&self) -> &Labels {
        match self {
            Self::Categorical(distribution) => distribution.labels(),
            Self::Gaussian(distribution) => distribution.labels(),
        }
    }
}

impl PartialEq for MixedCPD {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Categorical(a), Self::Categorical(b)) => a.eq(b),
            (Self::Gaussian(a), Self::Gaussian(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl AbsDiffEq for MixedCPD {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (self, other) {
            (Self::Categorical(a), Self::Categorical(b)) => a.abs_diff_eq(b, epsilon),
            (Self::Gaussian(a), Self::Gaussian(b)) => a.abs_diff_eq(b, epsilon),
            _ => false,
        }
    }
}

impl RelativeEq for MixedCPD {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        match (self, other) {
            (Self::Categorical(a), Self::Categorical(b)) => a.relative_eq(b, epsilon, max_relative),
            (Self::Gaussian(a), Self::Gaussian(b)) => a.relative_eq(b, epsilon, max_relative),
            _ => false,
        }
    }
}

impl CPD for MixedCPD {
    type Sample = MixedSample;
    type Support = MixedSupport;
    type Parameters = MixedCPD;
    type Statistics = MixedCPDS;

    fn conditioning_labels(&self) -> &Labels {
        match self {
            Self::Categorical(distribution) => distribution.conditioning_labels(),
            Self::Gaussian(distribution) => distribution.conditioning_labels(),
        }
    }

    fn support(&self) -> Cow<'_, Self::Support> {
        match self {
            Self::Categorical(distribution) => {
                Cow::Owned(MixedSupport::Categorical(distribution.support().clone()))
            }
            Self::Gaussian(distribution) => {
                Cow::Owned(MixedSupport::Gaussian(distribution.support().into_owned()))
            }
        }
    }

    fn conditioning_support(&self) -> Cow<'_, Self::Support> {
        match self {
            Self::Categorical(distribution) => Cow::Owned(MixedSupport::Categorical(
                distribution.conditioning_support().clone(),
            )),
            Self::Gaussian(distribution) => Cow::Owned(MixedSupport::Gaussian(
                distribution.conditioning_support().into_owned(),
            )),
        }
    }

    fn parameters(&self) -> &Self::Parameters {
        self
    }

    fn parameters_size(&self) -> usize {
        match self {
            Self::Categorical(distribution) => distribution.parameters_size(),
            Self::Gaussian(distribution) => distribution.parameters_size(),
        }
    }

    fn fitted_statistics(&self) -> Option<Cow<'_, Self::Statistics>> {
        match self {
            Self::Categorical(distribution) => distribution
                .fitted_statistics()
                .map(|stats| Cow::Owned(MixedCPDS::Categorical(stats.into_owned()))),
            Self::Gaussian(distribution) => distribution
                .fitted_statistics()
                .map(|stats| Cow::Owned(MixedCPDS::Gaussian(Box::new(stats.into_owned())))),
        }
    }

    fn fitted_log_likelihood(&self) -> Option<f64> {
        match self {
            Self::Categorical(distribution) => distribution.fitted_log_likelihood(),
            Self::Gaussian(distribution) => distribution.fitted_log_likelihood(),
        }
    }

    fn pf(&self, x: &Self::Sample, z: &Self::Sample) -> Result<f64> {
        match (self, x, z) {
            (
                Self::Categorical(distribution),
                MixedSample::Categorical(x),
                MixedSample::Categorical(z),
            ) => distribution.pf(x, z),
            (Self::Gaussian(distribution), MixedSample::Gaussian(x), MixedSample::Gaussian(z)) => {
                distribution.pf(x, z)
            }
            _ => Err(Error::InvalidParameter(
                "x/z",
                "sample type must match the CPD parameter type",
            )),
        }
    }

    fn sample<R: Rng>(&self, rng: &mut R, z: &Self::Sample) -> Result<Self::Sample> {
        match (self, z) {
            (Self::Categorical(distribution), MixedSample::Categorical(z)) => {
                let sample = distribution.sample(rng, z)?;
                Ok(MixedSample::Categorical(sample))
            }
            (Self::Gaussian(distribution), MixedSample::Gaussian(z)) => {
                let sample = distribution.sample(rng, z)?;
                Ok(MixedSample::Gaussian(sample))
            }
            _ => Err(Error::InvalidParameter(
                "z",
                "sample type must match the CPD parameter type",
            )),
        }
    }
}
