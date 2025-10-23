use std::ops::{Add, AddAssign};

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
    models::{CIM, Labelled},
    types::{EPSILON, Labels, Set, States},
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
    pub fn new(n_xz: Array3<f64>, t_xz: Array2<f64>, n: f64) -> Self {
        // Assert the dimensions are correct.
        assert_eq!(
            n_xz.shape()[1],
            n_xz.shape()[2],
            "The second and third dimensions of the conditional counts must be equal."
        );
        assert_eq!(
            n_xz.shape()[0],
            t_xz.shape()[0],
            "The first dimension of the conditional counts must match \n
            the first dimension of the conditional times."
        );
        assert_eq!(
            n_xz.shape()[1],
            t_xz.shape()[1],
            "The second dimension of the conditional counts must match \n
            the second dimension of the conditional times."
        );
        assert!(
            n_xz.iter().all(|&x| x.is_finite() && x >= 0.),
            "Conditional counts must be finite and non-negative."
        );
        assert!(
            t_xz.iter().all(|&x| x.is_finite() && x >= 0.),
            "Conditional times must be finite and non-negative."
        );
        assert!(
            n.is_finite() && n >= 0.,
            "Sample size must be finite and non-negative."
        );

        Self { n_xz, t_xz, n }
    }

    /// Returns the sample conditional counts |Z| x |X| x |X|.
    ///
    /// # Returns
    ///
    /// The sample conditional counts |Z| x |X| x |X|.
    ///
    #[inline]
    pub const fn sample_conditional_counts(&self) -> &Array3<f64> {
        &self.n_xz
    }

    /// Returns the sample conditional times |Z| x |X|.
    ///
    /// # Returns
    ///
    /// The sample conditional times |Z| x |X|.
    ///
    #[inline]
    pub const fn sample_conditional_times(&self) -> &Array2<f64> {
        &self.t_xz
    }

    /// Returns the sample size.
    ///
    /// # Returns
    ///
    /// The sample size.
    ///
    #[inline]
    pub const fn sample_size(&self) -> f64 {
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(3))?;

        // Convert the sample conditional counts to a flat format.
        let sample_conditional_counts: Vec<Vec<Vec<f64>>> = self
            .n_xz
            .outer_iter()
            .map(|sample_conditional_counts| {
                sample_conditional_counts
                    .rows()
                    .into_iter()
                    .map(|x| x.to_vec())
                    .collect()
            })
            .collect();

        // Serialize sample conditional counts.
        map.serialize_entry("sample_conditional_counts", &sample_conditional_counts)?;

        // Convert the sample conditional times to a flat format.
        let sample_conditional_times: Vec<Vec<f64>> =
            self.t_xz.rows().into_iter().map(|x| x.to_vec()).collect();

        // Serialize sample conditional times.
        map.serialize_entry("sample_conditional_times", &sample_conditional_times)?;

        // Serialize sample size.
        map.serialize_entry("sample_size", &self.n)?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatCIMS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        #[allow(clippy::enum_variant_names)]
        enum Field {
            SampleConditionalCounts,
            SampleConditionalTimes,
            SampleSize,
        }

        struct CatCIMSVisitor;

        impl<'de> Visitor<'de> for CatCIMSVisitor {
            type Value = CatCIMS;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatCIMS")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CatCIMS, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate fields
                let mut sample_conditional_counts = None;
                let mut sample_conditional_times = None;
                let mut sample_size = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::SampleConditionalCounts => {
                            if sample_conditional_counts.is_some() {
                                return Err(E::duplicate_field("sample_conditional_counts"));
                            }
                            sample_conditional_counts = Some(map.next_value()?);
                        }
                        Field::SampleConditionalTimes => {
                            if sample_conditional_times.is_some() {
                                return Err(E::duplicate_field("sample_conditional_times"));
                            }
                            sample_conditional_times = Some(map.next_value()?);
                        }
                        Field::SampleSize => {
                            if sample_size.is_some() {
                                return Err(E::duplicate_field("sample_size"));
                            }
                            sample_size = Some(map.next_value()?);
                        }
                    }
                }

                // Check all fields are present.
                let sample_conditional_counts = sample_conditional_counts
                    .ok_or_else(|| E::missing_field("sample_conditional_counts"))?;
                let sample_conditional_times = sample_conditional_times
                    .ok_or_else(|| E::missing_field("sample_conditional_times"))?;
                let sample_size = sample_size.ok_or_else(|| E::missing_field("sample_size"))?;

                // Convert sample conditional counts to ndarray.
                let sample_conditional_counts = {
                    let counts: Vec<Vec<Vec<f64>>> = sample_conditional_counts;
                    let shape = (counts.len(), counts[0].len(), counts[0][0].len());
                    let counts = counts.into_iter().flatten().flatten();
                    Array::from_iter(counts)
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample conditional counts shape"))?
                };

                // Convert sample conditional times to ndarray.
                let sample_conditional_times = {
                    let times: Vec<Vec<f64>> = sample_conditional_times;
                    let shape = (times.len(), times[0].len());
                    let times = times.into_iter().flatten();
                    Array::from_iter(times)
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample conditional times shape"))?
                };

                Ok(CatCIMS::new(
                    sample_conditional_counts,
                    sample_conditional_times,
                    sample_size,
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "sample_conditional_counts",
            "sample_conditional_times",
            "sample_size",
        ];

        deserializer.deserialize_struct("CatCIMS", FIELDS, CatCIMSVisitor)
    }
}

