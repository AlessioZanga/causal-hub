use std::ops::{Add, AddAssign};

use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::types::Error;

/// Sample (sufficient) statistics for a Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussCPDS {
    /// Response mean vector |X|.
    mu_x: Array1<f64>,
    /// Design mean vector |Z|.
    mu_z: Array1<f64>,
    /// Response scatter matrix (sum of centered outer products) |X| x |X|.
    s_xx: Array2<f64>,
    /// Cross scatter matrix (sum of centered outer products) |X| x |Z|.
    s_xz: Array2<f64>,
    /// Design scatter matrix (sum of centered outer products) |Z| x |Z|.
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
    /// * `s_xx` - Response scatter matrix (centered) |X| x |X|.
    /// * `s_xz` - Cross scatter matrix (centered) |X| x |Z|.
    /// * `s_zz` - Design scatter matrix (centered) |Z| x |Z|.
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
    ) -> std::result::Result<Self, Error> {
        // Check the dimensions are correct.
        if mu_x.len() != s_xx.nrows() {
            return Err(Error::IncompatibleShape(
                "mu_x",
                "Response mean vector length must match response covariance matrix size.",
            ));
        }
        if mu_z.len() != s_zz.nrows() {
            return Err(Error::IncompatibleShape(
                "mu_z",
                "Design mean vector length must match design covariance matrix size.",
            ));
        }
        if !s_xx.is_square() {
            return Err(Error::Shape("Response covariance matrix must be square."));
        }
        if s_xz.nrows() != s_xx.nrows() {
            return Err(Error::IncompatibleShape(
                "s_xz",
                "Cross-covariance matrix must have the same number of rows as the response covariance matrix.",
            ));
        }
        if s_xz.ncols() != s_zz.nrows() {
            return Err(Error::IncompatibleShape(
                "s_xz",
                "Cross-covariance matrix must have the same number of columns as the design covariance matrix.",
            ));
        }
        if !s_zz.is_square() {
            return Err(Error::Shape("Design covariance matrix must be square."));
        }
        // Check values are finite.
        if !mu_x.iter().all(|&x| x.is_finite()) {
            return Err(Error::InvalidParameter(
                "mu_x",
                "Response mean vector must have finite values.",
            ));
        }
        if !mu_z.iter().all(|&x| x.is_finite()) {
            return Err(Error::InvalidParameter(
                "mu_z",
                "Design mean vector must have finite values.",
            ));
        }
        if !s_xx.iter().all(|&x| x.is_finite()) {
            return Err(Error::InvalidParameter(
                "s_xx",
                "Response covariance matrix must have finite values.",
            ));
        }
        if !s_xz.iter().all(|&x| x.is_finite()) {
            return Err(Error::InvalidParameter(
                "s_xz",
                "Cross-covariance matrix must have finite values.",
            ));
        }
        if !s_zz.iter().all(|&x| x.is_finite()) {
            return Err(Error::InvalidParameter(
                "s_zz",
                "Design covariance matrix must have finite values.",
            ));
        }
        if !n.is_finite() || n < 0.0 {
            return Err(Error::InvalidParameter(
                "n",
                "Sample size must be finite and non-negative.",
            ));
        }

        Ok(Self {
            mu_x,
            mu_z,
            s_xx,
            s_xz,
            s_zz,
            n,
        })
    }

    /// Returns the response mean vector |X|.
    ///
    /// # Returns
    ///
    /// A reference to the response mean vector.
    ///
    #[inline]
    pub fn fitted_response_mean(&self) -> &Array1<f64> {
        &self.mu_x
    }

    /// Returns the design mean vector |Z|.
    ///
    /// # Returns
    ///
    /// A reference to the design mean vector.
    ///
    #[inline]
    pub fn fitted_design_mean(&self) -> &Array1<f64> {
        &self.mu_z
    }

    /// Returns the response scatter matrix (sum of squared deviations) |X| x |X|.
    ///
    /// # Returns
    ///
    /// The response scatter matrix.
    ///
    #[inline]
    pub fn fitted_response_covariance(&self) -> Array2<f64> {
        self.s_xx.clone()
    }

    /// Returns the cross-scatter matrix (sum of squared deviations) |X| x |Z|.
    ///
    /// # Returns
    ///
    /// The cross-scatter matrix.
    ///
    #[inline]
    pub fn fitted_cross_covariance(&self) -> Array2<f64> {
        self.s_xz.clone()
    }

    /// Returns the design scatter matrix (sum of squared deviations) |Z| x |Z|.
    ///
    /// # Returns
    ///
    /// The design scatter matrix.
    ///
    #[inline]
    pub fn fitted_design_covariance(&self) -> Array2<f64> {
        self.s_zz.clone()
    }

    /// Returns the fitted size.
    ///
    /// # Returns
    ///
    /// The fitted size.
    ///
    #[inline]
    pub fn fitted_size(&self) -> f64 {
        self.n
    }
}

