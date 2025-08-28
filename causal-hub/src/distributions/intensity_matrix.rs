use approx::{AbsDiffEq, RelativeEq, relative_eq};
use itertools::Itertools;
use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use super::CPD;
use crate::{
    impl_json_io,
    types::{EPSILON, Labels, Set, States},
    utils::MI,
};

/// A struct representing a categorical conditional intensity matrix.
#[derive(Clone, Debug)]
pub struct CatCIM {
    // Labels of the conditioned variable.
    labels: Labels,
    states: States,
    cardinality: Array1<usize>,
    multi_index: MI,
    // Labels of the conditioning variables.
    conditioning_labels: Labels,
    conditioning_states: States,
    conditioning_cardinality: Array1<usize>,
    conditioning_multi_index: MI,
    // Parameters.
    parameters: Array3<f64>,
    parameters_size: usize,
    // Fitted statistics, if any.
    sample_size: Option<f64>,
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
    /// * If the product of the cardinalities of the states does not match the length of the second and third axis.
    /// * If the product of the cardinalities of the conditioning states does not match the length of the first axis.
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

        // Get the states cardinality.
        let cardinality: Array1<_> = states.values().map(|x| x.len()).collect();

        // Check that the product of the cardinality matches the number of columns.
        assert!(
            parameters.is_empty() || parameters.shape()[1] == cardinality.product(),
            "Product of the number of states must match the number of columns: \n\
            \t expected:    parameters.shape[1] == {} , \n\
            \t found:       parameters.shape[1] == {} .",
            cardinality.product(),
            parameters.shape()[1],
        );

        // Check that the product of the cardinality matches the number of columns.
        assert!(
            parameters.is_empty() || parameters.shape()[2] == cardinality.product(),
            "Product of the number of states must match the number of columns: \n\
            \t expected:    parameters.shape[2] == {} , \n\
            \t found:       parameters.shape[2] == {} .",
            cardinality.product(),
            parameters.shape()[2],
        );

        // Get the cardinality of the set of states.
        let conditioning_cardinality: Array1<_> =
            conditioning_states.values().map(|x| x.len()).collect();

        // Check that the product of the conditioning cardinality matches the number of rows.
        assert!(
            parameters.is_empty() || parameters.shape()[0] == conditioning_cardinality.product(),
            "Product of the number of conditioning states must match the number of rows: \n\
            \t expected:    parameters.shape[0] == {} , \n\
            \t found:       parameters.shape[0] == {} .",
            conditioning_cardinality.product(),
            parameters.shape()[0],
        );

