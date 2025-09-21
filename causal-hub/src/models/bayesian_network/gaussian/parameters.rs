use std::ops::{Add, AddAssign};

use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    models::{CPD, Labelled},
    types::Labels,
};

/// Sample (sufficient) statistics for a Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussCPDS {
    /// Response mean vector |X|.
    mu_x: Array1<f64>,
    /// Design mean vector |Z|.
    mu_z: Array1<f64>,
    /// Response covariance matrix |X| x |X|.
    s_xx: Array2<f64>,
    /// Cross-covariance matrix |X| x |Z|.
    s_xz: Array2<f64>,
    /// Design covariance matrix |Z| x |Z|.
    s_zz: Array2<f64>,
    /// Sample size.
    n: f64,
}

impl GaussCPDS {
    /// Creates a new `GaussCPDS` instance.
    ///
    /// # Arguments
    ///
    /// * `mu_x` - Response mean vector |X|.
    /// * `mu_z` - Design mean vector |Z|.
    /// * `s_xx` - Response covariance matrix |X| x |X|.
    /// * `s_xz` - Cross-covariance matrix |X| x |Z|.
    /// * `s_zz` - Design covariance matrix |Z| x |Z|.
    /// * `n` - Sample size.
    ///
    /// # Panics
    ///
    /// * Panics if `mu_x` length does not match `s_xx` size.
    /// * Panics if `mu_z` length does not match `s_zz` size.
    /// * Panics if `s_xx` is not square.
    /// * Panics if the number of rows of `s_xz` does not match the size of `s_xx`.
    /// * Panics if the number of columns of `s_xz` does not match the size of `s_zz`.
    /// * Panics if `s_zz` is not square.
    /// * Panics if any of the values in `mu_x`, `mu_z`, `s_xx`, `s_xz`, or `s_zz` are not finite.
    /// * Panics if `n` is not finite or is negative.
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
            "Response mean vector length must match response covariance matrix size."
        );
        assert_eq!(
            mu_z.len(),
            s_zz.nrows(),
            "Design mean vector length must match design covariance matrix size."
        );
        assert!(
            s_xx.is_square(),
            "Response covariance matrix must be square."
        );
        assert_eq!(
            s_xz.nrows(),
            s_xx.nrows(),
            "Cross-covariance matrix must have the same \n\
            number of rows as the response covariance matrix."
        );
        assert_eq!(
            s_xz.ncols(),
            s_zz.nrows(),
            "Cross-covariance matrix must have the same \n\
            number of columns as the design covariance matrix."
        );
        assert!(s_zz.is_square(), "Design covariance matrix must be square.");
        // Assert values are finite.
        assert!(
            mu_x.iter().all(|&x| x.is_finite()),
            "Response mean vector must have finite values."
        );
        assert!(
            mu_z.iter().all(|&x| x.is_finite()),
            "Design mean vector must have finite values."
        );
        assert!(
            s_xx.iter().all(|&x| x.is_finite()),
            "Response covariance matrix must have finite values."
        );
        assert!(
            s_xz.iter().all(|&x| x.is_finite()),
            "Cross-covariance matrix must have finite values."
        );
        assert!(
            s_zz.iter().all(|&x| x.is_finite()),
            "Design covariance matrix must have finite values."
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

    /// Returns the response mean vector |X|.
    ///
    /// # Returns
    ///
    /// A reference to the response mean vector.
    ///
    #[inline]
    pub fn sample_response_mean(&self) -> &Array1<f64> {
        &self.mu_x
    }

    /// Returns the design mean vector |Z|.
    ///
    /// # Returns
    ///
    /// A reference to the design mean vector.
    ///
    #[inline]
    pub fn sample_design_mean(&self) -> &Array1<f64> {
        &self.mu_z
    }

    /// Returns the response covariance matrix |X| x |X|.
    ///
    /// # Returns
    ///
    /// A reference to the response covariance matrix.
    ///
    #[inline]
    pub fn sample_response_covariance(&self) -> &Array2<f64> {
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

    /// Returns the design covariance matrix (|Z| + 1) x (|Z| + 1).
    ///
    /// # Returns
    ///
    /// A reference to the design covariance matrix.
    ///
    #[inline]
    pub fn sample_design_covariance(&self) -> &Array2<f64> {
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

impl AddAssign for GaussCPDS {
    fn add_assign(&mut self, other: Self) {
        // Compute the total sample sizes.
        let n = self.n + other.n;
        // Update the response mean vector.
        self.mu_x = (self.n * &self.mu_x + other.n * &other.mu_x) / n;
        // Update the design mean vector.
        self.mu_z = (self.n * &self.mu_z + other.n * &other.mu_z) / n;
        // Update the response covariance matrix.
        self.s_xx =
            // Update the covariance.
            (self.n * &self.s_xx + other.n * &other.s_xx) / n
            // Update the centering contribution.
            + (self.n * other.n / n.powi(2)) *
                (&self.mu_x - &other.mu_x).insert_axis(Axis(1))
                .dot(&(&self.mu_x - &other.mu_x).insert_axis(Axis(0)));
        // Update the cross-covariance matrix.
        self.s_xz =
            // Update the covariance.
            (self.n * &self.s_xz + other.n * &other.s_xz) / n
            // Update the centering contribution.
            + (self.n * other.n / n.powi(2)) *
                (&self.mu_x - &other.mu_x).insert_axis(Axis(1))
                .dot(&(&self.mu_z - &other.mu_z).insert_axis(Axis(0)));
        // Update the design covariance matrix.
        self.s_zz =
            // Update the covariance.
            (self.n * &self.s_zz + other.n * &other.s_zz) / n
            // Update the centering contribution.
            + (self.n * other.n / n.powi(2)) *
                (&self.mu_z - &other.mu_z).insert_axis(Axis(1))
                .dot(&(&self.mu_z - &other.mu_z).insert_axis(Axis(0)));
        // Update the sample size.
        self.n = n;
    }
}

impl Add for GaussCPDS {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Serialize for GaussCPDS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(6))?;

        // Convert the sample response mean to a flat format.
        let sample_response_mean = self.mu_x.to_vec();
        // Serialize sample response mean.
        map.serialize_entry("sample_response_mean", &sample_response_mean)?;

        // Convert the sample design mean to a flat format.
        let sample_design_mean = self.mu_z.to_vec();
        // Serialize sample design mean.
        map.serialize_entry("sample_design_mean", &sample_design_mean)?;

        // Convert the sample response covariance to a flat format.
        let sample_response_covariance: Vec<_> =
            self.s_xx.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize sample response covariance.
        map.serialize_entry("sample_response_covariance", &sample_response_covariance)?;

        // Convert the sample cross covariance to a flat format.
        let sample_cross_covariance: Vec<_> =
            self.s_xz.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize sample cross covariance.
        map.serialize_entry("sample_cross_covariance", &sample_cross_covariance)?;

        // Convert the sample design covariance to a flat format.
        let sample_design_covariance: Vec<_> =
            self.s_zz.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize sample design covariance.
        map.serialize_entry("sample_design_covariance", &sample_design_covariance)?;

        // Serialize sample size.
        map.serialize_entry("sample_size", &self.n)?;

        // End the map.
        map.end()
    }
}

impl<'de> Deserialize<'de> for GaussCPDS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        #[allow(clippy::enum_variant_names)]
        enum Field {
            SampleResponseMean,
            SampleDesignMean,
            SampleResponseCovariance,
            SampleCrossCovariance,
            SampleDesignCovariance,
            SampleSize,
        }

        struct GaussCPDSVisitor;

        impl<'de> Visitor<'de> for GaussCPDSVisitor {
            type Value = GaussCPDS;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GaussCPDS")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GaussCPDS, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate the fields.
                let mut sample_response_mean = None;
                let mut sample_design_mean = None;
                let mut sample_response_covariance = None;
                let mut sample_cross_covariance = None;
                let mut sample_design_covariance = None;
                let mut sample_size = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::SampleResponseMean => {
                            if sample_response_mean.is_some() {
                                return Err(E::duplicate_field("sample_response_mean"));
                            }
                            sample_response_mean = Some(map.next_value()?);
                        }
                        Field::SampleDesignMean => {
                            if sample_design_mean.is_some() {
                                return Err(E::duplicate_field("sample_design_mean"));
                            }
                            sample_design_mean = Some(map.next_value()?);
                        }
                        Field::SampleResponseCovariance => {
                            if sample_response_covariance.is_some() {
                                return Err(E::duplicate_field("sample_response_covariance"));
                            }
                            sample_response_covariance = Some(map.next_value()?);
                        }
                        Field::SampleCrossCovariance => {
                            if sample_cross_covariance.is_some() {
                                return Err(E::duplicate_field("sample_cross_covariance"));
                            }
                            sample_cross_covariance = Some(map.next_value()?);
                        }
                        Field::SampleDesignCovariance => {
                            if sample_design_covariance.is_some() {
                                return Err(E::duplicate_field("sample_design_covariance"));
                            }
                            sample_design_covariance = Some(map.next_value()?);
                        }
                        Field::SampleSize => {
                            if sample_size.is_some() {
                                return Err(E::duplicate_field("sample_size"));
                            }
                            sample_size = Some(map.next_value()?);
                        }
                    }
                }

                // Extract the fields.
                let sample_response_mean =
                    sample_response_mean.ok_or_else(|| E::missing_field("sample_response_mean"))?;
                let sample_design_mean =
                    sample_design_mean.ok_or_else(|| E::missing_field("sample_design_mean"))?;
                let sample_response_covariance = sample_response_covariance
                    .ok_or_else(|| E::missing_field("sample_response_covariance"))?;
                let sample_cross_covariance = sample_cross_covariance
                    .ok_or_else(|| E::missing_field("sample_cross_covariance"))?;
                let sample_design_covariance = sample_design_covariance
                    .ok_or_else(|| E::missing_field("sample_design_covariance"))?;
                let sample_size = sample_size.ok_or_else(|| E::missing_field("sample_size"))?;

                // Convert sample response mean to array.
                let sample_response_mean = Array1::from_vec(sample_response_mean);
                // Convert sample design mean to array.
                let sample_design_mean = Array1::from_vec(sample_design_mean);
                // Convert sample response covariance to array.
                let sample_response_covariance = {
                    let values: Vec<Vec<f64>> = sample_response_covariance;
                    let shape = (values.len(), values[0].len());
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample response covariance shape"))?
                };
                // Convert sample cross covariance to array.
                let sample_cross_covariance = {
                    let values: Vec<Vec<f64>> = sample_cross_covariance;
                    let shape = (values.len(), values[0].len());
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample cross covariance shape"))?
                };
                // Convert sample design covariance to array.
                let sample_design_covariance = {
                    let values: Vec<Vec<f64>> = sample_design_covariance;
                    let shape = (values.len(), values[0].len());
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample design covariance shape"))?
                };

                Ok(GaussCPDS::new(
                    sample_response_mean,
                    sample_design_mean,
                    sample_response_covariance,
                    sample_cross_covariance,
                    sample_design_covariance,
                    sample_size,
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "sample_response_mean",
            "sample_design_mean",
            "sample_response_covariance",
            "sample_cross_covariance",
            "sample_design_covariance",
            "sample_size",
        ];

        deserializer.deserialize_struct("GaussCPDS", FIELDS, GaussCPDSVisitor)
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
    /// * Panics if `a` is not square.
    /// * Panics if the number of rows of `a` does not match the size of `b`.
    /// * Panics if the number of rows of `a` does not match the size of `s`.
    /// * Panics if `s` is not square.
    /// * Panics if any of the values in `a`, `b`, or `s` are not finite.
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

impl Serialize for GaussCPDP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(3))?;

        // Convert the coefficient matrix to a flat format.
        let coefficients: Vec<_> = self.a.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize coefficients.
        map.serialize_entry("coefficients", &coefficients)?;

        // Convert the intercept vector to a flat format.
        let intercept = self.b.to_vec();
        // Serialize intercept.
        map.serialize_entry("intercept", &intercept)?;

        // Convert the covariance matrix to a flat format.
        let covariance: Vec<_> = self.s.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize covariance.
        map.serialize_entry("covariance", &covariance)?;

        // End the map.
        map.end()
    }
}

