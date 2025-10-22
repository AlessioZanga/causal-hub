use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;
use ndarray_linalg::{CholeskyInto, Determinant, UPLO};
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::GaussSample,
    impl_json_io,
    models::{CPD, GaussCPDS, GaussPhi, Labelled, Phi},
    types::{EPSILON, LN_2_PI, Labels, Set},
    utils::PseudoInverse,
};

/// Parameters of a Gaussian CPD.
#[derive(Clone, Debug)]
pub struct GaussCPDP {
    /// Coefficient matrix |X| x |Z|.
    a: Array2<f64>,
    /// Intercept vector |X|.
    b: Array1<f64>,
    /// Covariance matrix |X| x |X|.
    s: Array2<f64>,
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
    /// * Panics if the number of rows of `a` does not match the size of `b`.
    /// * Panics if the number of rows of `a` does not match the size of `s`.
    /// * Panics if `s` is not square and symmetric.
    /// * Panics if any of the values in `a`, `b`, or `s` are not finite.
    ///
    /// # Returns
    ///
    /// A new `GaussCPDP` instance.
    ///
    pub fn new(a: Array2<f64>, b: Array1<f64>, s: Array2<f64>) -> Self {
        // Assert the dimensions are correct.
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
        assert_eq!(s, s.t(), "Covariance matrix must be symmetric.");

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
    /// # Panics
    ///
    /// * Panics if `labels` and `conditioning_labels` are not disjoint.
    /// * Panics if the dimensions of `parameters` do not match the lengths of `labels` and `conditioning_labels`.
    ///
    /// # Returns
    ///
    /// A new Gaussian CPD instance.
    ///
    pub fn new(
        mut labels: Labels,
        mut conditioning_labels: Labels,
        mut parameters: GaussCPDP,
    ) -> Self {
        // Assert labels and conditioning labels are disjoint.
        assert!(
            labels.is_disjoint(&conditioning_labels),
            "Labels and conditioning labels must be disjoint."
        );
        // Assert parameters dimensions match labels and conditioning labels lengths.
        assert_eq!(
            parameters.a.nrows(),
            labels.len(),
            "Coefficient matrix rows must match labels length."
        );
        assert_eq!(
            parameters.a.ncols(),
            conditioning_labels.len(),
            "Coefficient matrix columns must match conditioning labels length."
        );
        assert_eq!(
            parameters.b.len(),
            labels.len(),
            "Intercept vector size must match labels length."
        );
        assert_eq!(
            parameters.s.nrows(),
            labels.len(),
            "Covariance matrix rows must match labels length."
        );
        assert_eq!(
            parameters.s.ncols(),
            labels.len(),
            "Covariance matrix columns must match labels length."
        );

        // Check if labels are sorted.
        if !labels.is_sorted() {
            // Allocate indices to sort labels.
            let mut indices: Vec<usize> = (0..labels.len()).collect();
            // Sort the indices by labels.
            indices.sort_by_key(|&i| &labels[i]);
            // Sort the labels.
            labels.sort();
            // Reorder the parameters.
            let mut new_a = parameters.a.clone();
            let mut new_b = parameters.b.clone();
            let mut new_s = parameters.s.clone();
            // Reorder rows of A.
            for (i, &j) in indices.iter().enumerate() {
                new_a.row_mut(i).assign(&parameters.a.row(j));
            }
            // Reorder b.
            for (i, &j) in indices.iter().enumerate() {
                new_b[i] = parameters.b[j];
            }
            // Reorder rows of S.
            for (i, &j) in indices.iter().enumerate() {
                new_s.row_mut(i).assign(&parameters.s.row(j));
            }
            // Allocate a temporary copy of S to reorder columns.
            let _s = new_s.clone();
            // Reorder columns of S.
            for (i, &j) in indices.iter().enumerate() {
                new_s.column_mut(i).assign(&_s.column(j));
            }
            // Update parameters.
            parameters.a = new_a;
            parameters.b = new_b;
            parameters.s = new_s;
        }

        // Check if conditioning labels are sorted.
        if !conditioning_labels.is_sorted() {
            // Allocate indices to sort conditioning labels.
            let mut indices: Vec<usize> = (0..conditioning_labels.len()).collect();
            // Sort the indices by conditioning labels.
            indices.sort_by_key(|&i| &conditioning_labels[i]);
            // Sort the conditioning labels.
            conditioning_labels.sort();
            // Reorder the parameters.
            let mut new_a = parameters.a.clone();
            // Reorder columns of A.
            for (i, &j) in indices.iter().enumerate() {
                new_a.column_mut(i).assign(&parameters.a.column(j));
            }
            // Update parameters.
            parameters.a = new_a;
        }

        Self {
            labels,
            conditioning_labels,
            parameters,
            sample_statistics: None,
            sample_log_likelihood: None,
        }
    }

    /// Marginalizes the over the variables `X` and conditioning variables `Z`.
    ///
    /// # Arguments
    ///
    /// * `x` - The variables to marginalize over.
    /// * `z` - The conditioning variables to marginalize over.
    ///
    /// # Returns
    ///
    /// A new instance with the marginalized variables.
    ///
    pub fn marginalize(&self, x: &Set<usize>, z: &Set<usize>) -> Self {
        // Base case: if no variables to marginalize, return self clone.
        if x.is_empty() && z.is_empty() {
            return self.clone();
        }
        // Get labels.
        let labels_x = self.labels();
        let labels_z = self.conditioning_labels();
        // Get indices to preserve.
        let not_x = (0..labels_x.len()).filter(|i| !x.contains(i)).collect();
        let not_z = (0..labels_z.len()).filter(|i| !z.contains(i)).collect();
        // Convert to potential.
        let phi = self.clone().into_phi();
        // Map CPD indices to potential indices.
        let x = phi.indices_from(x, labels_x);
        let z = phi.indices_from(z, labels_z);
        // Marginalize the potential.
        let phi = phi.marginalize(&(&x | &z));
        // Map CPD indices to potential indices.
        let not_x = phi.indices_from(&not_x, labels_x);
        let not_z = phi.indices_from(&not_z, labels_z);
        // Convert back to CPD.
        phi.into_cpd(&not_x, &not_z)
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

    /// Converts a potential \phi(X \cup Z) to a CPD P(X | Z).
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Returns
    ///
    /// The corresponding CPD.
    ///
    #[inline]
    pub fn from_phi(phi: GaussPhi, x: &Set<usize>, z: &Set<usize>) -> Self {
        phi.into_cpd(x, z)
    }

    /// Converts a CPD P(X | Z) to a potential \phi(X \cup Z).
    ///
    /// # Arguments
    ///
    /// * `cpd` - The CPD to convert.
    ///
    /// # Returns
    ///
    /// The corresponding potential.
    ///
    #[inline]
    pub fn into_phi(self) -> GaussPhi {
        GaussPhi::from_cpd(self)
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
    type Support = GaussSample;
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
        let s = {
            // Covariance matrix is symmetric.
            let s = self.parameters.s.nrows();
            s * (s + 1) / 2
        };

        self.parameters.a.len() + self.parameters.b.len() + s
    }

    #[inline]
    fn sample_statistics(&self) -> Option<&Self::Statistics> {
        self.sample_statistics.as_ref()
    }

    #[inline]
    fn sample_log_likelihood(&self) -> Option<f64> {
        self.sample_log_likelihood
    }

    fn pf(&self, x: &Self::Support, z: &Self::Support) -> f64 {
        // Get number of variables.
        let n = self.labels.len();
        // Get number of conditioning variables.
        let m = self.conditioning_labels.len();

        // Assert X matches number of variables.
        assert_eq!(
            x.len(),
            n,
            "Vector X must match number of variables: \n\
            \t expected:    |X| == {} , \n\
            \t found:       |X| == {} .",
            n,
            x.len(),
        );
        // Assert Z matches number of conditioning variables.
        assert_eq!(
            z.len(),
            m,
            "Vector Z must match number of conditioning variables: \n\
            \t expected:    |Z| == {} , \n\
            \t found:       |Z| == {} .",
            m,
            z.len(),
        );

        // Get parameters.
        let (a, b, s) = (
            self.parameters.coefficients(),
            self.parameters.intercept(),
            self.parameters.covariance(),
        );

        // No variables.
        if n == 0 {
            return 1.;
        }

        // One variable ...
        if n == 1 {
            // Compute the mean.
            let mu = match m {
                // ... no conditioning variables.
                0 => b[0], // Get the mean.
                // ... one conditioning variable.
                1 => f64::mul_add(a[[0, 0]], z[0], b[0]), // Compute the mean.
                // ... multiple conditioning variables.
                _ => (a.dot(z) + b)[0], // Compute mean vector.
            };
            // Compute deviation from mean.
            let x_mu = x[0] - mu;
            // Get the variance.
            let k = s[[0, 0]];
            // Compute log probability density function.
            let ln_pf = -0.5 * (LN_2_PI + f64::ln(k) + f64::powi(x_mu, 2) / k);
            // Return probability density function.
            return f64::exp(ln_pf);
        }

        // Multiple variables, multiple conditioning variables.

        // Compute mean vector.
        let mu = a.dot(z) + b;
        // Compute deviation from mean.
        let x_mu = x - mu;
        // Compute precision matrix.
        let k = s.pinv();
        // Compute log probability density function.
        let n_ln_2_pi = s.nrows() as f64 * LN_2_PI;
        let (_, ln_det) = s.sln_det().expect("Failed to compute the determinant.");
        let ln_pf = -0.5 * (n_ln_2_pi + ln_det + x_mu.dot(&k).dot(&x_mu));
        // Return probability density function.
        f64::exp(ln_pf)
    }

    fn sample<R: Rng>(&self, rng: &mut R, z: &Self::Support) -> Self::Support {
        // Get number of variables.
        let n = self.labels.len();
        // Get number of conditioning variables.
        let m = self.conditioning_labels.len();

        // Assert Z matches number of conditioning variables.
        assert_eq!(
            z.len(),
            m,
            "Vector Z must match number of conditioning variables: \n\
            \t expected:    |Z| == {} , \n\
            \t found:       |Z| == {} .",
            m,
            z.len(),
        );

        // Get parameters.
        let (a, b, s) = (
            self.parameters.coefficients(),
            self.parameters.intercept(),
            self.parameters.covariance(),
        );

        // No variables.
        if n == 0 {
            return array![];
        }

        // One variable ...
        if n == 1 {
            // Compute the mean.
            let mu = match m {
                // ... no conditioning variables.
                0 => b[0], // Get the mean.
                // ... one conditioning variable.
                1 => f64::mul_add(a[[0, 0]], z[0], b[0]), // Compute the mean.
                // ... multiple conditioning variables.
                _ => (a.dot(z) + b)[0], // Compute mean vector.
            };
            // Sample from standard normal.
            let e: f64 = StandardNormal.sample(rng);
            // Compute the sample.
            let x = f64::mul_add(s[[0, 0]].sqrt(), e, mu);
            // Return the sample.
            return array![x];
        }

        // Multiple variables, multiple conditioning variables.

        // Compute the mean.
        let mu = a.dot(z) + b;
        // Compute the Cholesky decomposition of the covariance matrix.
        let l = (s + EPSILON * Array::eye(s.nrows()))
            .cholesky_into(UPLO::Lower)
            .expect("Failed to compute Cholesky decomposition.");
        // Sample from standard normal.
        let e = StandardNormal
            .sample_iter(rng)
            .take(s.nrows())
            .collect::<Array1<_>>();
        // Compute the sample.
        l.dot(&e) + mu
    }
}

impl Serialize for GaussCPD {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the elements to serialize.
        let mut size = 4;
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

        // Serialize type.
        map.serialize_entry("type", "gausscpd")?;

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
            Type,
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
                let mut type_ = None;

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
                        Field::Type => {
                            if type_.is_some() {
                                return Err(E::duplicate_field("type"));
                            }
                            type_ = Some(map.next_value()?);
                        }
                    }
                }

                // Extract the fields.
                let labels = labels.ok_or_else(|| E::missing_field("labels"))?;
                let conditioning_labels =
                    conditioning_labels.ok_or_else(|| E::missing_field("conditioning_labels"))?;
                let parameters = parameters.ok_or_else(|| E::missing_field("parameters"))?;

                // Assert type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                assert_eq!(type_, "gausscpd", "Invalid type for GaussCPD.");

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
            "type",
        ];

        deserializer.deserialize_struct("GaussCPD", FIELDS, GaussCPDVisitor)
    }
}

// Implement `JsonIO` for `GaussCPD`.
impl_json_io!(GaussCPD);
