use std::{
    borrow::Cow,
    ops::{Add, AddAssign},
};

use approx::{AbsDiffEq, RelativeEq, relative_eq};
use itertools::Itertools;
use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::CatSample,
    impl_json_io,
    models::{CIM, CatSupport, HasLabels},
    types::{EPSILON, Error, Labels, Result, Set},
    utils::MI,
};

/// Sample (sufficient) statistics for a categorical CIM.
#[derive(Clone, Debug)]
pub struct CatCIMS {
    /// Conditional counts |Z| x |X| x |X|.
    n_xz: Array3<f64>,
    /// Conditional times |Z| x |X|.
    t_xz: Array2<f64>,
    /// Sample size.
    n: f64,
}

impl CatCIMS {
    /// Creates a new sample (sufficient) statistics for the categorical CIM.
    ///
    /// # Arguments
    ///
    /// * `n_xz` - Conditional counts |Z| x |X| x |X|.
    /// * `t_xz` - Conditional times |Z| x |X|.
    /// * `n` - Sample size.
    ///
    /// # Returns
    ///
    /// A new sample (sufficient) statistics for the categorical CIM.
    ///
    #[inline]
    pub fn new(n_xz: Array3<f64>, t_xz: Array2<f64>, n: f64) -> Result<Self> {
        // Check the dimensions are correct.
        if n_xz.shape()[1] != n_xz.shape()[2] {
            return Err(Error::Shape(
                "The second and third dimensions of the conditional counts must be equal.",
            ));
        }
        if n_xz.shape()[0] != t_xz.shape()[0] {
            return Err(Error::IncompatibleShape(
                "n_xz",
                "The first dimension of the conditional counts must match the first dimension of the conditional times.",
            ));
        }
        if n_xz.shape()[1] != t_xz.shape()[1] {
            return Err(Error::IncompatibleShape(
                "n_xz",
                "The second dimension of the conditional counts must match the second dimension of the conditional times.",
            ));
        }
        if !n_xz.iter().all(|&x| x.is_finite() && x >= 0.) {
            return Err(Error::InvalidParameter(
                "n_xz",
                "Conditional counts must be finite and non-negative.",
            ));
        }
        if !t_xz.iter().all(|&x| x.is_finite() && x >= 0.) {
            return Err(Error::InvalidParameter(
                "t_xz",
                "Conditional times must be finite and non-negative.",
            ));
        }
        if !n.is_finite() || n < 0. {
            return Err(Error::InvalidParameter(
                "n",
                "Sample size must be finite and non-negative.",
            ));
        }

        Ok(Self { n_xz, t_xz, n })
    }

    /// Returns the fitted conditional counts |Z| x |X| x |X|.
    ///
    /// # Returns
    ///
    /// The fitted conditional counts |Z| x |X| x |X|.
    ///
    #[inline]
    pub const fn fitted_conditional_counts(&self) -> &Array3<f64> {
        &self.n_xz
    }

    /// Returns the fitted conditional times |Z| x |X|.
    ///
    /// # Returns
    ///
    /// The fitted conditional times |Z| x |X|.
    ///
    #[inline]
    pub const fn fitted_conditional_times(&self) -> &Array2<f64> {
        &self.t_xz
    }

    /// Returns the fitted size.
    ///
    /// # Returns
    ///
    /// The fitted size.
    ///
    #[inline]
    pub const fn fitted_size(&self) -> f64 {
        self.n
    }
}

impl AddAssign for CatCIMS {
    fn add_assign(&mut self, other: Self) {
        // Add the counts and times.
        self.n_xz += &other.n_xz;
        self.t_xz += &other.t_xz;
        self.n += other.n;
    }
}

impl Add for CatCIMS {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl Serialize for CatCIMS {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(3))?;

        // Convert the fitted conditional counts to a flat format.
        let fitted_conditional_counts: Vec<Vec<Vec<f64>>> = self
            .n_xz
            .outer_iter()
            .map(|fitted_conditional_counts| {
                fitted_conditional_counts
                    .rows()
                    .into_iter()
                    .map(|x| x.to_vec())
                    .collect()
            })
            .collect();

        // Serialize fitted conditional counts.
        map.serialize_entry("fitted_conditional_counts", &fitted_conditional_counts)?;

        // Convert the fitted conditional times to a flat format.
        let fitted_conditional_times: Vec<Vec<f64>> =
            self.t_xz.rows().into_iter().map(|x| x.to_vec()).collect();