impl<'de> Deserialize<'de> for GaussCPDP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Coefficients,
            Intercept,
            Covariance,
        }

        struct GaussCPDPVisitor;

        impl<'de> Visitor<'de> for GaussCPDPVisitor {
            type Value = GaussCPDP;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GaussCPDP")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GaussCPDP, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate the fields.
                let mut coefficients = None;
                let mut intercept = None;
                let mut covariance = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Coefficients => {
                            if coefficients.is_some() {
                                return Err(E::duplicate_field("coefficients"));
                            }
                            coefficients = Some(map.next_value()?);
                        }
                        Field::Intercept => {
                            if intercept.is_some() {
                                return Err(E::duplicate_field("intercept"));
                            }
                            intercept = Some(map.next_value()?);
                        }
                        Field::Covariance => {
                            if covariance.is_some() {
                                return Err(E::duplicate_field("covariance"));
                            }
                            covariance = Some(map.next_value()?);
                        }
                    }
                }

                // Extract the fields.
                let coefficients = coefficients.ok_or_else(|| E::missing_field("coefficients"))?;
                let intercept = intercept.ok_or_else(|| E::missing_field("intercept"))?;
                let covariance = covariance.ok_or_else(|| E::missing_field("covariance"))?;

                // Convert coefficients to array.
                let coefficients = {
                    let values: Vec<Vec<f64>> = coefficients;
                    let shape = (values.len(), values[0].len());
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid coefficients shape"))?
                };
                // Convert intercept to array.
                let intercept = Array1::from_vec(intercept);
                // Convert covariance to array.
                let covariance = {
                    let values: Vec<Vec<f64>> = covariance;
                    let shape = (values.len(), values[0].len());
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid covariance shape"))?
                };

                Ok(GaussCPDP::new(coefficients, intercept, covariance))
            }
        }

        const FIELDS: &[&str] = &["coefficients", "intercept", "covariance"];

        deserializer.deserialize_struct("GaussCPDP", FIELDS, GaussCPDPVisitor)
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

