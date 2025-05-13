use approx::relative_eq;
use ndarray::prelude::*;

use crate::{
    datasets::CatTrjEvT,
    types::{FxIndexMap, FxIndexSet},
};

/// Categorical evidence type.
#[derive(Clone, Debug)]
pub enum CategoricalEvidenceType {
    /// Certain positive evidence.
    CertainPositive {
        /// The state of the evidence.
        state: usize,
    },
    /// Certain negative evidence.
    CertainNegative {
        /// The states of the evidence.
        not_states: FxIndexSet<usize>,
    },
    /// Uncertain positive evidence.
    UncertainPositive {
        /// The probabilities of the states.
        p_states: Array1<f64>,
    },
    /// Uncertain negative evidence.
    UncertainNegative {
        /// The probabilities of the states.
        p_not_states: Array1<f64>,
    },
}

/// A type alias for the categorical evidence type.
pub type CatEvT = CategoricalEvidenceType;

impl From<CatTrjEvT> for CategoricalEvidenceType {
    fn from(evidence: CatTrjEvT) -> Self {
        // Get shortened variable types.
        use CatEvT as U;
        use CatTrjEvT as T;
        // Match the evidence type discard the temporal information.
        match evidence {
            T::CertainPositiveInterval { state, .. } => U::CertainPositive { state },
            T::CertainNegativeInterval { not_states, .. } => U::CertainNegative { not_states },
            T::UncertainPositiveInterval { p_states, .. } => U::UncertainPositive { p_states },
            T::UncertainNegativeInterval { p_not_states, .. } => {
                U::UncertainNegative { p_not_states }
            }
        }
    }
}

/// Categorical evidence structure.
pub struct CategoricalEvidence {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    evidences: FxIndexMap<String, Option<CatEvT>>,
}

/// A type alias for the categorical evidence structure.
pub type CatEv = CategoricalEvidence;

