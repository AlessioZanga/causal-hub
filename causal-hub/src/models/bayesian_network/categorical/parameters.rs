use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use approx::{AbsDiffEq, RelativeEq, relative_eq};
use itertools::Itertools;
use ndarray::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, weighted::WeightedIndex};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::{CatSample, CatType},
    impl_json_io,
    models::{CPD, CatPhi, Labelled, Phi},
    types::{EPSILON, Labels, Set, States},
    utils::MI,
};

/// Sample (sufficient) statistics for the categorical CPD.
#[derive(Clone, Debug)]
pub struct CatCPDS {
    /// Conditional counts |Z| x |X|.
    n_xz: Array2<f64>,
    /// Sample size.
    n: f64,
}

impl CatCPDS {
    /// Creates a new sample (sufficient) statistics for the categorical CPD.
    ///
    /// # Arguments
    ///
    /// * `n_xz` - The conditional counts |Z| x |X|.
    /// * `n` - The sample size.
    ///
    /// # Returns
    ///
    /// A new sample (sufficient) statistics instance.
    ///
    #[inline]
    pub fn new(n_xz: Array2<f64>, n: f64) -> Self {
        // Assert the counts are finite and non-negative.
        assert!(
            n_xz.iter().all(|&x| x.is_finite() && x >= 0.),
            "Counts must be finite and non-negative."
        );
        assert!(
            n.is_finite() && n >= 0.,
            "Sample size must be finite and non-negative."
        );

        Self { n_xz, n }
    }

    /// Returns the sample conditional counts |Z| x |X|.
    ///
    /// # Returns
    ///
    /// The sample conditional counts.
    ///
    #[inline]
    pub const fn sample_conditional_counts(&self) -> &Array2<f64> {
        &self.n_xz
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

impl AddAssign for CatCPDS {
    fn add_assign(&mut self, other: Self) {
        // Add the counts.
        self.n_xz += &other.n_xz;
        // Add the sample sizes.
        self.n += other.n;
    }
}

impl Add for CatCPDS {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Serialize for CatCPDS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(2))?;
        // Convert the sample conditional counts to a flat format.
        let sample_conditional_counts: Vec<Vec<f64>> =
            self.n_xz.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize sample conditional counts.
        map.serialize_entry("sample_conditional_counts", &sample_conditional_counts)?;
        // Serialize sample size.
        map.serialize_entry("sample_size", &self.n)?;
        // End the map.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatCPDS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            SampleConditionalCounts,
            SampleSize,
        }

        struct CatCPDSVisitor;

        impl<'de> Visitor<'de> for CatCPDSVisitor {
            type Value = CatCPDS;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatCPDS")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CatCPDS, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate the fields.
                let mut sample_conditional_counts = None;
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
                        Field::SampleSize => {
                            if sample_size.is_some() {
                                return Err(E::duplicate_field("sample_size"));
                            }
                            sample_size = Some(map.next_value()?);
                        }
                    }
                }

                // Extract the fields.
                let sample_conditional_counts = sample_conditional_counts
                    .ok_or_else(|| E::missing_field("sample_conditional_counts"))?;
                let sample_size = sample_size.ok_or_else(|| E::missing_field("sample_size"))?;

                // Convert sample conditional counts to ndarray.
                let sample_conditional_counts = {
                    let counts: Vec<Vec<f64>> = sample_conditional_counts;
                    let shape = (counts.len(), counts[0].len());
                    Array::from_iter(counts.into_iter().flatten())
                        .into_shape_with_order(shape)
                        .map_err(|_| E::custom("Invalid sample conditional counts shape"))?
                };

                Ok(CatCPDS::new(sample_conditional_counts, sample_size))
            }
        }

        const FIELDS: &[&str] = &["sample_conditional_counts", "sample_size"];

        deserializer.deserialize_struct("CatCPDS", FIELDS, CatCPDSVisitor)
    }
}

