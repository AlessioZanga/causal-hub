use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;

use crate::{
    models::{CPD, Labelled},
    types::Labels,
};

/// Sample (sufficient) statistics for a Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussCPDS {
    /// Design covariance matrix |X| x |X|.
    s_xx: Array2<f64>,
    /// Cross-covariance matrix |X| x (|Z| + 1).
    s_xz: Array2<f64>,
    /// Response covariance matrix (|Z| + 1) x (|Z| + 1).
    s_zz: Array2<f64>,
    /// Sample size.
    n: f64,
}

impl GaussCPDS {
    /// Creates a new `GaussCPDS` instance.
    ///
    /// # Arguments
    ///
    /// * `s_xx` - Design covariance matrix |X| x |X|.
    /// * `s_xz` - Cross-covariance matrix |X| x (|Z| + 1).
    /// * `s_zz` - Response covariance matrix (|Z| + 1) x (|Z| + 1).
    /// * `n` - Sample size.
    ///
    /// # Returns
    ///
    /// A new `GaussCPDS` instance.
    ///
    #[inline]
    pub fn new(s_xx: Array2<f64>, s_xz: Array2<f64>, s_zz: Array2<f64>, n: f64) -> Self {
        // Assert the dimensions are correct.
        assert_eq!(
            s_xx.nrows(),
            s_xx.ncols(),
            "Design covariance matrix must be square."
        );
        assert_eq!(
            s_xz.nrows(),
            s_xx.nrows(),
            "Cross-covariance matrix must have the same \n\
            number of rows as the design covariance matrix."
        );
        assert_eq!(
            s_xz.ncols(),
            s_zz.nrows(),
            "Cross-covariance matrix must have the same \n\
            number of columns as the response covariance matrix."
        );
        assert_eq!(
            s_zz.nrows(),
            s_zz.ncols(),
            "Response covariance matrix must be square."
        );

        Self {
            s_xx,
            s_xz,
            s_zz,
            n,
        }
    }

    /// Returns the design covariance matrix |X| x |X|.
    ///
    /// # Returns
    ///
    /// A reference to the design covariance matrix.
    ///
    #[inline]
    pub fn sample_design_covariance(&self) -> &Array2<f64> {
        &self.s_xx
    }

    /// Returns the cross-covariance matrix |X| x (|Z| + 1).
    ///
    /// # Returns
    ///
    /// A reference to the cross-covariance matrix.
    ///
    #[inline]
    pub fn sample_cross_covariance(&self) -> &Array2<f64> {
        &self.s_xz
    }

    /// Returns the response covariance matrix (|Z| + 1) x (|Z| + 1).
    ///
    /// # Returns
    ///
    /// A reference to the response covariance matrix.
    ///
    #[inline]
    pub fn sample_response_covariance(&self) -> &Array2<f64> {
        &self.s_zz
    }

    /// Returns the sample size.
    ///
    /// # Returns
    ///
    /// The sample size.
    ///
    #[inline]
    pub fn sample_size(&self) -> f64 {
        self.n
    }
}

/// Parameters of a Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussCPDP {
    /// Weight matrix |X| x |Z|.
    pub w: Array2<f64>,
    /// Intercept vector |X|.
    pub b: Array1<f64>,
    /// Covariance matrix |X| x |X|.
    pub s: Array2<f64>,
}

impl PartialEq for GaussCPDP {
    fn eq(&self, other: &Self) -> bool {
        self.w.eq(&other.w) && self.b.eq(&other.b) && self.s.eq(&other.s)
    }
}

impl AbsDiffEq for GaussCPDP {
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

impl RelativeEq for GaussCPDP {
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
    parameters: GaussCPDP,
    // Sample (sufficient) statistics, if any.
    sample_statistics: Option<GaussCPDS>,
    // Sample log-likelihood, if any.
    sample_log_likelihood: Option<f64>,
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
    type Parameters = GaussCPDP;
    type Statistics = GaussCPDS;

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

    #[inline]
    fn sample_statistics(&self) -> Option<&Self::Statistics> {
        self.sample_statistics.as_ref()
    }

    #[inline]
    fn sample_log_likelihood(&self) -> Option<f64> {
        self.sample_log_likelihood
    }
}