impl CategoricalEvidence {
    /// Creates a new categorical evidence structure.
    ///
    /// # Arguments
    ///
    /// * `states` - A collection of states, where each state is a tuple of a string and an iterator of strings.
    /// * `values` - A collection of values, where each value is a tuple of a string and a categorical evidence type.
    ///
    /// # Returns
    ///
    /// A new categorical evidence structure.
    ///
    pub fn new<I, J, K, L, M, N>(states: I, values: M) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = L>,
        K: AsRef<str>,
        L: AsRef<str>,
        M: IntoIterator<Item = (N, CatEvT)>,
        N: AsRef<str>,
    {
        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the variables.
        let mut states: FxIndexMap<_, _> = states
            .into_iter()
            .inspect(|_| n += 1)
            .map(|(label, states)| {
                // Convert the variable label to a string.
                let label = label.as_ref().to_owned();

                // Initialize states counter.
                let mut n = 0;
                // Convert the variable states to a set of strings.
                let states: FxIndexSet<_> = states
                    .into_iter()
                    .inspect(|_| n += 1)
                    .map(|x| x.as_ref().to_owned())
                    .collect();
                // Assert unique states.
                assert_eq!(states.len(), n, "Variables states must be unique.");

                (label, states)
            })
            .collect();

        // Assert unique labels.
        assert_eq!(states.len(), n, "Variables labels must be unique.");

        // Get the indices to sort the labels and states labels.
        let mut indices: Vec<(_, Vec<_>)> = states
            .values()
            .enumerate()
            .map(|(label_idx, states)| {
                // Allocate the indices of the states labels.
                let mut states_idx: Vec<_> = (0..states.len()).collect();
                // Sort the indices by the states labels.
                states_idx.sort_by_key(|&i| &states[i]);

                (label_idx, states_idx)
            })
            .collect();
        // Sort the indices by the states labels.
        indices.sort_by_key(|&(i, _)| states.get_index(i).unwrap().0);
        // Sort the states labels.
        states.values_mut().for_each(|states| states.sort());
        // Sort the labels.
        states.sort_keys();

        // Get the sorted labels.
        let labels = states.keys().cloned().collect();
        // Get the cardinality of the states.
        let cardinality = Array::from_iter(states.values().map(|x| x.len()));

        // Get shortened variable type.
        use CatEvT as E;

        // Allocate evidences.
        let mut evidences: FxIndexMap<_, Option<_>> =
            states.keys().map(|label| (label.clone(), None)).collect();

        // Iterate over the values and insert them into the events map using sorted indices.
        values.into_iter().for_each(|(label, evidence)| {
            // Convert the label to a string.
            let label = label.as_ref().to_owned();
            // Get the variable index.
            let variable = states
                .get_index_of(&label)
                .expect("Variable label not found in states.");
            // Sort variable index.
            let (variable, states) = &indices[variable];

            // Sort the variable states.
            let e = match evidence {
                E::CertainPositive { state, .. } => {
                    // Sort the variable states.
                    let state = states[state];
                    // Construct the sorted evidence.
                    E::CertainPositive { state }
                }
                E::CertainNegative { not_states, .. } => {
                    // Sort the variable states.
                    let not_states = not_states.iter().map(|state| states[*state]).collect();
                    // Construct the sorted evidence.
                    E::CertainNegative { not_states }
                }
                E::UncertainPositive { p_states, .. } => {
                    // Allocate new variable states.
                    let mut new_p_states = Array::zeros(p_states.len());
                    // Sort the variable states.
                    p_states.indexed_iter().for_each(|(i, &p)| {
                        new_p_states[states[i]] = p;
                    });
                    // Substitute the sorted states.
                    let p_states = new_p_states;
                    // Construct the sorted evidence.
                    E::UncertainPositive { p_states }
                }
                E::UncertainNegative { p_not_states, .. } => {
                    // Allocate new variable states.
                    let mut new_p_not_states = Array::zeros(p_not_states.len());
                    // Sort the variable states.
                    p_not_states.indexed_iter().for_each(|(i, &p)| {
                        new_p_not_states[states[i]] = p;
                    });
                    // Substitute the sorted states.
                    let p_not_states = new_p_not_states;
                    // Construct the sorted evidence.
                    E::UncertainNegative { p_not_states }
                }
            };

            // Push the value into the variable events.
            evidences[*variable] = Some(e);
        });

        // For each variable ...
        for (i, evidence) in evidences.values_mut().enumerate() {
            // Assert states distributions have the correct size.
            assert!(
                evidence.as_ref().is_none_or(|e| match e {
                    E::CertainPositive { .. } => true,
                    E::CertainNegative { .. } => true,
                    E::UncertainPositive { p_states, .. } => {
                        p_states.len() == cardinality[i]
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        p_not_states.len() == cardinality[i]
                    }
                }),
                "States distributions must have the correct size."
            );
            // Assert states distributions are not negative.
            assert!(
                evidence.as_ref().is_none_or(|e| match e {
                    E::CertainPositive { .. } => true,
                    E::CertainNegative { .. } => true,
                    E::UncertainPositive { p_states, .. } => {
                        p_states.iter().all(|&x| x >= 0.0)
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        p_not_states.iter().all(|&x| x >= 0.0)
                    }
                }),
                "States distributions must be non-negative."
            );
            // Assert states distributions sum to 1.
            assert!(
                evidence.as_ref().is_none_or(|e| match e {
                    E::CertainPositive { .. } => true,
                    E::CertainNegative { .. } => true,
                    E::UncertainPositive { p_states, .. } => {
                        relative_eq!(p_states.sum(), 1.)
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        relative_eq!(p_not_states.sum(), 1.)
                    }
                }),
                "States distributions must sum to 1."
            );
        }

        Self {
            labels,
            states,
            cardinality,
            evidences,
        }
    }

    /// The labels of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the labels of the evidence.
    ///
    #[inline]
    pub const fn labels(&self) -> &FxIndexSet<String> {
        &self.labels
    }

    /// The states of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the states of the evidence.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// The cardinality of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the cardinality of the evidence.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }

    /// The evidences of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the evidences of the evidence.
    ///
    #[inline]
    pub const fn evidences(&self) -> &FxIndexMap<String, Option<CatEvT>> {
        &self.evidences
    }
}