impl Serialize for GaussCPD {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the elements to serialize.
        let mut size = 3;
        // Add optional fields, if any.
        size += self.sample_statistics.is_some() as usize;
        size += self.sample_log_likelihood.is_some() as usize;
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(size))?;

        // Serialize labels.
        map.serialize_entry("labels", &self.labels)?;

        // Serialize conditioning labels.
        map.serialize_entry("conditioning_labels", &self.conditioning_labels)?;

        // Serialize parameters.
        map.serialize_entry("parameters", &self.parameters)?;

        // Serialize sample statistics, if any.
        if let Some(sample_statistics) = &self.sample_statistics {
            map.serialize_entry("sample_statistics", sample_statistics)?;
        }

        // Serialize sample log-likelihood, if any.
        if let Some(sample_log_likelihood) = &self.sample_log_likelihood {
            map.serialize_entry("sample_log_likelihood", sample_log_likelihood)?;
        }

        // End the map.
        map.end()
    }
}

impl<'de> Deserialize<'de> for GaussCPD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Labels,
            ConditioningLabels,
            Parameters,
            SampleStatistics,
            SampleLogLikelihood,
        }

        struct GaussCPDVisitor;

        impl<'de> Visitor<'de> for GaussCPDVisitor {
            type Value = GaussCPD;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GaussCPD")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GaussCPD, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate the fields.
                let mut labels = None;
                let mut conditioning_labels = None;
                let mut parameters = None;
                let mut sample_statistics = None;
                let mut sample_log_likelihood = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Labels => {
                            if labels.is_some() {
                                return Err(E::duplicate_field("labels"));
                            }
                            labels = Some(map.next_value()?);
                        }
                        Field::ConditioningLabels => {
                            if conditioning_labels.is_some() {
                                return Err(E::duplicate_field("conditioning_labels"));
                            }
                            conditioning_labels = Some(map.next_value()?);
                        }
                        Field::Parameters => {
                            if parameters.is_some() {
                                return Err(E::duplicate_field("parameters"));
                            }
                            parameters = Some(map.next_value()?);
                        }
                        Field::SampleStatistics => {
                            if sample_statistics.is_some() {
                                return Err(E::duplicate_field("sample_statistics"));
                            }
                            sample_statistics = Some(map.next_value()?);
                        }
                        Field::SampleLogLikelihood => {
                            if sample_log_likelihood.is_some() {
                                return Err(E::duplicate_field("sample_log_likelihood"));
                            }
                            sample_log_likelihood = Some(map.next_value()?);
                        }
                    }
                }

                // Extract the fields.
                let labels = labels.ok_or_else(|| E::missing_field("labels"))?;
                let conditioning_labels =
                    conditioning_labels.ok_or_else(|| E::missing_field("conditioning_labels"))?;
                let parameters = parameters.ok_or_else(|| E::missing_field("parameters"))?;

                Ok(GaussCPD::with_optionals(
                    labels,
                    conditioning_labels,
                    parameters,
                    sample_statistics,
                    sample_log_likelihood,
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "labels",
            "conditioning_labels",
            "parameters",
            "sample_statistics",
            "sample_log_likelihood",
        ];

        deserializer.deserialize_struct("GaussCPD", FIELDS, GaussCPDVisitor)
    }
}