        // Serialize fitted conditional times.
        map.serialize_entry("fitted_conditional_times", &fitted_conditional_times)?;

        // Serialize fitted size.
        map.serialize_entry("fitted_size", &self.n)?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatCIMS {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        #[allow(clippy::enum_variant_names)]
        enum Field {
            FittedConditionalCounts,
            FittedConditionalTimes,
            FittedSize,
        }

        struct CatCIMSVisitor;

        impl<'de> Visitor<'de> for CatCIMSVisitor {
            type Value = CatCIMS;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatCIMS")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CatCIMS, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate fields
                let mut fitted_conditional_counts = None;
                let mut fitted_conditional_times = None;
                let mut fitted_size = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::FittedConditionalCounts => {
                            if fitted_conditional_counts.is_some() {
                                return Err(E::duplicate_field("fitted_conditional_counts"));
                            }
                            fitted_conditional_counts = Some(map.next_value()?);
                        }
                        Field::FittedConditionalTimes => {
                            if fitted_conditional_times.is_some() {
                                return Err(E::duplicate_field("fitted_conditional_times"));
                            }
                            fitted_conditional_times = Some(map.next_value()?);
                        }
                        Field::FittedSize => {
                            if fitted_size.is_some() {
                                return Err(E::duplicate_field("fitted_size"));
                            }
                            fitted_size = Some(map.next_value()?);
                        }
                    }
                }

                // Check all fields are present.
                let fitted_conditional_counts = fitted_conditional_counts
                    .ok_or_else(|| E::missing_field("fitted_conditional_counts"))?;
                let fitted_conditional_times = fitted_conditional_times
                    .ok_or_else(|| E::missing_field("fitted_conditional_times"))?;
                let fitted_size = fitted_size.ok_or_else(|| E::missing_field("fitted_size"))?;

                // Convert fitted conditional counts to ndarray.
                let fitted_conditional_counts = {
                    let counts: Vec<Vec<Vec<f64>>> = fitted_conditional_counts;
                    let shape = (counts.len(), counts[0].len(), counts[0][0].len());
                    let counts = counts.into_iter().flatten().flatten();
                    Array::from_iter(counts)
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid fitted conditional counts shape"))?
                };

                // Convert fitted conditional times to ndarray.
                let fitted_conditional_times = {
                    let times: Vec<Vec<f64>> = fitted_conditional_times;
                    let shape = (times.len(), times[0].len());
                    let times = times.into_iter().flatten();
                    Array::from_iter(times)
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid fitted conditional times shape"))?
                };

                CatCIMS::new(
                    fitted_conditional_counts,
                    fitted_conditional_times,
                    fitted_size,
                )
                .map_err(|evidence| E::custom(evidence.to_string()))
            }
        }

        const FIELDS: &[&str] = &[
            "fitted_conditional_counts",
            "fitted_conditional_times",
            "fitted_size",
        ];

        deserializer.deserialize_struct("CatCIMS", FIELDS, CatCIMSVisitor)
    }
}

/// A struct representing a categorical conditional intensity matrix.
#[derive(Clone, Debug)]
pub struct CatCIM {
    // Labels of the conditioned variable.
    labels: Labels,
    support: CatSupport,
    shape: Array1<usize>,
    multi_index: MI,
    // Labels of the conditioning variables.
    conditioning_labels: Labels,
    conditioning_support: CatSupport,
    conditioning_shape: Array1<usize>,
    conditioning_multi_index: MI,
    // Parameters.
    parameters: Array3<f64>,
    parameters_size: usize,
    // Fitted sufficient statistics, if any.
    fitted_statistics: Option<CatCIMS>,
    fitted_log_likelihood: Option<f64>,
}

