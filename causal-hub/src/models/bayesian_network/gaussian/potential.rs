use std::ops::{Div, DivAssign, Mul, MulAssign};

use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;

use crate::{
    models::{GaussCPD, Labelled, Phi},
    types::{Labels, Set},
};

/// Parameters of a Gaussian potential.
#[derive(Clone, Debug)]
pub struct GaussK {
    /// Precision matrix |X| x |X|.
    pub k: Array2<f64>,
    /// Information vector |X|.
    pub h: Array1<f64>,
    /// Log-normalization constant.
    pub g: f64,
}

impl PartialEq for GaussK {
    fn eq(&self, other: &Self) -> bool {
        self.k.eq(&other.k) && self.h.eq(&other.h) && self.g.eq(&other.g)
    }
}

impl AbsDiffEq for GaussK {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.k.abs_diff_eq(&other.k, epsilon)
            && self.h.abs_diff_eq(&other.h, epsilon)
            && self.g.abs_diff_eq(&other.g, epsilon)
    }
}

impl RelativeEq for GaussK {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.k.relative_eq(&other.k, epsilon, max_relative)
            && self.h.relative_eq(&other.h, epsilon, max_relative)
            && self.g.relative_eq(&other.g, epsilon, max_relative)
    }
}

/// A Gaussian potential.
#[derive(Clone, Debug)]
pub struct GaussPhi {
    // Labels of the variables.
    labels: Labels,
    // Parameters.
    parameters: GaussK,
}

impl Labelled for GaussPhi {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl PartialEq for GaussPhi {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels) && self.parameters.eq(&other.parameters)
    }
}

impl AbsDiffEq for GaussPhi {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels) && self.parameters.abs_diff_eq(&other.parameters, epsilon)
    }
}

impl RelativeEq for GaussPhi {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.labels.eq(&other.labels)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl MulAssign<&GaussPhi> for GaussPhi {
    fn mul_assign(&mut self, _rhs: &GaussPhi) {
        todo!() // FIXME:
    }
}

impl Mul<&GaussPhi> for &GaussPhi {
    type Output = GaussPhi;

    #[inline]
    fn mul(self, rhs: &GaussPhi) -> Self::Output {
        let mut lhs = self.clone();
        lhs *= rhs;
        lhs
    }
}

impl DivAssign<&GaussPhi> for GaussPhi {
    fn div_assign(&mut self, _rhs: &GaussPhi) {
        todo!() // FIXME:
    }
}

impl Div<&GaussPhi> for &GaussPhi {
    type Output = GaussPhi;

    #[inline]
    fn div(self, rhs: &GaussPhi) -> Self::Output {
        let mut lhs = self.clone();
        lhs /= rhs;
        lhs
    }
}

impl Phi for GaussPhi {
    type CPD = GaussCPD;
    type Parameters = GaussK;
    type Evidence = (); // FIXME:

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    #[inline]
    fn parameters_size(&self) -> usize {
        self.parameters.k.len() + self.parameters.h.len() + 1
    }

    fn condition(&self, _e: &Self::Evidence) -> Self {
        todo!() // FIXME:
    }

    fn marginalize(&self, _x: &Set<usize>) -> Self {
        todo!() // FIXME:
    }

    fn normalize(&self) -> Self {
        todo!() // FIXME:
    }

    fn from_cpd(_cpd: Self::CPD) -> Self {
        todo!() // FIXME:
    }

    fn into_cpd(self, _x: &Set<usize>, _z: &Set<usize>) -> Self::CPD {
        todo!() // FIXME:
    }
}