impl AddAssign for GaussCPDS {
    fn add_assign(&mut self, other: Self) {
        // If the other set is empty, do nothing.
        if other.n == 0. {
            return;
        }

        // If the current set is empty, replace it with the other.
        if self.n == 0. {
            *self = other;
            return;
        }

        // Compute the total sample sizes.
        let n = self.n + other.n;
        // Compute the delta.
        let d_mu_x = &other.mu_x - &self.mu_x;
        let d_mu_z = &other.mu_z - &self.mu_z;

        // Update the response mean vector.
        self.mu_x = (self.n * &self.mu_x + other.n * &other.mu_x) / n;
        // Update the design mean vector.
        self.mu_z = (self.n * &self.mu_z + other.n * &other.mu_z) / n;

        // Compute the scaling factor.
        let scaling = self.n * other.n / n;
        // Update the response covariance matrix.
        self.s_xx = &self.s_xx
            + &other.s_xx
            + scaling
                * d_mu_x
                    .view()
                    .insert_axis(Axis(1))
                    .dot(&d_mu_x.view().insert_axis(Axis(0)));
        // Update the cross-covariance matrix.
        self.s_xz = &self.s_xz
            + &other.s_xz
            + scaling
                * d_mu_x
                    .view()
                    .insert_axis(Axis(1))
                    .dot(&d_mu_z.view().insert_axis(Axis(0)));
        // Update the design covariance matrix.
        self.s_zz = &self.s_zz
            + &other.s_zz
            + scaling
                * d_mu_z
                    .view()
                    .insert_axis(Axis(1))
                    .dot(&d_mu_z.view().insert_axis(Axis(0)));

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

        // Convert the fitted response mean to a flat format.
        let fitted_response_mean = self.mu_x.to_vec();
        // Serialize fitted response mean.
        map.serialize_entry("fitted_response_mean", &fitted_response_mean)?;

        // Convert the fitted design mean to a flat format.
        let fitted_design_mean = self.mu_z.to_vec();
        // Serialize fitted design mean.
        map.serialize_entry("fitted_design_mean", &fitted_design_mean)?;

        // Convert the fitted response covariance to a flat format.
        let fitted_response_covariance: Vec<_> =
            self.s_xx.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize fitted response covariance.
        map.serialize_entry("fitted_response_covariance", &fitted_response_covariance)?;

        // Convert the fitted cross covariance to a flat format.
        let fitted_cross_covariance: Vec<_> =
            self.s_xz.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize fitted cross covariance.
        map.serialize_entry("fitted_cross_covariance", &fitted_cross_covariance)?;

        // Convert the fitted design covariance to a flat format.
        let fitted_design_covariance: Vec<_> =
            self.s_zz.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize fitted design covariance.
        map.serialize_entry("fitted_design_covariance", &fitted_design_covariance)?;

        // Serialize fitted size.
        map.serialize_entry("fitted_size", &self.n)?;

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
            FittedResponseMean,
            FittedDesignMean,
            FittedResponseCovariance,
            FittedCrossCovariance,
            FittedDesignCovariance,
            FittedSize,
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
                let mut fitted_response_mean = None;
                let mut fitted_design_mean = None;
                let mut fitted_response_covariance = None;
                let mut fitted_cross_covariance = None;
                let mut fitted_design_covariance = None;
                let mut fitted_size = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::FittedResponseMean => {
                            if fitted_response_mean.is_some() {
                                return Err(E::duplicate_field("fitted_response_mean"));
                            }
                            fitted_response_mean = Some(map.next_value()?);
                        }
                        Field::FittedDesignMean => {
                            if fitted_design_mean.is_some() {
                                return Err(E::duplicate_field("fitted_design_mean"));
                            }
                            fitted_design_mean = Some(map.next_value()?);
                        }
                        Field::FittedResponseCovariance => {
                            if fitted_response_covariance.is_some() {
                                return Err(E::duplicate_field("fitted_response_covariance"));
                            }
                            fitted_response_covariance = Some(map.next_value()?);
                        }
                        Field::FittedCrossCovariance => {
                            if fitted_cross_covariance.is_some() {
                                return Err(E::duplicate_field("fitted_cross_covariance"));
                            }
                            fitted_cross_covariance = Some(map.next_value()?);
                        }
                        Field::FittedDesignCovariance => {
                            if fitted_design_covariance.is_some() {
                                return Err(E::duplicate_field("fitted_design_covariance"));
                            }
                            fitted_design_covariance = Some(map.next_value()?);
                        }
                        Field::FittedSize => {
                            if fitted_size.is_some() {
                                return Err(E::duplicate_field("fitted_size"));
                            }
                            fitted_size = Some(map.next_value()?);
                        }
                    }
                }

                // Extract the fields.
                let fitted_response_mean =
                    fitted_response_mean.ok_or_else(|| E::missing_field("fitted_response_mean"))?;
                let fitted_design_mean =
                    fitted_design_mean.ok_or_else(|| E::missing_field("fitted_design_mean"))?;
                let fitted_response_covariance = fitted_response_covariance
                    .ok_or_else(|| E::missing_field("fitted_response_covariance"))?;
                let fitted_cross_covariance = fitted_cross_covariance
                    .ok_or_else(|| E::missing_field("fitted_cross_covariance"))?;
                let fitted_design_covariance = fitted_design_covariance
                    .ok_or_else(|| E::missing_field("fitted_design_covariance"))?;
                let fitted_size = fitted_size.ok_or_else(|| E::missing_field("fitted_size"))?;

                // Convert fitted response mean to array.
                let fitted_response_mean = Array1::from_vec(fitted_response_mean);
                // Convert fitted design mean to array.
                let fitted_design_mean = Array1::from_vec(fitted_design_mean);
                // Convert fitted response covariance to array.
                let fitted_response_covariance = {
                    let values: Vec<Vec<f64>> = fitted_response_covariance;
                    let shape = (values.len(), values.first().map_or(0, |v| v.len()));
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid fitted response covariance shape"))?
                };
                // Convert fitted cross covariance to array.
                let fitted_cross_covariance = {
                    let values: Vec<Vec<f64>> = fitted_cross_covariance;
                    let shape = (values.len(), values.first().map_or(0, |v| v.len()));
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid fitted cross covariance shape"))?
                };
                // Convert fitted design covariance to array.
                let fitted_design_covariance = {
                    let values: Vec<Vec<f64>> = fitted_design_covariance;
                    let shape = (values.len(), values.first().map_or(0, |v| v.len()));
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid fitted design covariance shape"))?
                };

                GaussCPDS::new(
                    fitted_response_mean,
                    fitted_design_mean,
                    fitted_response_covariance,
                    fitted_cross_covariance,
                    fitted_design_covariance,
                    fitted_size,
                )
                .map_err(|evidence| E::custom(evidence.to_string()))
            }
        }

        const FIELDS: &[&str] = &[
            "fitted_response_mean",
            "fitted_design_mean",
            "fitted_response_covariance",
            "fitted_cross_covariance",
            "fitted_design_covariance",
            "fitted_size",
        ];

        deserializer.deserialize_struct("GaussCPDS", FIELDS, GaussCPDSVisitor)
    }
}