impl CatCIM {
    /// Creates a new categorical conditional intensity matrix.
    ///
    /// # Arguments
    ///
    /// * `support` - The variables support.
    /// * `conditioning_support` - The conditioning variables labels and support.
    /// * `parameters` - The intensity matrices of the support.
    ///
    /// # Errors
    ///
    /// * If the labels and conditioning labels are not disjoint.
    /// * If the product of the shape of the support does not match the length of the second and third axis.
    /// * If the product of the shape of the conditioning support does not match the length of the first axis.
    /// * If the parameters are not valid intensity matrices, unless empty.
    ///
    /// # Returns
    ///
    /// A new `CatCIM` instance.
    ///
    pub fn new(
        support: CatSupport,
        conditioning_support: CatSupport,
        parameters: Array3<f64>,
    ) -> Result<Self> {
        // Get the labels of the variables.
        let labels: Set<_> = support.keys().cloned().collect();
        // Get the labels of the variables.
        let conditioning_labels: Set<_> = conditioning_support.keys().cloned().collect();

        // Check labels and conditioning labels are disjoint.
        if !labels.is_disjoint(&conditioning_labels) {
            return Err(Error::SetsNotDisjoint(
                &format!("{:?}", labels),
                &format!("{:?}", conditioning_labels),
            ));
        }

        // Get the support shape.
        let shape = Array::from_iter(support.values().map(Set::len));

        // Check that the product of the shape matches the number of columns.
        if !parameters.is_empty() && parameters.shape()[1] != shape.product() {
            return Err(Error::IncompatibleShape(
                "parameters",
                &format!(
                    "Product of the number of support must match the number of columns: expected {} but found {}.",
                    shape.product(),
                    parameters.shape()[1],
                ),
            ));
        }

        // Check that the product of the shape matches the number of columns.
        if !parameters.is_empty() && parameters.shape()[2] != shape.product() {
            return Err(Error::IncompatibleShape(
                "parameters",
                &format!(
                    "Product of the number of support must match the third axis: expected {} but found {}.",
                    shape.product(),
                    parameters.shape()[2],
                ),
            ));
        }

        // Get the shape of the set of support.
        let conditioning_shape = Array::from_iter(conditioning_support.values().map(Set::len));

        // Check that the product of the conditioning shape matches the number of rows.
        if !parameters.is_empty() && parameters.shape()[0] != conditioning_shape.product() {
            return Err(Error::IncompatibleShape(
                "parameters",
                &format!(
                    "Product of the number of conditioning support must match the number of rows: expected {} but found {}.",
                    conditioning_shape.product(),
                    parameters.shape()[0],
                ),
            ));
        }

        // Check parameters validity.
        parameters.outer_iter().try_for_each(|q| {
            // Check Q is square.
            if !q.is_square() {
                return Err(Error::Shape("Q must be square."));
            }
            // Check Q has finite values.
            if !q.iter().all(|&x| x.is_finite()) {
                return Err(Error::InvalidParameter(
                    "parameters",
                    "Q must have finite values.",
                ));
            }
            // Check Q has non-positive diagonal.
            if !q.diag().iter().all(|&x| x <= 0.) {
                return Err(Error::InvalidParameter(
                    "parameters",
                    "Q diagonal must be non-positive.",
                ));
            }
            // Check Q has non-negative off-diagonal.
            if !q.indexed_iter().all(|((i, j), &x)| i == j || x >= 0.) {
                return Err(Error::InvalidParameter(
                    "parameters",
                    "Q off-diagonal must be non-negative.",
                ));
            }
            // Check Q rows sum to zero.
            if !q
                .rows()
                .into_iter()
                .all(|x| relative_eq!(x.sum(), 0., epsilon = EPSILON))
            {
                return Err(Error::InvalidParameter(
                    "parameters",
                    "Q rows must sum to zero.",
                ));
            }
            Ok(())
        })?;

        // Make parameters mutable.
        let mut parameters = parameters;

        // Make support mutable.
        let mut labels = labels;
        let mut support = support;
        let mut shape = shape;

        // Check if support are sorted.
        if !support.keys().is_sorted() || !support.values().all(|x| x.iter().is_sorted()) {
            // Compute the current support order.
            let mut sorted_states_idx: Vec<_> =
                support.values().multi_cartesian_product().collect();
            // Sort the labels.
            let mut sorted_labels_idx: Vec<_> = (0..labels.len()).collect();
            // Sort the labels.
            sorted_labels_idx.sort_by_key(|&i| &labels[i]);
            // Sort the support by the labels.
            sorted_states_idx.iter_mut().for_each(|sorted_states_idx| {
                *sorted_states_idx = sorted_labels_idx
                    .iter()
                    .map(|&i| sorted_states_idx[i])
                    .collect();
            });
            // Initialize the sorted row indices.
            let mut sorted_row_idx: Vec<_> = (0..parameters.shape()[1]).collect();
            // Sort the row indices.
            sorted_row_idx.sort_by_key(|&i| &sorted_states_idx[i]);
            // Sort the labels.
            support.sort_keys();
            support.values_mut().for_each(Set::sort);
            labels = support.keys().cloned().collect();
            shape = support.values().map(Set::len).collect();
            // Allocate new parameters, for axis 1.
            let mut new_parameters = parameters.clone();
            // Sort the values by multi indices.
            new_parameters.axis_iter_mut(Axis(1)).enumerate().for_each(
                |(i, mut new_parameters_axis)| {
                    // Assign the sorted values to the new values array.
                    new_parameters_axis.assign(&parameters.index_axis(Axis(1), sorted_row_idx[i]));
                },
            );
            // Update the values with the new sorted values.
            parameters = new_parameters;
            // Allocate new parameters, for axis 2.
            let mut new_parameters = parameters.clone();
            // Sort the values by multi indices.
            new_parameters.axis_iter_mut(Axis(2)).enumerate().for_each(
                |(i, mut new_parameters_axis)| {
                    // Assign the sorted values to the new values array.
                    new_parameters_axis.assign(&parameters.index_axis(Axis(2), sorted_row_idx[i]));
                },
            );
            // Update the values with the new sorted values.
            parameters = new_parameters;
        }

        // Make support immutable.
        let labels = labels;
        let support = support;
        let shape = shape;

        // Make conditioning support mutable.
        let mut conditioning_labels = conditioning_labels;
        let mut conditioning_support = conditioning_support;
        let mut conditioning_shape = conditioning_shape;

        // Check if conditioning support are sorted.
        if !conditioning_support.keys().is_sorted()
            || !conditioning_support.values().all(|x| x.iter().is_sorted())
        {
            // Compute the current support order.
            let mut sorted_states_idx: Vec<_> = conditioning_support
                .values()
                .multi_cartesian_product()
                .collect();
            // Sort the conditioning labels.
            let mut sorted_labels_idx: Vec<_> = (0..conditioning_labels.len()).collect();
            // Sort the conditioning labels.
            sorted_labels_idx.sort_by_key(|&i| &conditioning_labels[i]);
            // Sort the conditioning support by the labels.
            sorted_states_idx.iter_mut().for_each(|sorted_states_idx| {
                *sorted_states_idx = sorted_labels_idx
                    .iter()
                    .map(|&i| sorted_states_idx[i])
                    .collect();
            });
            // Initialize the sorted row indices.
            let mut sorted_row_idx: Vec<_> = (0..parameters.shape()[0]).collect();
            // Sort the row indices.
            sorted_row_idx.sort_by_key(|&i| &sorted_states_idx[i]);
            // Sort the labels.
            conditioning_support.sort_keys();
            conditioning_support.values_mut().for_each(Set::sort);
            conditioning_labels = conditioning_support.keys().cloned().collect();
            conditioning_shape = conditioning_support.values().map(Set::len).collect();
            // Allocate new parameters.
            let mut new_parameters = parameters.clone();
            // Sort the values by multi indices.
            new_parameters.axis_iter_mut(Axis(0)).enumerate().for_each(
                |(i, mut new_parameters_axis)| {
                    // Assign the sorted values to the new values array.
                    new_parameters_axis.assign(&parameters.index_axis(Axis(0), sorted_row_idx[i]));
                },
            );
            // Update the values with the new sorted values.
            parameters = new_parameters;
        }

        // Make conditioning support immutable.
        let conditioning_labels = conditioning_labels;
        let conditioning_support = conditioning_support;
        let conditioning_shape = conditioning_shape;

        // Make parameters immutable.
        let parameters = parameters;

        // Compute the multi index.
        let multi_index = MI::new(shape.clone());
        // Compute the conditioning multi index.
        let conditioning_multi_index = MI::new(conditioning_shape.clone());

        // Get the shape of the parameters.
        let stats = parameters.shape();
        // Compute the parameters size.
        let parameters_size = stats[0] * stats[1] * stats[2].saturating_sub(1);

        Ok(Self {
            labels,
            support,
            shape,
            multi_index,
            conditioning_labels,
            conditioning_support,
            conditioning_shape,
            conditioning_multi_index,
            parameters,
            parameters_size,
            fitted_statistics: None,
            fitted_log_likelihood: None,
        })
    }

