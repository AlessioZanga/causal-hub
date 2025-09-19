use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;

use crate::{
    models::{CPD, Labelled},
    types::Labels,
};

/// Sample (sufficient) statistics for a Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussCPDS {
    /// Design mean vector |X|.
    mu_x: Array1<f64>,
    /// Response mean vector |Z|.
    mu_z: Array1<f64>,
    /// Design covariance matrix |X| x |X|.
    s_xx: Array2<f64>,
    /// Cross-covariance matrix |X| x |Z|.
    s_xz: Array2<f64>,
    /// Response covariance matrix |Z| x |Z|.
    s_zz: Array2<f64>,
    /// Sample size.
    n: f64,
}

impl GaussCPDS {
    /// Creates a new `GaussCPDS` instance.
    ///
    /// # Arguments
    ///
    /// * `mu_x` - Design mean vector |X|.
    /// * `mu_z` - Response mean vector |Z|.
    /// * `s_xx` - Design covariance matrix |X| x |X|.
    /// * `s_xz` - Cross-covariance matrix |X| x |Z|.
    /// * `s_zz` - Response covariance matrix |Z| x |Z|.
    /// * `n` - Sample size.
    ///
    /// # Returns
    ///
    /// A new `GaussCPDS` instance.
    ///
    #[inline]
    pub fn new(
        mu_x: Array1<f64>,
        mu_z: Array1<f64>,
        s_xx: Array2<f64>,
        s_xz: Array2<f64>,
        s_zz: Array2<f64>,
        n: f64,
    ) -> Self {
        // Assert the dimensions are correct.
        assert_eq!(
            mu_x.len(),
            s_xx.nrows(),
            "Design mean vector length must match design covariance matrix size."
        );
        assert_eq!(
            mu_z.len(),
            s_zz.nrows(),
            "Response mean vector length must match response covariance matrix size."
        );
        assert!(s_xx.is_square(), "Design covariance matrix must be square.");
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
        assert!(
            s_zz.is_square(),
            "Response covariance matrix must be square."
        );
        // Assert values are finite.
        assert!(
            mu_x.iter().all(|&x| x.is_finite()),
            "Design mean vector must have finite values."
        );
        assert!(
            mu_z.iter().all(|&x| x.is_finite()),
            "Response mean vector must have finite values."
        );
        assert!(
            s_xx.iter().all(|&x| x.is_finite()),
            "Design covariance matrix must have finite values."
        );
        assert!(
            s_xz.iter().all(|&x| x.is_finite()),
            "Cross-covariance matrix must have finite values."
        );
        assert!(
            s_zz.iter().all(|&x| x.is_finite()),
            "Response covariance matrix must have finite values."
        );
        assert!(
            n.is_finite() && n >= 0.0,
            "Sample size must be non-negative."
        );

        Self {
            mu_x,
            mu_z,
            s_xx,
            s_xz,
            s_zz,
            n,
        }
    }

    /// Returns the design mean vector |X|.
    ///
    /// # Returns
    ///
    /// A reference to the design mean vector.
    ///
    #[inline]
    pub fn sample_design_mean(&self) -> &Array1<f64> {
        &self.mu_x
    }

