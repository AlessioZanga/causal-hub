use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;

use crate::{
    models::{CPD, Labelled},
    types::Labels,
};

/// Parameters of a Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussT {
    /// Weight matrix |X| x |Z|.
    pub w: Array2<f64>,
    /// Intercept vector |X|.
    pub b: Array1<f64>,
    /// Covariance matrix |X| x |X|.
    pub s: Array2<f64>,
}

impl PartialEq for GaussT {
    fn eq(&self, other: &Self) -> bool {
        self.w.eq(&other.w) && self.b.eq(&other.b) && self.s.eq(&other.s)
    }
}

impl AbsDiffEq for GaussT {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.w.abs_diff_eq(&other.w, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
            && self.s.abs_diff_eq(&other.s, epsilon)
    }
}

impl RelativeEq for GaussT {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.w.relative_eq(&other.w, epsilon, max_relative)
            && self.b.relative_eq(&other.b, epsilon, max_relative)
            && self.s.relative_eq(&other.s, epsilon, max_relative)
    }
}

/// A Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussCPD {
    // Labels of the variables.
    labels: Labels,
    // Labels of the conditioning variables.
    conditioning_labels: Labels,
    // Parameters.
    parameters: GaussT,
    // FIXME: Fitted statistics, if any.
}

impl Labelled for GaussCPD {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl PartialEq for GaussCPD {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.parameters.eq(&other.parameters)
    }
}

impl AbsDiffEq for GaussCPD {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.parameters.abs_diff_eq(&other.parameters, epsilon)
    }
}

impl RelativeEq for GaussCPD {
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
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl CPD for GaussCPD {
    type Parameters = GaussT;
    type SS = (); // FIXME:

    #[inline]
    fn conditioning_labels(&self) -> &Labels {
        &self.conditioning_labels
    }

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    #[inline]
    fn parameters_size(&self) -> usize {
        self.parameters.w.len() + self.parameters.b.len() + self.parameters.s.len()
    }
}