    /// Returns the support of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The support of the conditioned variable.
    ///
    #[inline]
    pub const fn support(&self) -> &CatSupport {
        &self.support
    }

    /// Returns the shape of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The shape of the conditioned variable.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        &self.shape
    }

    /// Returns the ravel multi index of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The ravel multi index of the conditioning variables.
    ///
    #[inline]
    pub const fn multi_index(&self) -> &MI {
        &self.multi_index
    }

    /// Returns the support of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The support of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_support(&self) -> &CatSupport {
        &self.conditioning_support
    }

    /// Returns the shape of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The shape of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_shape(&self) -> &Array1<usize> {
        &self.conditioning_shape
    }

    /// Returns the ravel multi index of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The ravel multi index of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_multi_index(&self) -> &MI {
        &self.conditioning_multi_index
    }

    /// Creates a new categorical conditional intensity matrix.
    ///
    /// # Arguments
    ///
    /// * `support` - The variables support.
    /// * `conditioning_support` - The conditioning variables labels and support.
    /// * `parameters` - The intensity matrices of the support.
    /// * `fitted_statistics` - The fitted statistics used to fit the distribution, if any.
    /// * `fitted_log_likelihood` - The log-likelihood given the distribution, if any.
    ///
    /// # Errors
    ///
    /// See `new` method for errors.
    ///
    /// # Returns
    ///
    /// A new `CatCIM` instance.
    ///
    pub fn with_optionals(
        support: CatSupport,
        conditioning_support: CatSupport,
        parameters: Array3<f64>,
        fitted_statistics: Option<CatCIMS>,
        fitted_log_likelihood: Option<f64>,
    ) -> Result<Self> {
        // Check the fitted conditional counts are finite and non-negative, with same shape as parameters.
        if let Some(fitted_statistics) = &fitted_statistics {
            // Get the fitted conditional counts.
            let fitted_conditional_counts = &fitted_statistics.n_xz;
            // Check the fitted conditional counts have the same shape as parameters.
            if fitted_conditional_counts.shape() != parameters.shape() {
                return Err(Error::IncompatibleShape(
                    "fitted_statistics",
                    &format!(
                        "Fitted conditional counts must have the same shape as parameters: expected {:?} but found {:?}.",
                        parameters.shape(),
                        fitted_conditional_counts.shape(),
                    ),
                ));
            }
        }
        // Check the fitted log-likelihood is finite.
        if let Some(fitted_log_likelihood) = &fitted_log_likelihood
            && !fitted_log_likelihood.is_finite()
        {
            return Err(Error::InvalidParameter(
                "fitted_log_likelihood",
                &format!(
                    "Fitted log-likelihood must be finite, found: {}.",
                    fitted_log_likelihood
                ),
            ));
        }

        // Construct the CIM.
        let mut intensity = Self::new(support, conditioning_support, parameters)?;

        // Set the fitted statistics and log-likelihood.
        intensity.fitted_statistics = fitted_statistics;
        intensity.fitted_log_likelihood = fitted_log_likelihood;

        Ok(intensity)
    }
}

