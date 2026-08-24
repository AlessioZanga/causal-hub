use std::{
    borrow::Cow,
    ops::{Div, DivAssign, Mul, MulAssign},
};

use approx::{AbsDiffEq, RelativeEq};
use serde::{Deserialize, Serialize};

use crate::{
    impl_json_io,
    models::{CatPhi, GaussPhi, HasLabels, Phi},
    types::{Error, Labels, Result, Set},
};

/// A unified potential for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MixedPhi {
    /// Categorical potential.
    Categorical(CatPhi),
    /// Gaussian potential.
    Gaussian(GaussPhi),
}

impl HasLabels for MixedPhi {
    #[inline]
    fn labels(&self) -> &Labels {
        match self {
            Self::Categorical(potential) => potential.labels(),
            Self::Gaussian(potential) => potential.labels(),
        }
    }
}

impl PartialEq for MixedPhi {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Categorical(a), Self::Categorical(b)) => a.eq(b),
            (Self::Gaussian(a), Self::Gaussian(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl AbsDiffEq for MixedPhi {
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

impl RelativeEq for MixedPhi {
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

impl Phi for MixedPhi {
    type CPD = crate::models::MixedCPD;
    type Support = crate::models::MixedSupport;
    type Parameters = MixedPhi;
    type Evidence = crate::models::MixedEv;

    fn support(&self) -> Cow<'_, Self::Support> {
        match self {
            Self::Categorical(potential) => Cow::Owned(crate::models::MixedSupport::Categorical(
                potential.support().clone(),
            )),
            Self::Gaussian(potential) => Cow::Owned(crate::models::MixedSupport::Gaussian(
                Cow::into_owned(potential.support()),
            )),
        }
    }

    fn parameters(&self) -> &Self::Parameters {
        self
    }

    fn parameters_size(&self) -> usize {
        match self {
            Self::Categorical(potential) => potential.parameters_size(),
            Self::Gaussian(potential) => potential.parameters_size(),
        }
    }

    fn condition(&self, evidence: &Self::Evidence) -> Result<Self> {
        match (self, evidence) {
            (Self::Categorical(potential), crate::models::MixedEv::Categorical(ev)) => {
                potential.condition(ev).map(Self::Categorical)
            }
            (Self::Gaussian(potential), crate::models::MixedEv::Gaussian(ev)) => {
                potential.condition(ev).map(Self::Gaussian)
            }
            _ => Err(Error::InvalidParameter(
                "e",
                "evidence type must match the potential variant",
            )),
        }
    }

    fn marginalize(&self, x: &Set<usize>) -> Result<Self> {
        match self {
            Self::Categorical(potential) => potential.marginalize(x).map(Self::Categorical),
            Self::Gaussian(potential) => potential.marginalize(x).map(Self::Gaussian),
        }
    }

    fn normalize(&self) -> Result<Self> {
        match self {
            Self::Categorical(potential) => potential.normalize().map(Self::Categorical),
            Self::Gaussian(potential) => potential.normalize().map(Self::Gaussian),
        }
    }

    fn from_cpd(distribution: Self::CPD) -> Result<Self> {
        match distribution {
            crate::models::MixedCPD::Categorical(c) => c.into_phi().map(Self::Categorical),
            crate::models::MixedCPD::Gaussian(c) => c.into_phi().map(Self::Gaussian),
        }
    }

    fn into_cpd(self, x: &Set<usize>, z: &Set<usize>) -> Result<Self::CPD> {
        match self {
            Self::Categorical(potential) => potential
                .into_cpd(x, z)
                .map(crate::models::MixedCPD::Categorical),
            Self::Gaussian(potential) => potential
                .into_cpd(x, z)
                .map(crate::models::MixedCPD::Gaussian),
        }
    }
}

impl MulAssign<&MixedPhi> for MixedPhi {
    fn mul_assign(&mut self, rhs: &MixedPhi) {
        match (self, rhs) {
            (Self::Categorical(a), MixedPhi::Categorical(b)) => {
                a.mul_assign(b);
            }
            (Self::Gaussian(a), MixedPhi::Gaussian(b)) => {
                a.mul_assign(b);
            }
            _ => unreachable!("cannot multiply mixed potential variants"),
        }
    }
}

impl Mul<&MixedPhi> for &MixedPhi {
    type Output = MixedPhi;

    #[inline]
    fn mul(self, rhs: &MixedPhi) -> Self::Output {
        let mut lhs = self.clone();
        MulAssign::mul_assign(&mut lhs, rhs);
        lhs
    }
}

impl DivAssign<&MixedPhi> for MixedPhi {
    fn div_assign(&mut self, rhs: &MixedPhi) {
        match (self, rhs) {
            (Self::Categorical(a), MixedPhi::Categorical(b)) => {
                DivAssign::div_assign(a, b);
            }
            (Self::Gaussian(a), MixedPhi::Gaussian(b)) => {
                DivAssign::div_assign(a, b);
            }
            _ => unreachable!("cannot divide mixed potential variants"),
        }
    }
}

impl Div<&MixedPhi> for &MixedPhi {
    type Output = MixedPhi;

    #[inline]
    fn div(self, rhs: &MixedPhi) -> Self::Output {
        let mut lhs = self.clone();
        DivAssign::div_assign(&mut lhs, rhs);
        lhs
    }
}

// Implement `JsonIO` for `MixedPhi`.
impl_json_io!(MixedPhi);