/// A categorical CPD.
#[derive(Clone, Debug)]
pub struct CatCPD {
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
    parameters: Array2<f64>,
    parameters_size: usize,
    // Sample (sufficient) statistics, if any.
    sample_statistics: Option<CatCPDS>,
    sample_log_likelihood: Option<f64>,
}

impl CatCPD {
    /// Creates a new categorical conditional probability distribution.
    ///
    /// # Arguments
    ///
    /// * `states` - The variable label and states.
    /// * `conditioning_states` - The conditioning variables labels and states.
    /// * `parameters` - The probabilities of the states.
    ///
    /// # Panics
    ///
    /// * If the labels and conditioning labels are not disjoint.
    /// * If the product of the shape of the of states does not match the number of columns.
    /// * If the product of the shape of the of conditioning states does not match the number of rows.
    /// * If the parameters do not sum to one by row, unless empty.
    ///
    /// # Returns
    ///
    /// A new `CatCPD` instance.
    ///
    pub fn new(states: States, conditioning_states: States, parameters: Array2<f64>) -> Self {
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
            parameters.is_empty() || parameters.ncols() == shape.product(),
            "Product of the number of states must match the number of columns: \n\
            \t expected:    parameters.ncols() == {} , \n\
            \t found:       parameters.ncols() == {} .",
            shape.product(),
            parameters.ncols(),
        );

        // Get the shape of the set of states.
        let conditioning_shape = Array::from_iter(conditioning_states.values().map(Set::len));

        // Check that the product of the conditioning shape matches the number of rows.
        assert!(
            parameters.is_empty() || parameters.nrows() == conditioning_shape.product(),
            "Product of the number of conditioning states must match the number of rows: \n\
            \t expected:    parameters.nrows() == {} , \n\
            \t found:       parameters.nrows() == {} .",
            conditioning_shape.product(),
            parameters.nrows(),
        );

