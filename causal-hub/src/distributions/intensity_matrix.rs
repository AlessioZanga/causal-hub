use approx::{AbsDiffEq, RelativeEq, relative_eq};
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::CPD;
use crate::{
    types::{FxIndexMap, FxIndexSet},
    utils::RMI,
};

/// A struct representing a categorical conditional intensity matrix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalConditionalIntensityMatrix {
    // Labels of the conditioned variable.
    label: String,
    states: FxIndexSet<String>,
    cardinality: usize,
    // Labels of the conditioning variables.
    conditioning_labels: FxIndexSet<String>,
    conditioning_states: FxIndexMap<String, FxIndexSet<String>>,
    conditioning_cardinality: Array1<usize>,
    // Ravel multi index.
    ravel_multi_index: RMI,
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
    pub fn new<I, J, K, L, M, N, O>(
        state: (L, I),
        conditioning_states: J,
        parameters: Array3<f64>,
    ) -> Self
    where
        I: IntoIterator<Item = M>,
        J: IntoIterator<Item = (N, K)>,
        K: IntoIterator<Item = O>,
        L: AsRef<str>,
        M: AsRef<str>,
        N: AsRef<str>,
        O: AsRef<str>,
    {
        // Unpack label and states.
        let (label, states) = state;
        // Convert variable label to a string.
        let label = label.as_ref().to_owned();

        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the variable.
        let mut states: FxIndexSet<_> = states
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
        let mut conditioning_states: FxIndexMap<_, _> = conditioning_states
            .into_iter()
            .inspect(|_| n += 1)
            .map(|(_label, _states)| {
                // Convert the variable label to a string.
                let _label = _label.as_ref().to_owned();
                // Assert conditioned variable is not a conditioning variable.
                assert_ne!(
                    _label, label,
                    "Conditioned variable cannot be a conditioning variable."
                );
                // Initialize states counter.
                let mut n = 0;
                // Convert the variable states to a set of strings.
                let _states: FxIndexSet<_> = _states
                    .into_iter()
                    .inspect(|_| n += 1)
                    .map(|x| x.as_ref().to_owned())
                    .collect();
                // Assert unique states.
                assert_eq!(_states.len(), n, "Variables states must be unique.");

                (_label, _states)
            })
            .collect();

        // Assert unique labels.
        assert_eq!(
            conditioning_states.len(),
            n,
            "Variables labels must be unique."
        );

        // Get the labels of the variables.
        let mut conditioning_labels: FxIndexSet<_> = conditioning_states.keys().cloned().collect();

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
            conditioning_cardinality.iter().product(),
            "Product of the number of conditioning states must match the first shape."
        );

        // Check parameters validity.
        parameters.outer_iter().for_each(|q| {
            // Assert Q is square.
            assert_eq!(q.nrows(), q.ncols(), "Q must be square.");
            // Assert Q has non-positive diagonal.
            assert!(
                q.diag().iter().all(|&x| x <= 0.0),
                "Q diagonal must be non-positive."
            );
            // Assert Q has non-negative off-diagonal.
            assert!(
                q.indexed_iter().all(|((i, j), &x)| i == j || x >= 0.0),
                "Q off-diagonal must be non-negative."
            );
            // Assert Q rows sum to zero.
            assert!(
                q.rows().into_iter().all(|x| relative_eq!(x.sum(), 0.)),
                "Q rows must sum to zero."
            );
        });

        // Compute the parameters size.
        let parameters_size = shape[0] * shape[1] * shape[2].saturating_sub(1);

        // FIXME: Sort states and labels.

        // Construct the ravel multi index.
        let ravel_multi_index = RMI::new(conditioning_cardinality.iter().copied());

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

        Self {
            label,
            states,
            cardinality,
            conditioning_labels,
            conditioning_states,
            conditioning_cardinality,
            ravel_multi_index,
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
    pub const fn states(&self) -> &FxIndexSet<String> {
        &self.states
    }

    /// Returns the cardinality of the conditioned variable.
    ///
    /// # Returns
    ///
    /// The cardinality of the conditioned variable.
    ///
    #[inline]
    pub const fn cardinality(&self) -> usize {
        self.cardinality
    }

    /// Returns the states of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The states of the conditioning variables.
    ///
    #[inline]
    pub const fn conditioning_states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
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
    pub const fn ravel_multi_index(&self) -> &RMI {
        &self.ravel_multi_index
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
    pub fn with_sample_size<I, J, K, L, M, N, O>(
        state: (L, I),
        conditioning_states: J,
        parameters: Array3<f64>,
        sample_size: Option<f64>,
        sample_log_likelihood: Option<f64>,
    ) -> Self
    where
        I: IntoIterator<Item = M>,
        J: IntoIterator<Item = (N, K)>,
        K: IntoIterator<Item = O>,
        L: AsRef<str>,
        M: AsRef<str>,
        N: AsRef<str>,
        O: AsRef<str>,
    {
        // Construct the CIM.
        let mut cim = Self::new(state, conditioning_states, parameters);
        // Set the sample size and log-likelihood.
        cim.sample_size = sample_size;
        cim.sample_log_likelihood = sample_log_likelihood;

        cim
    }
}

impl PartialEq for CatCIM {
    fn eq(&self, other: &Self) -> bool {
        // Check for equality, excluding the sample values.
        self.label.eq(&other.label)
            && self.states.eq(&other.states)
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
            && self.ravel_multi_index.eq(&other.ravel_multi_index)
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
        self.label.eq(&other.label)
            && self.states.eq(&other.states)
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
            && self.ravel_multi_index.eq(&other.ravel_multi_index)
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
        self.label.eq(&other.label)
            && self.states.eq(&other.states)
            && self.cardinality.eq(&other.cardinality)
            && self.conditioning_labels.eq(&other.conditioning_labels)
            && self.conditioning_states.eq(&other.conditioning_states)
            && self
                .conditioning_cardinality
                .eq(&other.conditioning_cardinality)
            && self.ravel_multi_index.eq(&other.ravel_multi_index)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl CPD for CatCIM {
    type Label = String;
    type ConditioningLabels = FxIndexSet<String>;
    type Parameters = Array3<f64>;

    #[inline]
    fn label(&self) -> &Self::Label {
        &self.label
    }

    #[inline]
    fn conditioning_labels(&self) -> &Self::ConditioningLabels {
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