        // Check parameters validity.
        parameters.outer_iter().for_each(|q| {
            // Assert Q is square.
            assert_eq!(q.nrows(), q.ncols(), "Q must be square.");
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
        let mut cardinality = cardinality;

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
            states.values_mut().for_each(|x| x.sort());
            labels = states.keys().cloned().collect();
            cardinality = states.values().map(|x| x.len()).collect();
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
        let cardinality = cardinality;

        // Make conditioning states mutable.
        let mut conditioning_labels = conditioning_labels;
        let mut conditioning_states = conditioning_states;
        let mut conditioning_cardinality = conditioning_cardinality;

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
            conditioning_states.values_mut().for_each(|x| x.sort());
            conditioning_labels = conditioning_states.keys().cloned().collect();
            conditioning_cardinality = conditioning_states.values().map(|x| x.len()).collect();
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
        let conditioning_cardinality = conditioning_cardinality;

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
        let multi_index = MI::new(cardinality.iter().copied());
        // Compute the conditioning multi index.
        let conditioning_multi_index = MI::new(conditioning_cardinality.iter().copied());

        // Get the shape of the parameters.
        let shape = parameters.shape();
        // Compute the parameters size.
        let parameters_size = shape[0] * shape[1] * shape[2].saturating_sub(1);

        Self {
            labels,
            states,
            cardinality,
            multi_index,
            conditioning_labels,
            conditioning_states,
            conditioning_cardinality,
            conditioning_multi_index,
            parameters,
            parameters_size,
            sample_size: None,
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

    /// Returns the cardinality of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioned variable.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
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

    /// Returns the cardinality of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_cardinality(&self) -> &Array1<usize> {
        &self.conditioning_cardinality
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

    /// Returns the sample size of the dataset used to fit the distribution, if any.
    ///
    /// # Note
    ///
    /// The sample size could be non-integer if the distribution was fitted using a weighted dataset.
    ///
    /// # Returns
    ///
    /// The sample size of the dataset used to fit the distribution.
    ///
    #[inline]
    pub const fn sample_size(&self) -> Option<f64> {
        self.sample_size
    }

    /// Returns the sample log-likelihood of the dataset given the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample log-likelihood of the dataset given the distribution.
    ///
    #[inline]
    pub const fn sample_log_likelihood(&self) -> Option<f64> {
        self.sample_log_likelihood
    }

    /// Creates a new categorical conditional intensity matrix.
    ///
    /// # Arguments
    ///
    /// * `states` - The variables states.
    /// * `parameters` - The intensity matrices of the states.
    /// * `sample_size` - The sample size of the dataset used to fit the distribution, if any.
    /// * `sample_log_likelihood` - The sample log-likelihood of the dataset given the distribution, if any.
    ///
    /// # Panics
    ///
    /// See `new` method for panics.
    ///
    /// # Returns
    ///
    /// A new `CatCIM` instance.
    ///
    pub fn with_sample_size(
        states: States,
        conditioning_states: States,
        parameters: Array3<f64>,
        sample_size: Option<f64>,
        sample_log_likelihood: Option<f64>,
    ) -> Self {
        // Assert the sample size is finite and non-negative.
        sample_size.inspect(|&x| {
            assert!(
                x.is_finite() && x >= 0.,
                "Sample size must be finite and non-negative: \n\
                \t expected: sample_size >= 0, \n\
                \t found:    sample_size == {x} ."
            )
        });
        // Assert the sample log-likelihood is finite.
        sample_log_likelihood.inspect(|&x| {
            assert!(
                x.is_finite(),
                "Sample log-likelihood must be finite: \n\
                \t expected: sample_ll is finite, \n\
                \t found:    sample_ll is {x} ."
            )
        });

        // Construct the CIM.
        let mut cim = Self::new(states, conditioning_states, parameters);

        // Set the sample size and log-likelihood.
        cim.sample_size = sample_size;
        cim.sample_log_likelihood = sample_log_likelihood;

        cim
    }
}

impl PartialEq for CatCIM {
    fn eq(&self, other: &Self) -> bool {
        // Check for equality, excluding the sample values.
        self.labels.eq(&other.labels)
            && self.states.eq(&other.states)
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
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
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
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
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
            && self.multi_index.eq(&other.multi_index)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl CPD for CatCIM {
    type Parameters = Array3<f64>;
    type SS = (Array3<f64>, Array2<f64>, f64);

    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
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
}

impl Serialize for CatCIM {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the elements to serialize.
        let mut size = 3;
        size += self.sample_size.is_some() as usize;
        size += self.sample_log_likelihood.is_some() as usize;
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(size))?;
        // Serialize states.
        map.serialize_entry("states", &self.states)?;
        // Serialize conditioning states.
        map.serialize_entry("conditioning_states", &self.conditioning_states)?;
        // Serialize parameters.
        map.serialize_entry("parameters", &self.parameters)?;
        // Serialize sample size, if any.
        if let Some(sample_size) = self.sample_size {
            map.serialize_entry("sample_size", &sample_size)?;
        }
        // Serialize sample log likelihood, if any.
        if let Some(sample_log_likelihood) = self.sample_log_likelihood {
            map.serialize_entry("sample_log_likelihood", &sample_log_likelihood)?;
        }
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
            SampleSize,
            SampleLogLikelihood,
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
                let mut sample_size = None;
                let mut sample_log_likelihood = None;

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
                        Field::SampleSize => {
                            if sample_size.is_some() {
                                return Err(E::duplicate_field("sample_size"));
                            }
                            sample_size = Some(map.next_value()?);
                        }
                        Field::SampleLogLikelihood => {
                            if sample_log_likelihood.is_some() {
                                return Err(E::duplicate_field("sample_log_likelihood"));
                            }
                            sample_log_likelihood = Some(map.next_value()?);
                        }
                    }
                }

                // Check required fields.
                let states = states.ok_or_else(|| E::missing_field("states"))?;
                let conditioning_states =
                    conditioning_states.ok_or_else(|| E::missing_field("conditioning_states"))?;
                let parameters = parameters.ok_or_else(|| E::missing_field("parameters"))?;

                Ok(CatCIM::with_sample_size(
                    states,
                    conditioning_states,
                    parameters,
                    sample_size,
                    sample_log_likelihood,
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "states",
            "conditioning_states",
            "parameters",
            "sample_size",
            "sample_log_likelihood",
        ];

        deserializer.deserialize_struct("CatCIM", FIELDS, CatCIMVisitor)
    }
}

// Implement `JsonIO` for `CatCIM`.
impl_json_io!(CatCIM);