        // Check parameters validity.
        parameters
            .sum_axis(Axis(1))
            .iter()
            .enumerate()
            .for_each(|(i, &x)| {
                if !relative_eq!(x, 1.0, epsilon = EPSILON) {
                    panic!("Failed to sum probability to one: {}.", parameters.row(i));
                }
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
            let mut sorted_row_idx: Vec<_> = (0..parameters.ncols()).collect();
            // Sort the row indices.
            sorted_row_idx.sort_by_key(|&i| &sorted_states_idx[i]);
            // Sort the labels.
            states.sort_keys();
            states.values_mut().for_each(Set::sort);
            labels = states.keys().cloned().collect();
            shape = states.values().map(|x| x.len()).collect();
            // Allocate new parameters.
            let mut new_parameters = parameters.clone();
            // Sort the values by multi indices.
            new_parameters
                .columns_mut()
                .into_iter()
                .enumerate()
                .for_each(|(i, mut new_parameters_col)| {
                    // Assign the sorted values to the new values array.
                    new_parameters_col.assign(&parameters.column(sorted_row_idx[i]));
                });
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
            let mut sorted_row_idx: Vec<_> = (0..parameters.nrows()).collect();
            // Sort the row indices.
            sorted_row_idx.sort_by_key(|&i| &sorted_states_idx[i]);
            // Sort the labels.
            conditioning_states.sort_keys();
            conditioning_states.values_mut().for_each(Set::sort);
            conditioning_labels = conditioning_states.keys().cloned().collect();
            conditioning_shape = conditioning_states.values().map(|x| x.len()).collect();
            // Allocate new parameters.
            let mut new_parameters = parameters.clone();
            // Sort the values by multi indices.
            new_parameters.rows_mut().into_iter().enumerate().for_each(
                |(i, mut new_parameters_row)| {
                    // Assign the sorted values to the new values array.
                    new_parameters_row.assign(&parameters.row(sorted_row_idx[i]));
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
        // Compute the parameters size.
        let parameters_size = parameters.ncols().saturating_sub(1) * parameters.nrows();

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

    /// Creates a new categorical conditional probability distribution with optional fields.
    ///
    /// # Arguments
    ///
    /// * `states` - The variables states.
    /// * `parameters` - The probabilities of the states.
    /// * `statistics` - The sufficient statistics used to fit the distribution, if any.
    ///
    /// # Panics
    ///
    /// See `new` method for panics.
    ///
    /// # Returns
    ///
    /// A new `CatCPD` instance.
    ///
    pub fn with_optionals(
        state: States,
        conditioning_states: States,
        parameters: Array2<f64>,
        sample_statistics: Option<CatCPDS>,
        sample_log_likelihood: Option<f64>,
    ) -> Self {
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
        // Assert the sample log-likelihood is finite and non-positive.
        if let Some(sample_log_likelihood) = &sample_log_likelihood {
            assert!(
                sample_log_likelihood.is_finite() && *sample_log_likelihood <= 0.,
                "Sample log-likelihood must be finite and non-positive: \n\
                \t expected: sample_ll <= 0 , \n\
                \t found:    sample_ll == {sample_log_likelihood} ."
            );
        }

        // Construct the categorical CPD.
        let mut cpd = Self::new(state, conditioning_states, parameters);

        // FIXME: Check labels alignment with optional fields.

        // Set the optionals.
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
    pub fn from_phi(phi: CatPhi, x: &Set<usize>, z: &Set<usize>) -> Self {
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
    pub fn into_phi(self) -> CatPhi {
        CatPhi::from_cpd(self)
    }
}

impl Labelled for CatCPD {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl PartialEq for CatCPD {
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

impl AbsDiffEq for CatCPD {
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

impl RelativeEq for CatCPD {
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

impl CPD for CatCPD {
    type Support = CatSample;
    type Parameters = Array2<f64>;
    type Statistics = CatCPDS;

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

        // No variables.
        if n == 0 {
            return 1.;
        }

        // Convert states to indices.
        let x = match n {
            // ... one variable.
            1 => x[0] as usize,
            // ... multiple variables.
            _ => {
                // Convert states to indices.
                let x = x.iter().map(|&x| x as usize);
                // Ravel the variables.
                self.multi_index.ravel(x)
            }
        };

        // Convert conditioning states to indices.
        let z = match m {
            // ... no conditioning variables.
            0 => 0,
            // ... one conditioning variable.
            1 => z[0] as usize,
            // ... multiple conditioning variables.
            _ => {
                // Convert conditioning states to indices.
                let z = z.iter().map(|&z| z as usize);
                // Ravel the conditioning variables.
                self.conditioning_multi_index.ravel(z)
            }
        };

        // Get the probability.
        self.parameters[[z, x]]
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

        // No variables.
        if n == 0 {
            return array![];
        }

        // Convert conditioning states to indices.
        let z = match m {
            // ... no conditioning variables.
            0 => 0,
            // ... one conditioning variable.
            1 => z[0] as usize,
            // ... multiple conditioning variables.
            _ => {
                // Convert conditioning states to indices.
                let z = z.iter().map(|&z| z as usize);
                // Ravel the conditioning variables.
                self.conditioning_multi_index.ravel(z)
            }
        };

        // Get the distribution of the vertex.
        let p = self.parameters.row(z);
        // Construct the sampler.
        let s = WeightedIndex::new(&p).unwrap();
        // Sample from the distribution.
        let x = s.sample(rng);

        // Convert indices to states.
        match n {
            // ... one variable.
            1 => array![x as CatType],
            // ... multiple variables.
            _ => {
                // Unravel the sample.
                let x = self.multi_index.unravel(x);
                // Convert indices to states.
                let x = x.iter().map(|&x| x as CatType);
                // Return the sample.
                x.collect()
            }
        }
    }
}

impl Display for CatCPD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: This assumes `x` has a single element.
        assert_eq!(self.labels().len(), 1);

        // Determine the maximum width for formatting based on the labels and states.
        let n = std::iter::once(&self.labels()[0])
            .chain(&self.states()[0])
            .chain(self.conditioning_labels())
            .chain(self.conditioning_states().values().flatten())
            .map(|x| x.len())
            .max()
            .unwrap_or(0)
            .max(8);
        // Get the number of variables to condition on.
        let z = self.conditioning_shape().len();
        // Get the number of states for the first variable.
        let s = self.shape()[0];

        // Create a horizontal line for table formatting.
        let hline = "-".repeat((n + 3) * (z + s) + 1);
        writeln!(f, "{hline}")?;

        // Create the header row for the table.
        let header = std::iter::repeat_n("", z) // Empty columns for the conditioning variables.
            .chain([self.labels()[0].as_str()]) // Label for the first variable.
            .chain(std::iter::repeat_n("", s.saturating_sub(1))) // Empty columns for remaining states.
            .map(|x| format!("{x:n$}")) // Format each column with fixed width.
            .join(" | ");
        writeln!(f, "| {header} |")?;

        // Create a separator row for the table.
        let separator = std::iter::repeat_n("-".repeat(n), z + s).join(" | ");
        writeln!(f, "| {separator} |")?;

        // Create the second header row with labels and states.
        let header = self
            .conditioning_labels()
            .iter()
            .chain(&self.states()[0]) // Include states of the first variable.
            .map(|x| format!("{x:n$}")) // Format each column with fixed width.
            .join(" | ");
        writeln!(f, "| {header} |")?;
        writeln!(f, "| {separator} |")?;

        // Iterate over the Cartesian product of states and parameter rows.
        for (states, values) in self
            .conditioning_states()
            .values()
            .multi_cartesian_product()
            .zip(self.parameters().rows())
        {
            // Format the states for the current row.
            let states = states.iter().map(|x| format!("{x:n$}"));
            // Format the parameter values for the current row.
            let values = values.iter().map(|x| format!("{x:n$.6}"));
            // Join the states and values for the current row.
            let states_values = states.chain(values).join(" | ");
            writeln!(f, "| {states_values} |")?;
        }

        // Write the closing horizontal line for the table.
        writeln!(f, "{hline}")?;

        Ok(())
    }
}

impl Serialize for CatCPD {
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

        // Serialize states.
        map.serialize_entry("states", &self.states)?;
        // Serialize conditioning states.
        map.serialize_entry("conditioning_states", &self.conditioning_states)?;

        // Convert parameters to a flat format.
        let parameters: Vec<Vec<f64>> = self
            .parameters
            .rows()
            .into_iter()
            .map(|x| x.to_vec())
            .collect();
        // Serialize parameters.
        map.serialize_entry("parameters", &parameters)?;

        // Serialize the sufficient statistics, if any.
        if let Some(sample_statistics) = &self.sample_statistics {
            map.serialize_entry("sample_statistics", sample_statistics)?;
        }
        // Serialize the sample log-likelihood, if any.
        if let Some(sample_log_likelihood) = &self.sample_log_likelihood {
            map.serialize_entry("sample_log_likelihood", sample_log_likelihood)?;
        }

        // Serialize type.
        map.serialize_entry("type", "catcpd")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatCPD {
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

        struct CatCPDVisitor;

        impl<'de> Visitor<'de> for CatCPDVisitor {
            type Value = CatCPD;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatCPD")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CatCPD, V::Error>
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
                assert_eq!(type_, "catcpd", "Invalid type for CatCPD.");

                // Convert parameters to ndarray.
                let parameters: Vec<Vec<f64>> = parameters;
                let shape = (parameters.len(), parameters[0].len());
                let parameters = Array::from_iter(parameters.into_iter().flatten())
                    .into_shape_with_order(shape)
                    .map_err(|_| E::custom("Invalid parameters shape"))?;

                Ok(CatCPD::with_optionals(
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

        deserializer.deserialize_struct("CatCPD", FIELDS, CatCPDVisitor)
    }
}

// Implement `JsonIO` for `CatCPD`.
impl_json_io!(CatCPD);
