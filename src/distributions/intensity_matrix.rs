use ndarray::prelude::*;

use crate::types::{FxIndexMap, FxIndexSet};

use super::CPD;

/// A struct representing a categorical conditional intensity matrix.
#[derive(Clone, Debug)]
pub struct CategoricalConditionalIntensityMatrix {
    // Labels of the conditioned variable.
    label: String,
    states: FxIndexSet<String>,
    cardinality: usize,
    // Labels of the conditioning variables.
    conditioning_labels: FxIndexSet<String>,
    conditioning_states: FxIndexMap<String, FxIndexSet<String>>,
    conditioning_cardinality: Array1<usize>,
    // Parameters.
    parameters: Array3<f64>,
    parameters_size: usize,
    // Fitted statistics.
    sample_size: Option<usize>,
    sample_log_likelihood: Option<f64>,
}

/// A type alias for the categorical conditional intensity matrix.
pub type CategoricalCIM = CategoricalConditionalIntensityMatrix;

impl CategoricalCIM {
    pub fn new<I, J, K, L, M, N, O>(
        state: (L, I),
        conditioning_states: J,
        parameters: Array3<f64>,
    ) -> Self
    where
        I: IntoIterator<Item = M>,
        J: IntoIterator<Item = (N, K)>,
        K: IntoIterator<Item = O>,
        L: Into<String>,
        M: Into<String>,
        N: Into<String>,
        O: Into<String>,
    {
        // Unpack label and states.
        let (label, states) = state;
        // Convert variable label to a string.
        let label = label.into();

        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the variable.
        let mut states: FxIndexSet<_> = states
            .into_iter()
            .inspect(|_| n += 1)
            .map(|state| state.into())
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
                let _label = _label.into();
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
                    .map(|x| x.into())
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

        // FIXME: Check parameters are normalized.

        // Compute the parameters size.
        let parameters_size = shape[0] * shape[1] * shape[2].saturating_sub(1);

        // FIXME: Sort states and labels.

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

    /// Returns the sample size of the dataset used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample size of the dataset used to fit the distribution.
    ///
    #[inline]
    pub const fn sample_size(&self) -> Option<usize> {
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

    pub fn with_sample_size<I, J, K, L, M, N, O>(
        state: (L, I),
        conditioning_states: J,
        parameters: Array3<f64>,
        sample_size: Option<usize>,
        sample_log_likelihood: Option<f64>,
    ) -> Self
    where
        I: IntoIterator<Item = M>,
        J: IntoIterator<Item = (N, K)>,
        K: IntoIterator<Item = O>,
        L: Into<String>,
        M: Into<String>,
        N: Into<String>,
        O: Into<String>,
    {
        // Construct the CIM.
        let mut cim = Self::new(state, conditioning_states, parameters);
        // Set the sample size and log-likelihood.
        cim.sample_size = sample_size;
        cim.sample_log_likelihood = sample_log_likelihood;

        cim
    }
}

impl CPD for CategoricalCIM {
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
