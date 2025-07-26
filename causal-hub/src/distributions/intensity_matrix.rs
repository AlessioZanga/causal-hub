use approx::{AbsDiffEq, RelativeEq, relative_eq};
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::CPD;
use crate::{
    types::{EPSILON, Labels, Set, States},
    utils::{MI, collect_states},
};

/// A struct representing a categorical conditional intensity matrix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalConditionalIntensityMatrix {
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
    // Fitted statistics.
    sample_size: Option<f64>,
    sample_log_likelihood: Option<f64>,
}

/// A type alias for the categorical conditional intensity matrix.
pub type CatCIM = CategoricalConditionalIntensityMatrix;

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
    /// FIXME: Add panics.
    ///
    /// # Returns
    ///
    /// A new `CatCIM` instance.
    ///
    pub fn new(states: States, conditioning_states: States, parameters: Array3<f64>) -> Self {
        /* FIXME:

        // Unpack label and states.
        let (label, states) = state;
        // Convert variable label to a string.
        let label = label.as_ref().to_owned();

        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the variable.
        let mut states: Set<_> = states
            .into_iter()
            .inspect(|_| n += 1)
            .map(|state| state.as_ref().to_owned())
            .collect();

        // Assert unique labels.
        assert_eq!(states.len(), n, "Variables states must be unique.");

        // Get the states cardinality.
        let cardinality = states.len();

        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the conditioning variables.
        let mut conditioning_states = collect_states(conditioning_states);
        // Get the labels of the variables.
        let mut conditioning_labels: Set<_> = conditioning_states.keys().cloned().collect();

        // Get the cardinality of the set of states.
        let conditioning_cardinality: Array1<_> =
            conditioning_states.values().map(|i| i.len()).collect();

        // Get the shape of the parameters.
        let shape = parameters.shape();

        // Check if the number of states of the first variable matches the number of columns.
        assert_eq!(
            shape[1],
            states.len(),
            "Number of states must match the number of the second shape."
        );
        // Check if the number of states of the first variable matches the number of columns.
        assert_eq!(
            shape[2],
            states.len(),
            "Number of states must match the number of the third shape."
        );
        // Check if the product of the number of states of the remaining variables matches the number of rows.
        assert_eq!(
            shape[0],
            conditioning_cardinality.iter().product::<usize>(),
            "Product of the number of conditioning states must match the first shape."
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

        // Compute the parameters size.
        let parameters_size = shape[0] * shape[1] * shape[2].saturating_sub(1);

        // FIXME: Sort states and labels.

        // Construct the ravel multi index.
        let multi_index = MI::new(conditioning_cardinality.iter().copied());

        // Debug assert to check the sorting of the labels.
        debug_assert!(
            states.iter().is_sorted(),
            "Conditioned states must be sorted."
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

        */

        // FIXME: This is a temporary solution to avoid the above commented code.
        let labels = states.keys().cloned().collect();
        let cardinality: Array1<_> = states.values().map(|x| x.len()).collect();
        let multi_index = MI::new(cardinality.iter().copied());
        let conditioning_labels = conditioning_states.keys().cloned().collect();
        let conditioning_cardinality: Array1<_> =
            conditioning_states.values().map(|x| x.len()).collect();
        let conditioning_multi_index = MI::new(conditioning_cardinality.iter().copied());
        let shape = parameters.shape();
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