impl PartialEq for CatCIM {
    fn eq(&self, other: &Self) -> bool {
        // Check for equality, excluding the sample values.
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self.shape.eq(&other.shape)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_support.eq(&other.conditioning_support)
            && self.conditioning_shape.eq(&other.conditioning_shape)
            && self.multi_index.eq(&other.multi_index)
            && self.parameters.eq(&other.parameters)
    }
}

impl AbsDiffEq for CatCIM {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        // Check for equality, excluding the sample values.
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self.shape.eq(&other.shape)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_support.eq(&other.conditioning_support)
            && self.conditioning_shape.eq(&other.conditioning_shape)
            && self.multi_index.eq(&other.multi_index)
            && self.parameters.abs_diff_eq(&other.parameters, epsilon)
    }
}

impl RelativeEq for CatCIM {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        // Check for equality, excluding the sample values.
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self.shape.eq(&other.shape)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_support.eq(&other.conditioning_support)
            && self.conditioning_shape.eq(&other.conditioning_shape)
            && self.multi_index.eq(&other.multi_index)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl HasLabels for CatCIM {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CIM for CatCIM {
    type Sample = CatSample;
    type Support = CatSupport;
    type Parameters = Array3<f64>;
    type Statistics = CatCIMS;

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.support)
    }

    #[inline]
    fn conditioning_support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.conditioning_support)
    }

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
        self.parameters_size
    }

    #[inline]
    fn fitted_statistics(&self) -> Option<Cow<'_, Self::Statistics>> {
        self.fitted_statistics.as_ref().map(Cow::Borrowed)
    }

    #[inline]
    fn fitted_log_likelihood(&self) -> Option<f64> {
        self.fitted_log_likelihood
    }
}