    /// Returns the response mean vector |Z|.
    ///
    /// # Returns
    ///
    /// A reference to the response mean vector.
    ///
    #[inline]
    pub fn sample_response_mean(&self) -> &Array1<f64> {
        &self.mu_z
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
    /// Coefficient matrix |X| x |Z|.
    pub a: Array2<f64>,
    /// Intercept vector |X|.
    pub b: Array1<f64>,
    /// Covariance matrix |X| x |X|.
    pub s: Array2<f64>,
}

impl GaussCPDP {
    /// Creates a new `GaussCPDP` instance.
    ///
    /// # Arguments
    ///
    /// * `a` - Coefficient matrix |X| x |Z|.
    /// * `b` - Intercept vector |X|.
    /// * `s` - Covariance matrix |X| x |X|.
    ///
    /// # Panics
    ///
    /// * Panics if `s` is not square or if the number of rows in `a` does not match the size of `s`.
    ///
    /// # Returns
    ///
    /// A new `GaussCPDP` instance.
    ///
    pub fn new(a: Array2<f64>, b: Array1<f64>, s: Array2<f64>) -> Self {
        // Assert the dimensions are correct.
        assert!(a.is_square(), "Coefficient matrix must be square.");
        assert_eq!(
            a.nrows(),
            b.len(),
            "Coefficient matrix rows must match intercept vector size."
        );
        assert_eq!(
            a.nrows(),
            s.nrows(),
            "Coefficient matrix rows must match covariance matrix size."
        );
        assert!(s.is_square(), "Covariance matrix must be square.");
        // Assert values are finite.
        assert!(
            a.iter().all(|&x| x.is_finite()),
            "Coefficient matrix must have finite values."
        );
        assert!(
            b.iter().all(|&x| x.is_finite()),
            "Intercept vector must have finite values."
        );
        assert!(
            s.iter().all(|&x| x.is_finite()),
            "Covariance matrix must have finite values."
        );

        Self { a, b, s }
    }

    /// Returns the coefficient matrix |X| x |Z|.
    ///
    /// # Returns
    ///
    /// A reference to the coefficient matrix.
    ///
    #[inline]
    pub const fn coefficients(&self) -> &Array2<f64> {
        &self.a
    }

    /// Returns the intercept vector |X|.
    ///
    /// # Returns
    ///
    /// A reference to the intercept vector.
    ///
    #[inline]
    pub const fn intercept(&self) -> &Array1<f64> {
        &self.b
    }

    /// Returns the covariance matrix |X| x |X|.
    ///
    /// # Returns
    ///
    /// A reference to the covariance matrix.
    ///
    #[inline]
    pub const fn covariance(&self) -> &Array2<f64> {
        &self.s
    }
}

impl PartialEq for GaussCPDP {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a) && self.b.eq(&other.b) && self.s.eq(&other.s)
    }
}

impl AbsDiffEq for GaussCPDP {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.a.abs_diff_eq(&other.a, epsilon)
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
        self.a.relative_eq(&other.a, epsilon, max_relative)
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

impl GaussCPD {
    /// Creates a new Gaussian CPD instance.
    ///
    /// # Arguments
    ///
    /// * `labels` - Labels of the variables.
    /// * `conditioning_labels` - Labels of the conditioning variables.
    /// * `parameters` - Parameters of the CPD.
    ///
    /// # Returns
    ///
    /// A new Gaussian CPD instance.
    ///
    pub fn new(labels: Labels, conditioning_labels: Labels, parameters: GaussCPDP) -> Self {
        // FIXME: Check inputs.

        Self {
            labels,
            conditioning_labels,
            parameters,
            sample_statistics: None,
            sample_log_likelihood: None,
        }
    }

    /// Creates a new Gaussian CPD instance.
    ///
    /// # Arguments
    ///
    /// * `labels` - Labels of the variables.
    /// * `conditioning_labels` - Labels of the conditioning variables.
    /// * `parameters` - Parameters of the CPD.
    /// * `sample_statistics` - Sample (sufficient) statistics, if any.
    /// * `sample_log_likelihood` - Sample log-likelihood, if any.
    ///
    /// # Returns
    ///
    /// A new Gaussian CPD instance.
    ///
    pub fn with_optionals(
        labels: Labels,
        conditioning_labels: Labels,
        parameters: GaussCPDP,
        sample_statistics: Option<GaussCPDS>,
        sample_log_likelihood: Option<f64>,
    ) -> Self {
        // FIXME: Check inputs.

        // Create the CPD.
        let mut cpd = Self::new(labels, conditioning_labels, parameters);

        // FIXME: Check labels alignment with optional fields.

        // Set the optional fields.
        cpd.sample_statistics = sample_statistics;
        cpd.sample_log_likelihood = sample_log_likelihood;

        cpd
    }
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
        self.parameters.a.len() + self.parameters.b.len() + self.parameters.s.len()
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
