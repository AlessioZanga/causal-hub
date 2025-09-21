use std::ops::{Add, AddAssign};

use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
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
