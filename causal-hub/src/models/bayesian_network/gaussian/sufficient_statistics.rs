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
    /// Response covariance (uncentered) matrix |X| x |X|.
    m_xx: Array2<f64>,
    /// Cross-covariance (uncentered) matrix |X| x |Z|.
    m_xz: Array2<f64>,
    /// Design covariance (uncentered) matrix |Z| x |Z|.
    m_zz: Array2<f64>,
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
    /// * `m_xx` - Response covariance (uncentered) matrix |X| x |X|.
    /// * `m_xz` - Cross-covariance (uncentered) matrix |X| x |Z|.
    /// * `m_zz` - Design covariance (uncentered) matrix |Z| x |Z|.
    /// * `n` - Sample size.
    ///
    /// # Panics
    ///
    /// * Panics if `mu_x` length does not match `m_xx` size.
    /// * Panics if `mu_z` length does not match `m_zz` size.
    /// * Panics if `m_xx` is not square.
    /// * Panics if the number of rows of `m_xz` does not match the size of `m_xx`.
    /// * Panics if the number of columns of `m_xz` does not match the size of `m_zz`.
    /// * Panics if `m_zz` is not square.
    /// * Panics if any of the values in `mu_x`, `mu_z`, `m_xx`, `m_xz`, or `m_zz` are not finite.
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
        m_xx: Array2<f64>,
        m_xz: Array2<f64>,
        m_zz: Array2<f64>,
        n: f64,
    ) -> Self {
        // Assert the dimensions are correct.
        assert_eq!(
            mu_x.len(),
            m_xx.nrows(),
            "Response mean vector length must match response covariance matrix size."
        );
        assert_eq!(
            mu_z.len(),
            m_zz.nrows(),
            "Design mean vector length must match design covariance matrix size."
        );
        assert!(
            m_xx.is_square(),
            "Response covariance matrix must be square."
        );
        assert_eq!(
            m_xz.nrows(),
            m_xx.nrows(),
            "Cross-covariance matrix must have the same \n\
            number of rows as the response covariance matrix."
        );
        assert_eq!(
            m_xz.ncols(),
            m_zz.nrows(),
            "Cross-covariance matrix must have the same \n\
            number of columns as the design covariance matrix."
        );
        assert!(m_zz.is_square(), "Design covariance matrix must be square.");
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
            m_xx.iter().all(|&x| x.is_finite()),
            "Response covariance matrix must have finite values."
        );
        assert!(
            m_xz.iter().all(|&x| x.is_finite()),
            "Cross-covariance matrix must have finite values."
        );
        assert!(
            m_zz.iter().all(|&x| x.is_finite()),
            "Design covariance matrix must have finite values."
        );
        assert!(
            n.is_finite() && n >= 0.0,
            "Sample size must be non-negative."
        );

        Self {
            mu_x,
            mu_z,
            m_xx,
            m_xz,
            m_zz,
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
    pub fn sample_response_covariance(&self) -> Array2<f64> {
        // Compute the centering factor.
        let col_mu_x = self.mu_x.view().insert_axis(Axis(1));
        let row_mu_x = self.mu_x.view().insert_axis(Axis(0));
        // Apply centering.
        &self.m_xx - self.n * &col_mu_x.dot(&row_mu_x)
    }

    /// Returns the cross-covariance matrix |X| x (|Z| + 1).
    ///
    /// # Returns
    ///
    /// A reference to the cross-covariance matrix.
    ///
    #[inline]
    pub fn sample_cross_covariance(&self) -> Array2<f64> {
        // Compute the centering factor.
        let col_mu_x = self.mu_x.view().insert_axis(Axis(1));
        let row_mu_z = self.mu_z.view().insert_axis(Axis(0));
        // Apply centering.
        &self.m_xz - self.n * &col_mu_x.dot(&row_mu_z)
    }

    /// Returns the design covariance matrix (|Z| + 1) x (|Z| + 1).
    ///
    /// # Returns
    ///
    /// A reference to the design covariance matrix.
    ///
    #[inline]
    pub fn sample_design_covariance(&self) -> Array2<f64> {
        // Compute the centering factor.
        let col_mu_z = self.mu_z.view().insert_axis(Axis(1));
        let row_mu_z = self.mu_z.view().insert_axis(Axis(0));
        // Apply centering.
        &self.m_zz - self.n * &col_mu_z.dot(&row_mu_z)
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
        self.m_xx = (self.n * &self.m_xx + other.n * &other.m_xx) / n;
        // Update the cross-covariance matrix.
        self.m_xz = (self.n * &self.m_xz + other.n * &other.m_xz) / n;
        // Update the design covariance matrix.
        self.m_zz = (self.n * &self.m_zz + other.n * &other.m_zz) / n;
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
            self.m_xx.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize sample response covariance.
        map.serialize_entry("sample_response_covariance", &sample_response_covariance)?;

        // Convert the sample cross covariance to a flat format.
        let sample_cross_covariance: Vec<_> =
            self.m_xz.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize sample cross covariance.
        map.serialize_entry("sample_cross_covariance", &sample_cross_covariance)?;

        // Convert the sample design covariance to a flat format.
        let sample_design_covariance: Vec<_> =
            self.m_zz.rows().into_iter().map(|x| x.to_vec()).collect();
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
                    let shape = (values.len(), values.first().map_or(0, |v| v.len()));
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample response covariance shape"))?
                };
                // Convert sample cross covariance to array.
                let sample_cross_covariance = {
                    let values: Vec<Vec<f64>> = sample_cross_covariance;
                    let shape = (values.len(), values.first().map_or(0, |v| v.len()));
                    Array::from_iter(values.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample cross covariance shape"))?
                };
                // Convert sample design covariance to array.
                let sample_design_covariance = {
                    let values: Vec<Vec<f64>> = sample_design_covariance;
                    let shape = (values.len(), values.first().map_or(0, |v| v.len()));
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