/// A struct representing a categorical conditional intensity matrix.
#[derive(Clone, Debug)]
pub struct CatCIM {
    // Labels of the conditioned variable.
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    multi_index: MI,
    // Labels of the conditioning variables.
    conditioning_labels: Labels,
    conditioning_states: States,
    conditioning_shape: Array1<usize>,
    conditioning_multi_index: MI,
    // Parameters.
    parameters: Array3<f64>,
    parameters_size: usize,
    // Sample (sufficient) statistics, if any.
    sample_statistics: Option<CatCIMS>,
    sample_log_likelihood: Option<f64>,
}

impl CatCIM {
    /// Creates a new categorical conditional intensity matrix.
    ///
    /// # Arguments
    ///
    /// * `states` - The variables states.
    /// * `parameters` - The intensity matrices of the states.
    ///
    /// # Panics
    ///
    /// * If the labels and conditioning labels are not disjoint.
    /// * If the product of the shape of the states does not match the length of the second and third axis.
    /// * If the product of the shape of the conditioning states does not match the length of the first axis.
    /// * If the parameters are not valid intensity matrices, unless empty.
    ///
    /// # Returns
    ///
    /// A new `CatCIM` instance.
    ///
    pub fn new(states: States, conditioning_states: States, parameters: Array3<f64>) -> Self {
        // Get the labels of the variables.
        let labels: Set<_> = states.keys().cloned().collect();
        // Get the labels of the variables.
        let conditioning_labels: Set<_> = conditioning_states.keys().cloned().collect();

        // Assert labels and conditioning labels are disjoint.
        assert!(
            labels.is_disjoint(&conditioning_labels),
            "Labels and conditioning labels must be disjoint."
        );

        // Get the states shape.
        let shape = Array::from_iter(states.values().map(Set::len));

        // Check that the product of the shape matches the number of columns.
        assert!(
            parameters.is_empty() || parameters.shape()[1] == shape.product(),
            "Product of the number of states must match the number of columns: \n\
            \t expected:    parameters.shape[1] == {} , \n\
            \t found:       parameters.shape[1] == {} .",
            shape.product(),
            parameters.shape()[1],
        );

        // Check that the product of the shape matches the number of columns.
        assert!(
            parameters.is_empty() || parameters.shape()[2] == shape.product(),
            "Product of the number of states must match the number of columns: \n\
            \t expected:    parameters.shape[2] == {} , \n\
            \t found:       parameters.shape[2] == {} .",
            shape.product(),
            parameters.shape()[2],
        );

        // Get the shape of the set of states.
        let conditioning_shape = Array::from_iter(conditioning_states.values().map(Set::len));

        // Check that the product of the conditioning shape matches the number of rows.
        assert!(
            parameters.is_empty() || parameters.shape()[0] == conditioning_shape.product(),
            "Product of the number of conditioning states must match the number of rows: \n\
            \t expected:    parameters.shape[0] == {} , \n\
            \t found:       parameters.shape[0] == {} .",
            conditioning_shape.product(),
            parameters.shape()[0],
        );

        // Check parameters validity.
        parameters.outer_iter().for_each(|q| {
            // Assert Q is square.
            assert!(q.is_square(), "Q must be square.");
            // Assert Q has finite values.
            assert!(
                q.iter().all(|&x| x.is_finite()),
                "Q must have finite values."
            );
            // Assert Q has non-positive diagonal.
            assert!(
                q.diag().iter().all(|&x| x <= 0.),
                "Q diagonal must be non-positive."
            );
            // Assert Q has non-negative off-diagonal.
            assert!(
                q.indexed_iter().all(|((i, j), &x)| i == j || x >= 0.),
                "Q off-diagonal must be non-negative."
            );
            // Assert Q rows sum to zero.
            assert!(
                q.rows()
                    .into_iter()
                    .all(|x| relative_eq!(x.sum(), 0., epsilon = EPSILON)),
                "Q rows must sum to zero."
            );
        });

        // Make parameters mutable.
        let mut parameters = parameters;

        // Make states mutable.
        let mut labels = labels;
        let mut states = states;
        let mut shape = shape;

        // Check if states are sorted.
        if !states.keys().is_sorted() || !states.values().all(|x| x.iter().is_sorted()) {
            // Compute the current states order.
            let mut sorted_states_idx: Vec<_> = states.values().multi_cartesian_product().collect();
            // Sort the labels.
            let mut sorted_labels_idx: Vec<_> = (0..labels.len()).collect();
            // Sort the labels.
            sorted_labels_idx.sort_by_key(|&i| &labels[i]);
            // Sort the states by the labels.
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
            states.sort_keys();
            states.values_mut().for_each(Set::sort);
            labels = states.keys().cloned().collect();
            shape = states.values().map(Set::len).collect();
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

        // Make states immutable.
        let labels = labels;
        let states = states;
        let shape = shape;

        // Make conditioning states mutable.
        let mut conditioning_labels = conditioning_labels;
        let mut conditioning_states = conditioning_states;
        let mut conditioning_shape = conditioning_shape;

        // Check if conditioning states are sorted.
        if !conditioning_states.keys().is_sorted()
            || !conditioning_states.values().all(|x| x.iter().is_sorted())
        {
            // Compute the current states order.
            let mut sorted_states_idx: Vec<_> = conditioning_states
                .values()
                .multi_cartesian_product()
                .collect();
            // Sort the conditioning labels.
            let mut sorted_labels_idx: Vec<_> = (0..conditioning_labels.len()).collect();
            // Sort the conditioning labels.
            sorted_labels_idx.sort_by_key(|&i| &conditioning_labels[i]);
            // Sort the conditioning states by the labels.
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
            conditioning_states.sort_keys();
            conditioning_states.values_mut().for_each(Set::sort);
            conditioning_labels = conditioning_states.keys().cloned().collect();
            conditioning_shape = conditioning_states.values().map(Set::len).collect();
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

        // Make conditioning states immutable.
        let conditioning_labels = conditioning_labels;
        let conditioning_states = conditioning_states;
        let conditioning_shape = conditioning_shape;

        // Make parameters immutable.
        let parameters = parameters;

        // Debug assert to check the sorting of the labels.
        debug_assert!(labels.iter().is_sorted(), "Labels must be sorted.");
        debug_assert!(states.keys().is_sorted(), "Labels must be sorted.");
        debug_assert!(
            states.values().all(|x| x.iter().is_sorted()),
            "States must be sorted."
        );
        debug_assert!(
            conditioning_labels.iter().is_sorted(),
            "Conditioning labels must be sorted."
        );
        debug_assert!(
            conditioning_states.keys().is_sorted(),
            "Conditioning labels must be sorted."
        );
        debug_assert!(
            conditioning_states.values().all(|x| x.iter().is_sorted()),
            "Conditioning states must be sorted."
        );

        // Compute the multi index.
        let multi_index = MI::new(shape.clone());
        // Compute the conditioning multi index.
        let conditioning_multi_index = MI::new(conditioning_shape.clone());

        // Get the shape of the parameters.
        let s = parameters.shape();
        // Compute the parameters size.
        let parameters_size = s[0] * s[1] * s[2].saturating_sub(1);

        Self {
            labels,
            states,
            shape,
            multi_index,
            conditioning_labels,
            conditioning_states,
            conditioning_shape,
            conditioning_multi_index,
            parameters,
            parameters_size,
            sample_statistics: None,
            sample_log_likelihood: None,
        }
    }

    /// Returns the states of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The states of the conditioned variable.
    ///
    #[inline]
    pub const fn states(&self) -> &States {
        &self.states
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

    /// Returns the states of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The states of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_states(&self) -> &States {
        &self.conditioning_states
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
    /// * `states` - The variables states.
    /// * `parameters` - The intensity matrices of the states.
    /// * `sample_statistics` - The sample statistics used to fit the distribution, if any.
    /// * `sample_log_likelihood` - The sample log-likelihood given the distribution, if any.
    ///
    /// # Panics
    ///
    /// See `new` method for panics.
    ///
    /// # Returns
    ///
    /// A new `CatCIM` instance.
    ///
    pub fn with_optionals(
        states: States,
        conditioning_states: States,
        parameters: Array3<f64>,
        sample_statistics: Option<CatCIMS>,
        sample_log_likelihood: Option<f64>,
    ) -> Self {
        // Assert the sample conditional counts are finite and non-negative, with same shape as parameters.
        if let Some(sample_statistics) = &sample_statistics {
            // Get the sample conditional counts.
            let sample_conditional_counts = &sample_statistics.n_xz;
            // Assert the sample conditional counts have the same shape as parameters.
            assert!(
                sample_conditional_counts.shape() == parameters.shape(),
                "Sample conditional counts must have the same shape as parameters: \n\
                \t expected:    sample_conditional_counts.shape() == {:?} , \n\
                \t found:       sample_conditional_counts.shape() == {:?} .",
                parameters.shape(),
                sample_conditional_counts.shape(),
            );
        }
        // Assert the sample log-likelihood is finite.
        if let Some(sample_log_likelihood) = &sample_log_likelihood {
            assert!(
                sample_log_likelihood.is_finite(),
                "Sample log-likelihood must be finite: \n\
                \t expected: sample_ll is finite, \n\
                \t found:    sample_ll is {sample_log_likelihood} ."
            )
        }

        // Construct the CIM.
        let mut cim = Self::new(states, conditioning_states, parameters);

        // FIXME: Check labels alignment with optional fields.

        // Set the sample statistics and log-likelihood.
        cim.sample_statistics = sample_statistics;
        cim.sample_log_likelihood = sample_log_likelihood;

        cim
    }
}

impl PartialEq for CatCIM {
    fn eq(&self, other: &Self) -> bool {
        // Check for equality, excluding the sample values.
        self.labels.eq(&other.labels)
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
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
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
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
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self.conditioning_shape.eq(&other.conditioning_shape)
            && self.multi_index.eq(&other.multi_index)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl Labelled for CatCIM {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CIM for CatCIM {
    type Support = CatSample;
    type Parameters = Array3<f64>;
    type Statistics = CatCIMS;

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
    fn sample_statistics(&self) -> Option<&Self::Statistics> {
        self.sample_statistics.as_ref()
    }

    #[inline]
    fn sample_log_likelihood(&self) -> Option<f64> {
        self.sample_log_likelihood
    }
}

impl Serialize for CatCIM {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the elements to serialize.
        let mut size = 4;
        size += self.sample_statistics.is_some() as usize;
        size += self.sample_log_likelihood.is_some() as usize;

        // Allocate the map.
        let mut map = serializer.serialize_map(Some(size))?;

        // Serialize states.
        map.serialize_entry("states", &self.states)?;
        // Serialize conditioning states.
        map.serialize_entry("conditioning_states", &self.conditioning_states)?;

        // Convert parameters to a flat format.
        let parameters: Vec<Vec<Vec<f64>>> = self
            .parameters
            .outer_iter()
            .map(|parameters| parameters.rows().into_iter().map(|x| x.to_vec()).collect())
            .collect();

        // Serialize parameters.
        map.serialize_entry("parameters", &parameters)?;

        // Serialize sample statistics, if any.
        if let Some(sample_statistics) = &self.sample_statistics {
            map.serialize_entry("sample_statistics", &sample_statistics)?;
        }
        // Serialize sample log likelihood, if any.
        if let Some(sample_log_likelihood) = self.sample_log_likelihood {
            map.serialize_entry("sample_log_likelihood", &sample_log_likelihood)?;
        }

        // Serialize type.
        map.serialize_entry("type", "catcim")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatCIM {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            States,
            ConditioningStates,
            Parameters,
            SampleStatistics,
            SampleLogLikelihood,
            Type,
        }

        struct CatCIMVisitor;

        impl<'de> Visitor<'de> for CatCIMVisitor {
            type Value = CatCIM;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatCIM")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CatCIM, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate fields
                let mut states = None;
                let mut conditioning_states = None;
                let mut parameters = None;
                let mut sample_statistics = None;
                let mut sample_log_likelihood = None;
                let mut type_ = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::States => {
                            if states.is_some() {
                                return Err(E::duplicate_field("states"));
                            }
                            states = Some(map.next_value()?);
                        }
                        Field::ConditioningStates => {
                            if conditioning_states.is_some() {
                                return Err(E::duplicate_field("conditioning_states"));
                            }
                            conditioning_states = Some(map.next_value()?);
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

                // Check required fields.
                let states = states.ok_or_else(|| E::missing_field("states"))?;
                let conditioning_states =
                    conditioning_states.ok_or_else(|| E::missing_field("conditioning_states"))?;
                let parameters = parameters.ok_or_else(|| E::missing_field("parameters"))?;

                // Assert type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                assert_eq!(type_, "catcim", "Invalid type for CatCIM.");

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

                Ok(CatCIM::with_optionals(
                    states,
                    conditioning_states,
                    parameters,
                    sample_statistics,
                    sample_log_likelihood,
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "states",
            "conditioning_states",
            "parameters",
            "sample_statistics",
            "sample_log_likelihood",
            "type",
        ];

        deserializer.deserialize_struct("CatCIM", FIELDS, CatCIMVisitor)
    }
}

// Implement `JsonIO` for `CatCIM`.
impl_json_io!(CatCIM);