impl Serialize for CatCIM {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the elements to serialize.
        let mut size = 4;
        size += self.fitted_statistics.is_some() as usize;
        size += self.fitted_log_likelihood.is_some() as usize;

        // Allocate the map.
        let mut map = serializer.serialize_map(Some(size))?;

        // Serialize support.
        map.serialize_entry("support", &self.support)?;
        // Serialize conditioning support.
        map.serialize_entry("conditioning_support", &self.conditioning_support)?;

        // Convert parameters to a flat format.
        let parameters: Vec<Vec<Vec<f64>>> = self
            .parameters
            .outer_iter()
            .map(|parameters| parameters.rows().into_iter().map(|x| x.to_vec()).collect())
            .collect();

        // Serialize parameters.
        map.serialize_entry("parameters", &parameters)?;

        // Serialize fitted statistics, if any.
        if let Some(fitted_statistics) = &self.fitted_statistics {
            map.serialize_entry("fitted_statistics", &fitted_statistics)?;
        }
        // Serialize fitted log likelihood, if any.
        if let Some(fitted_log_likelihood) = self.fitted_log_likelihood {
            map.serialize_entry("fitted_log_likelihood", &fitted_log_likelihood)?;
        }

        // Serialize type.
        map.serialize_entry("type", "catcim")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatCIM {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Support,
            ConditioningSupport,
            Parameters,
            FittedStatistics,
            FittedLogLikelihood,
            Type,
        }

        struct CatCIMVisitor;

        impl<'de> Visitor<'de> for CatCIMVisitor {
            type Value = CatCIM;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatCIM")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CatCIM, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate fields
                let mut support = None;
                let mut conditioning_support = None;
                let mut parameters = None;
                let mut fitted_statistics = None;
                let mut fitted_log_likelihood = None;
                let mut type_ = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Support => {
                            if support.is_some() {
                                return Err(E::duplicate_field("support"));
                            }
                            support = Some(map.next_value()?);
                        }
                        Field::ConditioningSupport => {
                            if conditioning_support.is_some() {
                                return Err(E::duplicate_field("conditioning_support"));
                            }
                            conditioning_support = Some(map.next_value()?);
                        }
                        Field::Parameters => {
                            if parameters.is_some() {
                                return Err(E::duplicate_field("parameters"));
                            }
                            parameters = Some(map.next_value()?);
                        }
                        Field::FittedStatistics => {
                            if fitted_statistics.is_some() {
                                return Err(E::duplicate_field("fitted_statistics"));
                            }
                            fitted_statistics = Some(map.next_value()?);
                        }
                        Field::FittedLogLikelihood => {
                            if fitted_log_likelihood.is_some() {
                                return Err(E::duplicate_field("fitted_log_likelihood"));
                            }
                            fitted_log_likelihood = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(E::duplicate_field("type"));
                            }
                            type_ = Some(map.next_value()?);
                        }
                    }
                }

                // Check required fields.
                let support = support.ok_or_else(|| E::missing_field("support"))?;
                let conditioning_support =
                    conditioning_support.ok_or_else(|| E::missing_field("conditioning_support"))?;
                let parameters = parameters.ok_or_else(|| E::missing_field("parameters"))?;

                // Check type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                if type_ != "catcim" {
                    return Err(E::custom(format!(
                        "Invalid type for CatCIM: expected 'catcim', found '{type_}'"
                    )));
                }

                // Convert parameters to ndarray.
                let parameters: Vec<Vec<Vec<f64>>> = parameters;
                let shape = (
                    parameters.len(),
                    parameters[0].len(),
                    parameters[0][0].len(),
                );
                let parameters = parameters.into_iter().flatten().flatten();
                let parameters = Array::from_iter(parameters)
                    .into_shape_with_order(shape)
                    .map_err(|_| E::custom("Invalid parameters shape"))?;

                CatCIM::with_optionals(
                    support,
                    conditioning_support,
                    parameters,
                    fitted_statistics,
                    fitted_log_likelihood,
                )
                .map_err(|evidence| E::custom(evidence.to_string()))
            }
        }

        const FIELDS: &[&str] = &[
            "support",
            "conditioning_support",
            "parameters",
            "fitted_statistics",
            "fitted_log_likelihood",
            "type",
        ];

        deserializer.deserialize_struct("CatCIM", FIELDS, CatCIMVisitor)
    }
}

// Implement `JsonIO` for `CatCIM`.
impl_json_io!(CatCIM);
